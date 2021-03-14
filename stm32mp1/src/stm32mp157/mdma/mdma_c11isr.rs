#[doc = "Reader of register MDMA_C11ISR"]
pub type R = crate::R<u32, super::MDMA_C11ISR>;
#[doc = "Reader of field `TEIF`"]
pub type TEIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF`"]
pub type CTCIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF`"]
pub type BRTIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF`"]
pub type BTIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF`"]
pub type TCIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA`"]
pub type CRQA_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TEIF"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF"]
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF"]
    #[inline(always)]
    pub fn brtif(&self) -> BRTIF_R {
        BRTIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF"]
    #[inline(always)]
    pub fn btif(&self) -> BTIF_R {
        BTIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF"]
    #[inline(always)]
    pub fn tcif(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA"]
    #[inline(always)]
    pub fn crqa(&self) -> CRQA_R {
        CRQA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
