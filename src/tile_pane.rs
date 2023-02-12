use std::path::PathBuf;

use iced::{widget::{scrollable, image}, Length, Element};

const DEFAULT_TILE_SIZE: u16 = 128;

#[derive(Debug, Clone, Copy)]
pub enum ScrollMessage {
    ScrollToBeginning,
    Scrolled(scrollable::RelativeOffset),
    ZoomChanged(u16),
}

pub struct TilePane {
    tile_size: u16,
    scroll_offset: scrollable::RelativeOffset,
    images: Vec<ImageTile>,
}

impl TilePane {
    pub fn new() -> Self {
        Self::from_images(Vec::new())
    }

    pub fn from_images(images: Vec<ImageTile>) -> Self {
        Self {
            tile_size: DEFAULT_TILE_SIZE,
            scroll_offset: scrollable::RelativeOffset::START,
            images,
        }
    }
}



pub struct ImageTile {
    uid: u32,
    path: PathBuf,
    handle: image::Handle,
}

impl ImageTile {
    pub fn load(uid: u32, path: &str) -> Self {
        Self {
            uid,
            path: path.into(),
            handle: image::Handle::from_path(path),
        }
    }

    pub fn view(&self, size: Length) -> Element<ScrollMessage> {
        image::Image::new(self.handle.clone())
            .width(size)
            .height(size)
            .into()
    }
}
