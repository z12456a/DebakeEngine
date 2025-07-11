#[cfg(feature = "root_path_run_time")]
pub const CONFIG_PATH: &'static str = "config\\app\\config.toml";

#[cfg(feature = "root_path_debug")]
pub const CONFIG_PATH: &'static str = "asset\\config\\app\\config.toml";

#[cfg(target_os = "unix" )]
pub mod env{
       use ash::vk;
    use std::ptr::null_mut;
    use std::str::FromStr;
    use std::{env, path::PathBuf};
    use walkdir::WalkDir;
  
    use crate::convert;
    use crate::convert::toml::env::TomlDecoderE;

    #[derive(Debug)]
    pub enum GraphicAPIType {
        Soft,
        Vk,
        Ogl,
        Egl,
    }

    pub struct ApplicationD {
        id: u64,
        app_handle: *mut HINSTANCE__,
        app_name: String,
        app_name_CStr: std::ffi::CString,
        app_version_major: u32,
        app_version_minor: u32,
        graphic_API_Type: GraphicAPIType,
        graphic_api_version: u32,
        hwnd: u64,
        app_path: PathBuf,
        editor_path: PathBuf,
        config_path: PathBuf,
        app_attachment: ApplicationAttachment,
    }

    #[derive(Default, serde::Deserialize, Debug)]
    pub struct ApplicationAttachment {
        pub asset_path: Option<PathBuf>,
        pub resource_path: Option<PathBuf>,
    }

    impl Default for ApplicationD {
        fn default() -> Self {
            let hin = unsafe { libloaderapi::GetModuleHandleW(null_mut()) };
            return Self {
                id: 0,
                app_handle: hin,
                app_name: Default::default(),
                app_name_CStr: unsafe {
                    std::ffi::CString::from_vec_with_nul_unchecked(b"dse\0".to_vec())
                },
                app_version_major: 0,
                app_version_minor: 0,
                graphic_api_version: 0,
                graphic_API_Type: GraphicAPIType::Soft,
                hwnd: 0,
                app_path: env::current_exe().unwrap(),
                editor_path: env::current_dir().unwrap(),
                config_path: PathBuf::from_str(super::CONFIG_PATH).unwrap(),
                app_attachment: Default::default(),
            };
        }
    }

    #[allow(dead_code)]
    impl ApplicationD {
        pub fn build() -> Self {
            crate::send2logger_dev!(
                crate::log::code::TYPE_EXE_INFO
                    | crate::log::code::CONDI_BUILD_START
                    | crate::log::code::FILE_APPLICATION
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogCodePart::Line)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogCodePart::Id)
                        .get_code()
            );

            return Default::default();
        }

        pub fn build_id(mut self, id_in: u64) -> Self {
            self.id = id_in;
            return self;
        }

        pub fn build_graphic_api(mut self, tin: GraphicAPIType) -> Self {
            self.graphic_API_Type = tin;
            return self;
        }

        pub fn build_graphic_api_version(mut self, major: u32, minor: u32) -> Self {
            match self.graphic_API_Type {
                GraphicAPIType::Soft => todo!(),
                GraphicAPIType::Vk => {
                    self.graphic_api_version = vk::make_api_version(0, major, minor, 0);
                }
                GraphicAPIType::Ogl => todo!(),
                GraphicAPIType::Egl => todo!(),
            }
            return self;
        }

        pub fn build_custom_app_config_path(mut self) -> Self {
            return self;
        }

        pub fn build_load_app_config(mut self, config: String) -> Self {
            
            let mut _decoder = TomlDecoderE::default().build_raw(config);
            self.app_attachment = toml::from_str(_decoder.into_block().as_str()).unwrap_or_default();
            return self;
        }

        pub fn build_appname(mut self, sin: String) -> Self {
            self.app_name = sin;
            return self;
        }

        pub fn get_graphic_api_version(&self) -> [u32; 2] {
            return [self.app_version_major, self.app_version_minor];
        }

        pub fn get_app_version(&self) -> u32 {
            return self.app_version_major;
        }

        pub fn get_app_name_clone(&self) -> String {
            return self.app_name.clone();
        }

        pub unsafe fn get_app_name_i8_pointer(&self) -> *const i8 {
            return self.app_name_CStr.as_ptr();
        }

        pub fn get_winhandle(&self) -> u64 {
            return self.hwnd.clone();
        }

        pub fn get_app_path(&self) -> PathBuf {
            return self.app_path.parent().unwrap().to_path_buf().clone();
        }

        pub fn editor_path(&self) -> PathBuf {
            return self.editor_path.parent().unwrap().to_path_buf().clone();
        }

        pub fn get_app_handle(&self) -> *mut HINSTANCE__ {
            return self.app_handle;
        }

        pub fn get_child_path_iter(&self) -> WalkDir {
            return WalkDir::new(self.get_app_path().parent().unwrap().to_path_buf()).max_depth(1);
        }

        pub fn get_editor_child_path_iter(&self) -> WalkDir {
            return WalkDir::new(self.editor_path().parent().unwrap().to_path_buf())
                .max_depth(1);
        }

        pub fn get_asset_path(&self) -> PathBuf {
            return PathBuf::new()
                .join(self.app_attachment.asset_path.clone().unwrap());
        }

        pub fn get_example_path_iter(&self) -> WalkDir {
            return WalkDir::new(
                self.editor_path()
                    .parent()
                    .unwrap()
                    .join("example")
                    .to_path_buf(),
            )
            .max_depth(1);
        }

        // pub fn drop(){

        // }

        //pub

        //#[cfg(test)]
        // pub fn test(&self) {}
    }
}

