use windows::core::Error as WindowsError;
#[derive(Debug)]
pub enum AutoItError {
    DllError(libloading::Error),
    InitError(&'static str),
    WindowError(&'static str),
    LockError(&'static str),
    AutoItError(i32),
    WindowsCoreError(WindowsError), // 新增
}

impl From<libloading::Error> for AutoItError {
    fn from(err: libloading::Error) -> Self {
        AutoItError::DllError(err)
    }
}

impl From<WindowsError> for AutoItError {
    fn from(err: WindowsError) -> Self {
        AutoItError::WindowsCoreError(err)
    }
}
pub type AutoItResult<T> = core::result::Result<T, AutoItError>;