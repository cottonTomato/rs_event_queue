use crate::kqueue;
use std::{ffi, io, net::TcpStream, os::fd::AsRawFd, ptr};

type Events = Vec<kqueue::kevent_t>;

pub enum Intreasts {
    FdRead,
    FdWrite,
}

impl From<Intreasts> for (ffi::c_short, ffi::c_uint) {
    fn from(value: Intreasts) -> Self {
        match value {
            Intreasts::FdRead => (kqueue::EVFILT_READ, 0),
            Intreasts::FdWrite => (kqueue::EVFILT_WRITE, 0),
        }
    }
}

pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    pub fn register(
        &self,
        source: &TcpStream,
        token: usize,
        intreasts: Intreasts,
    ) -> io::Result<()> {
        let (filter, fflags) = intreasts.into();

        let change = kqueue::kevent_t {
            ident: source.as_raw_fd() as ffi::c_ulong,
            filter,
            flags: kqueue::EV_ADD | kqueue::EV_CLEAR | kqueue::EV_ONESHOT,
            fflags,
            data: 0,
            udata: token as *const ffi::c_void,
        };

        let res =
            unsafe { kqueue::kevent(self.raw_fd, &change, 1, ptr::null_mut(), 0, ptr::null()) };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let res = unsafe { kqueue::close(self.raw_fd) };
        if res < 0 {
            eprintln!("ERROR: {:?}", io::Error::last_os_error());
        }
    }
}

pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> io::Result<Self> {
        let kq = unsafe { kqueue::kqueue() };

        if kq < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            registry: Registry { raw_fd: kq },
        })
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn poll(
        &mut self,
        events: &mut Events,
        timeout: Option<std::time::Duration>,
    ) -> io::Result<()> {
        let timeout = timeout
            .map(|t| kqueue::timespec {
                tv_sec: t.as_secs() as ffi::c_long,
                tv_nsec: t.subsec_nanos() as ffi::c_long,
            })
            .map(|t| &t as *const kqueue::timespec)
            .unwrap_or(ptr::null());

        let res = unsafe {
            kqueue::kevent(
                self.registry.raw_fd,
                ptr::null(),
                0,
                events.as_mut_ptr(),
                events.capacity() as ffi::c_int,
                timeout,
            )
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        unsafe { events.set_len(res as usize) };

        Ok(())
    }
}
