#[doc = "Reader of register DMA_HISR"]
pub type R = crate::R<u32, super::DMA_HISR>;
#[doc = "Reader of field `FEIF4`"]
pub type FEIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMEIF4`"]
pub type DMEIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF4`"]
pub type TEIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF4`"]
pub type HTIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF4`"]
pub type TCIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `FEIF5`"]
pub type FEIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMEIF5`"]
pub type DMEIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF5`"]
pub type TEIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF5`"]
pub type HTIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF5`"]
pub type TCIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `FEIF6`"]
pub type FEIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMEIF6`"]
pub type DMEIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF6`"]
pub type TEIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF6`"]
pub type HTIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF6`"]
pub type TCIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `FEIF7`"]
pub type FEIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMEIF7`"]
pub type DMEIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF7`"]
pub type TEIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF7`"]
pub type HTIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF7`"]
pub type TCIF7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - FEIF4"]
    #[inline(always)]
    pub fn feif4(&self) -> FEIF4_R {
        FEIF4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMEIF4"]
    #[inline(always)]
    pub fn dmeif4(&self) -> DMEIF4_R {
        DMEIF4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TEIF4"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HTIF4"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TCIF4"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FEIF5"]
    #[inline(always)]
    pub fn feif5(&self) -> FEIF5_R {
        FEIF5_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMEIF5"]
    #[inline(always)]
    pub fn dmeif5(&self) -> DMEIF5_R {
        DMEIF5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEIF5"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HTIF5"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TCIF5"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FEIF6"]
    #[inline(always)]
    pub fn feif6(&self) -> FEIF6_R {
        FEIF6_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DMEIF6"]
    #[inline(always)]
    pub fn dmeif6(&self) -> DMEIF6_R {
        DMEIF6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TEIF6"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - HTIF6"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TCIF6"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FEIF7"]
    #[inline(always)]
    pub fn feif7(&self) -> FEIF7_R {
        FEIF7_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMEIF7"]
    #[inline(always)]
    pub fn dmeif7(&self) -> DMEIF7_R {
        DMEIF7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TEIF7"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - HTIF7"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TCIF7"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
