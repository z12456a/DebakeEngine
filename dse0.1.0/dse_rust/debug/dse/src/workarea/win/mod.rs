#[cfg(windows)]
pub mod win32impl;

#[cfg(windows)]
// #[allow(dead_code)]
// #[allow(unused)]
pub mod env {

    use std::ptr::{null, null_mut};

    use crate::model::rectangle::env::{Rect, RectMode};
    use crate::tool::dev_dbg;
    use crate::tool::ext_lib_debug::os_err;
    use crate::workarea::win::win32impl;
    use crate::{application::env::ApplicationD, renderer::cfg::env::RECT};
    use crate::{dev_dbg, workarea};
    use winapi::shared::{
        minwindef::{HINSTANCE__, LPARAM, LRESULT, UINT, WPARAM},
        windef,
    };
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::wingdi::{DEVMODEA, LPDEVMODEA};
    use winapi::um::winuser::{
        self, CreateWindowExW, DefWindowProcW, DestroyWindow, EnumDisplaySettingsA,
        GetDesktopWindow, GetMonitorInfoA, GetWindowInfo, GetWindowRect, MessageBoxExW,
        MonitorFromWindow, PostQuitMessage, RegisterClassExW, SetWindowPos, ShowWindow,
        UpdateWindow, ENUM_CURRENT_SETTINGS, LPMONITORINFO, MONITORINFO, MONITOR_DEFAULTTOPRIMARY,
        WINDOWINFO, WM_COMMAND, WM_DESTROY,
    };
    use winapi::um::winuser::{
        CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, SW_SHOW, WM_CLOSE, WM_PAINT, WNDCLASSEXW,
        WS_EX_APPWINDOW,
    };
    use winsafe::WString;

    use crate::workarea::{
        DEFAULT_WORKAREA_RECT, WORKAREA_CLOSE, MUL_VIRTUAL_SCALING,
        VIRTUAL_SCREEN_RECT,
    };

    use super::win32impl::{debug_windowinfo, default_windowinfo};

    pub struct WinWinodwE {
        id: u64,
        wnd_handle: Option<windef::HWND>,
        mod_handle: Option<*mut HINSTANCE__>,
        win_calss: Option<WNDCLASSEXW>,
        win_name_w16: Option<WString>,
        win_name: Option<String>,
        win_devmode: Option<DEVMODEA>,
        physics_width: u64,
        physics_height: u64,
        border_width: u64,
        header_height: u64,
        is_active: bool,
    }

    impl WinWinodwE {
        pub fn set_id(&mut self, id_in: u64) {
            self.id = id_in;
        }
        pub fn id_mut(&mut self) -> &mut u64 {
            return &mut self.id;
        }
        pub fn build(self) -> Self {
            crate::send2logger_dev!(
                crate::log::code::TYPE_EXE_INFO
                    | crate::log::code::CONDI_BUILD_START
                    | crate::log::code::FILE_WINDOW
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogCodePart::Line)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(self.id as u128, crate::log::LogCodePart::Id)
                        .get_code()
            );
            return self;
        }

        pub fn build_wnd_width(mut self, width: u64) -> Self {
            self.physics_width = width;
            return self;
        }

        pub fn build_wnd_height(mut self, height: u64) -> Self {
            self.physics_height = height;
            return self;
        }

        pub fn build_link_app(mut self, app_refin: &ApplicationD) -> Self {
            self.win_name = Option::Some(app_refin.get_app_name_clone());
            self.win_name_w16 = Option::Some(WString::from_str(self.win_name.as_ref().unwrap()));
            return self;
        }

        pub fn build_current_module_handle(mut self) -> Self {
            self.mod_handle = Option::Some(unsafe { GetModuleHandleW(null_mut()) });
            return self;
        }

        pub fn wndhandle_ptr(&self) -> windef::HWND {
            return self.wnd_handle.unwrap();
        }

        pub fn module_handle_ptr(&self) -> *mut HINSTANCE__ {
            return self.mod_handle.expect("mod_handle is empty");
        }

        pub unsafe fn show_window(&self) {
            ShowWindow(self.wnd_handle.unwrap(), SW_SHOW);
        }

        pub unsafe fn update_window(&self) {
            UpdateWindow(self.wnd_handle.unwrap());
        }

        pub unsafe fn update_window_rect(&mut self) {
            let mut _wininfo = default_windowinfo();
            let mut _wininfo_p = &mut _wininfo;
            let mut _r: windef::RECT = win32impl::default_rect();
            let mut _rp = &mut _r;

            unsafe {
                SetWindowPos(self.wnd_handle.unwrap(), null_mut(), 0, 0, 0, 0, 0);

                GetWindowInfo(self.wnd_handle.unwrap(), _wininfo_p);

                self.border_width = u64::try_from(_wininfo.rcClient.left).unwrap();
                self.header_height =
                    u64::try_from(_wininfo.rcClient.top).unwrap() - self.border_width;
                workarea::WORKAREA_OFFSET_X = self.border_width * 2;
                workarea::WORKAREA_OFFSET_Y = self.header_height + workarea::WORKAREA_OFFSET_X;

                SetWindowPos(
                    self.wnd_handle.unwrap(),
                    null_mut(),
                    0,
                    0,
                    i32::try_from(self.virtue_x() + self.border_width * 2).unwrap(),
                    i32::try_from(self.virtue_y() + self.border_width * 2 + self.header_height)
                        .unwrap(),
                    0,
                );

                GetWindowRect(self.wnd_handle.unwrap(), _rp);
            }
        }

        pub unsafe fn destroy_window(&self) {
            DestroyWindow(self.wnd_handle.unwrap());
        }

