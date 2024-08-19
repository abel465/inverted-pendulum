mod controller;
mod graphics;
mod ml;
mod pendulum;

use std::borrow::Cow;

struct CompiledShaderModules {
    spv_module: wgpu::ShaderModuleDescriptorSpirV<'static>,
}

fn maybe_watch(
    on_watch: Option<Box<dyn FnMut(CompiledShaderModules) + Send + 'static>>,
) -> CompiledShaderModules {
    {
        use spirv_builder::{CompileResult, MetadataPrintout, SpirvBuilder};
        use std::path::PathBuf;
        // Hack: spirv_builder builds into a custom directory if running under cargo, to not
        // deadlock, and the default target directory if not. However, packages like `proc-macro2`
        // have different configurations when being built here vs. when building
        // rustc_codegen_spirv normally, so we *want* to build into a separate target directory, to
        // not have to rebuild half the crate graph every time we run. So, pretend we're running
        // under cargo by setting these environment variables.
        std::env::set_var(
            "OUT_DIR",
            option_env!("SHADERS_TARGET_DIR").unwrap_or(env!("OUT_DIR")),
        );
        std::env::set_var("PROFILE", env!("PROFILE"));
        let manifest_dir = option_env!("SHADERS_DIR").unwrap_or(env!("CARGO_MANIFEST_DIR"));
        let crate_path = [manifest_dir, "..", "shader"]
            .iter()
            .copied()
            .collect::<PathBuf>();

        let builder = SpirvBuilder::new(crate_path, "spirv-unknown-vulkan1.1")
            .print_metadata(MetadataPrintout::None)
            .shader_panic_strategy(spirv_builder::ShaderPanicStrategy::SilentExit);
        let initial_result = if let Some(mut f) = on_watch {
            builder
                .watch(move |compile_result| f(handle_compile_result(compile_result)))
                .expect("Configuration is correct for watching")
        } else {
            builder.build().unwrap()
        };
        fn handle_compile_result(compile_result: CompileResult) -> CompiledShaderModules {
            let path = compile_result.module.unwrap_single();
            let data = std::fs::read(path).unwrap();
            let spirv = Cow::Owned(wgpu::util::make_spirv_raw(&data).into_owned());
            CompiledShaderModules {
                spv_module: wgpu::ShaderModuleDescriptorSpirV {
                    label: None,
                    source: spirv,
                },
            }
        }
        handle_compile_result(initial_result)
    }
}

pub fn main() {
    graphics::start();
}
