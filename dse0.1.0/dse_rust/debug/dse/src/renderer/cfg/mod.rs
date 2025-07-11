#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
#[allow(non_snake_case)]
pub mod env{



    pub mod RECT{
        use ash::vk::{self, Extent2D, Offset2D};

        pub const DEFAULT_RECT2D: vk::Rect2D= vk::Rect2D{
            offset: Offset2D { x: 0, y: 0 },
            extent: Extent2D {
                width:  { crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_WIDTH },
                height:  { crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_HEIGHT },
            },
        };
    } 
    
    //call it before in loop
    #[allow(unused)]
    pub mod IMG_FORMAT {
        use std::ptr::null;

        use ash::vk;
        pub const DEFAULT_DEPTH_IMG: vk::ImageCreateInfo = vk::ImageCreateInfo {
            s_type: vk::StructureType::IMAGE_CREATE_INFO,
            p_next: null(),
            flags: vk::ImageCreateFlags::TYPE_2D_VIEW_COMPATIBLE_EXT,
            image_type: vk::ImageType::TYPE_2D,
            format: vk::Format::D16_UNORM,
            extent: vk::Extent3D {
                width: unsafe {
                    crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_WIDTH
                },
                height: unsafe {
                    crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_HEIGHT
                },
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::OPTIMAL,
            usage: vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            sharing_mode: vk::SharingMode::EXCLUSIVE, //set up it sync the device you cllocate from
            queue_family_index_count: 0,
            p_queue_family_indices: &0,
            initial_layout: vk::ImageLayout::UNDEFINED,
        };

        pub const DEFAULT_COLOR_IMG: vk::ImageCreateInfo = vk::ImageCreateInfo {
            s_type: vk::StructureType::IMAGE_CREATE_INFO,
            p_next: null(),
            flags: vk::ImageCreateFlags::TYPE_2D_VIEW_COMPATIBLE_EXT,
            image_type: vk::ImageType::TYPE_2D,
            format: vk::Format::R8G8B8A8_UNORM,
            extent: vk::Extent3D {
                width: unsafe {
                    crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_WIDTH
                },
                height: unsafe {
                    crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_HEIGHT
                },
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::OPTIMAL,
            usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            sharing_mode: vk::SharingMode::EXCLUSIVE, //set up it sync the device you cllocate from
            queue_family_index_count: 0,
            p_queue_family_indices: &0,
            initial_layout: vk::ImageLayout::UNDEFINED,
        };
    }
    #[allow(unused)]
    pub mod IMG2VIEW {
        use std::ptr::{null, null_mut};

        use ash::vk;
        pub const DEFAULT_DEPTH: vk::ImageViewCreateInfo = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: null(),
            flags: vk::ImageViewCreateFlags::empty(),
            image: vk::Image::null(), //set up it sync the img you bind
            view_type: vk::ImageViewType::TYPE_2D,
            format: vk::Format::from_raw(0), //set up it sync the img you bind
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::R,
                g: vk::ComponentSwizzle::G,
                b: vk::ComponentSwizzle::B,
                a: vk::ComponentSwizzle::A,
            },
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::DEPTH,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        };

