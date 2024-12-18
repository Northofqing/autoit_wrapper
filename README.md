# Examples
```rust
use autoit_wrapper::AutoIt;

fn main() {
    AutoIt::init();

    let result = AutoIt::win_get_handle("MainWindow").unwrap();
    let exists_hwnd = AutoIt::win_exists_by_handle(result.clone());
    println!("{:?}", exists_hwnd.unwrap());
    let title = AutoIt::win_get_title_by_handle(result.clone()).unwrap();
    println!("{}", &title);
    let exists_wnd = AutoIt::win_exists(&title,"");
    println!("{:?}", exists_wnd.unwrap());
    let _ = AutoIt::mouse_click("left", 1, 2, 1, 200);
}
```
# AutoIT All API
```C++
AU3_API void WINAPI AU3_Init(void);
AU3_API int AU3_error(void);

AU3_API int WINAPI AU3_AutoItSetOption(LPCWSTR szOption, int nValue);

AU3_API void WINAPI AU3_ClipGet(LPWSTR szClip, int nBufSize);
AU3_API void WINAPI AU3_ClipPut(LPCWSTR szClip);
AU3_API int WINAPI AU3_ControlClick(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl, LPCWSTR szButton, int nNumClicks, int nX = AU3_IMagicDEFAULT, int nY = AU3_IMagicDEFAULT);
AU3_API int WINAPI AU3_ControlClickByHandle(HWND hWnd, HWND hCtrl, LPCWSTR szButton, int nNumClicks, int nX = AU3_IMagicDEFAULT, int nY = AU3_IMagicDEFAULT);
AU3_API void WINAPI AU3_ControlCommand(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl, LPCWSTR szCommand, LPCWSTR szExtra, LPWSTR szResult, int nBufSize);
AU3_API void WINAPI AU3_ControlCommandByHandle(HWND hWnd, HWND hCtrl, LPCWSTR szCommand, LPCWSTR szExtra, LPWSTR szResult, int nBufSize);
AU3_API void WINAPI AU3_ControlListView(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl, LPCWSTR szCommand, LPCWSTR szExtra1, LPCWSTR szExtra2, LPWSTR szResult, int nBufSize);
AU3_API void WINAPI AU3_ControlListViewByHandle(HWND hWnd, HWND hCtrl, LPCWSTR szCommand, LPCWSTR szExtra1, LPCWSTR szExtra2, LPWSTR szResult, int nBufSize);
AU3_API int WINAPI AU3_ControlDisable(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl);
AU3_API int WINAPI AU3_ControlDisableByHandle(HWND hWnd, HWND hCtrl);
AU3_API int WINAPI AU3_ControlEnable(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl);
AU3_API int WINAPI AU3_ControlEnableByHandle(HWND hWnd, HWND hCtrl);
AU3_API int WINAPI AU3_ControlFocus(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl);
AU3_API int WINAPI AU3_ControlFocusByHandle(HWND hWnd, HWND hCtrl);
AU3_API void WINAPI AU3_ControlGetFocus(LPCWSTR szTitle, LPCWSTR szText, LPWSTR szControlWithFocus, int nBufSize);
AU3_API void WINAPI AU3_ControlGetFocusByHandle(HWND hWnd, LPWSTR szControlWithFocus, int nBufSize);
AU3_API HWND WINAPI AU3_ControlGetHandle(HWND hWnd, LPCWSTR szControl);
AU3_API void WINAPI AU3_ControlGetHandleAsText(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, LPCWSTR szControl, LPWSTR szRetText, int nBufSize);
AU3_API int WINAPI AU3_ControlGetPos(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl, LPRECT lpRect);
AU3_API int WINAPI AU3_ControlGetPosByHandle(HWND hWnd, HWND hCtrl, LPRECT lpRect);
AU3_API void WINAPI AU3_ControlGetText(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl, LPWSTR szControlText, int nBufSize);
AU3_API void WINAPI AU3_ControlGetTextByHandle(HWND hWnd, HWND hCtrl, LPWSTR szControlText, int nBufSize);
AU3_API int WINAPI AU3_ControlHide(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl);
AU3_API int WINAPI AU3_ControlHideByHandle(HWND hWnd, HWND hCtrl);
AU3_API int WINAPI AU3_ControlMove(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl, int nX, int nY, int nWidth = -1, int nHeight = -1);
AU3_API int WINAPI AU3_ControlMoveByHandle(HWND hWnd, HWND hCtrl, int nX, int nY, int nWidth = -1, int nHeight = -1);
AU3_API int WINAPI AU3_ControlSend(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl, LPCWSTR szSendText, int nMode = 0);
AU3_API int WINAPI AU3_ControlSendByHandle(HWND hWnd, HWND hCtrl, LPCWSTR szSendText, int nMode = 0);
AU3_API int WINAPI AU3_ControlSetText(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl, LPCWSTR szControlText);
AU3_API int WINAPI AU3_ControlSetTextByHandle(HWND hWnd, HWND hCtrl, LPCWSTR szControlText);
AU3_API int WINAPI AU3_ControlShow(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl);
AU3_API int WINAPI AU3_ControlShowByHandle(HWND hWnd, HWND hCtrl);
AU3_API void WINAPI AU3_ControlTreeView(LPCWSTR szTitle, LPCWSTR szText, LPCWSTR szControl, LPCWSTR szCommand, LPCWSTR szExtra1, LPCWSTR szExtra2, LPWSTR szResult, int nBufSize);
AU3_API void WINAPI AU3_ControlTreeViewByHandle(HWND hWnd, HWND hCtrl, LPCWSTR szCommand, LPCWSTR szExtra1, LPCWSTR szExtra2, LPWSTR szResult, int nBufSize);

AU3_API void WINAPI AU3_DriveMapAdd(LPCWSTR szDevice, LPCWSTR szShare, int nFlags, /*[in,defaultvalue("")]*/LPCWSTR szUser, /*[in,defaultvalue("")]*/LPCWSTR szPwd, LPWSTR szResult, int nBufSize);
AU3_API int WINAPI AU3_DriveMapDel(LPCWSTR szDevice);
AU3_API void WINAPI AU3_DriveMapGet(LPCWSTR szDevice, LPWSTR szMapping, int nBufSize);

AU3_API int WINAPI AU3_IsAdmin(void);

AU3_API int WINAPI AU3_MouseClick(/*[in,defaultvalue("LEFT")]*/LPCWSTR szButton, int nX = AU3_IMagicDEFAULT, int nY = AU3_IMagicDEFAULT, int nClicks = 1, int nSpeed = -1);
AU3_API int WINAPI AU3_MouseClickDrag(LPCWSTR szButton, int nX1, int nY1, int nX2, int nY2, int nSpeed = -1);
AU3_API void WINAPI AU3_MouseDown(/*[in,defaultvalue("LEFT")]*/LPCWSTR szButton);
AU3_API int WINAPI AU3_MouseGetCursor(void);
AU3_API void WINAPI AU3_MouseGetPos(LPPOIMagic lpPoint);
AU3_API int WINAPI AU3_MouseMove(int nX, int nY, int nSpeed = -1);
AU3_API void WINAPI AU3_MouseUp(/*[in,defaultvalue("LEFT")]*/LPCWSTR szButton);
AU3_API void WINAPI AU3_MouseWheel(LPCWSTR szDirection, int nClicks);

AU3_API int WINAPI AU3_Opt(LPCWSTR szOption, int nValue);

AU3_API unsigned int WINAPI AU3_PixelChecksum(LPRECT lpRect, int nStep = 1);
AU3_API int WINAPI AU3_PixelGetColor(int nX, int nY);
AU3_API void WINAPI AU3_PixelSearch(LPRECT lpRect, int nCol, /*default 0*/int nVar, /*default 1*/int nStep, LPPOIMagic pPointResult);
AU3_API int WINAPI AU3_ProcessClose(LPCWSTR szProcess);
AU3_API int WINAPI AU3_ProcessExists(LPCWSTR szProcess);
AU3_API int WINAPI AU3_ProcessSetPriority(LPCWSTR szProcess, int nPriority);
AU3_API int WINAPI AU3_ProcessWait(LPCWSTR szProcess, int nTimeout = 0);
AU3_API int WINAPI AU3_ProcessWaitClose(LPCWSTR szProcess, int nTimeout = 0);

AU3_API int WINAPI AU3_Run(LPCWSTR szProgram, /*[in,defaultvalue("")]*/LPCWSTR szDir, int nShowFlag = SW_SHOWNORMAL);
AU3_API int WINAPI AU3_RunWait(LPCWSTR szProgram, /*[in,defaultvalue("")]*/LPCWSTR szDir, int nShowFlag = SW_SHOWNORMAL);
AU3_API int WINAPI AU3_RunAs(LPCWSTR szUser, LPCWSTR szDomain, LPCWSTR szPassword, int nLogonFlag, LPCWSTR szProgram, /*[in,defaultvalue("")]*/LPCWSTR szDir, int nShowFlag = SW_SHOWNORMAL);
AU3_API int WINAPI AU3_RunAsWait(LPCWSTR szUser, LPCWSTR szDomain, LPCWSTR szPassword, int nLogonFlag, LPCWSTR szProgram, /*[in,defaultvalue("")]*/LPCWSTR szDir, int nShowFlag = SW_SHOWNORMAL);

AU3_API void WINAPI AU3_Send(LPCWSTR szSendText, int nMode = 0);
AU3_API int WINAPI AU3_Shutdown(int nFlags);
AU3_API void WINAPI AU3_Sleep(int nMilliseconds);
AU3_API int WINAPI AU3_StatusbarGetText(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, /*[in,defaultvalue(1)]*/int nPart, LPWSTR szStatusText, int nBufSize);
AU3_API int WINAPI AU3_StatusbarGetTextByHandle(HWND hWnd, /*[in,defaultvalue(1)]*/int nPart, LPWSTR szStatusText, int nBufSize);

AU3_API void WINAPI AU3_ToolTip(LPCWSTR szTip, int nX = AU3_IMagicDEFAULT, int nY = AU3_IMagicDEFAULT);

AU3_API int WINAPI AU3_WinActivate(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText);
AU3_API int WINAPI AU3_WinActivateByHandle(HWND hWnd);
AU3_API int WINAPI AU3_WinActive(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText);
AU3_API int WINAPI AU3_WinActiveByHandle(HWND hWnd);
AU3_API int WINAPI AU3_WinClose(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText);
AU3_API int WINAPI AU3_WinCloseByHandle(HWND hWnd);
AU3_API int WINAPI AU3_WinExists(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText);
AU3_API int WINAPI AU3_WinExistsByHandle(HWND hWnd);
AU3_API int WINAPI AU3_WinGetCaretPos(LPPOIMagic lpPoint);
AU3_API void WINAPI AU3_WinGetClassList(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, LPWSTR szRetText, int nBufSize);
AU3_API void WINAPI AU3_WinGetClassListByHandle(HWND hWnd, LPWSTR szRetText, int nBufSize);
AU3_API int WINAPI AU3_WinGetClientSize(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, LPRECT lpRect);
AU3_API int WINAPI AU3_WinGetClientSizeByHandle(HWND hWnd, LPRECT lpRect);
AU3_API HWND WINAPI AU3_WinGetHandle(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText);
AU3_API void WINAPI AU3_WinGetHandleAsText(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, LPWSTR szRetText, int nBufSize);
AU3_API int WINAPI AU3_WinGetPos(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, LPRECT lpRect);
AU3_API int WINAPI AU3_WinGetPosByHandle(HWND hWnd, LPRECT lpRect);
AU3_API DWORD WINAPI AU3_WinGetProcess(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText);
AU3_API DWORD WINAPI AU3_WinGetProcessByHandle(HWND hWnd);
AU3_API int WINAPI AU3_WinGetState(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText);
AU3_API int WINAPI AU3_WinGetStateByHandle(HWND hWnd);
AU3_API void WINAPI AU3_WinGetText(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, LPWSTR szRetText, int nBufSize);
AU3_API void WINAPI AU3_WinGetTextByHandle(HWND hWnd, LPWSTR szRetText, int nBufSize);
AU3_API void WINAPI AU3_WinGetTitle(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, LPWSTR szRetText, int nBufSize);
AU3_API void WINAPI AU3_WinGetTitleByHandle(HWND hWnd, LPWSTR szRetText, int nBufSize);
AU3_API int WINAPI AU3_WinKill(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText);
AU3_API int WINAPI AU3_WinKillByHandle(HWND hWnd);
AU3_API int WINAPI AU3_WinMenuSelectItem(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, LPCWSTR szItem1, LPCWSTR szItem2, LPCWSTR szItem3, LPCWSTR szItem4, LPCWSTR szItem5, LPCWSTR szItem6, LPCWSTR szItem7, LPCWSTR szItem8);
AU3_API int WINAPI AU3_WinMenuSelectItemByHandle(HWND hWnd, LPCWSTR szItem1, LPCWSTR szItem2, LPCWSTR szItem3, LPCWSTR szItem4, LPCWSTR szItem5, LPCWSTR szItem6, LPCWSTR szItem7, LPCWSTR szItem8);
AU3_API void WINAPI AU3_WinMinimizeAll();
AU3_API void WINAPI AU3_WinMinimizeAllUndo();
AU3_API int WINAPI AU3_WinMove(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, int nX, int nY, int nWidth = -1, int nHeight = -1);
AU3_API int WINAPI AU3_WinMoveByHandle(HWND hWnd, int nX, int nY, int nWidth = -1, int nHeight = -1);
AU3_API int WINAPI AU3_WinSetOnTop(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, int nFlag);
AU3_API int WINAPI AU3_WinSetOnTopByHandle(HWND hWnd, int nFlag);
AU3_API int WINAPI AU3_WinSetState(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, int nFlags);
AU3_API int WINAPI AU3_WinSetStateByHandle(HWND hWnd, int nFlags);
AU3_API int WINAPI AU3_WinSetTitle(LPCWSTR szTitle,/*[in,defaultvalue("")]*/ LPCWSTR szText, LPCWSTR szNewTitle);
AU3_API int WINAPI AU3_WinSetTitleByHandle(HWND hWnd, LPCWSTR szNewTitle);
AU3_API int WINAPI AU3_WinSetTrans(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, int nTrans);
AU3_API int WINAPI AU3_WinSetTransByHandle(HWND hWnd, int nTrans);
AU3_API int WINAPI AU3_WinWait(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, int nTimeout = 0);
AU3_API int WINAPI AU3_WinWaitByHandle(HWND hWnd, int nTimeout);
AU3_API int WINAPI AU3_WinWaitActive(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, int nTimeout = 0);
AU3_API int WINAPI AU3_WinWaitActiveByHandle(HWND hWnd, int nTimeout);
AU3_API int WINAPI AU3_WinWaitClose(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, int nTimeout = 0);
AU3_API int WINAPI AU3_WinWaitCloseByHandle(HWND hWnd, int nTimeout);
AU3_API int WINAPI AU3_WinWaitNotActive(LPCWSTR szTitle, /*[in,defaultvalue("")]*/LPCWSTR szText, int nTimeout);
AU3_API int WINAPI AU3_WinWaitNotActiveByHandle(HWND hWnd, int nTimeout = 0);
```


### [only windows frameworke](https://www.autoitscript.com/trac/autoit/wiki/AutoItNotOnToDoList)