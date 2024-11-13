use windows::core::*;
use windows::Win32::Foundation::*;

pub type AU3_Init = unsafe extern "system" fn();
pub type AU3_error = unsafe extern "system" fn() -> i32;

pub type AU3_AutoItSetOption = unsafe extern "system" fn(PCWSTR, i32) -> i32;
pub type AU3_ClipGet = unsafe extern "system" fn(PWSTR, i32);
pub type AU3_ClipPut = unsafe extern "system" fn(PCWSTR);
pub type AU3_ControlClick =
    unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR, PCWSTR, i32, i32, i32) -> i32;
pub type AU3_ControlClickByHandle = unsafe extern "system" fn(HWND, HWND, PCWSTR, i32, i32, i32) -> i32;
pub type AU3_ControlCommand =
    unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR, PCWSTR, PCWSTR, PWSTR, i32);
pub type AU3_ControlCommandByHandle = unsafe extern "system" fn(HWND, HWND, PCWSTR, PCWSTR, PWSTR, i32);
pub type AU3_ControlListView =
    unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR, PCWSTR, PCWSTR, PCWSTR, PWSTR, i32);
pub type AU3_ControlListViewByHandle =
    unsafe extern "system" fn(HWND, HWND, PCWSTR, PCWSTR, PCWSTR, PWSTR, i32);
pub type AU3_ControlDisable = unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR) -> i32;
pub type AU3_ControlDisableByHandle = unsafe extern "system" fn(HWND, HWND) -> i32;
pub type AU3_ControlEnable = unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR) -> i32;
pub type AU3_ControlEnableByHandle = unsafe extern "system" fn(HWND, HWND) -> i32;
pub type AU3_ControlFocus = unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR) -> i32;
pub type AU3_ControlFocusByHandle = unsafe extern "system" fn(HWND, HWND) -> i32;
pub type AU3_ControlGetFocus = unsafe extern "system" fn(PCWSTR, PCWSTR, PWSTR, i32);
pub type AU3_ControlGetFocusByHandle = unsafe extern "system" fn(HWND, PWSTR, i32);
pub type AU3_ControlGetHandle = unsafe extern "system" fn(HWND, PCWSTR) -> HWND;
pub type AU3_ControlGetHandleAsText =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, PCWSTR, PWSTR, i32);
pub type AU3_ControlGetPos = unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR, RECT) -> i32;
pub type AU3_ControlGetPosByHandle = unsafe extern "system" fn(HWND, HWND, RECT) -> i32;
pub type AU3_ControlGetText = unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR, PWSTR, i32);
pub type AU3_ControlGetTextByHandle = unsafe extern "system" fn(HWND, HWND, PWSTR, i32);
pub type AU3_ControlHide = unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR) -> i32;
pub type AU3_ControlHideByHandle = unsafe extern "system" fn(HWND, HWND) -> i32;
pub type AU3_ControlMove = unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR, i32, i32, i32, i32) -> i32;
pub type AU3_ControlMoveByHandle = unsafe extern "system" fn(HWND, HWND, i32, i32, i32, i32) -> i32;
pub type AU3_ControlSend = unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR, PCWSTR, i32) -> i32;
pub type AU3_ControlSendByHandle = unsafe extern "system" fn(HWND, HWND, PCWSTR, i32) -> i32;
pub type AU3_ControlSetText = unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR, PCWSTR) -> i32;
pub type AU3_ControlSetTextByHandle = unsafe extern "system" fn(HWND, HWND, PCWSTR) -> i32;
pub type AU3_ControlShow = unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR) -> i32;
pub type AU3_ControlShowByHandle = unsafe extern "system" fn(HWND, HWND) -> i32;
pub type AU3_ControlTreeView =
    unsafe extern "system" fn(PCWSTR, PCWSTR, PCWSTR, PCWSTR, PCWSTR, PCWSTR, PWSTR, i32);
pub type AU3_ControlTreeViewByHandle =
    unsafe extern "system" fn(HWND, HWND, PCWSTR, PCWSTR, PCWSTR, PWSTR, i32);

pub type AU3_DriveMapAdd = unsafe extern "system" fn(
    PCWSTR,
    PCWSTR,
    i32,
    /*[in,defaultvalue("")]*/ PCWSTR,
    /*[in,defaultvalue("")]*/ PCWSTR,
    PWSTR,
    i32,
);
pub type AU3_DriveMapDel = unsafe extern "system" fn(PCWSTR) -> i32;
pub type AU3_DriveMapGet = unsafe extern "system" fn(PCWSTR, PWSTR, i32);

