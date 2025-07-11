use ash::vk::{self};
#[cfg(feature = "graphic_api_vulkan_1_3")]
pub static DEFAULT_VK_API_VERSION_MAJOR: u32 = 1;
#[cfg(feature = "graphic_api_vulkan_1_3")]
pub static DEFAULT_VK_API_VERSION_MINOR: u32 = 3;

#[cfg(feature = "graphic_api_vulkan_1_3")]
pub mod convert {
    use ash::vk::{self, Extent2D, Offset2D};

    use crate::workarea::{self, MUL_PHYSICAL_SCALING, MUL_VIRTUAL_SCALING};

    impl crate::workarea::TScreenScaling for vk::Rect2D {
        fn mul_physical_scaling(self) -> Self {
            return Self {
                offset: Offset2D {
                    x: i32::try_from(MUL_PHYSICAL_SCALING(self.offset.x as u64)).unwrap(),
                    y: i32::try_from(MUL_PHYSICAL_SCALING(self.offset.y as u64)).unwrap(),
                },
                extent: Extent2D {
                    width: u32::try_from(MUL_PHYSICAL_SCALING(self.extent.width as u64)).unwrap(),
                    height: u32::try_from(MUL_PHYSICAL_SCALING(self.extent.height as u64)).unwrap(),
                },
            };
        }

        fn mul_virtual_scaling(self) -> Self {
            return Self {
                offset: Offset2D {
                    x: i32::try_from(MUL_VIRTUAL_SCALING(self.offset.x as u64)).unwrap(),
                    y: i32::try_from(MUL_VIRTUAL_SCALING(self.offset.y as u64)).unwrap(),
                },
                extent: Extent2D {
                    width: u32::try_from(MUL_VIRTUAL_SCALING(self.extent.width as u64)).unwrap(),
                    height: u32::try_from(MUL_VIRTUAL_SCALING(self.extent.height as u64)).unwrap(),
                },
            };
        }
    }
}

#[cfg(feature = "graphic_api_vulkan_1_3")]
pub mod env {
    use std::{
        default,
        ops::Deref,
        ptr::{null, null_mut},
    };

    use ash::vk::{self};

    use crate::dev_dbg_iter;

    use super::{DEFAULT_VK_API_VERSION_MAJOR, DEFAULT_VK_API_VERSION_MINOR};

