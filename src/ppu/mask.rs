bitfield! {
    #[derive(Copy, Clone)]
    pub struct Mask(u8);
    impl Debug;
    pub greyscale,              _: 0;
    pub show_background_left_8, _: 1;
    pub show_sprites_left_8,    _: 2;
    pub show_background,        _: 3;
    pub show_sprites,           _: 4;
    pub emphasize_red,          _: 5;
    pub emphasize_green,        _: 6;
    pub emphasize_blue,         _: 7;
}

impl Mask {
    pub fn rendering(&self) -> bool {
        self.show_sprites() || self.show_background()
    }

    pub fn rendering_background(&self, x: usize) -> bool {
        self.show_background() && (self.show_background_left_8() || x >= 8)
    }

    pub fn rendering_sprites(&self, x: usize) -> bool {
        self.show_sprites() && (self.show_sprites_left_8() || x >= 8)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_rendering_sprites() {
        // Sprites shown, but left 8 pixels hidden
        assert!(!Mask(0b0001_0000).rendering_sprites(7));
        assert!(Mask(0b0001_0000).rendering_sprites(8));

        // Sprites shown, and left 8 pixels shown
        assert!(Mask(0b0001_0100).rendering_sprites(7));
        assert!(Mask(0b0001_0100).rendering_sprites(8));

        // Sprites not shown, and left 8 pixels shown
        assert!(!Mask(0b0000_0100).rendering_sprites(7));
        assert!(!Mask(0b0000_0100).rendering_sprites(8));
    }

    #[test]
    fn test_rendering_background() {
        // Background shown, but left 8 pixels hidden
        assert!(!Mask(0b0000_1000).rendering_background(7));
        assert!(Mask(0b0000_1000).rendering_background(8));

        // background shown, and left 8 pixels shown
        assert!(Mask(0b0000_1010).rendering_background(7));
        assert!(Mask(0b0000_1010).rendering_background(8));

        // background not shown, and left 8 pixels shown
        assert!(!Mask(0b0000_0010).rendering_background(7));
        assert!(!Mask(0b0000_0010).rendering_background(8));
    }
}
