#[doc = "Register `ADC_HTR1` reader"]
pub type R = crate::R<ADC_HTR1rs>;
#[doc = "Register `ADC_HTR1` writer"]
pub type W = crate::W<ADC_HTR1rs>;
#[doc = "Field `HTR1` reader - HTR1"]
pub type HTR1_R = crate::FieldReader<u32>;
#[doc = "Field `HTR1` writer - HTR1"]
pub type HTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - HTR1"]
    #[inline(always)]
    pub fn htr1(&self) -> HTR1_R {
        HTR1_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - HTR1"]
    #[inline(always)]
    #[must_use]
    pub fn htr1(&mut self) -> HTR1_W<ADC_HTR1rs> {
        HTR1_W::new(self, 0)
    }
}
#[doc = "ADC watchdog threshold register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_htr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_htr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_HTR1rs;
impl crate::RegisterSpec for ADC_HTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_htr1::R`](R) reader structure"]
impl crate::Readable for ADC_HTR1rs {}
#[doc = "`write(|w| ..)` method takes [`adc_htr1::W`](W) writer structure"]
impl crate::Writable for ADC_HTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_HTR1 to value 0x03ff_ffff"]
impl crate::Resettable for ADC_HTR1rs {
    const RESET_VALUE: u32 = 0x03ff_ffff;
}
