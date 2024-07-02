///Register `ADF_DFLT0CR` reader
pub type R = crate::R<ADF_DFLT0CRrs>;
///Register `ADF_DFLT0CR` writer
pub type W = crate::W<ADF_DFLT0CRrs>;
///Field `DFLTEN` reader - DFLT0 enable
pub type DFLTEN_R = crate::BitReader;
///Field `DFLTEN` writer - DFLT0 enable
pub type DFLTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMA requests enable
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMA requests enable
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTH` reader - RXFIFO threshold selection
pub type FTH_R = crate::BitReader;
///Field `FTH` writer - RXFIFO threshold selection
pub type FTH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACQMOD` reader - DFLT0 trigger mode
pub type ACQMOD_R = crate::FieldReader;
///Field `ACQMOD` writer - DFLT0 trigger mode
pub type ACQMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRGSRC` reader - DFLT0 trigger signal selection
pub type TRGSRC_R = crate::FieldReader;
///Field `TRGSRC` writer - DFLT0 trigger signal selection
pub type TRGSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `NBDIS` reader - Number of samples to be discarded
pub type NBDIS_R = crate::FieldReader;
///Field `NBDIS` writer - Number of samples to be discarded
pub type NBDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DFLTRUN` reader - DFLT0 run status flag
pub type DFLTRUN_R = crate::BitReader;
///Field `DFLTRUN` writer - DFLT0 run status flag
pub type DFLTRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFLTACTIVE` reader - DFLT0 active flag
pub type DFLTACTIVE_R = crate::BitReader;
///Field `DFLTACTIVE` writer - DFLT0 active flag
pub type DFLTACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DFLT0 enable
    #[inline(always)]
    pub fn dflten(&self) -> DFLTEN_R {
        DFLTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA requests enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXFIFO threshold selection
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - DFLT0 trigger mode
    #[inline(always)]
    pub fn acqmod(&self) -> ACQMOD_R {
        ACQMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 12:15 - DFLT0 trigger signal selection
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 20:27 - Number of samples to be discarded
    #[inline(always)]
    pub fn nbdis(&self) -> NBDIS_R {
        NBDIS_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bit 30 - DFLT0 run status flag
    #[inline(always)]
    pub fn dfltrun(&self) -> DFLTRUN_R {
        DFLTRUN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DFLT0 active flag
    #[inline(always)]
    pub fn dfltactive(&self) -> DFLTACTIVE_R {
        DFLTACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADF_DFLT0CR")
            .field("dfltactive", &self.dfltactive())
            .field("dfltrun", &self.dfltrun())
            .field("nbdis", &self.nbdis())
            .field("trgsrc", &self.trgsrc())
            .field("acqmod", &self.acqmod())
            .field("fth", &self.fth())
            .field("dmaen", &self.dmaen())
            .field("dflten", &self.dflten())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFLT0 enable
    #[inline(always)]
    #[must_use]
    pub fn dflten(&mut self) -> DFLTEN_W<ADF_DFLT0CRrs> {
        DFLTEN_W::new(self, 0)
    }
    ///Bit 1 - DMA requests enable
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<ADF_DFLT0CRrs> {
        DMAEN_W::new(self, 1)
    }
    ///Bit 2 - RXFIFO threshold selection
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<ADF_DFLT0CRrs> {
        FTH_W::new(self, 2)
    }
    ///Bits 4:6 - DFLT0 trigger mode
    #[inline(always)]
    #[must_use]
    pub fn acqmod(&mut self) -> ACQMOD_W<ADF_DFLT0CRrs> {
        ACQMOD_W::new(self, 4)
    }
    ///Bits 12:15 - DFLT0 trigger signal selection
    #[inline(always)]
    #[must_use]
    pub fn trgsrc(&mut self) -> TRGSRC_W<ADF_DFLT0CRrs> {
        TRGSRC_W::new(self, 12)
    }
    ///Bits 20:27 - Number of samples to be discarded
    #[inline(always)]
    #[must_use]
    pub fn nbdis(&mut self) -> NBDIS_W<ADF_DFLT0CRrs> {
        NBDIS_W::new(self, 20)
    }
    ///Bit 30 - DFLT0 run status flag
    #[inline(always)]
    #[must_use]
    pub fn dfltrun(&mut self) -> DFLTRUN_W<ADF_DFLT0CRrs> {
        DFLTRUN_W::new(self, 30)
    }
    ///Bit 31 - DFLT0 active flag
    #[inline(always)]
    #[must_use]
    pub fn dfltactive(&mut self) -> DFLTACTIVE_W<ADF_DFLT0CRrs> {
        DFLTACTIVE_W::new(self, 31)
    }
}
/**ADF digital filter control register 0

You can [`read`](crate::Reg::read) this register and get [`adf_dflt0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_dflt0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADF1:ADF_DFLT0CR)*/
pub struct ADF_DFLT0CRrs;
impl crate::RegisterSpec for ADF_DFLT0CRrs {
    type Ux = u32;
}
///`read()` method returns [`adf_dflt0cr::R`](R) reader structure
impl crate::Readable for ADF_DFLT0CRrs {}
///`write(|w| ..)` method takes [`adf_dflt0cr::W`](W) writer structure
impl crate::Writable for ADF_DFLT0CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADF_DFLT0CR to value 0
impl crate::Resettable for ADF_DFLT0CRrs {
    const RESET_VALUE: u32 = 0;
}
