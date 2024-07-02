///Register `ADC_HTR3` reader
pub type R = crate::R<ADC_HTR3rs>;
///Register `ADC_HTR3` writer
pub type W = crate::W<ADC_HTR3rs>;
///Field `HTR3` reader - HTR3
pub type HTR3_R = crate::FieldReader<u32>;
///Field `HTR3` writer - HTR3
pub type HTR3_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:24 - HTR3
    #[inline(always)]
    pub fn htr3(&self) -> HTR3_R {
        HTR3_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_HTR3")
            .field("htr3", &self.htr3())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - HTR3
    #[inline(always)]
    #[must_use]
    pub fn htr3(&mut self) -> HTR3_W<ADC_HTR3rs> {
        HTR3_W::new(self, 0)
    }
}
/**ADC watchdog higher threshold register 3

You can [`read`](crate::Reg::read) this register and get [`adc_htr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_htr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#ADC1:ADC_HTR3)*/
pub struct ADC_HTR3rs;
impl crate::RegisterSpec for ADC_HTR3rs {
    type Ux = u32;
}
///`read()` method returns [`adc_htr3::R`](R) reader structure
impl crate::Readable for ADC_HTR3rs {}
///`write(|w| ..)` method takes [`adc_htr3::W`](W) writer structure
impl crate::Writable for ADC_HTR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_HTR3 to value 0x01ff_ffff
impl crate::Resettable for ADC_HTR3rs {
    const RESET_VALUE: u32 = 0x01ff_ffff;
}
