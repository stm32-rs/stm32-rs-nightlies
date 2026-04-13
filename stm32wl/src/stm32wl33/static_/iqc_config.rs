///Register `IQC_CONFIG` reader
pub type R = crate::R<IQC_CONFIGrs>;
///Register `IQC_CONFIG` writer
pub type W = crate::W<IQC_CONFIGrs>;
///Field `IQC_CORRECT_IN` reader - Correction value Input for the IQ compensation engine (to be used as starting point or when the engine is disabled).
pub type IQC_CORRECT_IN_R = crate::FieldReader<u32>;
///Field `IQC_CORRECT_IN` writer - Correction value Input for the IQ compensation engine (to be used as starting point or when the engine is disabled).
pub type IQC_CORRECT_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `LOAD_IQC_INIT` writer - Action bit to load the IQC_CORRECT_IN\[23:0\] bit field in the recirculation register when this bit is written to 1.
pub type LOAD_IQC_INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REUSE_CORRECTION` reader - Reuse last correction value
pub type REUSE_CORRECTION_R = crate::BitReader;
///Field `REUSE_CORRECTION` writer - Reuse last correction value
pub type REUSE_CORRECTION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IQC_ENABLE` reader - Enable IQC
pub type IQC_ENABLE_R = crate::BitReader;
///Field `IQC_ENABLE` writer - Enable IQC
pub type IQC_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:23 - Correction value Input for the IQ compensation engine (to be used as starting point or when the engine is disabled).
    #[inline(always)]
    pub fn iqc_correct_in(&self) -> IQC_CORRECT_IN_R {
        IQC_CORRECT_IN_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 30 - Reuse last correction value
    #[inline(always)]
    pub fn reuse_correction(&self) -> REUSE_CORRECTION_R {
        REUSE_CORRECTION_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Enable IQC
    #[inline(always)]
    pub fn iqc_enable(&self) -> IQC_ENABLE_R {
        IQC_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IQC_CONFIG")
            .field("iqc_correct_in", &self.iqc_correct_in())
            .field("reuse_correction", &self.reuse_correction())
            .field("iqc_enable", &self.iqc_enable())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Correction value Input for the IQ compensation engine (to be used as starting point or when the engine is disabled).
    #[inline(always)]
    pub fn iqc_correct_in(&mut self) -> IQC_CORRECT_IN_W<'_, IQC_CONFIGrs> {
        IQC_CORRECT_IN_W::new(self, 0)
    }
    ///Bit 29 - Action bit to load the IQC_CORRECT_IN\[23:0\] bit field in the recirculation register when this bit is written to 1.
    #[inline(always)]
    pub fn load_iqc_init(&mut self) -> LOAD_IQC_INIT_W<'_, IQC_CONFIGrs> {
        LOAD_IQC_INIT_W::new(self, 29)
    }
    ///Bit 30 - Reuse last correction value
    #[inline(always)]
    pub fn reuse_correction(&mut self) -> REUSE_CORRECTION_W<'_, IQC_CONFIGrs> {
        REUSE_CORRECTION_W::new(self, 30)
    }
    ///Bit 31 - Enable IQC
    #[inline(always)]
    pub fn iqc_enable(&mut self) -> IQC_ENABLE_W<'_, IQC_CONFIGrs> {
        IQC_ENABLE_W::new(self, 31)
    }
}
/**IQC_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`iqc_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:IQC_CONFIG)*/
pub struct IQC_CONFIGrs;
impl crate::RegisterSpec for IQC_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`iqc_config::R`](R) reader structure
impl crate::Readable for IQC_CONFIGrs {}
///`write(|w| ..)` method takes [`iqc_config::W`](W) writer structure
impl crate::Writable for IQC_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IQC_CONFIG to value 0xc000_0000
impl crate::Resettable for IQC_CONFIGrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
