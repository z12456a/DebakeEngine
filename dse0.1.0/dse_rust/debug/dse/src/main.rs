mod application;
mod asset;
mod convert;
#[cfg(feature = "example_use")]
mod examples;
mod ext_api;
mod input;
mod log;
mod manager;
mod material;
mod meta;
mod model;
mod node;
mod renderer;
mod resource;
mod shader;
#[cfg(feature = "test")]
mod test;
mod time;
mod tool;
mod workarea;

use std::path::PathBuf;

use application::env::ApplicationD;
use ash::vk;
use convert::{
    shader::env::{ShaderDecoderE, ShaderDecoderTask, ShaderResult},
    stdfile::env::Defile,
};
use ext_api::graphic::env::AshVkBaseD;
use input::{env::InputE, win::env::WinInputE};
use manager::{buffer::env::Buffers, datum::env::Datum, execute::env::TaskQueue};
use model::{
    env::{ModelD, ModelE},
    mesh::env::MeshD,
    transform::env::TransformD,
};
use renderer::{
    cmd::env::{CommandRenderE, RenderCmdTask},
    env::{PreTypeSurfaceIMG, RendererE, RendererTask, SurfaceIMG, SurfaceIMGUsage},
    pipeline::env::{GraphicPipeLinePCO, GraphicPipeLinePSO, RenderPipelineD, RenderPipelineType},
};
use resource::env::ResourceE;
use shader::env::{ShaderModuleD, ShaderTextD};
use time::env::{TimerE, UtcTimeD};
use tool::stop_point;
use workarea::win::env::WinWinodwE;

use crate::log::sorry;


extern crate dse_macros;

