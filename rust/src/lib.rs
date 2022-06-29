#[macro_use]
extern crate log;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::str::FromStr;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};

use mimalloc::MiMalloc;
use serde::{de, Deserialize};
use tokio::runtime::Runtime;

use crate::common::cipher::Aes128Ctr;
use crate::common::net::get_interface_addr;
use crate::common::net::proto::ProtocolMode;

use anyhow::Error;
extern crate android_logger;
extern crate jni;
use jni::objects::{JClass, JObject, JValue, JString};
use jni::sys::{jint, jstring};
use jni::JNIEnv;
use log::Level;
use std::ffi::CString;
use std::os::raw::c_char;
use std::result::Result::Err;
pub type Callback = unsafe extern "C" fn(*const c_char) -> ();
use android_logger::Config;

mod client;
mod common;
mod tun;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Deserialize, Clone)]
struct TunIpAddr {
    ip: Ipv4Addr,
    netmask: Ipv4Addr,
}

#[derive(Deserialize, Clone)]
struct NetworkRange {
    server_addr: String,
    tun: TunIpAddr,
    key: String,
    mode: Option<String>,
    lan_ip_addr: Option<IpAddr>,
    try_send_to_lan_addr: Option<bool>,
}

#[derive(Deserialize, Clone)]
struct ClientConfig {
    mtu: Option<usize>,
    channel_limit: Option<usize>,
    api_addr: Option<SocketAddr>,
    tcp_heartbeat_interval_secs: Option<u64>,
    udp_heartbeat_interval_secs: Option<u64>,
    reconnect_interval_secs: Option<u64>,
    udp_socket_recv_buffer_size: Option<usize>,
    udp_socket_send_buffer_size: Option<usize>,
    tun_handler_thread_count: Option<usize>,
    udp_handler_thread_count: Option<usize>,
    network_ranges: Vec<NetworkRange>,
    raw_fd: std::os::raw::c_int,
}

#[derive(Clone)]
struct NetworkRangeFinalize {
    server_addr: String,
    tun: TunIpAddr,
    key: Aes128Ctr,
    mode: ProtocolMode,
    lan_ip_addr: Option<IpAddr>,
    try_send_to_lan_addr: bool,
}

#[derive(Clone)]
struct ClientConfigFinalize {
    mtu: usize,
    channel_limit: usize,
    api_addr: SocketAddr,
    tcp_heartbeat_interval: Duration,
    udp_heartbeat_interval: Duration,
    reconnect_interval: Duration,
    udp_socket_recv_buffer_size: Option<usize>,
    udp_socket_send_buffer_size: Option<usize>,
    tun_handler_thread_count: usize,
    udp_handler_thread_count: usize,
    network_ranges: Vec<NetworkRangeFinalize>,
    raw_fd: std::os::raw::c_int,
}

impl TryFrom<ClientConfig> for ClientConfigFinalize {
    type Error = anyhow::Error;

    fn try_from(config: ClientConfig) -> Result<Self> {
        let mut ranges = Vec::with_capacity(config.network_ranges.len());

        for range in config.network_ranges {
            let mode = ProtocolMode::from_str(range.mode.as_deref().unwrap_or("UDP_AND_TCP"))?;

            let resolve_server_addr = range
                .server_addr
                .to_socket_addrs()?
                .next()
                .ok_or_else(|| anyhow!("Server host not found"))?;

            let lan_ip_addr = match range.lan_ip_addr {
                None => {
                    if mode.udp_support() {
                        let lan_addr = get_interface_addr(resolve_server_addr)?;
                        Some(lan_addr)
                    } else {
                        None
                    }
                }
                Some(addr) => {
                    if addr.is_loopback() {
                        return Err(anyhow!("LAN address cannot be a loopback address"));
                    }

                    if addr.is_unspecified() {
                        return Err(anyhow!("LAN address cannot be unspecified address"));
                    }
                    Some(addr)
                }
            };

            let range_finalize = NetworkRangeFinalize {
                server_addr: {
                    if resolve_server_addr.ip().is_loopback() {
                        return Err(anyhow!("Server address cannot be a loopback address"));
                    }
                    range.server_addr
                },
                tun: range.tun,
                key: Aes128Ctr::new(range.key.as_bytes()),
                mode,
                lan_ip_addr,
                try_send_to_lan_addr: range.try_send_to_lan_addr.unwrap_or(false),
            };
            ranges.push(range_finalize)
        }

        let config_finalize = ClientConfigFinalize {
            mtu: config.mtu.unwrap_or(1462),
            channel_limit: config.channel_limit.unwrap_or(100),
            api_addr: config
                .api_addr
                .unwrap_or_else(|| SocketAddr::from((Ipv4Addr::LOCALHOST, 3030))),
            tcp_heartbeat_interval: config
                .tcp_heartbeat_interval_secs
                .map(|sec| Duration::from_secs(ternary!(sec > 10, 10, sec)))
                .unwrap_or(Duration::from_secs(5)),
            udp_heartbeat_interval: config
                .udp_heartbeat_interval_secs
                .map(|sec| Duration::from_secs(ternary!(sec > 10, 10, sec)))
                .unwrap_or(Duration::from_secs(5)),
            reconnect_interval: Duration::from_secs(config.reconnect_interval_secs.unwrap_or(3)),
            udp_socket_recv_buffer_size: config.udp_socket_recv_buffer_size,
            udp_socket_send_buffer_size: config.udp_socket_send_buffer_size,
            tun_handler_thread_count: config.tun_handler_thread_count.unwrap_or(1),
            udp_handler_thread_count: config.udp_handler_thread_count.unwrap_or(1),
            network_ranges: ranges,
            raw_fd: config.raw_fd,
        };
        Ok(config_finalize)
    }
}

