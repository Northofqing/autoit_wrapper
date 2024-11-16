#![allow(dead_code, non_camel_case_types)]
pub mod autoit;
pub mod autoit_error;

#[macro_use]
extern crate lazy_static;

use crate::autoit_error::AutoItError;
use crate::autoit_error::AutoItResult;
use libloading::{Library, Symbol};

use std::sync::Mutex;
use windows::core::*;
use windows::Win32::Foundation::*;

#[derive(Debug)]
struct AutoItCore {
    lib: Library,
    initialized: bool,
}
impl AutoItCore {
    fn new() -> AutoItResult<Self> {
        let lib = unsafe { Library::new("lib/AutoItX3_x64.dll")? };
        let mut autoit = AutoItCore {
            lib,
            initialized: false,
        };

        unsafe {
            let init: Symbol<autoit::AU3_Init> = autoit.lib.get(b"AU3_Init").unwrap();
            init();
            autoit.initialized = true;
        }

        Ok(autoit)
    }

    fn check_error(&self) -> AutoItResult<()> {
        unsafe {
            let error: Symbol<autoit::AU3_error> = self.lib.get(b"AU3_error")?;
            let error_code = error();
            if error_code != 0 {
                return Err(AutoItError::AutoItError(error_code));
            }
        }
        Ok(())
    }
}

lazy_static! {
    static ref AUTO_IT: Mutex<Option<AutoItCore>> = Mutex::new(None);
}

pub struct AutoIt;

impl AutoIt {
    pub fn init() -> AutoItResult<()> {
        let mut auto_it = AUTO_IT
            .lock()
            .map_err(|_| AutoItError::LockError("Failed to lock AutoIt"))?;
        if auto_it.is_none() {
            *auto_it = Some(AutoItCore::new()?);
        }
        Ok(())
    }

