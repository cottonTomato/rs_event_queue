use std::ffi;

// FLAGS

// Actions
pub(crate) const EV_ADD: ffi::c_ushort = 0x0001;
// pub(crate) const EV_ENABLE: ffi::c_ushort = 0x0004;
// pub(crate) const EV_DISABLE: ffi::c_ushort = 0x0008;
// pub(crate) const EV_DELETE: ffi::c_ushort = 0x0002;

// Flags
// pub(crate) const EV_RECIEPT: ffi::c_ushort = 0x0040;
pub(crate) const EV_ONESHOT: ffi::c_ushort = 0x0010;
pub(crate) const EV_CLEAR: ffi::c_ushort = 0x0020;

// Returned Value
// pub(crate) const EV_EOF: ffi::c_ushort = 0x8000;
// pub(crate) const EV_OOBAND: ffi::c_ushort = 0x2000;
// pub(crate) const EV_ERROR: ffi::c_ushort = 0x4000;

// FILTERS

pub(crate) const EVFILT_READ: ffi::c_short = -1;
pub(crate) const EVFILT_WRITE: ffi::c_short = -2;

#[repr(C, align(4))]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct kevent_t {
    pub(crate) ident: ffi::c_ulong,
    pub(crate) filter: ffi::c_short,
    pub(crate) flags: ffi::c_ushort,
    pub(crate) fflags: ffi::c_uint,
    pub(crate) data: ffi::c_long,
    pub udata: *const ffi::c_void,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub(crate) struct timespec {
    pub(crate) tv_sec: ffi::c_long,
    pub(crate) tv_nsec: ffi::c_long,
}

#[link(name = "c")]
extern "C" {
    pub(crate) fn kqueue() -> ffi::c_int;

    pub(crate) fn kevent(
        kq: ffi::c_int,
        changelist: *const kevent_t,
        nchanges: ffi::c_int,
        eventlist: *mut kevent_t,
        nevents: ffi::c_int,
        timeout: *const timespec,
    ) -> ffi::c_int;

    pub(crate) fn close(fildes: ffi::c_int) -> ffi::c_int;
}
