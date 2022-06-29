use std::io::{Error, Result};
use std::sync::Arc;

mod android;

pub trait TunDevice: Send + Sync {
    fn send_packet(&self, packet: &[u8]) -> Result<()>;

    fn recv_packet(&self, buff: &mut [u8]) -> Result<usize>;
}

impl<T: TunDevice> TunDevice for Arc<T> {
    fn send_packet(&self, packet: &[u8]) -> Result<()> {
        (**self).send_packet(packet)
    }

    fn recv_packet(&self, buff: &mut [u8]) -> Result<usize> {
        (**self).recv_packet(buff)
    }
}

pub(crate) fn create_device_from_fd(fd: std::os::raw::c_int) -> Result<impl TunDevice> {
    android::AndroidTun::create(fd)
}

pub(crate) fn skip_error(err: &Error) -> bool {
    if cfg!(target_os = "linux") {
        const INVALID_ARGUMENT: i32 = 22;
        err.raw_os_error() == Some(INVALID_ARGUMENT)
    } else {
        false
    }
}