    fn get() -> AutoItResult<std::sync::MutexGuard<'static, Option<AutoItCore>>> {
        AUTO_IT
            .lock()
            .map_err(|_| AutoItError::LockError("Failed to lock AutoIt"))
    }

    fn to_wide_string(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }
    // 窗口操作 - 同时提供字符串和句柄两种方式
    pub fn win_exists(title: &str, text: &str) -> AutoItResult<bool> {
        let auto_it = Self::get()?;
        let auto_it = auto_it
            .as_ref()
            .ok_or(AutoItError::InitError("AutoIt not initialized"))?;

        let title_wide = Self::to_wide_string(title);
        let text_wide = Self::to_wide_string(text);

        unsafe {
            let win_exists: Symbol<autoit::AU3_WinExists> = auto_it.lib.get(b"AU3_WinExists")?;
            let result = win_exists(
                PCWSTR::from_raw(title_wide.as_ptr()),
                PCWSTR::from_raw(text_wide.as_ptr()),
            );
            auto_it.check_error()?;
            Ok(result == 1)
        }
    }

    pub fn win_exists_by_handle(hwnd: HWND) -> AutoItResult<bool> {
        let auto_it = Self::get()?;
        let auto_it = auto_it
            .as_ref()
            .ok_or(AutoItError::InitError("AutoIt not initialized"))?;

        unsafe {
            let win_exists: Symbol<autoit::AU3_WinExistsByHandle> =
                auto_it.lib.get(b"AU3_WinExistsByHandle")?;
            let result = win_exists(hwnd);
            auto_it.check_error()?;
            Ok(result == 1)
        }
    }

    pub fn win_get_handle(title: &str) -> AutoItResult<HWND> {
        let auto_it = Self::get()?;
        let auto_it = auto_it
            .as_ref()
            .ok_or(AutoItError::InitError("AutoIt not initialized"))?;

        let title_wide = Self::to_wide_string(title);
        let text_wide = Self::to_wide_string("");

        unsafe {
            let win_get_handle: Symbol<autoit::AU3_WinGetHandle> =
                auto_it.lib.get(b"AU3_WinGetHandle")?;
            let hwnd = win_get_handle(
                PCWSTR::from_raw(title_wide.as_ptr()),
                PCWSTR::from_raw(text_wide.as_ptr()),
            );
            auto_it.check_error()?;
            Ok(hwnd)
        }
    }

    // 控件操作 - 同时提供字符串和句柄两种方式
    pub fn control_click(
        title: &str,
        text: &str,
        control: &str,
        button: &str,
        clicks: i32,
        x: i32,
        y: i32,
    ) -> AutoItResult<bool> {
        let auto_it = Self::get()?;
        let auto_it = auto_it
            .as_ref()
            .ok_or(AutoItError::InitError("AutoIt not initialized"))?;

        let title_wide = Self::to_wide_string(title);
        let text_wide = Self::to_wide_string(text);
        let control_wide = Self::to_wide_string(control);
        let button_wide = Self::to_wide_string(button);

        unsafe {
            let control_click: Symbol<autoit::AU3_ControlClick> =
                auto_it.lib.get(b"AU3_ControlClick")?;
            let result = control_click(
                PCWSTR::from_raw(title_wide.as_ptr()),
                PCWSTR::from_raw(text_wide.as_ptr()),
                PCWSTR::from_raw(control_wide.as_ptr()),
                PCWSTR::from_raw(button_wide.as_ptr()),
                clicks,
                x,
                y,
            );
            auto_it.check_error()?;
            Ok(result == 1)
        }
    }

    pub fn control_click_by_handle(
        hwnd: HWND,
        control_hwnd: HWND,
        button: &str,
        clicks: i32,
        x: i32,
        y: i32,
    ) -> AutoItResult<bool> {
        let auto_it = Self::get()?;
        let auto_it = auto_it
            .as_ref()
            .ok_or(AutoItError::InitError("AutoIt not initialized"))?;

        let button_wide = Self::to_wide_string(button);

        unsafe {
            let control_click: Symbol<autoit::AU3_ControlClickByHandle> =
                auto_it.lib.get(b"AU3_ControlClickByHandle")?;
            let result = control_click(
                hwnd,
                control_hwnd,
                PCWSTR::from_raw(button_wide.as_ptr()),
                clicks,
                x,
                y,
            );
            auto_it.check_error()?;
            Ok(result == 1)
        }
    }

    pub fn control_get_text_by_handle(hwnd: HWND, control_hwnd: HWND) -> AutoItResult<String> {
        let auto_it = Self::get()?;
        let auto_it = auto_it
            .as_ref()
            .ok_or(AutoItError::InitError("AutoIt not initialized"))?;

        let mut buffer = vec![0u16; 1024];

        unsafe {
            let control_get_text: Symbol<autoit::AU3_ControlGetTextByHandle> =
                auto_it.lib.get(b"AU3_ControlGetTextByHandle")?;
            control_get_text(
                hwnd,
                control_hwnd,
                PWSTR::from_raw(buffer.as_mut_ptr()),
                buffer.len() as i32,
            );
            auto_it.check_error()?;

            let len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
            Ok(String::from_utf16_lossy(&buffer[..len]))
        }
    }

    // 其他基本操作
    pub fn mouse_click(
        button: &str,
        x: i32,
        y: i32,
        clicks: i32,
        speed: i32,
    ) -> AutoItResult<bool> {
        let auto_it = Self::get()?;
        let auto_it = auto_it
            .as_ref()
            .ok_or(AutoItError::InitError("AutoIt not initialized"))?;

        let button_wide = Self::to_wide_string(button);

        unsafe {
            let mouse_click: Symbol<autoit::AU3_MouseClick> = auto_it.lib.get(b"AU3_MouseClick")?;
            let result = mouse_click(PCWSTR::from_raw(button_wide.as_ptr()), x, y, clicks, speed);
            auto_it.check_error()?;
            Ok(result == 1)
        }
    }
    pub fn win_get_text(title: &str) -> AutoItResult<String> {
        let auto_it = Self::get()?;
        let auto_it = auto_it
            .as_ref()
            .ok_or(AutoItError::InitError("AutoIt not initialized"))?;

        let title_wide = Self::to_wide_string(title);
        let text_wide = Self::to_wide_string("");
        let mut buffer = vec![0u16; 1024];

        unsafe {
            let win_get_text: Symbol<autoit::AU3_WinGetText> =
                auto_it.lib.get(b"AU3_WinGetText")?;
            win_get_text(
                PCWSTR::from_raw(title_wide.as_ptr()),
                PCWSTR::from_raw(text_wide.as_ptr()),
                PWSTR::from_raw(buffer.as_mut_ptr()),
                buffer.len() as i32,
            );
            auto_it.check_error()?;

            let len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
            Ok(String::from_utf16_lossy(&buffer[..len]))
        }
    }

    pub fn win_get_text_by_handle(hwnd: HWND) -> AutoItResult<String> {
        let auto_it = Self::get()?;
        let auto_it = auto_it
            .as_ref()
            .ok_or(AutoItError::InitError("AutoIt not initialized"))?;

        let mut buffer = vec![0u16; 1024];

        unsafe {
            let win_get_text: Symbol<autoit::AU3_WinGetTextByHandle> =
                auto_it.lib.get(b"AU3_WinGetTextByHandle")?;
            win_get_text(
                hwnd,
                PWSTR::from_raw(buffer.as_mut_ptr()),
                buffer.len() as i32,
            );
            auto_it.check_error()?;

            let len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
            Ok(String::from_utf16_lossy(&buffer[..len]))
        }
    }
    pub fn win_get_title(title: &str) -> AutoItResult<String> {
        let auto_it = Self::get()?;
        let auto_it = auto_it
            .as_ref()
            .ok_or(AutoItError::InitError("AutoIt not initialized"))?;

        let title_wide = Self::to_wide_string(title);
        let text_wide = Self::to_wide_string("");
        let mut buffer = vec![0u16; 1024];

        unsafe {
            let win_get_title: Symbol<autoit::AU3_WinGetTitle> =
                auto_it.lib.get(b"AU3_WinGetTitle")?;
            win_get_title(
                PCWSTR::from_raw(title_wide.as_ptr()),
                PCWSTR::from_raw(text_wide.as_ptr()),
                PWSTR::from_raw(buffer.as_mut_ptr()),
                buffer.len() as i32,
            );
            auto_it.check_error()?;

            let len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
            Ok(String::from_utf16_lossy(&buffer[..len]))
        }
    }

    // 通过窗口句柄获取标题
    pub fn win_get_title_by_handle(hwnd: HWND) -> AutoItResult<String> {
        let auto_it = Self::get()?;
        let auto_it = auto_it
            .as_ref()
            .ok_or(AutoItError::InitError("AutoIt not initialized"))?;

        let mut buffer = vec![0u16; 1024];

        unsafe {
            let win_get_title: Symbol<autoit::AU3_WinGetTitleByHandle> =
                auto_it.lib.get(b"AU3_WinGetTitleByHandle")?;
            win_get_title(
                hwnd,
                PWSTR::from_raw(buffer.as_mut_ptr()),
                buffer.len() as i32,
            );
            auto_it.check_error()?;

            let len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
            Ok(String::from_utf16_lossy(&buffer[..len]))
        }
    }
}
