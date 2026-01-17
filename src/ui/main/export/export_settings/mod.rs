mod load;
use load::AsyncTextureLoader;

use super::*;

pub struct Texture {
    texture: Texture2D,
    average: [f32; 4],
    // noise: f32
}


impl Texture {
    pub fn from_texture(texture: Texture2D, settings: &ExportSettings) -> Self {
        Self {
            average: get_average(&texture, settings.col_sel),
            texture
        }
    }

    pub fn draw(&mut self, rect: Rect) {
        draw_texture_ex(
            &self.texture,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(rect.w, rect.h)),
                ..Default::default()
            }
        );
    }
}

fn get_average(texture: &Texture2D, col_sel: ColSelection) -> [f32; 4] {
    let image = texture.get_texture_data();

    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut sz = 0.0;
    let mut a = 0.0;

    let count = image.width() as f32 * image.height() as f32;

    for col in image.get_image_data() {
        let col = [
            col[0] as f32 / 255.0,
            col[1] as f32 / 255.0,
            col[2] as f32 / 255.0,
            col[3] as f32 / 255.0,
        ];

        a += col[3];

        let col = col_sel.col_from_rgba_arr(col);

        let col = col.to_wheel();

        sx += col.0;
        sy += col.1;
        sz += col.2;
    }

    sx /= count;
    sy /= count;
    sz /= count;

    let mut result = col_sel.col_from_wheel(sx, sy, sz).to_rgba();
    result[3] = a;
    result
}

enum ColTex {
    Col([f32; 4]),
    Tex(Texture)
}

#[derive(Clone, Copy)]
pub struct ExportSettings {
    temperature: f32,
    col_sel: ColSelection,
}

pub struct ExportSettingsNode {
    texture_loader: Option<AsyncTextureLoader>,
    settings: ExportSettings,
}

impl ExportSettingsNode {
    pub fn generate(&self) -> Result<(), String> {
        if let Some(_) = self.texture_loader {
            return Err("Already generating an export.\nWait for that to finish or cancel it.".to_string())
        }
        let folder = todo!(); // Take from store
        self.texture_loader = Some(AsyncTextureLoader::new_recursive(folder, self.settings)?);
        Ok(())
    }
}

impl New for ExportSettingsNode {
    fn new(handler: &mut GenHandler) -> Self {
        let settings = todo!(); // Generate from save.json
        Self {
            texture_loader: None,
            settings
        }
    }
}

impl Node for ExportSettingsNode {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        disabled_ui_button(Rect::new(150.0, 150.0, 300.0, 38.0), "There are no settings yet", DISABLEDCOL)
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode> {
        node.hit_detect_children_and_self(pos, store)
    }
}