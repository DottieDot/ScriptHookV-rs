use libc::c_void;

pub type PresentCallback =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;

pub type KeyboardHandler = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_uint,
        arg2: ::std::os::raw::c_ushort,
        arg3: ::std::os::raw::c_uchar,
        arg4: ::std::os::raw::c_char,
        arg5: ::std::os::raw::c_char,
        arg6: ::std::os::raw::c_char,
        arg7: ::std::os::raw::c_char,
    ),
>;

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GameVersion {
    VER_1_0_335_2_STEAM,
    VER_1_0_335_2_NOSTEAM,

    VER_1_0_350_1_STEAM,
    VER_1_0_350_2_NOSTEAM,

    VER_1_0_372_2_STEAM,
    VER_1_0_372_2_NOSTEAM,

    VER_1_0_393_2_STEAM,
    VER_1_0_393_2_NOSTEAM,

    VER_1_0_393_4_STEAM,
    VER_1_0_393_4_NOSTEAM,

    VER_1_0_463_1_STEAM,
    VER_1_0_463_1_NOSTEAM,

    VER_1_0_505_2_STEAM,
    VER_1_0_505_2_NOSTEAM,

    VER_1_0_573_1_STEAM,
    VER_1_0_573_1_NOSTEAM,

    VER_1_0_617_1_STEAM,
    VER_1_0_617_1_NOSTEAM,

    VER_SIZE,
    VER_UNK = -1,
}

#[link(name="./lib/ScriptHookV")]
extern "C" {
    #[link_name = "?createTexture@@YAHPEBD@Z"]
    pub fn createTexture(texFileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;

    #[link_name = "?drawTexture@@YAXHHHHMMMMMMMMMMMM@Z"]
    pub fn drawTexture(
        id: ::std::os::raw::c_int,
        index: ::std::os::raw::c_int,
        level: ::std::os::raw::c_int,
        time: ::std::os::raw::c_int,
        sizeX: f32,
        sizeY: f32,
        centerX: f32,
        centerY: f32,
        posX: f32,
        posY: f32,
        rotation: f32,
        screenHeightScaleFactor: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );

    #[link_name = "?presentCallbackRegister@@YAXP6AXPEAX@Z@Z"]
    pub fn presentCallbackRegister(cb: PresentCallback);

    #[link_name = "?presentCallbackUnregister@@YAXP6AXPEAX@Z@Z"]
    pub fn presentCallbackUnregister(cb: PresentCallback);

    #[link_name = "?keyboardHandlerRegister@@YAXP6AXKGEHHHH@Z@Z"]
    pub fn keyboardHandlerRegister(handler: KeyboardHandler);

    #[link_name = "?keyboardHandlerUnregister@@YAXP6AXKGEHHHH@Z@Z"]
    pub fn keyboardHandlerUnregister(handler: KeyboardHandler);

    #[link_name = "?scriptWait@@YAXK@Z"]
    pub fn scriptWait(time: u32);

    #[link_name = "?scriptRegister@@YAXPEAUHINSTANCE__@@P6AXXZ@Z"]
    pub fn scriptRegister(
        module: *mut c_void,
        LP_SCRIPT_MAIN: extern "C" fn(),
    );

    #[link_name = "?scriptRegisterAdditionalThread@@YAXPEAUHINSTANCE__@@P6AXXZ@Z"]
    pub fn scriptRegisterAdditionalThread(
        module: *mut c_void,
        LP_SCRIPT_MAIN: extern "C" fn(),
    );

    #[link_name = "?scriptUnregister@@YAXPEAUHINSTANCE__@@@Z"]
    pub fn scriptUnregister(module: *mut c_void);

    #[link_name = "?nativeInit@@YAX_K@Z"]
    pub fn nativeInit(hash: u64);

    #[link_name = "?nativePush64@@YAX_K@Z"]
    pub fn nativePush64(val: u64);

    #[link_name = "?nativeCall@@YAPEA_KXZ"]
    pub fn nativeCall() -> *mut u64;

    #[link_name = "?getGlobalPtr@@YAPEA_KH@Z"]
    pub fn getGlobalPtr(globalId: i32) -> *mut u64;

    #[link_name = "?worldGetAllVehicles@@YAHPEAHH@Z"]
    pub fn worldGetAllVehicles(
        arr: *mut i32,
        arrSize: i32,
    ) -> i32;

    #[link_name = "?worldGetAllPeds@@YAHPEAHH@Z"]
    pub fn worldGetAllPeds(
        arr: *mut i32,
        arrSize: i32,
    ) -> i32;

    #[link_name = "?worldGetAllObjects@@YAHPEAHH@Z"]
    pub fn worldGetAllObjects(
        arr: *mut i32,
        arrSize: i32,
    ) -> i32;

    #[link_name = "?worldGetAllPickups@@YAHPEAHH@Z"]
    pub fn worldGetAllPickups(
        arr: *mut i32,
        arrSize: i32,
    ) -> i32;

    #[link_name = "?getScriptHandleBaseAddress@@YAPEAEH@Z"]
    pub fn getScriptHandleBaseAddress(
        handle: i32,
    ) -> *mut u8;

    #[link_name = "?getGameVersion@@YA?AW4eGameVersion@@XZ"]
    pub fn getGameVersion() -> GameVersion;
}