fn load_config_from_string<T: de::DeserializeOwned>(jsonStr: &str) -> Result<T> {
    serde_json::from_str(jsonStr).context("Failed to parse string config")
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn invokeCallbackViaJNA(callback: Callback) {
    let s = CString::new("Hello from Rust").unwrap();
    unsafe {
        callback(s.as_ptr());
    }
}

#[no_mangle]
pub extern "C" fn Java_com_fubukidaze_vpn_NativeVpnService_TestJni(
    env: JNIEnv,
    _: JObject,
) -> jstring {
    env.new_string("Hello from Rust")
        .expect("Couldn't create java string!")
        .into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_fubukidaze_vpn_NativeVpnService_invokeCallbackViaJNI(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) {
    let s = String::from("Hello from Rust");
    let response = env.new_string(&s).expect("Couldn't create java string!");
    env.call_method(
        callback,
        "callback",
        "(Ljava/lang/String;)V",
        &[JValue::from(JObject::from(response))],
    )
    .unwrap();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_fubukidaze_vpn_NativeVpnService_DestoryFubukiClient(
    env: JNIEnv,
    _: JObject,
) {
    // TODO
    return;
}

#[derive(Deserialize, Debug)]
struct DazeConfig {
    lanIpAddr: String,
    localIp: String,
    serverIp: String,
    serverPort: String,
    localRoute: String,
    key: String,
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_fubukidaze_vpn_NativeVpnService_LaunchFubukiClient(
    env: JNIEnv,
    _: JObject,
    fds: jint,
    fconfig: JString,
) {
    // let cfg = env.get_field(obj, "config", "Ljava/lang/String;")
    android_logger::init_once(Config::default().with_min_level(Level::Debug));

    debug!("Fubuki Lanuch Client");

    let dzcfgStr = env.get_string(fconfig);
    match dzcfgStr {
        Ok(dzcfgStr) => {
            let dzcfg: Result<DazeConfig, Error> = serde_json::from_str(dzcfgStr.to_str().unwrap()).context("Failed to parse string dzconfig");
            match dzcfg {
                Ok(dzcfg) => {
                    debug!("Cfg from rn {}", dzcfgStr.to_str().unwrap());
                    let jsonStr = format!(
                        "{{\"raw_fd\":{},\"mtu\":1462,\"channel_limit\":100,\"api_addr\":\"127.0.0.1:3030\",\"tcp_heartbeat_interval_secs\":5,\"udp_heartbeat_interval_secs\":5,\"reconnect_interval_secs\":3,\"udp_socket_recv_buffer_size\":8192,\"udp_socket_send_buffer_size\":8192,\"tun_handler_thread_count\":1,\"udp_handler_thread_count\":1,\"network_ranges\":[{{\"server_addr\":\"{}:{}\",\"tun\":{{\"ip\":\"{}\",\"netmask\":\"255.255.255.0\"}},\"key\":\"{}\",\"mode\":\"TCP_ONLY\",\"lan_ip_addr\":\"{}\",\"try_send_to_lan_addr\":false}}]}}",
                         fds,
                          dzcfg.serverIp, 
                          dzcfg.serverPort, 
                          dzcfg.localIp,
                          dzcfg.key, 
                          dzcfg.localIp
                        );
        
        
                        let config: Result<ClientConfig, Error> = load_config_from_string(jsonStr.as_str());
        
                        debug!("Fubuki Get Tun fd {}", &fds);
                        match config {
                            Ok(v) => {
                                debug!("Fubuki Parsed Client Config");
                    
                                let clCfg = ClientConfigFinalize::try_from(v);
                                match clCfg {
                                    Ok(cv) => {
                                        debug!("Fubuki Loaded Client Config");
                    
                                        let ctx = Runtime::new().context("Failed start runtime");
                    
                                        match ctx {
                                            Ok(ctx) => {
                                                debug!("Fubuki Start Runtime");
                    
                                                let res = ctx.block_on(client::start(cv));
                                                ctx.shutdown_background();
                                                match res {
                                                    Ok(_) => {
                                                        debug!("Fubuki Start Client");
                                                    }
                                                    Err(e) => {
                                                        error!("Fubuki Start Err: {:?}", e);
                                                        env.throw(("java/lang/Exception", e.to_string())).unwrap();
                                                    }
                                                }
                                            }
                                            Err(_) => {
                                                env.throw(("java/lang/Exception", "Start Runtime Failed!"))
                                                    .unwrap();
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        env.throw(("java/lang/Exception", "Load Config Failed!"))
                                            .unwrap();
                                    }
                                }
                            }
                            Err(_) => {
                                env.throw(("java/lang/Exception", "Parse Config Failed!"))
                                    .unwrap();
                            }
                        }
                }
                Err(_) => {
                    env.throw(("java/lang/Exception", "Bad dzcfg!")).unwrap();
                }
            }
        },
        Err(_) => {
            env.throw(("java/lang/Exception", "Bad jni string dzcfg!")).unwrap();
        },
    }
}
