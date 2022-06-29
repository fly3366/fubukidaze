use std::cell::UnsafeCell;
use std::io;
use std::io::{Error, ErrorKind, Result};
use std::io::{Read, Write};

use crate::tun::TunDevice;

pub struct AndroidTun {
    fd: UnsafeCell<tun::platform::Device>,
}

unsafe impl Sync for AndroidTun {}

impl AndroidTun {
    pub(super) fn create(fd: std::os::raw::c_int) -> Result<AndroidTun> {
        debug!("1111111111");
        let mut cfg = tun::Configuration::default();
        debug!("2222222222");
        cfg.raw_fd(fd);
        let device = tun::create(&cfg).map_err(|e| io::Error::new(io::ErrorKind::Other, e));

        debug!("333333333");
        match device {
            Ok(tund) => {
                Ok(AndroidTun {
                    fd: UnsafeCell::new(tund),
                })
            },
            Err(e) => {
                return Err(Error::new(ErrorKind::Other, e));
            }
        }

    }
}

impl TunDevice for AndroidTun {
    fn send_packet(&self, packet: &[u8]) -> Result<()> {
        let fd = unsafe { &mut *self.fd.get() };
        fd.write(packet)?;
        Ok(())
    }

    fn recv_packet(&self, buff: &mut [u8]) -> Result<usize> {
        let fd = unsafe { &mut *self.fd.get() };
        fd.read(buff)
    }
}
