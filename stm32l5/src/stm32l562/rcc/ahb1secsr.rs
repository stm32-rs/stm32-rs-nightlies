#[doc = "Reader of register AHB1SECSR"]
pub type R = crate::R<u32, super::AHB1SECSR>;
#[doc = "Reader of field `ICACHESECF`"]
pub type ICACHESECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GTZCSECF`"]
pub type GTZCSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSCSECF`"]
pub type TSCSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRCSECF`"]
pub type CRCSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRAM1SECF`"]
pub type SRAM1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLASHSECF`"]
pub type FLASHSECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAMUX1SECF`"]
pub type DMAMUX1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA2SECF`"]
pub type DMA2SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA1SECF`"]
pub type DMA1SECF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 23 - ICACHESECF"]
    #[inline(always)]
    pub fn icachesecf(&self) -> ICACHESECF_R {
        ICACHESECF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - GTZCSECF"]
    #[inline(always)]
    pub fn gtzcsecf(&self) -> GTZCSECF_R {
        GTZCSECF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TSCSECF"]
    #[inline(always)]
    pub fn tscsecf(&self) -> TSCSECF_R {
        TSCSECF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRCSECF"]
    #[inline(always)]
    pub fn crcsecf(&self) -> CRCSECF_R {
        CRCSECF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAM1SECF"]
    #[inline(always)]
    pub fn sram1secf(&self) -> SRAM1SECF_R {
        SRAM1SECF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FLASHSECF"]
    #[inline(always)]
    pub fn flashsecf(&self) -> FLASHSECF_R {
        FLASHSECF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1SECF"]
    #[inline(always)]
    pub fn dmamux1secf(&self) -> DMAMUX1SECF_R {
        DMAMUX1SECF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2SECF"]
    #[inline(always)]
    pub fn dma2secf(&self) -> DMA2SECF_R {
        DMA2SECF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA1SECF"]
    #[inline(always)]
    pub fn dma1secf(&self) -> DMA1SECF_R {
        DMA1SECF_R::new((self.bits & 0x01) != 0)
    }
}