pub type AU3_IsAdmin = unsafe extern "system" fn() -> i32;

pub type AU3_MouseClick =
    unsafe extern "system" fn(/*[in,defaultvalue("LEFT")]*/ PCWSTR, i32, i32, i32, i32) -> i32;
pub type AU3_MouseClickDrag = unsafe extern "system" fn(PCWSTR, i32, i32, i32, i32, i32) -> i32;
pub type AU3_MouseDown = unsafe extern "system" fn(/*[in,defaultvalue("LEFT")]*/ PCWSTR);
pub type AU3_MouseGetCursor = unsafe extern "system" fn() -> i32;
pub type AU3_MouseGetPos = unsafe extern "system" fn(POINT);
pub type AU3_MouseMove = unsafe extern "system" fn(i32, i32, i32) -> i32;
pub type AU3_MouseUp = unsafe extern "system" fn(/*[in,defaultvalue("LEFT")]*/ PCWSTR);
pub type AU3_MouseWheel = unsafe extern "system" fn(PCWSTR, i32);

pub type AU3_Opt = unsafe extern "system" fn(PCWSTR, i32) -> i32;

pub type AU3_PixelChecksum = unsafe extern "system" fn(RECT, i32) -> u32;
pub type AU3_PixelGetColor = unsafe extern "system" fn(i32, i32) -> i32;
pub type AU3_PixelSearch =
    unsafe extern "system" fn(RECT, i32, /*default 0*/ i32, /*default 1*/ i32, POINT);
pub type AU3_ProcessClose = unsafe extern "system" fn(PCWSTR) -> i32;
pub type AU3_ProcessExists = unsafe extern "system" fn(PCWSTR) -> i32;
pub type AU3_ProcessSetPriority = unsafe extern "system" fn(PCWSTR, i32) -> i32;
pub type AU3_ProcessWait = unsafe extern "system" fn(PCWSTR, i32) -> i32;
pub type AU3_ProcessWaitClose = unsafe extern "system" fn(PCWSTR, i32) -> i32;

pub type AU3_Run = unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, i32) -> i32;
pub type AU3_RunWait = unsafe extern "system" fn(
    PCWSTR,
    /*[in,defaultvaluedefaultvalue("")]*/ PCWSTR,
    i32,
) -> i32;
pub type AU3_RunAs = unsafe extern "system" fn(
    PCWSTR,
    PCWSTR,
    PCWSTR,
    i32,
    PCWSTR,
    /*[in,defaultvalue("")]*/ PCWSTR,
    i32,
) -> i32;
pub type AU3_RunAsWait = unsafe extern "system" fn(
    PCWSTR,
    PCWSTR,
    PCWSTR,
    i32,
    PCWSTR,
    /*[in,defaultvalue("")]*/ PCWSTR,
    i32,
) -> i32;

pub type AU3_Send = unsafe extern "system" fn(PCWSTR, i32);
pub type AU3_Shutdown = unsafe extern "system" fn(i32) -> i32;
pub type AU3_Sleep = unsafe extern "system" fn(i32);
pub type AU3_StatusbarGetText = unsafe extern "system" fn(
    PCWSTR,
    /*[in,defaultvalue("")]*/ PCWSTR,
    /*[in,defaultvalue(1)]*/ i32,
    PWSTR,
    i32,
) -> i32;
pub type AU3_StatusbarGetTextByHandle =
    unsafe extern "system" fn(HWND, /*[in,defaultvalue(1)]*/ i32, PWSTR, i32) -> i32;

pub type AU3_ToolTip = unsafe extern "system" fn(PCWSTR, i32, i32);