#[test]
pub fn test(){
    let mut a1 =std::process::Command::new("dir");
    let mut b1 = a1.spawn().unwrap();
    b1.wait();
    println!("{:?}", String::from_utf8(a1.output().unwrap().stdout).unwrap() );

}

#[cfg(target_os = "windows" )]
//#[cfg(no)]
pub mod env {
    use ash::vk;
    use std::ptr::null_mut;
    use std::str::FromStr;
    use std::{env, path::PathBuf};
    use walkdir::WalkDir;
    use winapi::{
        shared::minwindef::HINSTANCE__,
        um::libloaderapi::{self, GetModuleHandleW},
    };

    use crate::convert;
    use crate::convert::toml::env::TomlDecoderE;

    #[derive(Debug)]
    pub enum GraphicAPIType {
        Soft,
        Vk,
        Ogl,
        Egl,
    }

    pub struct ApplicationD {
        id: u64,
        app_handle: *mut HINSTANCE__,
        app_name: String,
        app_name_CStr: std::ffi::CString,
        app_version_major: u32,
        app_version_minor: u32,
        graphic_API_Type: GraphicAPIType,
        graphic_api_version: u32,
        hwnd: u64,
        app_path: PathBuf,
        editor_path: PathBuf,
        config_path: PathBuf,
        app_attachment: ApplicationAttachment,
    }

    #[derive(Default, serde::Deserialize, Debug)]
    pub struct ApplicationAttachment {
        pub asset_path: Option<PathBuf>,
        pub resource_path: Option<PathBuf>,
    }

    impl Default for ApplicationD {
        fn default() -> Self {
            let hin = unsafe { libloaderapi::GetModuleHandleW(null_mut()) };
            return Self {
                id: 0,
                app_handle: hin,
                app_name: Default::default(),
                app_name_CStr: unsafe {
                    std::ffi::CString::from_vec_with_nul_unchecked(b"dse\0".to_vec())
                },
                app_version_major: 0,
                app_version_minor: 0,
                graphic_api_version: 0,
                graphic_API_Type: GraphicAPIType::Soft,
                hwnd: 0,
                app_path: env::current_exe().unwrap(),
                editor_path: env::current_dir().unwrap(),
                config_path: PathBuf::from_str(super::CONFIG_PATH).unwrap(),
                app_attachment: Default::default(),
            };
        }
    }

    #[allow(dead_code)]
    impl ApplicationD {
        pub fn build() -> Self {
            crate::send2logger_dev!(
                crate::log::code::TYPE_EXE_INFO
                    | crate::log::code::CONDI_BUILD_START
                    | crate::log::code::FILE_APPLICATION
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogCodePart::Line)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogCodePart::Id)
                        .get_code()
            );

            return Default::default();
        }

        pub fn build_id(mut self, id_in: u64) -> Self {
            self.id = id_in;
            return self;
        }

        pub fn build_graphic_api(mut self, tin: GraphicAPIType) -> Self {
            self.graphic_API_Type = tin;
            return self;
        }

        pub fn build_graphic_api_version(mut self, major: u32, minor: u32) -> Self {
            match self.graphic_API_Type {
                GraphicAPIType::Soft => todo!(),
                GraphicAPIType::Vk => {
                    self.graphic_api_version = vk::make_api_version(0, major, minor, 0);
                }
                GraphicAPIType::Ogl => todo!(),
                GraphicAPIType::Egl => todo!(),
            }
            return self;
        }

        pub fn build_custom_app_config_path(mut self) -> Self {
            return self;
        }

        pub fn build_load_app_config(mut self, config: String) -> Self {
            
            let mut _decoder = TomlDecoderE::default().build_raw(config);
            self.app_attachment = toml::from_str(_decoder.into_block().as_str()).unwrap_or_default();
            return self;
        }

        pub fn build_appname(mut self, sin: String) -> Self {
            self.app_name = sin;
            return self;
        }

        pub fn get_graphic_api_version(&self) -> [u32; 2] {
            return [self.app_version_major, self.app_version_minor];
        }

        pub fn get_app_version(&self) -> u32 {
            return self.app_version_major;
        }

        pub fn get_app_name_clone(&self) -> String {
            return self.app_name.clone();
        }

        pub unsafe fn get_app_name_i8_pointer(&self) -> *const i8 {
            return self.app_name_CStr.as_ptr();
        }

        pub fn get_winhandle(&self) -> u64 {
            return self.hwnd.clone();
        }

        pub fn get_app_path(&self) -> PathBuf {
            return self.app_path.parent().unwrap().to_path_buf().clone();
        }

        pub fn editor_path(&self) -> PathBuf {
            return self.editor_path.parent().unwrap().to_path_buf().clone();
        }

        pub fn get_app_handle(&self) -> *mut HINSTANCE__ {
            return self.app_handle;
        }

        pub fn get_child_path_iter(&self) -> WalkDir {
            return WalkDir::new(self.get_app_path().parent().unwrap().to_path_buf()).max_depth(1);
        }

        pub fn get_editor_child_path_iter(&self) -> WalkDir {
            return WalkDir::new(self.editor_path().parent().unwrap().to_path_buf())
                .max_depth(1);
        }

        pub fn get_asset_path(&self) -> PathBuf {
            return PathBuf::new()
                .join(self.app_attachment.asset_path.clone().unwrap());
        }

        pub fn get_example_path_iter(&self) -> WalkDir {
            return WalkDir::new(
                self.editor_path()
                    .parent()
                    .unwrap()
                    .join("example")
                    .to_path_buf(),
            )
            .max_depth(1);
        }

        // pub fn drop(){

        // }

        //pub

        //#[cfg(test)]
        // pub fn test(&self) {}
    }
}
