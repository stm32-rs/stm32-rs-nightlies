///Register `DFLT3CR` reader
pub type R = crate::R<DFLT3CRrs>;
///Register `DFLT3CR` writer
pub type W = crate::W<DFLT3CRrs>;
///Field `DFLTEN` writer - Digital filter enable
pub type DFLTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMA requests enable
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMA requests enable
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTH` reader - RXFIFO Threshold selection
pub type FTH_R = crate::BitReader;
///Field `FTH` writer - RXFIFO Threshold selection
pub type FTH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACQMOD` reader - Digital filter trigger mode
pub type ACQMOD_R = crate::FieldReader;
///Field `ACQMOD` writer - Digital filter trigger mode
pub type ACQMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRGSENS` reader - Digital filter trigger sensitivity selection
pub type TRGSENS_R = crate::BitReader;
///Field `TRGSENS` writer - Digital filter trigger sensitivity selection
pub type TRGSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGSRC` reader - Digital filter trigger signal selection
pub type TRGSRC_R = crate::FieldReader;
///Field `TRGSRC` writer - Digital filter trigger signal selection
pub type TRGSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SNPSFMT` reader - Snapshot data format
pub type SNPSFMT_R = crate::BitReader;
///Field `SNPSFMT` writer - Snapshot data format
pub type SNPSFMT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBDIS` reader - Number of samples to be discarded
pub type NBDIS_R = crate::FieldReader;
///Field `NBDIS` writer - Number of samples to be discarded
pub type NBDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DFLTRUN` reader - Digital filter run status flag
pub type DFLTRUN_R = crate::BitReader;
///Field `DFLTACTIVE` reader - Digital filter active flag
pub type DFLTACTIVE_R = crate::BitReader;
impl R {
    ///Bit 1 - DMA requests enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXFIFO Threshold selection
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - Digital filter trigger mode
    #[inline(always)]
    pub fn acqmod(&self) -> ACQMOD_R {
        ACQMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - Digital filter trigger sensitivity selection
    #[inline(always)]
    pub fn trgsens(&self) -> TRGSENS_R {
        TRGSENS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:15 - Digital filter trigger signal selection
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - Snapshot data format
    #[inline(always)]
    pub fn snpsfmt(&self) -> SNPSFMT_R {
        SNPSFMT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:27 - Number of samples to be discarded
    #[inline(always)]
    pub fn nbdis(&self) -> NBDIS_R {
        NBDIS_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bit 30 - Digital filter run status flag
    #[inline(always)]
    pub fn dfltrun(&self) -> DFLTRUN_R {
        DFLTRUN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Digital filter active flag
    #[inline(always)]
    pub fn dfltactive(&self) -> DFLTACTIVE_R {
        DFLTACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT3CR")
            .field("dmaen", &self.dmaen())
            .field("fth", &self.fth())
            .field("acqmod", &self.acqmod())
            .field("trgsens", &self.trgsens())
            .field("trgsrc", &self.trgsrc())
            .field("snpsfmt", &self.snpsfmt())
            .field("nbdis", &self.nbdis())
            .field("dfltrun", &self.dfltrun())
            .field("dfltactive", &self.dfltactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - Digital filter enable
    #[inline(always)]
    pub fn dflten(&mut self) -> DFLTEN_W<'_, DFLT3CRrs> {
        DFLTEN_W::new(self, 0)
    }
    ///Bit 1 - DMA requests enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, DFLT3CRrs> {
        DMAEN_W::new(self, 1)
    }
    ///Bit 2 - RXFIFO Threshold selection
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<'_, DFLT3CRrs> {
        FTH_W::new(self, 2)
    }
    ///Bits 4:6 - Digital filter trigger mode
    #[inline(always)]
    pub fn acqmod(&mut self) -> ACQMOD_W<'_, DFLT3CRrs> {
        ACQMOD_W::new(self, 4)
    }
    ///Bit 8 - Digital filter trigger sensitivity selection
    #[inline(always)]
    pub fn trgsens(&mut self) -> TRGSENS_W<'_, DFLT3CRrs> {
        TRGSENS_W::new(self, 8)
    }
    ///Bits 12:15 - Digital filter trigger signal selection
    #[inline(always)]
    pub fn trgsrc(&mut self) -> TRGSRC_W<'_, DFLT3CRrs> {
        TRGSRC_W::new(self, 12)
    }
    ///Bit 16 - Snapshot data format
    #[inline(always)]
    pub fn snpsfmt(&mut self) -> SNPSFMT_W<'_, DFLT3CRrs> {
        SNPSFMT_W::new(self, 16)
    }
    ///Bits 20:27 - Number of samples to be discarded
    #[inline(always)]
    pub fn nbdis(&mut self) -> NBDIS_W<'_, DFLT3CRrs> {
        NBDIS_W::new(self, 20)
    }
}
/**MDF digital filter control register 3

You can [`read`](crate::Reg::read) this register and get [`dflt3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:DFLT3CR)*/
pub struct DFLT3CRrs;
impl crate::RegisterSpec for DFLT3CRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt3cr::R`](R) reader structure
impl crate::Readable for DFLT3CRrs {}
///`write(|w| ..)` method takes [`dflt3cr::W`](W) writer structure
impl crate::Writable for DFLT3CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT3CR to value 0
impl crate::Resettable for DFLT3CRrs {}
