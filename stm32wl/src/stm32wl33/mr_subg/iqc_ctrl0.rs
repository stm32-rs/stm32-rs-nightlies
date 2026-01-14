///Register `IQC_CTRL0` reader
pub type R = crate::R<IQC_CTRL0rs>;
///Register `IQC_CTRL0` writer
pub type W = crate::W<IQC_CTRL0rs>;
///Field `FAST_GAIN` reader - Gain of the correction loop in fast mode.
pub type FAST_GAIN_R = crate::FieldReader;
///Field `FAST_GAIN` writer - Gain of the correction loop in fast mode.
pub type FAST_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SLOW_GAIN` reader - Gain of the correction loop in slow mode.
pub type SLOW_GAIN_R = crate::FieldReader;
///Field `SLOW_GAIN` writer - Gain of the correction loop in slow mode.
pub type SLOW_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Gain of the correction loop in fast mode.
    #[inline(always)]
    pub fn fast_gain(&self) -> FAST_GAIN_R {
        FAST_GAIN_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Gain of the correction loop in slow mode.
    #[inline(always)]
    pub fn slow_gain(&self) -> SLOW_GAIN_R {
        SLOW_GAIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IQC_CTRL0")
            .field("fast_gain", &self.fast_gain())
            .field("slow_gain", &self.slow_gain())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Gain of the correction loop in fast mode.
    #[inline(always)]
    pub fn fast_gain(&mut self) -> FAST_GAIN_W<'_, IQC_CTRL0rs> {
        FAST_GAIN_W::new(self, 0)
    }
    ///Bits 4:7 - Gain of the correction loop in slow mode.
    #[inline(always)]
    pub fn slow_gain(&mut self) -> SLOW_GAIN_W<'_, IQC_CTRL0rs> {
        SLOW_GAIN_W::new(self, 4)
    }
}
/**IQC_CTRL0 register

You can [`read`](crate::Reg::read) this register and get [`iqc_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:IQC_CTRL0)*/
pub struct IQC_CTRL0rs;
impl crate::RegisterSpec for IQC_CTRL0rs {
    type Ux = u32;
}
///`read()` method returns [`iqc_ctrl0::R`](R) reader structure
impl crate::Readable for IQC_CTRL0rs {}
///`write(|w| ..)` method takes [`iqc_ctrl0::W`](W) writer structure
impl crate::Writable for IQC_CTRL0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IQC_CTRL0 to value 0xe3
impl crate::Resettable for IQC_CTRL0rs {
    const RESET_VALUE: u32 = 0xe3;
}
