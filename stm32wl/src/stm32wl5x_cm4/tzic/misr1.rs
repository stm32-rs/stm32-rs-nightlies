#[doc = "Reader of register MISR1"]
pub type R = crate::R<u32, super::MISR1>;
#[doc = "Reader of field `TZICMF`"]
pub type TZICMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TZSCMF`"]
pub type TZSCMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `AESMF`"]
pub type AESMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RNGMF`"]
pub type RNGMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUBGHZSPIMF`"]
pub type SUBGHZSPIMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWRMF`"]
pub type PWRMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLASHIFMF`"]
pub type FLASHIFMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA1MF`"]
pub type DMA1MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA2MF`"]
pub type DMA2MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAMUX1MF`"]
pub type DMAMUX1MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLASHMF`"]
pub type FLASHMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRAM1MF`"]
pub type SRAM1MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRAM2MF`"]
pub type SRAM2MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PKAMF`"]
pub type PKAMF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TZICMF"]
    #[inline(always)]
    pub fn tzicmf(&self) -> TZICMF_R {
        TZICMF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZSCMF"]
    #[inline(always)]
    pub fn tzscmf(&self) -> TZSCMF_R {
        TZSCMF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AESMF"]
    #[inline(always)]
    pub fn aesmf(&self) -> AESMF_R {
        AESMF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RNGMF"]
    #[inline(always)]
    pub fn rngmf(&self) -> RNGMF_R {
        RNGMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPIMF"]
    #[inline(always)]
    pub fn subghzspimf(&self) -> SUBGHZSPIMF_R {
        SUBGHZSPIMF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWRMF"]
    #[inline(always)]
    pub fn pwrmf(&self) -> PWRMF_R {
        PWRMF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLASHIFMF"]
    #[inline(always)]
    pub fn flashifmf(&self) -> FLASHIFMF_R {
        FLASHIFMF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA1MF"]
    #[inline(always)]
    pub fn dma1mf(&self) -> DMA1MF_R {
        DMA1MF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA2MF"]
    #[inline(always)]
    pub fn dma2mf(&self) -> DMA2MF_R {
        DMA2MF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMAMUX1MF"]
    #[inline(always)]
    pub fn dmamux1mf(&self) -> DMAMUX1MF_R {
        DMAMUX1MF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FLASHMF"]
    #[inline(always)]
    pub fn flashmf(&self) -> FLASHMF_R {
        FLASHMF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SRAM1MF"]
    #[inline(always)]
    pub fn sram1mf(&self) -> SRAM1MF_R {
        SRAM1MF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SRAM2MF"]
    #[inline(always)]
    pub fn sram2mf(&self) -> SRAM2MF_R {
        SRAM2MF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PKAMF"]
    #[inline(always)]
    pub fn pkamf(&self) -> PKAMF_R {
        PKAMF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
