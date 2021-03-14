#[doc = "Reader of register DMA_LISR"]
pub type R = crate::R<u32, super::DMA_LISR>;
#[doc = "Reader of field `FEIF0`"]
pub type FEIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMEIF0`"]
pub type DMEIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF0`"]
pub type TEIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF0`"]
pub type HTIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF0`"]
pub type TCIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `FEIF1`"]
pub type FEIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMEIF1`"]
pub type DMEIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF1`"]
pub type TEIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF1`"]
pub type HTIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF1`"]
pub type TCIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `FEIF2`"]
pub type FEIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMEIF2`"]
pub type DMEIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF2`"]
pub type TEIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF2`"]
pub type HTIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF2`"]
pub type TCIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `FEIF3`"]
pub type FEIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMEIF3`"]
pub type DMEIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEIF3`"]
pub type TEIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTIF3`"]
pub type HTIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF3`"]
pub type TCIF3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - FEIF0"]
    #[inline(always)]
    pub fn feif0(&self) -> FEIF0_R {
        FEIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMEIF0"]
    #[inline(always)]
    pub fn dmeif0(&self) -> DMEIF0_R {
        DMEIF0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TEIF0"]
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HTIF0"]
    #[inline(always)]
    pub fn htif0(&self) -> HTIF0_R {
        HTIF0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TCIF0"]
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FEIF1"]
    #[inline(always)]
    pub fn feif1(&self) -> FEIF1_R {
        FEIF1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMEIF1"]
    #[inline(always)]
    pub fn dmeif1(&self) -> DMEIF1_R {
        DMEIF1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEIF1"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HTIF1"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TCIF1"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FEIF2"]
    #[inline(always)]
    pub fn feif2(&self) -> FEIF2_R {
        FEIF2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DMEIF2"]
    #[inline(always)]
    pub fn dmeif2(&self) -> DMEIF2_R {
        DMEIF2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TEIF2"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - HTIF2"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TCIF2"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FEIF3"]
    #[inline(always)]
    pub fn feif3(&self) -> FEIF3_R {
        FEIF3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMEIF3"]
    #[inline(always)]
    pub fn dmeif3(&self) -> DMEIF3_R {
        DMEIF3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TEIF3"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - HTIF3"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TCIF3"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
