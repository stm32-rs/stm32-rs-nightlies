///Register `DFLT0CR` reader
pub type R = crate::R<DFLT0CRrs>;
///Register `DFLT0CR` writer
pub type W = crate::W<DFLT0CRrs>;
///Field `DFLTEN` writer - DFLT0 enable This bit is set and cleared by software. It is used to control the start of acquisition of the DFLT0 path. This bit behavior depends on ACQMOD\[2:0\] and external events. The serial or parallel interface delivering the samples must be enabled as well.
pub type DFLTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMA requests enable This bit is set and cleared by software. It is used to control the generation of DMA request to transfer the processed samples into the memory. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMA requests enable This bit is set and cleared by software. It is used to control the generation of DMA request to transfer the processed samples into the memory. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FTH` reader - RXFIFO threshold selection This bit is set and cleared by software. It is used to select the RXFIFO threshold. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
pub type FTH_R = crate::BitReader;
///Field `FTH` writer - RXFIFO threshold selection This bit is set and cleared by software. It is used to select the RXFIFO threshold. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
pub type FTH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACQMOD` reader - DFLT0 trigger mode This field is set and cleared by software. It is used to select the filter trigger mode. others: same as 000 Note: This field can be write-protected (see Section 46.4.13: Register protection for details)..
pub type ACQMOD_R = crate::FieldReader;
///Field `ACQMOD` writer - DFLT0 trigger mode This field is set and cleared by software. It is used to select the filter trigger mode. others: same as 000 Note: This field can be write-protected (see Section 46.4.13: Register protection for details)..
pub type ACQMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TRGSENS` reader - DFLT0 trigger sensitivity selection This field is set and cleared by software. It is used to select the trigger sensitivity of the external signals When the trigger source is TRGO, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
pub type TRGSENS_R = crate::BitReader;
///Field `TRGSENS` writer - DFLT0 trigger sensitivity selection This field is set and cleared by software. It is used to select the trigger sensitivity of the external signals When the trigger source is TRGO, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
pub type TRGSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGSRC` reader - DFLT0 trigger signal selection This field is set and cleared by software. It is used to select which external signals trigger DFLT0. others: Reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type TRGSRC_R = crate::FieldReader;
///Field `TRGSRC` writer - DFLT0 trigger signal selection This field is set and cleared by software. It is used to select which external signals trigger DFLT0. others: Reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type TRGSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `NBDIS` reader - Number of samples to be discarded This field is set and cleared by software. It is used to define the number of samples to be discarded every time DFLT0 is re-started. ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type NBDIS_R = crate::FieldReader;
///Field `NBDIS` writer - Number of samples to be discarded This field is set and cleared by software. It is used to define the number of samples to be discarded every time DFLT0 is re-started. ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
pub type NBDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DFLTRUN` reader - DFLT0 run status flag This bit is set and cleared by hardware. It indicates if DFLT0 is running or not.
pub type DFLTRUN_R = crate::BitReader;
///Field `DFLTACTIVE` reader - DFLT0 active flag This bit is set and cleared by hardware. It indicates if DFLT0 is active: can be running or waiting for events.
pub type DFLTACTIVE_R = crate::BitReader;
impl R {
    ///Bit 1 - DMA requests enable This bit is set and cleared by software. It is used to control the generation of DMA request to transfer the processed samples into the memory. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXFIFO threshold selection This bit is set and cleared by software. It is used to select the RXFIFO threshold. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:6 - DFLT0 trigger mode This field is set and cleared by software. It is used to select the filter trigger mode. others: same as 000 Note: This field can be write-protected (see Section 46.4.13: Register protection for details)..
    #[inline(always)]
    pub fn acqmod(&self) -> ACQMOD_R {
        ACQMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - DFLT0 trigger sensitivity selection This field is set and cleared by software. It is used to select the trigger sensitivity of the external signals When the trigger source is TRGO, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn trgsens(&self) -> TRGSENS_R {
        TRGSENS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:15 - DFLT0 trigger signal selection This field is set and cleared by software. It is used to select which external signals trigger DFLT0. others: Reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 20:27 - Number of samples to be discarded This field is set and cleared by software. It is used to define the number of samples to be discarded every time DFLT0 is re-started. ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn nbdis(&self) -> NBDIS_R {
        NBDIS_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bit 30 - DFLT0 run status flag This bit is set and cleared by hardware. It indicates if DFLT0 is running or not.
    #[inline(always)]
    pub fn dfltrun(&self) -> DFLTRUN_R {
        DFLTRUN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DFLT0 active flag This bit is set and cleared by hardware. It indicates if DFLT0 is active: can be running or waiting for events.
    #[inline(always)]
    pub fn dfltactive(&self) -> DFLTACTIVE_R {
        DFLTACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFLT0CR")
            .field("dmaen", &self.dmaen())
            .field("fth", &self.fth())
            .field("acqmod", &self.acqmod())
            .field("trgsens", &self.trgsens())
            .field("trgsrc", &self.trgsrc())
            .field("nbdis", &self.nbdis())
            .field("dfltrun", &self.dfltrun())
            .field("dfltactive", &self.dfltactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFLT0 enable This bit is set and cleared by software. It is used to control the start of acquisition of the DFLT0 path. This bit behavior depends on ACQMOD\[2:0\] and external events. The serial or parallel interface delivering the samples must be enabled as well.
    #[inline(always)]
    pub fn dflten(&mut self) -> DFLTEN_W<DFLT0CRrs> {
        DFLTEN_W::new(self, 0)
    }
    ///Bit 1 - DMA requests enable This bit is set and cleared by software. It is used to control the generation of DMA request to transfer the processed samples into the memory. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<DFLT0CRrs> {
        DMAEN_W::new(self, 1)
    }
    ///Bit 2 - RXFIFO threshold selection This bit is set and cleared by software. It is used to select the RXFIFO threshold. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W<DFLT0CRrs> {
        FTH_W::new(self, 2)
    }
    ///Bits 4:6 - DFLT0 trigger mode This field is set and cleared by software. It is used to select the filter trigger mode. others: same as 000 Note: This field can be write-protected (see Section 46.4.13: Register protection for details)..
    #[inline(always)]
    pub fn acqmod(&mut self) -> ACQMOD_W<DFLT0CRrs> {
        ACQMOD_W::new(self, 4)
    }
    ///Bit 8 - DFLT0 trigger sensitivity selection This field is set and cleared by software. It is used to select the trigger sensitivity of the external signals When the trigger source is TRGO, TRGSENS value is not taken into account. When TRGO is selected, the sensitivity is forced to falling edge. Note: This bit can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn trgsens(&mut self) -> TRGSENS_W<DFLT0CRrs> {
        TRGSENS_W::new(self, 8)
    }
    ///Bits 12:15 - DFLT0 trigger signal selection This field is set and cleared by software. It is used to select which external signals trigger DFLT0. others: Reserved Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn trgsrc(&mut self) -> TRGSRC_W<DFLT0CRrs> {
        TRGSRC_W::new(self, 12)
    }
    ///Bits 20:27 - Number of samples to be discarded This field is set and cleared by software. It is used to define the number of samples to be discarded every time DFLT0 is re-started. ... Note: This field can be write-protected (see Section 46.4.13: Register protection for details).
    #[inline(always)]
    pub fn nbdis(&mut self) -> NBDIS_W<DFLT0CRrs> {
        NBDIS_W::new(self, 20)
    }
}
/**ADF digital filter control register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ADF:DFLT0CR)*/
pub struct DFLT0CRrs;
impl crate::RegisterSpec for DFLT0CRrs {
    type Ux = u32;
}
///`read()` method returns [`dflt0cr::R`](R) reader structure
impl crate::Readable for DFLT0CRrs {}
///`write(|w| ..)` method takes [`dflt0cr::W`](W) writer structure
impl crate::Writable for DFLT0CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFLT0CR to value 0
impl crate::Resettable for DFLT0CRrs {}