    // #[allow(unused)]
    pub mod name {
        pub mod layer {
            pub static KHR_VALIDATION: &'static std::ffi::CStr = unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0")
            };
        }
        pub mod ext {
            pub static DEBUG_UTILS: &'static std::ffi::CStr =
                unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_utils\0") };
        }
        pub mod khr {
            pub static SURFACE: &'static std::ffi::CStr =
                unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_surface\0") };
            pub static PORTABILITY_SUBSET: &'static std::ffi::CStr = unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_portability_subset\0")
            };
            pub static WIN32_SURFACE: &'static std::ffi::CStr =
                unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_surface\0") };
            pub static SWAPCHAIN: &'static std::ffi::CStr =
                unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain\0") };
            pub static SWAPCHAIN_EXTENSION_NAME: &'static std::ffi::CStr = unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_SWAPCHAIN_EXTENSION_NAME\0")
            };
        }
        pub mod nv {
            pub static GLSL_SHADER: &'static std::ffi::CStr =
                unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_glsl_shader\0") };
        }
    }

    // use for
    // #[allow(unused)]
    pub const GPU_FEATURES_LEN: u32 = (std::mem::size_of::<vk::PhysicalDeviceFeatures>()
        / std::mem::size_of::<vk::Bool32>()) as u32;

    pub struct AshVkBaseConfigD {
        gpu_id: u64,
        gpu_queue_priority: f32,
    }
    pub struct AshVkBaseD {
        id: u64,
        base_feature: String,
        lock: bool,
        config: AshVkBaseConfigD,
        priority: [f32; 1],

        ash_entry: Option<ash::Entry>,
        ash_instance: Option<ash::Instance>, //ash pack warp instance include entry& vk instance
        ash_device: Option<ash::Device>,
        ash_debug_utils: Option<ash::extensions::ext::DebugUtils>,
        ash_surface_loader: Option<ash::extensions::khr::Surface>,
        //change by cross plat compiling setting
        ash_os_drive_loader: Option<ash::extensions::khr::Win32Surface>,

        vk_app_info: Option<vk::ApplicationInfo>,
        vk_instance_info: Option<vk::InstanceCreateInfo>,
        vk_instance: Option<vk::Instance>,
        vk_instance_extension_names: Option<Vec<*const i8>>,
        vk_instance_layer_names: Option<Vec<*const i8>>,
        vk_debug_create_info: Option<vk::DebugUtilsMessengerCreateInfoEXT>,
        vk_debug_msger: Option<vk::DebugUtilsMessengerEXT>,
        vk_gpu_device: Option<vk::PhysicalDevice>,
        vk_gpu_properties: Option<vk::PhysicalDeviceProperties>,
        vk_gpu_feature: Option<vk::PhysicalDeviceFeatures>,
        vk_gpu_mem_properties: Option<vk::PhysicalDeviceMemoryProperties>,
        vk_gpu_queue_families: Option<Vec<vk::QueueFamilyProperties>>,
        //vk_gpu_queue_suitable_families: Option<Vec<vk::QueueFamilyProperties>>,
        vk_logical_device_quque_create_info: Option<Vec<vk::DeviceQueueCreateInfo>>,
        vk_logical_device_info: Option<vk::DeviceCreateInfo>,
        vk_logical_device_ext_names: Option<Vec<*const i8>>,
    }

    // iter union for enumerat gpu feature struct
    // useless content!!!
    #[repr(C)]
    // #[allow(unused)]
    pub union FeatureUnionIter {
        features: [vk::Bool32; GPU_FEATURES_LEN as usize],
        data: vk::PhysicalDeviceFeatures,
    }

    impl AshVkBaseConfigD {}

    impl AshVkBaseD {
        /// 确保在创建ash_instance与vk_gpu_device后调用
        pub unsafe fn build_gpu_feature(mut self) -> Self {
            self.vk_gpu_feature = Some(
                self.ash_instance
                    .as_mut()
                    .unwrap()
                    .get_physical_device_features(*self.vk_gpu_device.as_ref().unwrap()),
            );
            return self;
        }

        // 确保在创建ash_instance与vk_gpu_device后调用
        pub unsafe fn build_mem_properties(mut self) -> Self {
            self.vk_gpu_mem_properties = Option::Some(
                self.ash_instance
                    .as_mut()
                    .unwrap()
                    .get_physical_device_memory_properties(*self.vk_gpu_device.as_ref().unwrap()),
            );
            return self;
        }

        pub unsafe fn build_specify_config(mut self, config_in: AshVkBaseConfigD) -> Self {
            self.config = config_in;
            return self;
        }

        pub unsafe fn build_base_feature(mut self, feature_name: std::string::String) -> Self {
            return self;
        }

        pub unsafe fn build_appname(mut self, name_in: *const i8) -> Self {
            self.vk_app_info.as_mut().unwrap().p_application_name = name_in;
            self.vk_app_info.as_mut().unwrap().p_engine_name = name_in;
            return self;
        }

        pub unsafe fn build_appversion(mut self, name_in: *const i8) -> Self {
            return self;
        }

        pub unsafe fn build_apiversion(mut self, vers_in: u32) -> Self {
            self.vk_app_info.as_mut().unwrap().api_version = vers_in;
            return self;
        }

        pub unsafe fn build_add_p_instance_ext_name(mut self, pin: *const i8) -> Self {
            self.vk_instance_extension_names.as_mut().unwrap().push(pin);
            self.vk_instance_info
                .as_mut()
                .unwrap()
                .enabled_extension_count =
                self.vk_instance_extension_names.as_ref().unwrap().len() as u32;
            self.vk_instance_info
                .as_mut()
                .unwrap()
                .pp_enabled_extension_names =
                self.vk_instance_extension_names.as_mut().unwrap().as_ptr();
            return self;
        }

        pub unsafe fn build_add_p_instance_layer_name(mut self, pin: *const i8) -> Self {
            self.vk_instance_layer_names.as_mut().unwrap().push(pin);
            self.vk_instance_info.as_mut().unwrap().enabled_layer_count =
                self.vk_instance_layer_names.as_ref().unwrap().len() as u32;
            self.vk_instance_info
                .as_mut()
                .unwrap()
                .pp_enabled_layer_names = self.vk_instance_layer_names.as_mut().unwrap().as_ptr();
            return self;
        }

        // ensure call this func in first build base
        // use to init instance not just vk instance but ash pack
        pub unsafe fn build_instance_checked(mut self) -> Self {
            if true {
                self.ash_instance = Option::Some(
                    self.ash_entry
                        .as_ref()
                        .unwrap()
                        .create_instance(self.vk_instance_info.as_ref().unwrap(), Option::None)
                        .expect("build_instance_fail"),
                )
            }
            return self;
        }

        // need ensure ash instance & ash entry active
        pub unsafe fn build_debug_util_checked(mut self) -> Self {
            #[cfg(feature = "vk_debug_true")]
            {
                self.ash_debug_utils = Option::Some(ash::extensions::ext::DebugUtils::new(
                    self.ash_entry.as_ref().unwrap(),
                    self.ash_instance.as_ref().unwrap(),
                ));
                self.vk_debug_msger = Option::Some(
                    self.ash_debug_utils
                        .as_ref()
                        .unwrap()
                        .create_debug_utils_messenger(
                            self.vk_debug_create_info.as_ref().unwrap(),
                            Option::None,
                        )
                        .unwrap(),
                );
            }
            return self;
        }

        pub unsafe fn build_choose_debug_call_back(
            mut self,
            call_specify: vk::PFN_vkDebugUtilsMessengerCallbackEXT,
        ) -> Self {
            self.vk_debug_create_info
                .as_mut()
                .unwrap()
                .pfn_user_callback = call_specify;
            return self;
        }

        // ensure include "VK_KHR_surface" ext name
        // ensure build instance & entry first
        // ensure plat
        pub unsafe fn build_os_drive_loader(mut self) -> Self {
            self.ash_os_drive_loader = Option::Some(ash::extensions::khr::Win32Surface::new(
                self.ash_entry.as_ref().unwrap(),
                self.ash_instance.as_ref().unwrap(),
            ));
            return self;
        }

        pub unsafe fn build_gpu_properties(mut self) -> Self {
            self.vk_gpu_properties = Some(
                self.ash_instance
                    .as_ref()
                    .unwrap()
                    .get_physical_device_properties(*self.gpu_instance_ref().unwrap()),
            );
            return self;
        }

        // detect local physical device & select custom specifying one
        // detect what will rely on what ext and layer name and config you decide and link on
        // ensure link config by assertE before or you will get a default config gpu setting but panic!
        pub unsafe fn build_gpu_device_custom(mut self) -> Self {
            let vk_gpu_device_list = self
                .ash_instance
                .as_ref()
                .unwrap()
                .enumerate_physical_devices()
                .expect("error:there is no physical device found");

            if self.config.gpu_id < vk_gpu_device_list.len() as u64 {
                self.vk_gpu_device = Option::Some(vk_gpu_device_list[self.config.gpu_id as usize]);
            } else {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_EXT_ERROR
                        | crate::log::code::CONDI_VK_DEVICE_NOT_FOUND
                        | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogCodePart::Id)
                            .get_code()
                );
            }
            return self;
        }

        // detect local physical device by os drive define
        pub unsafe fn build_gpu_device_auto(mut self) -> Self {
            let vk_gpu_device_list = self
                .ash_instance
                .as_ref()
                .unwrap()
                .enumerate_physical_devices()
                .expect("error:there is no physical device found");
            if self.ash_os_drive_loader.is_some() {
                for gpu_i in vk_gpu_device_list.iter() {
                    for eqi in self
                        .ash_instance
                        .as_ref()
                        .unwrap()
                        .get_physical_device_queue_family_properties(*gpu_i)
                        .iter()
                        .enumerate()
                    {
                        if self
                            .ash_os_drive_loader
                            .as_ref()
                            .unwrap()
                            .get_physical_device_win32_presentation_support(*gpu_i, eqi.0 as u32)
                        {
                            self.vk_gpu_device = Option::Some(*gpu_i);
                        }
                    }
                }
            } else {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_EXT_ERROR
                        | crate::log::code::CONDI_VK_BUILDER_PREBUILD_NOT_BUILD
                        | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogCodePart::Id)
                            .get_code()
                );
            }

            return self;
        }

        // ensure build current gpu before
        pub unsafe fn build_get_queue_family_from_gpu(mut self) -> Self {
            if self.vk_gpu_device.is_some() {
                self.vk_gpu_queue_families = Option::Some(
                    self.ash_instance
                        .as_ref()
                        .unwrap()
                        .get_physical_device_queue_family_properties(
                            self.vk_gpu_device.clone().unwrap(),
                        ),
                );
                // dev_dbg_iter!(self.vk_gpu_queue_families.as_ref().unwrap());
            } else {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_EXT_ERROR
                        | crate::log::code::CONDI_VK_BUILDER_PREBUILD_NOT_BUILD
                        | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogCodePart::Id)
                            .get_code()
                )
            }
            return self;
        }

        pub unsafe fn build_set_queue_priority(mut self, fin: f32) -> Self {
            self.priority[0] = fin;
            return self;
        }

        // Uncompleted Experimental function !!!!
        // ensure you know what you addon ext name to use it
        pub unsafe fn build_add_p_device_ext_name_checked(mut self, pin: *const i8) -> Self {
            let _features = self
                .ash_instance
                .as_ref()
                .unwrap()
                .get_physical_device_features(self.vk_gpu_device.clone().unwrap());
            // here you see
            //fk this is true!!!!! always no check!!!!
            if true {
                self.vk_logical_device_ext_names.as_mut().unwrap().push(pin);
                self.vk_logical_device_info
                    .as_mut()
                    .unwrap()
                    .enabled_extension_count =
                    self.vk_logical_device_ext_names.as_ref().unwrap().len() as u32;
                self.vk_logical_device_info
                    .as_mut()
                    .unwrap()
                    .pp_enabled_extension_names =
                    self.vk_logical_device_ext_names.as_mut().unwrap().as_ptr();
            } else {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_EXT_ERROR
                        | crate::log::code::CONDI_VK_UNEXPRCTED_EXT_NAME
                        | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogCodePart::Id)
                            .get_code()
                )
            }

            return self;
        }
        //
        pub unsafe fn build_vk_device_checked(mut self) -> Self {
            if self.vk_gpu_queue_families.is_some() {
                let mut queue_result: Vec<vk::DeviceQueueCreateInfo> =
                    Vec::<vk::DeviceQueueCreateInfo>::new();
                for eqi in self
                    .vk_gpu_queue_families
                    .as_ref()
                    .unwrap()
                    .iter()
                    .enumerate()
                {
                    // ensure this pcie is pipe toward gpu
                    if eqi.1.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                        queue_result.push(vk::DeviceQueueCreateInfo {
                            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                            p_next: null(),
                            flags: vk::DeviceQueueCreateFlags::default(),
                            queue_family_index: eqi.0 as u32,
                            queue_count: self.vk_gpu_queue_families.as_ref().unwrap().len() as u32,
                            p_queue_priorities: self.priority.as_ptr(),
                        })
                    }
                }

                self.vk_logical_device_quque_create_info = Option::Some(queue_result);
                self.vk_logical_device_info
                    .as_mut()
                    .unwrap()
                    .queue_create_info_count = self
                    .vk_logical_device_quque_create_info
                    .as_ref()
                    .unwrap()
                    .len() as u32;
                self.vk_logical_device_info
                    .as_mut()
                    .unwrap()
                    .p_queue_create_infos = self
                    .vk_logical_device_quque_create_info
                    .as_ref()
                    .unwrap()
                    .as_ptr();

                self.ash_device = Option::Some(
                    self.ash_instance
                        .as_ref()
                        .unwrap()
                        .create_device(
                            self.vk_gpu_device.clone().unwrap(),
                            self.vk_logical_device_info.as_ref().unwrap(),
                            Option::None,
                        )
                        .expect("msg"),
                );
            } else {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_EXT_ERROR
                        | crate::log::code::CONDI_VK_BUILDER_PREBUILD_NOT_BUILD
                        | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogCodePart::Id)
                            .get_code()
                )
            }
            return self;
        }

        pub unsafe fn build_surface_loader_checked(mut self) -> Self {
            self.ash_surface_loader = Option::Some(ash::extensions::khr::Surface::new(
                self.ash_entry.as_ref().unwrap(),
                self.ash_instance.as_ref().unwrap(),
            ));
            return self;
        }

        pub unsafe fn build() -> Self {
            let ash_entry = ash::Entry::load().unwrap();
            let vk_layer_names = Vec::<*const i8>::new();
            let vk_extension_names = Vec::<*const i8>::new();
            let vk_app_info = Option::Some(vk::ApplicationInfo {
                s_type: vk::StructureType::APPLICATION_INFO,
                p_next: null(),
                p_application_name: null(),
                application_version: 0,
                p_engine_name: null(),
                engine_version: 0,
                api_version: vk::make_api_version(
                    0,
                    DEFAULT_VK_API_VERSION_MAJOR,
                    DEFAULT_VK_API_VERSION_MINOR,
                    0,
                ),
            });
            let vk_instance_info = Option::Some(vk::InstanceCreateInfo {
                s_type: vk::StructureType::INSTANCE_CREATE_INFO,
                p_next: null(),
                flags: vk::InstanceCreateFlags::default(),
                p_application_info: null(),
                enabled_layer_count: vk_layer_names.len() as u32,
                pp_enabled_layer_names: vk_layer_names.as_ptr(),
                enabled_extension_count: vk_extension_names.len() as u32,
                pp_enabled_extension_names: vk_extension_names.as_ptr(),
            });

            let vk_debug_info = vk::DebugUtilsMessengerCreateInfoEXT {
                s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
                p_next: null(),
                flags: vk::DebugUtilsMessengerCreateFlagsEXT::default(),
                message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
                message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                pfn_user_callback: Option::Some(Self::callback_vk_debug_2logsys),
                p_user_data: null_mut(),
            };

            let vk_logical_device_ext_names = Vec::<*const i8>::new();

            let vk_logical_device_info = vk::DeviceCreateInfo {
                s_type: vk::StructureType::DEVICE_CREATE_INFO,
                p_next: null(),
                flags: vk::DeviceCreateFlags::default(),
                queue_create_info_count: 0,
                p_queue_create_infos: null(),
                enabled_layer_count: 0,
                pp_enabled_layer_names: null(),
                enabled_extension_count: 0,
                pp_enabled_extension_names: null(),
                p_enabled_features: null(),
            };

            Self {
                base_feature: String::from("AshVkBase"),
                config: AshVkBaseConfigD::default(),
                ash_entry: Option::Some(ash_entry),
                ash_instance: Option::None,
                ash_device: Option::None,
                ash_debug_utils: Option::None,
                vk_app_info: vk_app_info,
                vk_instance_info: vk_instance_info,
                vk_instance: Option::None,
                vk_instance_extension_names: Option::Some(vk_extension_names),
                vk_instance_layer_names: Option::Some(vk_layer_names),
                vk_debug_create_info: Option::Some(vk_debug_info),
                vk_debug_msger: Option::None,
                vk_gpu_device: Option::None,
                vk_gpu_mem_properties: Option::None,
                vk_gpu_properties: Option::None,
                vk_gpu_feature: Option::None,
                vk_logical_device_info: Option::Some(vk_logical_device_info),
                vk_logical_device_quque_create_info: Option::None,
                vk_gpu_queue_families: Option::None,
                ash_os_drive_loader: Option::None,
                priority: [1.0f32],
                vk_logical_device_ext_names: Option::Some(vk_logical_device_ext_names),
                id: 0,
                lock: false,
                ash_surface_loader: Option::None,
                // vk_gpu_queue_suitable_families:: Option::None ,
            }
        }

        pub unsafe fn check_push_out(self) -> Self {
            return self;
        }

        pub fn gpu_properties_clone(&self) -> Result<vk::PhysicalDeviceProperties, ()> {
            match self.vk_gpu_properties {
                Some(val) => Ok(val.clone()),
                None => {
                    return Err(crate::send2logger_dev!(
                        crate::log::code::TYPE_EXE_ERROR
                            | crate::log::code::CONDI_OPTION_NONE
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code()
                    ));
                }
            }
        }

        // get entry ref will not consunm itself
        pub fn ash_entry_ref(&self) -> Result<&ash::Entry, ()> {
            match self.ash_entry {
                Some(ref i) => return std::result::Result::Ok(i),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_VK_INSTANCE_NOT_FOUND
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code(),
                    ));
                }
            }
        }

        // get instance ref will not consunm itself
        pub fn ash_instance_ref(&self) -> Result<&ash::Instance, ()> {
            match self.ash_instance {
                Some(ref i) => return std::result::Result::Ok(i),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_VK_INSTANCE_NOT_FOUND
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code(),
                    ));
                }
            }
        }

        // get device clone will not consunm itself
        // but api will get a lock after call this func successful
        pub fn ash_device_clone(&self) -> Result<ash::Device, ()> {
            match self.ash_device.clone() {
                Some(d) => return std::result::Result::Ok(d),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_VK_DEVICE_NOT_FOUND
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code(),
                    ));
                }
            }
        }

        pub fn gpu_instance_ref(&self) -> Result<&vk::PhysicalDevice, ()> {
            match self.vk_gpu_device {
                Some(ref i) => return std::result::Result::Ok(i),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_VK_INSTANCE_NOT_FOUND
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code(),
                    ));
                }
            }
        }

        pub fn gueue_info_ref(&self) -> Result<&Vec<vk::QueueFamilyProperties>, ()> {
            match self.vk_gpu_queue_families {
                Some(ref i) => return std::result::Result::Ok(i),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_VK_INSTANCE_NOT_FOUND
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code(),
                    ));
                }
            }
        }

        pub fn gpu_suitable_queue_count_currrent(&self) -> u32 {
            return self
                .vk_logical_device_info
                .as_ref()
                .unwrap()
                .queue_create_info_count;
            return 0;
        }

        pub fn surface_loader_ref(&self) -> Result<&ash::extensions::khr::Surface, ()> {
            match self.ash_surface_loader.as_ref() {
                Some(s) => return std::result::Result::Ok(s),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_VK_INSTANCE_NOT_FOUND
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code(),
                    ));
                }
            }
        }

        pub fn gpu_feature_ref(&self) -> Result<&vk::PhysicalDeviceFeatures, ()> {
            match self.vk_gpu_feature.as_ref() {
                Some(s) => return std::result::Result::Ok(s),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_OPTION_NONE
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code(),
                    ));
                }
            }
        }

        pub fn gpu_mem_properties_current_ref(
            &self,
        ) -> Result<&vk::PhysicalDeviceMemoryProperties, ()> {
            match self.vk_gpu_mem_properties.as_ref() {
                Some(s) => return std::result::Result::Ok(s),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_OPTION_NONE
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code(),
                    ));
                }
            }
        }

        pub fn check_ext_name_exist(&self, name_ptr: *const i8) -> bool {
            match &self.vk_instance_extension_names {
                Some(names) => {
                    for ni in names.iter() {
                        if *ni == name_ptr {
                            return true;
                        }
                    }
                }
                None => {}
            }
            return false;
        }

        pub unsafe fn drop(mut self) {}

        pub unsafe extern "system" fn callback_vk_debug_2logsys(
            message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
            message_type: vk::DebugUtilsMessageTypeFlagsEXT,
            p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
            _user_data: *mut std::os::raw::c_void,
        ) -> vk::Bool32 {
            let mut logcode = crate::log::LogCodeD::new();
            match message_severity.as_raw(){
                0b1_0000/*info */=>{
                    logcode.encode(
                        crate::log::code::CONDI_VK_DEBUG_UTIL
                        |crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                        |crate::log::code::TYPE_EXT_INFO
                        , crate::log::LogCodePart::Default);},
                0b1_0000_0000/*warning */=>{
                    logcode.encode(
                        crate::log::code::CONDI_VK_DEBUG_UTIL
                        |crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                        |crate::log::code::TYPE_EXT_WARN
                        , crate::log::LogCodePart::Default);},
                0b1_0000_0000_0000/*error */=>{
                    logcode.encode(
                        crate::log::code::CONDI_VK_DEBUG_UTIL
                        |crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                        |crate::log::code::TYPE_EXT_ERROR
                        , crate::log::LogCodePart::Default);},
                _=>{}
            }
            crate::send2logger_dev!(
                crate::log::code::TYPE_EXT_INFO
                    | crate::log::code::CONDI_DEFAULT
                    | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogCodePart::Line)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogCodePart::Id)
                        .get_code()
            );
            crate::log::send_complete_code2logger(logcode.get_code());
            vk::FALSE
        }

        #[cfg(feature = "log_mode_rt")]
        pub unsafe extern "system" fn callback_vk_debug(
            message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
            message_type: vk::DebugUtilsMessageTypeFlagsEXT,
            p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
            _user_data: *mut std::os::raw::c_void,
        ) -> vk::Bool32 {
            vk::FALSE
        }

        #[cfg(feature = "log_mode_dev")]
        pub unsafe extern "system" fn callback_vk_debug(
            message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
            message_type: vk::DebugUtilsMessageTypeFlagsEXT,
            p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
            _user_data: *mut std::os::raw::c_void,
        ) -> vk::Bool32 {
            let callback_data = *p_callback_data;
            let message_id_number: i32 = callback_data.message_id_number as i32;

            let message_id_name = if callback_data.p_message_id_name.is_null() {
                std::borrow::Cow::from("unknow message_id_name err")
            } else {
                std::ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
            };

            let message = if callback_data.p_message.is_null() {
                std::string::String::from("unknow p_message err")
            } else {
                std::ffi::CStr::from_ptr(callback_data.p_message)
                    .to_string_lossy()
                    .to_string()
            };

            let message = message.replace(":", ":\n");
            let message = message.replace(". The Vulkan spec states", ".\n The Vulkan spec states");
            let message = message.replace("https:\n", "https:");

            println!(
                "{:?}:\n{:?} [{} ({})] :\n \n {}\n{}\n",
                message_severity,
                message_type,
                message_id_name,
                &message_id_number.to_string(),
                message,
                "################################################################################################",
            );

            vk::FALSE
        }
    }

    impl Default for AshVkBaseD {
        fn default() -> Self {
            unsafe {
                return Self::build()
                    .build_add_p_instance_layer_name(name::layer::KHR_VALIDATION.as_ptr())
                    .build_add_p_instance_ext_name(name::khr::WIN32_SURFACE.as_ptr())
                    .build_add_p_instance_ext_name(name::khr::SURFACE.as_ptr())
                    .build_add_p_instance_ext_name(name::ext::DEBUG_UTILS.as_ptr())
                    .build_instance_checked()
                    .build_choose_debug_call_back(Option::Some(Self::callback_vk_debug))
                    .build_debug_util_checked()
                    .build_os_drive_loader()
                    .build_gpu_device_auto()
                    .build_get_queue_family_from_gpu()
                    .build_add_p_device_ext_name_checked(name::khr::SWAPCHAIN.as_ptr())
                    .build_vk_device_checked()
                    .build_surface_loader_checked()
                    .build_mem_properties()
                    .build_gpu_properties()
                    .build_gpu_feature()
                    .check_push_out();
            }
        }
    }

    impl Default for AshVkBaseConfigD {
        fn default() -> Self {
            Self {
                gpu_id: Default::default(),
                gpu_queue_priority: 1.0f32,
            }
        }
    }
}