pub type AU3_WinActivate =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR) -> i32;
pub type AU3_WinActivateByHandle = unsafe extern "system" fn(HWND) -> i32;
pub type AU3_WinActive =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR) -> i32;
pub type AU3_WinActiveByHandle = unsafe extern "system" fn(HWND) -> i32;
pub type AU3_WinClose = unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR) -> i32;
pub type AU3_WinCloseByHandle = unsafe extern "system" fn(HWND) -> i32;
pub type AU3_WinExists =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR) -> i32;
pub type AU3_WinExistsByHandle = unsafe extern "system" fn(HWND) -> i32;
pub type AU3_WinGetCaretPos = unsafe extern "system" fn(POINT) -> i32;
pub type AU3_WinGetClassList =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, PWSTR, i32);
pub type AU3_WinGetClassListByHandle = unsafe extern "system" fn(HWND, PWSTR, i32);
pub type AU3_WinGetClientSize =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, RECT) -> i32;
pub type AU3_WinGetClientSizeByHandle = unsafe extern "system" fn(HWND, RECT) -> i32;
pub type AU3_WinGetHandle =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR) -> HWND;
pub type AU3_WinGetHandleAsText =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, PWSTR, i32);
pub type AU3_WinGetPos =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, RECT) -> i32;
pub type AU3_WinGetPosByHandle = unsafe extern "system" fn(HWND, RECT) -> i32;
pub type AU3_WinGetProcess =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR) -> u32; //DWORD
pub type AU3_WinGetProcessByHandle = unsafe extern "system" fn(HWND) -> u32; //DWORD
pub type AU3_WinGetState =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR) -> i32;
pub type AU3_WinGetStateByHandle = unsafe extern "system" fn(HWND) -> i32;
pub type AU3_WinGetText =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, PWSTR, i32);
pub type AU3_WinGetTextByHandle = unsafe extern "system" fn(HWND, PWSTR, i32);
pub type AU3_WinGetTitle =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, PWSTR, i32);
pub type AU3_WinGetTitleByHandle = unsafe extern "system" fn(HWND, PWSTR, i32);
pub type AU3_WinKill = unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR) -> i32;
pub type AU3_WinKillByHandle = unsafe extern "system" fn(HWND) -> i32;
pub type AU3_WinMenuSelectItem = unsafe extern "system" fn(
    PCWSTR,
    /*[in,defaultvalue("")]*/ PCWSTR,
    PCWSTR,
    PCWSTR,
    PCWSTR,
    PCWSTR,
    PCWSTR,
    PCWSTR,
    PCWSTR,
    PCWSTR,
) -> i32;
pub type AU3_WinMenuSelectItemByHandle = unsafe extern "system" fn(
    HWND,
    PCWSTR,
    PCWSTR,
    PCWSTR,
    PCWSTR,
    PCWSTR,
    PCWSTR,
    PCWSTR,
    PCWSTR,
) -> i32;
pub type AU3_WinMinimizeAll = unsafe extern "system" fn();
pub type AU3_WinMinimizeAllUndo = unsafe extern "system" fn();
pub type AU3_WinMove = unsafe extern "system" fn(
    PCWSTR,
    /*[in,defaultvalue("")]*/ PCWSTR,
    i32,
    i32,
    i32,
    i32,
) -> i32;
pub type AU3_WinMoveByHandle = unsafe extern "system" fn(HWND, i32, i32, i32, i32) -> i32;
pub type AU3_WinSetOnTop =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, i32) -> i32;
pub type AU3_WinSetOnTopByHandle = unsafe extern "system" fn(HWND, i32) -> i32;
pub type AU3_WinSetState =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, i32) -> i32;
pub type AU3_WinSetStateByHandle = unsafe extern "system" fn(HWND, i32) -> i32;
pub type AU3_WinSetTitle =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, PCWSTR) -> i32;
pub type AU3_WinSetTitleByHandle = unsafe extern "system" fn(HWND, PCWSTR) -> i32;
pub type AU3_WinSetTrans =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, i32) -> i32;
pub type AU3_WinSetTransByHandle = unsafe extern "system" fn(HWND, i32) -> i32;
pub type AU3_WinWait =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, i32) -> i32;
pub type AU3_WinWaitByHandle = unsafe extern "system" fn(HWND, i32) -> i32;
pub type AU3_WinWaitActive =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, i32) -> i32;
pub type AU3_WinWaitActiveByHandle = unsafe extern "system" fn(HWND, i32) -> i32;
pub type AU3_WinWaitClose =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, i32) -> i32;
pub type AU3_WinWaitCloseByHandle = unsafe extern "system" fn(HWND, i32) -> i32;
pub type AU3_WinWaitNotActive =
    unsafe extern "system" fn(PCWSTR, /*[in,defaultvalue("")]*/ PCWSTR, i32) -> i32;
pub type AU3_WinWaitNotActiveByHandle = unsafe extern "system" fn(HWND, i32) -> i32;