fn main() -> std::io::Result<()> {

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////                 MANAGER INIT                   ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    // 管理器 初始化
    // buf 一次性数据
    // dat 长期数据块
    // exe 任务队列执行器
    // tak 任务队列
    // log 全局日志系统
    let mut dat: Box<DatumM> = Box::new(Default::default());
    let mut buf: Box<BufferM> = Box::new(Default::default());
    let mut exe: ExecuteM = Default::default();
    let mut tak: TaskM = Default::default();

    exe.id_sort();
    log::init();

    // app init
    // 应用程序数据初始化
    // 加载配置文件
    //

    dat.application = dat
        .application
        .build_appname(String::from("Dse Editor"))
        .build_graphic_api(application::env::GraphicAPIType::Vk)
        .build_graphic_api_version(1, 3)
        .build_load_app_config(
            Defile::build(
                ResourceE::build()
                    .build_current_path(PathBuf::from(application::CONFIG_PATH))
                    .load_single_sync()
                    .unwrap(),
            )
            .into_string()
            .unwrap(),
        );

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////              REGISTER INIT                     ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    //dat.register = dat.register.build_set_root(exe.resource_loader.);

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////               WINDOW INIT                      ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    unsafe {
        exe.win_window = exe
            .win_window
            .build_devmode()
            .build_link_app(&dat.application)
            .build_current_module_handle();
        // window
        exe.win_window.create_window();
        exe.win_window.show_window();

        exe.win_input = WinInputE::build()
            .build_link_wndhandle(exe.win_window.wndhandle_ptr())
            .build_hook_keyboard();

        exe.input = InputE::build()
            .build_active_keys(input::env::active_optional::DEFAULT_4X_1D.to_vec(), 1)
            .build_active_keys(input::env::active_optional::DEFAULT_4X_2D.to_vec(), 2);
    }

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////                 TIMER INIT                     ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    let mut utc_counter = UtcTimeD::build();

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////                 MANAGER INIT                   ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    exe.renderer1 = exe
        .renderer1
        .build_specify_handle(
            exe.win_window.wndhandle_ptr(),
            exe.win_window.module_handle_ptr(),
        )
        .build_specify_api_base2create_surface(&mut dat.renderer_base)
        .build_device_suitable_surface(&mut dat.renderer_base)
        .build_swap_buffer(&mut dat.renderer_base)
        .build_cmd_pool()
        .build_set_pipeline_dynamic_state_auto();

    dat.surface_img
        .alloc_data(Datum::default(), Some(exe.renderer1.id))
        .end();
    dat.cmd_buffer
        .alloc_data(Datum::default(), Some(exe.renderer1.id))
        .end();
    dat.shader_mod
        .alloc_data(Datum::default(), Some(exe.renderer1.id))
        .end();
    dat.graphic_renderer_pipeline
        .alloc_data(Datum::default(), Some(exe.renderer1.id))
        .end();
    dat.fbo
        .alloc_data(Datum::default(), Some(exe.renderer1.id))
        .end();

    tak.decoder_task
        .alloc_data(Default::default(), Some(exe.shader_decoder.id))
        .end();

    tak.render_task
        .alloc_data(Default::default(), Some(exe.renderer1.id))
        .end();
    exe.renderer1
        .link_task_queue(tak.render_task.exe_data_mut(exe.renderer1.id).unwrap());

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////                   SHADER INIT                  ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    exe.resource_loader.set_current_path(
        dat.application
            .editor_path()
            .join("debug")
            .join("asset")
            .join("shader"),
    );

    for fi in exe
        .resource_loader
        .load_current_specify_suffix_sync(".vert".to_string())
        .unwrap()
    {
        if fi.is_some() {
            buf.shader_source_code.push_buffer(
                ShaderTextD::default()
                    .build_raw(
                        Defile::build(fi.unwrap().expect_stream().unwrap())
                            .to_string()
                            .unwrap(),
                    )
                    .build_stage(vk::ShaderStageFlags::VERTEX),
            )
        }
    }

    for fi in exe
        .resource_loader
        .load_current_specify_suffix_sync(".frag".to_string())
        .unwrap()
    {
        if fi.is_some() {
            buf.shader_source_code.push_buffer(
                ShaderTextD::default()
                    .build_raw(
                        Defile::build(fi.unwrap().expect_stream().unwrap())
                            .to_string()
                            .unwrap(),
                    )
                    .build_stage(vk::ShaderStageFlags::FRAGMENT),
            )
        }
    }

    while !buf.shader_source_code.is_empty() {
        buf.shader_binary_code.push_buffer(
            exe.shader_decoder
                .decode_sync(buf.shader_source_code.consume_front().back_mut().unwrap())
                .decode_to_binary_u32()
                .unwrap(),
        );
    }

    for sbi in buf.shader_binary_code.consume_all().iter() {
        let _r = ShaderModuleD::build().build_source(sbi.clone());
        dat.shader_mod
            .exe_data_mut(exe.renderer1.id)
            .unwrap()
            .alloc_data(_r, Option::None)
            .end()
    }

    exe.renderer1
        .create_shader_module(tak.render_task.exe_data_mut(exe.renderer1.id).unwrap());

    exe.renderer1.exe_shader_module(
        dat.shader_mod.exe_data_mut(exe.renderer1.id).unwrap(),
        tak.render_task.exe_data_mut(exe.renderer1.id).unwrap(),
    );

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////         MESH & TRANSFORM INIT                  ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    dat.mesh
        .alloc_data(Default::default(), Some(exe.model.id))
        .end();
    dat.transform
        .alloc_data(Default::default(), Some(exe.model.id))
        .end();

    let _test_mesh = MeshD::build().build_default_2D_spirit();
    dat.mesh
        .exe_data_mut(exe.model.id)
        .unwrap()
        .alloc_data(_test_mesh, Option::None)
        .end();

    let _test_trans = TransformD::default();
    dat.transform
        .exe_data_mut(exe.model.id)
        .unwrap()
        .alloc_data(_test_trans, Option::None)
        .end();

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////                   MODEL INIT                   ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    dat.model
        .alloc_data(Default::default(), Some(exe.model.id))
        .end();

    let mut _test_model: ModelD = ModelD::build();
    _test_model.set_mesh(0);
    _test_model.set_transform(0);

    dat.model
        .exe_data_mut(exe.model.id)
        .unwrap()
        .alloc_data(_test_model, Option::None)
        .end();

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////              RENDERER IMG BIND                 ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    // exe.renderer1.create_color_surface_img_view(
    //     0,
    //     0,
    //     tak.render_task.exe_data_mut(exe.renderer1.id).unwrap(),
    // );

    exe.renderer1.create_custom_surface_img_view(
        dat.surface_img.exe_data_index(exe.renderer1.id).unwrap(),
        SurfaceIMGUsage::Storage(PreTypeSurfaceIMG::DefaultColor),
        tak.render_task.exe_data_mut(exe.renderer1.id).unwrap(),
    );

    exe.renderer1.create_custom_surface_img_view(
        dat.surface_img.exe_data_index(exe.renderer1.id).unwrap(),
        SurfaceIMGUsage::Storage(PreTypeSurfaceIMG::DefaultDepth),
        tak.render_task.exe_data_mut(exe.renderer1.id).unwrap(),
    );

    exe.renderer1.exe_surface_img(
        dat.surface_img.exe_data_mut(exe.renderer1.id).unwrap(),
        &mut dat.renderer_base,
        &mut tak.render_task.exe_data_mut(exe.renderer1.id).unwrap(),
    );

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////            RENDERER PIPELINE INIT              ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    dat.graphic_renderer_pipeline
        .alloc_data(Default::default(), Some(exe.renderer1.id))
        .end();

    let mut _pipeline = RenderPipelineD::<GraphicPipeLinePSO, GraphicPipeLinePCO>::build()
        .build_layout_info(renderer::cfg::env::PSO::DEFAULT_LAYOUT)
        .build_render_pass(renderer::cfg::env::PSO::DEFAULT_RENDER_PASS)
        .build_push_subpass(renderer::cfg::env::PSO::DEFAULT_SUBPASS_DESCRIPTION)
        .build_push_shader_stages(dat.shader_mod.exe_data_mut(exe.renderer1.id).unwrap());

    dat.graphic_renderer_pipeline
        .exe_data_mut(exe.renderer1.id)
        .unwrap()
        .alloc_data(_pipeline, Option::None)
        .end();

    exe.renderer1.create_pipeline_layout(
        RenderPipelineType::Graphic,
        tak.render_task.exe_data_mut(exe.renderer1.id).unwrap(),
    );

    exe.renderer1
        .create_graphic_pipeline_pass(tak.render_task.exe_data_mut(exe.renderer1.id).unwrap());

    exe.renderer1.exe_graphic_pipeline(
        dat.graphic_renderer_pipeline
            .exe_data_mut(exe.renderer1.id)
            .unwrap(),
        tak.render_task.exe_data_mut(exe.renderer1.id).unwrap(),
    );

    exe.renderer1
        .create_graphic_pipeline(tak.render_task.exe_data_mut(exe.renderer1.id).unwrap());

    exe.renderer1.exe_graphic_pipeline(
        dat.graphic_renderer_pipeline
            .exe_data_mut(exe.renderer1.id)
            .unwrap(),
        tak.render_task.exe_data_mut(exe.renderer1.id).unwrap(),
    );

    buf.shader_binary_code.release_buffer();

    //dev_stop!();

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////            RENDERER BUFFER INIT                ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    // FBO
    exe.renderer1
        .create_fbo(tak.render_task.exe_data_mut(exe.renderer1.id).unwrap());

    exe.renderer1.exe_fbo(
        dat.fbo.exe_data_mut(exe.renderer1.id).unwrap(),
        dat.surface_img.exe_data_mut(exe.renderer1.id).unwrap(),
        dat.graphic_renderer_pipeline
            .exe_data_mut(exe.renderer1.id)
            .unwrap(),
        tak.render_task.exe_data_mut(exe.renderer1.id).unwrap(),
    );
    // CBO
    exe.renderer1.create_cmd_buffer(
        dat.surface_img.exe_data_index(exe.renderer1.id).unwrap(),
        vk::CommandBufferLevel::PRIMARY.as_raw(),
        tak.render_task.exe_data_mut(exe.renderer1.id).unwrap(),
    );

    // VBO
    //todo!();
    // exe.renderer1.create_vbo(tin);



    exe.renderer1.exe_cmdbuffer(
        dat.cmd_buffer.exe_data_mut(exe.renderer1.id).unwrap(),
        &mut tak.render_task.exe_data_mut(exe.renderer1.id).unwrap(),
    );

    //dev_stop!();

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////            RENDERER COMMAND INIT               ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    exe.render_cmd.link_renderer(&exe.renderer1);

    exe.render_cmd
        .begin_cmd_sync(dat.cmd_buffer.exe_data_ref(exe.renderer1.id).unwrap(), 0);

    exe.render_cmd.bind_pipe_sync(
        dat.cmd_buffer.exe_data_mut(exe.renderer1.id).unwrap(),
        dat.graphic_renderer_pipeline
            .exe_data_mut(exe.renderer1.id)
            .unwrap(),
        0,
        0,
    );

    // dev_stop!();
    // Error_todo

    exe.render_cmd.begin_render_pass_sync(
        0,
        0,
        0,
        dat.graphic_renderer_pipeline
            .exe_data_mut(exe.renderer1.id)
            .unwrap(),
        dat.cmd_buffer.exe_data_mut(exe.renderer1.id).unwrap(),
        dat.fbo.exe_data_mut(exe.renderer1.id).unwrap(),
    );

    

    exe.render_cmd
        .draw_sync(dat.cmd_buffer.exe_data_mut(exe.renderer1.id).unwrap());

    exe.render_cmd
        .end_render_pass_sync(0, dat.cmd_buffer.exe_data_mut(exe.renderer1.id).unwrap());

    // 记得重新开回来
    //dev_stop!();

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////             INPUT SYS                          ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    //stop!();

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////              MAIN LOOP                         ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    let mut count = 0;

    while unsafe { workarea::WORKAREA_CLOSE == false } {
        count = count + 1;
        utc_counter.from1970(&exe.timer.systime().as_secs());

        unsafe {
            exe.input.clear();
            exe.win_input.peek();
            exe.win_window.update_window();
            exe.win_window.show_window();
        }

        buf.release();

        std::thread::sleep(std::time::Duration::new(0, 0010000000));

        #[cfg(feature = "log_print_during_dev")]
        log::print2console_once();
        log::output_clear_log2file_once();
    }

    ////////////////////////////////////////////////////////
    ////                                                ////
    ////               DROP                             ////
    ////                                                ////
    ////////////////////////////////////////////////////////

    exe.win_window.drop();

    std::thread::sleep(std::time::Duration::new(3, 0));
    return Ok(crate::send2logger_dev!(
        crate::log::code::TYPE_DEFAULT
            | crate::log::code::CONDI_DEFAULT
            | crate::log::code::FILE_MAIN
            | 0
            | 0
    ));
}

