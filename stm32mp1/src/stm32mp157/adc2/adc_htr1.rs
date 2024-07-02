///Register `ADC_HTR1` reader
pub type R = crate::R<ADC_HTR1rs>;
///Register `ADC_HTR1` writer
pub type W = crate::W<ADC_HTR1rs>;
///Field `HTR1` reader - HTR1
pub type HTR1_R = crate::FieldReader<u32>;
///Field `HTR1` writer - HTR1
pub type HTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - HTR1
    #[inline(always)]
    pub fn htr1(&self) -> HTR1_R {
        HTR1_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_HTR1")
            .field("htr1", &self.htr1())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - HTR1
    #[inline(always)]
    #[must_use]
    pub fn htr1(&mut self) -> HTR1_W<ADC_HTR1rs> {
        HTR1_W::new(self, 0)
    }
}
/**ADC watchdog threshold register 1

You can [`read`](crate::Reg::read) this register and get [`adc_htr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_htr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ADC2:ADC_HTR1)*/
pub struct ADC_HTR1rs;
impl crate::RegisterSpec for ADC_HTR1rs {
    type Ux = u32;
}
///`read()` method returns [`adc_htr1::R`](R) reader structure
impl crate::Readable for ADC_HTR1rs {}
///`write(|w| ..)` method takes [`adc_htr1::W`](W) writer structure
impl crate::Writable for ADC_HTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_HTR1 to value 0x03ff_ffff
impl crate::Resettable for ADC_HTR1rs {
    const RESET_VALUE: u32 = 0x03ff_ffff;
}
