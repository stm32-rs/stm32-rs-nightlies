#[doc = "Reader of register DMAISR"]
pub type R = crate::R<u32, super::DMAISR>;
#[doc = "Reader of field `DC0IS`"]
pub type DC0IS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MTLIS`"]
pub type MTLIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MACIS`"]
pub type MACIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DMA Channel Interrupt Status"]
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status"]
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MAC Interrupt Status"]
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
