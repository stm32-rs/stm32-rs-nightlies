#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `TEIF8`"]
pub type TEIF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF8`"]
pub type HTIF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF8`"]
pub type TCIF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF8`"]
pub type GIF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF7`"]
pub type TEIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF7`"]
pub type HTIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF7`"]
pub type TCIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF7`"]
pub type GIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF6`"]
pub type TEIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF6`"]
pub type HTIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF6`"]
pub type TCIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF6`"]
pub type GIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF5`"]
pub type TEIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF5`"]
pub type HTIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF5`"]
pub type TCIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF5`"]
pub type GIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF4`"]
pub type TEIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF4`"]
pub type HTIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF4`"]
pub type TCIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF4`"]
pub type GIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF3`"]
pub type TEIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF3`"]
pub type HTIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF3`"]
pub type TCIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF3`"]
pub type GIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF2`"]
pub type TEIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF2`"]
pub type HTIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF2`"]
pub type TCIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF2`"]
pub type GIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF1`"]
pub type TEIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF1`"]
pub type HTIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF1`"]
pub type TCIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF1`"]
pub type GIF1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - TEIF8"]
    #[inline(always)]
    pub fn teif8(&self) -> TEIF8_R {
        TEIF8_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - HTIF8"]
    #[inline(always)]
    pub fn htif8(&self) -> HTIF8_R {
        HTIF8_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TCIF8"]
    #[inline(always)]
    pub fn tcif8(&self) -> TCIF8_R {
        TCIF8_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - GIF8"]
    #[inline(always)]
    pub fn gif8(&self) -> GIF8_R {
        GIF8_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TEIF7"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - HTIF7"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TCIF7"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GIF7"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TEIF6"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - HTIF6"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TCIF6"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GIF6"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TEIF5"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HTIF5"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TCIF5"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GIF5"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TEIF4"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - HTIF4"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TCIF4"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GIF4"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TEIF3"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HTIF3"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TCIF3"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GIF3"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TEIF2"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HTIF2"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TCIF2"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GIF2"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TEIF1"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HTIF1"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TCIF1"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GIF1"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 0x01) != 0)
    }
}