        pub unsafe fn set_current_module_handle(&mut self) {
            self.mod_handle = Option::Some(GetModuleHandleW(null_mut()));
        }

        pub fn set_wnd_width(&mut self, width: u64) {
            self.physics_width = width;
        }

        pub fn set_wnd_height(&mut self, height: u64) {
            self.physics_height = height;
        }

        pub fn link_app(&mut self, app_refin: &ApplicationD) {
            self.win_name = Option::Some(app_refin.get_app_name_clone());
            self.win_name_w16 = Option::Some(WString::from_str(self.win_name.as_ref().unwrap()));
        }

        fn create_winclass(&mut self) {
            unsafe {
                //crate::dev_dbg!(std::io::Error::last_os_error());
                self.set_current_module_handle();
            }

            self.win_calss = Option::Some(WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_VREDRAW | CS_HREDRAW,
                lpfnWndProc: Option::Some(Self::wnd_procedure_func),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: self.mod_handle.unwrap(),
                hIcon: null_mut(),
                hCursor: null_mut(),
                hbrBackground: null_mut(),
                lpszMenuName: null_mut(),
                lpszClassName: unsafe { self.win_name_w16.as_ref().unwrap().as_ptr() },
                hIconSm: null_mut(),
            });

            unsafe {
                //
                //crate::dev_dbg!(std::io::Error::last_os_error());
                if RegisterClassExW(&self.win_calss.unwrap()) == 0 {
                    MessageBoxExW(null_mut(), null_mut(), null_mut(), 0, 0);
                    panic!("RegisterClassExW fail");
                }
                //
                //crate::dev_dbg!(std::io::Error::last_os_error());
            }
        }

        pub unsafe fn set_widows_rect(&mut self) {
            match self.physics_width {
                0 => {}
                _ => {}
            }
            // winuser::SetWindowPos(hWnd, hWndInsertAfter, X, Y, cx, cy, uFlags)
        }

        pub unsafe fn create_window(&mut self) {
            self.create_winclass();

            self.wnd_handle = Option::Some(CreateWindowExW(
                WS_EX_APPWINDOW,
                self.win_name_w16.as_ref().unwrap().as_ptr(),
                self.win_name_w16.as_ref().unwrap().as_ptr(),
                0x0,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                i32::try_from(self.physics_width).unwrap(),
                i32::try_from(self.physics_height).unwrap(),
                null_mut(),
                null_mut(),
                self.module_handle_ptr(),
                null_mut(),
            ));

            self.update_window_rect();

            //
            //dbg!(std::io::Error::last_os_error());
        }

        pub fn virtue_x(&self) -> u64 {
            return MUL_VIRTUAL_SCALING(self.physics_width);
        }

        pub fn virtue_y(&self) -> u64 {
            return MUL_VIRTUAL_SCALING(self.physics_height);
        }

        pub unsafe extern "system" fn wnd_procedure_func(
            _h_wnd: windef::HWND,
            _msg: UINT,
            _w_param: WPARAM,
            _l_param: LPARAM,
        ) -> LRESULT {
            match _msg {
                WM_COMMAND => {
                    return DefWindowProcW(_h_wnd, _msg, _w_param, _l_param);
                }
                WM_PAINT => {
                    // 注意这里是0 不是默认回调！
                    return 0;
                }
                WM_CLOSE => unsafe {
                    WORKAREA_CLOSE = true;
                },
                WM_DESTROY => {
                    PostQuitMessage(0);
                }
                _ => return DefWindowProcW(_h_wnd, _msg, _w_param, _l_param),
            }
            return 0;
        }

        pub fn drop(self) {
            unsafe { self.destroy_window() };
        }

        pub fn build_devmode(mut self) -> Self {
            self.win_devmode = Some(win32impl::default_devmode());
            let mut _win_rect_entity: windef::RECT = win32impl::default_rect();
            unsafe { GetWindowRect(GetDesktopWindow(), &mut _win_rect_entity) };
            // win32impl::debug_rect(&_win_rect_entity);

            unsafe {
                let lp_dm: LPDEVMODEA = self.win_devmode.as_mut().unwrap();
                EnumDisplaySettingsA(null(), ENUM_CURRENT_SETTINGS, lp_dm);
                // win32impl::debug_devmode(self.win_devmode.as_ref().unwrap());
                super::super::SCREEN_REFLESH_FREQUENCY =
                    self.win_devmode.as_ref().unwrap().dmDisplayFrequency as u64;
            }
            let _virtual_rect = glam::u64vec4(
                _win_rect_entity.left as u64,
                _win_rect_entity.top as u64,
                _win_rect_entity.right as u64,
                _win_rect_entity.bottom as u64,
            );
            let _physics_rect = glam::u64vec4(
                0,
                0,
                self.win_devmode.unwrap().dmPelsWidth as u64,
                self.win_devmode.unwrap().dmPelsHeight as u64,
            );

            super::super::UPDATE_SCREEN_RECT(_virtual_rect, _physics_rect);

            return self;
        }
    }

    impl Default for WinWinodwE {
        fn default() -> Self {
            return Self {
                wnd_handle: Option::None,
                mod_handle: Option::None,
                win_calss: Option::None,
                win_name_w16: Option::None,
                win_name: Option::None,
                border_width: Default::default(),
                header_height: Default::default(),
                physics_width: unsafe { DEFAULT_WORKAREA_RECT.weight(RectMode::DEFAULT).unwrap() },
                physics_height: unsafe { DEFAULT_WORKAREA_RECT.height(RectMode::DEFAULT).unwrap() },
                is_active: true,
                id: 0,
                win_devmode: Option::None,
            };
        }
    }
}
