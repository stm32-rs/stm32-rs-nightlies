///Register `ADC_HTR2` reader
pub type R = crate::R<ADC_HTR2rs>;
///Register `ADC_HTR2` writer
pub type W = crate::W<ADC_HTR2rs>;
///Field `HTR2` reader - HTR2
pub type HTR2_R = crate::FieldReader<u32>;
///Field `HTR2` writer - HTR2
pub type HTR2_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:24 - HTR2
    #[inline(always)]
    pub fn htr2(&self) -> HTR2_R {
        HTR2_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_HTR2")
            .field("htr2", &self.htr2())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - HTR2
    #[inline(always)]
    #[must_use]
    pub fn htr2(&mut self) -> HTR2_W<ADC_HTR2rs> {
        HTR2_W::new(self, 0)
    }
}
/**ADC watchdog higher threshold register 2

You can [`read`](crate::Reg::read) this register and get [`adc_htr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_htr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC1:ADC_HTR2)*/
pub struct ADC_HTR2rs;
impl crate::RegisterSpec for ADC_HTR2rs {
    type Ux = u32;
}
///`read()` method returns [`adc_htr2::R`](R) reader structure
impl crate::Readable for ADC_HTR2rs {}
///`write(|w| ..)` method takes [`adc_htr2::W`](W) writer structure
impl crate::Writable for ADC_HTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_HTR2 to value 0x01ff_ffff
impl crate::Resettable for ADC_HTR2rs {
    const RESET_VALUE: u32 = 0x01ff_ffff;
}