        pub const DEFAULT_COLOR: vk::ImageViewCreateInfo = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: null(),
            flags: vk::ImageViewCreateFlags::empty(),
            image: vk::Image::null(), //set up it sync the img you bind
            view_type: vk::ImageViewType::TYPE_2D,
            format: vk::Format::from_raw(0), //set up it sync the img you bind
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::R,
                g: vk::ComponentSwizzle::G,
                b: vk::ComponentSwizzle::B,
                a: vk::ComponentSwizzle::A,
            },
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        };
    }

    #[allow(unused)]
    #[cfg(feature = "config_ENGINE_VERTEX_BUFFER_STEP_64bit")]
    #[cfg(feature = "config_ENGINE_VERTEX_BUFFER_FLOAT_true")]
    pub mod PSO{
        use std::ptr::null;

        use crate::renderer::cfg::env::RECT::DEFAULT_RECT2D;

        use ash::vk::{
            self, DescriptorSetLayout, Extent2D, Offset2D, PipelineInputAssemblyStateCreateFlags,
            PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineLayoutCreateInfo,
            PipelineTessellationStateCreateFlags, PipelineTessellationStateCreateInfo,
            PipelineVertexInputStateCreateFlags, PipelineVertexInputStateCreateInfo,
            PipelineViewportStateCreateFlags, PipelineViewportStateCreateInfo, PrimitiveTopology,
            Rect2D, VertexInputAttributeDescription, VertexInputBindingDescription, Viewport,
        };

        pub const DEFAULT_ATTACHMENT: vk::AttachmentDescription = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: vk::Format::R8G8B8A8_UNORM,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::LOAD,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::LOAD,
            stencil_store_op: vk::AttachmentStoreOp::STORE,
            initial_layout: vk::ImageLayout::GENERAL,
            final_layout: vk::ImageLayout::GENERAL,
        };

        pub const DEFAULT_VAD_4X4_RGBA64F: VertexInputAttributeDescription =
            VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R64G64B64A64_SFLOAT,
                offset: 4 * 3 * 64,
            };

        pub const DEFAULT_VBD: VertexInputBindingDescription = VertexInputBindingDescription {
            binding: 0,
            stride: 4 * 4 * 64, //unit:[vertex,uv,normal,bitangent] size: 4*4*64
            input_rate: vk::VertexInputRate::VERTEX, // default no going to use GPU instance
        };

        /// 如果渲染管线有定义 则优先使用渲染管线中视窗
        pub const DEFAULT_VIEWPORT: Viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: unsafe { crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_WIDTH as f32 },
            height: unsafe {  crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_HEIGHT as f32 },
            min_depth: 0.0,
            max_depth: 1.0,
        };

        /// 如果渲染管线有定义 则优先使用渲染管线中裁剪
        pub const DEFAULT_SCISSROS: Rect2D = DEFAULT_RECT2D;

        pub const DEFAULT_VERTEX_INPUT_STATE: PipelineVertexInputStateCreateInfo =
            PipelineVertexInputStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                p_next: null(),
                flags: PipelineVertexInputStateCreateFlags::empty(),
                vertex_binding_description_count: 1,
                p_vertex_binding_descriptions: &DEFAULT_VBD,
                vertex_attribute_description_count: 1,
                p_vertex_attribute_descriptions: &DEFAULT_VAD_4X4_RGBA64F,
            };

        /// 默认 顺时针三角形配装图元
        /// 每三个顶点对应专门的三角形图元
        /// 不会压缩 不会重复使用
        /// 默认不开启GPU实例化
        pub const DEFAULT_INPUT_ASSEMBLY_STATE: PipelineInputAssemblyStateCreateInfo =
            PipelineInputAssemblyStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
                p_next: null(),
                flags: PipelineInputAssemblyStateCreateFlags::empty(),
                topology: PrimitiveTopology::TRIANGLE_LIST,
                primitive_restart_enable: vk::FALSE,
            };

        /// 默认 不开启细分着色器
        /// 注： vk API原则上控制点必须大于0
        /// 程序上将
        /// 控制点小于0指定为不开启细分着色
        /// 控制点为MAX指定为依据选定GPU匹配最大细分控制点
        pub const DEFAULT_TESSELLATION_STATE: PipelineTessellationStateCreateInfo =
            PipelineTessellationStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO,
                p_next: null(),
                flags: PipelineTessellationStateCreateFlags::empty(),
                patch_control_points: 0,
            };

        /// 默认与配置一致 不可更改
        /// 需要动态配置 请与配装器中构造
        pub const DEFAULT_VIEWPORT_STATE: PipelineViewportStateCreateInfo =
            PipelineViewportStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
                p_next: null(),
                flags: PipelineViewportStateCreateFlags::empty(),
                viewport_count: 1,
                p_viewports: &DEFAULT_VIEWPORT,
                scissor_count: 1,
                p_scissors: &DEFAULT_SCISSROS,
            };

        /// 默认 关闭抗锯齿
        /// 建议 在管线执行器中动态设置
        pub const DEFAULT_MULTISAMPLE_STATE: vk::PipelineMultisampleStateCreateInfo =
            vk::PipelineMultisampleStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
                p_next: null(),
                flags: vk::PipelineMultisampleStateCreateFlags::empty(),
                rasterization_samples: vk::SampleCountFlags::TYPE_1,
                sample_shading_enable: vk::FALSE,
                min_sample_shading: 0.0,
                p_sample_mask: null(),
                alpha_to_coverage_enable: vk::FALSE,
                alpha_to_one_enable: vk::FALSE,
            };

        /// 默认丢弃视窗锥体外片元 (包括远平面之外)
        /// 默认绘制模式为 填充
        /// 默认 背面渲染片元丢弃
        /// 默认 正面片元符合左手定则
        /// 默认关闭 斜率深度偏置 （用于避免阴影重影）
        pub const DEFAULT_RASTERIZATION_STATE: vk::PipelineRasterizationStateCreateInfo =
            vk::PipelineRasterizationStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
                p_next: null(),
                flags: vk::PipelineRasterizationStateCreateFlags::empty(),
                depth_clamp_enable: vk::FALSE,
                rasterizer_discard_enable: vk::FALSE,
                polygon_mode: vk::PolygonMode::FILL,
                cull_mode: vk::CullModeFlags::BACK,
                front_face: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_bias_enable: vk::FALSE,
                depth_bias_constant_factor: 0.0,
                depth_bias_clamp: 0.0,
                depth_bias_slope_factor: 0.0,
                line_width: 1.0,
            };

        pub const DEFAULT_DEPTH_STENCIL: vk::PipelineDepthStencilStateCreateInfo =
            vk::PipelineDepthStencilStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
                p_next: null(),
                flags: vk::PipelineDepthStencilStateCreateFlags::empty(),
                depth_test_enable: vk::TRUE,
                depth_write_enable: vk::TRUE,
                depth_compare_op: vk::CompareOp::LESS,
                depth_bounds_test_enable: vk::FALSE,
                stencil_test_enable: vk::TRUE,
                front: vk::StencilOpState {
                    fail_op: vk::StencilOp::ZERO,
                    pass_op: vk::StencilOp::KEEP,
                    depth_fail_op: vk::StencilOp::ZERO,
                    compare_op: vk::CompareOp::GREATER_OR_EQUAL,
                    compare_mask: 1,
                    write_mask: 1,
                    reference: 1,
                },
                back: vk::StencilOpState {
                    fail_op: vk::StencilOp::KEEP,
                    pass_op: vk::StencilOp::KEEP,
                    depth_fail_op: vk::StencilOp::KEEP,
                    compare_op: vk::CompareOp::ALWAYS,
                    compare_mask: 0,
                    write_mask: 0,
                    reference: 0,
                },
                min_depth_bounds: 0.0,
                max_depth_bounds: 0.0,
            };

        /// 默认关闭颜色混合 渲染模式为不透明
        pub const DEFAULT_COLOR_BLEND: vk::PipelineColorBlendStateCreateInfo =
            vk::PipelineColorBlendStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
                p_next: null(),
                flags: vk::PipelineColorBlendStateCreateFlags::empty(),
                logic_op_enable: vk::FALSE,
                logic_op: vk::LogicOp::AND,
                attachment_count: 0,
                p_attachments: null(),
                blend_constants: [0.0, 0.0, 0.0, 0.0],
            };

        /// 默认仅仅传入 translate数据
        ///
        pub const DEFAULT_LAYOUT: PipelineLayoutCreateInfo = PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: 0,
            p_set_layouts: null(),
            push_constant_range_count: 0,
            p_push_constant_ranges: null(),
        };

        pub const DEFAULT_SUBPASS_DESCRIPTION: vk::SubpassDescription = vk::SubpassDescription {
            flags: vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: null(),
            color_attachment_count: 0,
            p_color_attachments: null(),
            p_resolve_attachments: null(),
            p_depth_stencil_attachment: null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: null(),
        };

        pub const DEFAULT_RENDER_PASS: vk::RenderPassCreateInfo = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
            p_next: null(),
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: 1,
            p_attachments: &DEFAULT_ATTACHMENT,
            subpass_count: 1,
            p_subpasses: &DEFAULT_SUBPASS_DESCRIPTION,
            dependency_count: 0,
            p_dependencies: null(),
        };
    }
}