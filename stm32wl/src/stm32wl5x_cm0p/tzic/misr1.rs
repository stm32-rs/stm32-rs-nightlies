///Register `MISR1` reader
pub type R = crate::R<MISR1rs>;
///Field `TZICMF` reader - TZICMF
pub type TZICMF_R = crate::BitReader;
///Field `TZSCMF` reader - TZSCMF
pub type TZSCMF_R = crate::BitReader;
///Field `AESMF` reader - AESMF
pub type AESMF_R = crate::BitReader;
///Field `RNGMF` reader - RNGMF
pub type RNGMF_R = crate::BitReader;
///Field `SUBGHZSPIMF` reader - SUBGHZSPIMF
pub type SUBGHZSPIMF_R = crate::BitReader;
///Field `PWRMF` reader - PWRMF
pub type PWRMF_R = crate::BitReader;
///Field `FLASHIFMF` reader - FLASHIFMF
pub type FLASHIFMF_R = crate::BitReader;
///Field `DMA1MF` reader - DMA1MF
pub type DMA1MF_R = crate::BitReader;
///Field `DMA2MF` reader - DMA2MF
pub type DMA2MF_R = crate::BitReader;
///Field `DMAMUX1MF` reader - DMAMUX1MF
pub type DMAMUX1MF_R = crate::BitReader;
///Field `FLASHMF` reader - FLASHMF
pub type FLASHMF_R = crate::BitReader;
///Field `SRAM1MF` reader - SRAM1MF
pub type SRAM1MF_R = crate::BitReader;
///Field `SRAM2MF` reader - SRAM2MF
pub type SRAM2MF_R = crate::BitReader;
///Field `PKAMF` reader - PKAMF
pub type PKAMF_R = crate::BitReader;
impl R {
    ///Bit 0 - TZICMF
    #[inline(always)]
    pub fn tzicmf(&self) -> TZICMF_R {
        TZICMF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TZSCMF
    #[inline(always)]
    pub fn tzscmf(&self) -> TZSCMF_R {
        TZSCMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AESMF
    #[inline(always)]
    pub fn aesmf(&self) -> AESMF_R {
        AESMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RNGMF
    #[inline(always)]
    pub fn rngmf(&self) -> RNGMF_R {
        RNGMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SUBGHZSPIMF
    #[inline(always)]
    pub fn subghzspimf(&self) -> SUBGHZSPIMF_R {
        SUBGHZSPIMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PWRMF
    #[inline(always)]
    pub fn pwrmf(&self) -> PWRMF_R {
        PWRMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FLASHIFMF
    #[inline(always)]
    pub fn flashifmf(&self) -> FLASHIFMF_R {
        FLASHIFMF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA1MF
    #[inline(always)]
    pub fn dma1mf(&self) -> DMA1MF_R {
        DMA1MF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DMA2MF
    #[inline(always)]
    pub fn dma2mf(&self) -> DMA2MF_R {
        DMA2MF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMAMUX1MF
    #[inline(always)]
    pub fn dmamux1mf(&self) -> DMAMUX1MF_R {
        DMAMUX1MF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - FLASHMF
    #[inline(always)]
    pub fn flashmf(&self) -> FLASHMF_R {
        FLASHMF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SRAM1MF
    #[inline(always)]
    pub fn sram1mf(&self) -> SRAM1MF_R {
        SRAM1MF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SRAM2MF
    #[inline(always)]
    pub fn sram2mf(&self) -> SRAM2MF_R {
        SRAM2MF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PKAMF
    #[inline(always)]
    pub fn pkamf(&self) -> PKAMF_R {
        PKAMF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR1")
            .field("tzicmf", &self.tzicmf())
            .field("tzscmf", &self.tzscmf())
            .field("aesmf", &self.aesmf())
            .field("rngmf", &self.rngmf())
            .field("subghzspimf", &self.subghzspimf())
            .field("pwrmf", &self.pwrmf())
            .field("flashifmf", &self.flashifmf())
            .field("dma1mf", &self.dma1mf())
            .field("dma2mf", &self.dma2mf())
            .field("dmamux1mf", &self.dmamux1mf())
            .field("flashmf", &self.flashmf())
            .field("sram1mf", &self.sram1mf())
            .field("sram2mf", &self.sram2mf())
            .field("pkamf", &self.pkamf())
            .finish()
    }
}
/**TZIC status register 1

You can [`read`](crate::Reg::read) this register and get [`misr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#TZIC:MISR1)*/
pub struct MISR1rs;
impl crate::RegisterSpec for MISR1rs {
    type Ux = u32;
}
///`read()` method returns [`misr1::R`](R) reader structure
impl crate::Readable for MISR1rs {}
///`reset()` method sets MISR1 to value 0
impl crate::Resettable for MISR1rs {}
