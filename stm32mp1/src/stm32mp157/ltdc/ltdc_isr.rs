#[doc = "Reader of register LTDC_ISR"]
pub type R = crate::R<u32, super::LTDC_ISR>;
#[doc = "Reader of field `LIF`"]
pub type LIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FUIF`"]
pub type FUIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TERRIF`"]
pub type TERRIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RRIF`"]
pub type RRIF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - LIF"]
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FUIF"]
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TERRIF"]
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RRIF"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
