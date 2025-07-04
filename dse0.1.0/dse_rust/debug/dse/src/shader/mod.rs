use std::ffi;

pub static SHADER_ENTRY_NAME: &'static str = "main";
// pub static SHADER_ENTRY_NAME_RAW: &'static ffi::CStr =
//     unsafe { ffi::CStr::from_bytes_with_nul_unchecked(b"aaa main\0") };

#[cfg(feature = "graphic_api_vulkan_1_3")]
pub mod env {

    use ash::vk::{self, ShaderModule};
    use std::{fmt::Debug, ptr::null};

    use crate::convert::shader::env::ShaderResult;

    use super::SHADER_ENTRY_NAME;

    // use super::{SHADER_ENTRY_NAME, SHADER_ENTRY_NAME_RAW};

    pub struct ShaderTextD {
        id: u64,
        raw: std::string::String,
        source: Vec<u32>,
        stage: vk::ShaderStageFlags,
    }

    #[cfg(feature = "log_mode_dev")]
    impl Debug for ShaderTextD {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ShaderTextD")
                .field("id", &self.id)
                .field("raw", &self.raw)
                .field("source", &self.source)
                .field("stage", &self.stage)
                .finish()
        }
    }

    #[cfg(feature = "log_mode_rt")]
    impl Debug for ShaderTextD {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Result::Ok(())
        }
    }

    #[repr(C, align(4))]
    #[derive(Default, Debug)]
    pub struct ShaderModuleD {
        pub id: u32,
        pub source: Option<Vec<u32>>,
        pub entity: Option<vk::ShaderModule>,
        pub info: vk::ShaderModuleCreateInfo,
        pub stage: vk::ShaderStageFlags,
    }

    impl ShaderModuleD {
        pub fn pipe_stage_info(&self) -> Result<vk::PipelineShaderStageCreateInfo, ()> {
            if self.entity.is_some() {
                return Ok(vk::PipelineShaderStageCreateInfo {
                    s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                    p_next: null(),
                    flags: Default::default(),
                    stage: self.stage,
                    module: self.entity.unwrap(),
                    p_name: SHADER_ENTRY_NAME.as_ptr() as *const i8,
                    p_specialization_info: null(),
                });
            }

            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_DAT_ERROR
                    | crate::log::code::CONDI_ENTITY_NOT_INIT
                    | crate::log::code::FILE_SHADER
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogCodePart::Line)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(self.id as u128, crate::log::LogCodePart::Id)
                        .get_code()
            ));
        }

        pub fn build() -> Self {
            return Self {
                id: 0,
                entity: Option::None,
                source: Default::default(),
                info: vk::ShaderModuleCreateInfo {
                    s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
                    p_next: null(),
                    flags: Default::default(),
                    code_size: 0,
                    p_code: null(),
                },
                stage: Default::default(),
            };
        }

        pub fn build_source(mut self, sin: ShaderResult<Vec<u32>>) -> Self {
            self.source = Some(sin.get_source());
            self.stage = sin.get_stage();
            self.info.code_size = self.source.as_ref().unwrap().len() * std::mem::size_of::<u32>();
            self.info.p_code = self.source.as_ref().unwrap().as_ptr();
            return self;
        }
    }

    impl ShaderTextD {
        pub fn get_source(&self) -> String {
            return self.raw.to_string();
        }

        pub fn get_stage(&self) -> vk::ShaderStageFlags {
            return self.stage;
        }

        pub fn get_id(&self) -> u64 {
            return self.id;
        }

        pub fn build_raw(mut self, sin: std::string::String) -> Self {
            if sin.is_empty() {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_DAT_ERROR
                        | crate::log::code::CONDI_NULL_STRING_EXPECTED
                        | crate::log::code::FILE_SHADER
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogCodePart::Id)
                            .get_code()
                )
            } else {
                self.raw = sin;
            }
            return self;
        }

        pub fn build_id(mut self, name_in: u64) -> Self {
            self.id = name_in;
            return self;
        }
        pub fn build_stage(mut self, sin: vk::ShaderStageFlags) -> Self {
            self.stage = sin;
            return self;
        }
    }

    impl Default for ShaderTextD {
        fn default() -> Self {
            Self {
                id: Default::default(),
                raw: Default::default(),
                stage: Default::default(),
                source: Default::default(),
            }
        }
    }

    pub mod fixed_shader {}
    pub mod compute_shader {}

    pub trait ShaderCompiler {
        //pub fn
    }
}
