use eframe::{egui, epi};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DecompApp {
    load_poly: bool,
    draw_poly: bool,
    triangulate: bool,
    decompose: bool,
}

impl Default for DecompApp {
    fn default() -> Self {
        Self {
            load_poly: false,
            draw_poly: false,
            triangulate: false,
            decompose: false,
        }
    }
}

impl epi::App for DecompApp {
    fn name(&self) -> &str {
        "Convex decomposition for simple polygons"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
        }
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 180).into()
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            load_poly,
            draw_poly,
            triangulate,
            decompose,
        } = self;

        
    }
}