#[derive(Default)]
struct TaskM {
    render_task: Datum<Datum<TaskQueue<RendererTask>>>,
    decoder_task: Datum<Datum<TaskQueue<ShaderDecoderTask>>>,
    rendercmd_task: Datum<Datum<TaskQueue<RenderCmdTask>>>,
}

#[derive(Default, crate::dse_macros::BufferMImplement)]
// #[derive(Default)]
struct BufferM {
    shader_source_code: Buffers<ShaderTextD>,
    shader_binary_code: Buffers<ShaderResult<Vec<u32>>>,
}

#[derive(Default)]
struct DatumM {
    application: ApplicationD,
    renderer_base: AshVkBaseD,
    graphic_renderer_pipeline:
        Datum<Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>>,
    cmd_buffer: Datum<Datum<vk::CommandBuffer>>,
    fbo: Datum<Datum<vk::Framebuffer>>,
    surface_img: Datum<Datum<SurfaceIMG>>,
    shader_mod: Datum<Datum<ShaderModuleD>>,
    model: Datum<Datum<ModelD>>,
    transform: Datum<Datum<TransformD>>,
    mesh: Datum<Datum<MeshD>>,
}

#[derive(Default, crate::dse_macros::ExecuteMImplement)]
struct ExecuteM {
    timer: TimerE,
    win_input: WinInputE,
    input: InputE,
    win_window: WinWinodwE,
    resource_loader: ResourceE,
    shader_decoder: ShaderDecoderE,
    model: ModelE,
    renderer1: RendererE,
    render_cmd: CommandRenderE,
}
