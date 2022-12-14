/* automatically generated by rust-bindgen 0.59.2 */

pub const MAX_ARGUMENTS: u32 = 6;
pub const SDTError_t_noError: SDTError_t = -1;
pub const SDTError_t_elfCreationError: SDTError_t = 0;
pub const SDTError_t_tmpCreationError: SDTError_t = 1;
pub const SDTError_t_sharedLibraryOpenError: SDTError_t = 2;
pub const SDTError_t_symbolLoadingError: SDTError_t = 3;
pub const SDTError_t_sharedLibraryCloseError: SDTError_t = 4;
pub type SDTError_t = ::std::os::raw::c_int;
pub const ArgType_t_noarg: ArgType_t = 0;
pub const ArgType_t_uint8: ArgType_t = 1;
pub const ArgType_t_int8: ArgType_t = -1;
pub const ArgType_t_uint16: ArgType_t = 2;
pub const ArgType_t_int16: ArgType_t = -2;
pub const ArgType_t_uint32: ArgType_t = 4;
pub const ArgType_t_int32: ArgType_t = -4;
pub const ArgType_t_uint64: ArgType_t = 8;
pub const ArgType_t_int64: ArgType_t = -8;
pub type ArgType_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDTProbe {
    pub name: *mut ::std::os::raw::c_char,
    pub argFmt: [ArgType_t; 6usize],
    pub _fire: *mut ::std::os::raw::c_void,
    pub provider: *mut SDTProvider,
    pub argCount: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_SDTProbe() {
    assert_eq!(
        ::std::mem::size_of::<SDTProbe>(),
        56usize,
        concat!("Size of: ", stringify!(SDTProbe))
    );
    assert_eq!(
        ::std::mem::align_of::<SDTProbe>(),
        8usize,
        concat!("Alignment of ", stringify!(SDTProbe))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProbe>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProbe),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProbe>())).argFmt as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProbe),
            "::",
            stringify!(argFmt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProbe>()))._fire as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProbe),
            "::",
            stringify!(_fire)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProbe>())).provider as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProbe),
            "::",
            stringify!(provider)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProbe>())).argCount as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProbe),
            "::",
            stringify!(argCount)
        )
    );
}
pub type SDTProbe_t = SDTProbe;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDTProbeList_ {
    pub probe: SDTProbe_t,
    pub next: *mut SDTProbeList_,
}
#[test]
fn bindgen_test_layout_SDTProbeList_() {
    assert_eq!(
        ::std::mem::size_of::<SDTProbeList_>(),
        64usize,
        concat!("Size of: ", stringify!(SDTProbeList_))
    );
    assert_eq!(
        ::std::mem::align_of::<SDTProbeList_>(),
        8usize,
        concat!("Alignment of ", stringify!(SDTProbeList_))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProbeList_>())).probe as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProbeList_),
            "::",
            stringify!(probe)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProbeList_>())).next as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProbeList_),
            "::",
            stringify!(next)
        )
    );
}
pub type SDTProbeList_t = SDTProbeList_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SDTProvider {
    pub name: *mut ::std::os::raw::c_char,
    pub probes: *mut SDTProbeList_t,
    pub errno: SDTError_t,
    pub error: *mut ::std::os::raw::c_char,
    pub _handle: *mut ::std::os::raw::c_void,
    pub _filename: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_SDTProvider() {
    assert_eq!(
        ::std::mem::size_of::<SDTProvider>(),
        48usize,
        concat!("Size of: ", stringify!(SDTProvider))
    );
    assert_eq!(
        ::std::mem::align_of::<SDTProvider>(),
        8usize,
        concat!("Alignment of ", stringify!(SDTProvider))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProvider>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProvider),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProvider>())).probes as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProvider),
            "::",
            stringify!(probes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProvider>())).errno as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProvider),
            "::",
            stringify!(errno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProvider>())).error as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProvider),
            "::",
            stringify!(error)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProvider>()))._handle as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProvider),
            "::",
            stringify!(_handle)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SDTProvider>()))._filename as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SDTProvider),
            "::",
            stringify!(_filename)
        )
    );
}
pub type SDTProvider_t = SDTProvider;
extern "C" {
    pub fn providerInit(name: *const ::std::os::raw::c_char) -> *mut SDTProvider_t;
}
extern "C" {
    pub fn providerAddProbe(
        provider: *mut SDTProvider_t,
        name: *const ::std::os::raw::c_char,
        argCount: ::std::os::raw::c_int,
        ...
    ) -> *mut SDTProbe_t;
}
extern "C" {
    pub fn providerLoad(provider: *mut SDTProvider_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn providerUnload(provider: *mut SDTProvider_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn providerDestroy(provider: *mut SDTProvider_t);
}
extern "C" {
    pub fn probeFire(probe: *mut SDTProbe_t, ...);
}
extern "C" {
    pub fn probeIsEnabled(probe: *mut SDTProbe_t) -> ::std::os::raw::c_int;
}
