#[doc = "Reader of register GICD_PPISR"]
pub type R = crate::R<u32, super::GICD_PPISR>;
#[doc = "Reader of field `PPI6`"]
pub type PPI6_R = crate::R<bool, bool>;
#[doc = "Reader of field `PPI5`"]
pub type PPI5_R = crate::R<bool, bool>;
#[doc = "Reader of field `PPI4`"]
pub type PPI4_R = crate::R<bool, bool>;
#[doc = "Reader of field `PPI0`"]
pub type PPI0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PPI1`"]
pub type PPI1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PPI2`"]
pub type PPI2_R = crate::R<bool, bool>;
#[doc = "Reader of field `PPI3`"]
pub type PPI3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 9 - PPI6"]
    #[inline(always)]
    pub fn ppi6(&self) -> PPI6_R {
        PPI6_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PPI5"]
    #[inline(always)]
    pub fn ppi5(&self) -> PPI5_R {
        PPI5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PPI4"]
    #[inline(always)]
    pub fn ppi4(&self) -> PPI4_R {
        PPI4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PPI0"]
    #[inline(always)]
    pub fn ppi0(&self) -> PPI0_R {
        PPI0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PPI1"]
    #[inline(always)]
    pub fn ppi1(&self) -> PPI1_R {
        PPI1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PPI2"]
    #[inline(always)]
    pub fn ppi2(&self) -> PPI2_R {
        PPI2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PPI3"]
    #[inline(always)]
    pub fn ppi3(&self) -> PPI3_R {
        PPI3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
