///Register `ADC_LTR1` reader
pub type R = crate::R<ADC_LTR1rs>;
///Register `ADC_LTR1` writer
pub type W = crate::W<ADC_LTR1rs>;
///Field `LTR1` reader - LTR1
pub type LTR1_R = crate::FieldReader<u32>;
///Field `LTR1` writer - LTR1
pub type LTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:24 - LTR1
    #[inline(always)]
    pub fn ltr1(&self) -> LTR1_R {
        LTR1_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_LTR1")
            .field("ltr1", &self.ltr1())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - LTR1
    #[inline(always)]
    #[must_use]
    pub fn ltr1(&mut self) -> LTR1_W<ADC_LTR1rs> {
        LTR1_W::new(self, 0)
    }
}
/**ADC watchdog threshold register 1

You can [`read`](crate::Reg::read) this register and get [`adc_ltr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ltr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#ADC1:ADC_LTR1)*/
pub struct ADC_LTR1rs;
impl crate::RegisterSpec for ADC_LTR1rs {
    type Ux = u32;
}
///`read()` method returns [`adc_ltr1::R`](R) reader structure
impl crate::Readable for ADC_LTR1rs {}
///`write(|w| ..)` method takes [`adc_ltr1::W`](W) writer structure
impl crate::Writable for ADC_LTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_LTR1 to value 0
impl crate::Resettable for ADC_LTR1rs {
    const RESET_VALUE: u32 = 0;
}
