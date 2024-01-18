use std::cell::RefCell;
use std::collections::HashMap;
use std::num::NonZeroU16;

use smithay::backend::renderer::element::{Element, Kind};
use smithay::backend::renderer::gles::element::PixelShaderElement;
use smithay::backend::renderer::gles::{
    GlesPixelProgram, GlesRenderer, Uniform, UniformName, UniformType,
};
use smithay::desktop::Window;
use smithay::utils::IsAlive;

use crate::render_helpers::PixelShaderRenderElement;

#[derive(Debug)]
pub struct RoundingShader {
    shader: GlesPixelProgram,
}

struct RoundingShaderElements(RefCell<HashMap<Window, PixelShaderRenderElement>>);

pub type RoundingRenderElement = PixelShaderRenderElement;

impl RoundingShader {
    pub fn init(renderer: &mut GlesRenderer) {
        let shader = renderer
            .compile_custom_pixel_shader(
                include_str!("shaders/rounding.frag"),
                &[UniformName::new("radius", UniformType::_1f)],
            )
            .unwrap();

        renderer
            .egl_context()
            .user_data()
            .insert_if_missing(|| RoundingShader { shader });

        renderer
            .egl_context()
            .user_data()
            .insert_if_missing(|| RoundingShaderElements(RefCell::new(HashMap::new())));
    }

    pub fn get(renderer: &GlesRenderer) -> &RoundingShader {
        renderer
            .egl_context()
            .user_data()
            .get::<RoundingShader>()
            .expect("Rounding Shader not initialized")
    }

    pub fn element(
        renderer: &mut GlesRenderer,
        window: &Window,
        radius: NonZeroU16,
    ) -> PixelShaderRenderElement {
        let elements = &mut renderer
            .egl_context()
            .user_data()
            .get::<RoundingShaderElements>()
            .expect("Rounding Shader not initialized")
            .0
            .borrow_mut();

        if let Some(elem) = elements.get_mut(window) {
            if elem.geometry(1.0.into()).to_logical(1) != window.bbox() {
                elem.0.resize(window.bbox(), None);
            }

            elem.clone()
        } else {
            let elem = PixelShaderRenderElement(PixelShaderElement::new(
                Self::get(renderer).shader.clone(),
                window.bbox(),
                None,
                1.0,
                vec![Uniform::new("radius", radius.get() as f32)],
                Kind::Unspecified,
            ));
            elements.insert(window.clone(), elem.clone());
            elem
        }
    }

    pub fn cleanup(renderer: &mut GlesRenderer) {
        let elements = &mut renderer
            .egl_context()
            .user_data()
            .get::<RoundingShaderElements>()
            .expect("Rounding Shader not initialized")
            .0
            .borrow_mut();

        elements.retain(|w, _| w.alive());
    }
}
