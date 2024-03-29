#[doc = "Register `ADC_LTR1` reader"]
pub type R = crate::R<ADC_LTR1rs>;
#[doc = "Register `ADC_LTR1` writer"]
pub type W = crate::W<ADC_LTR1rs>;
#[doc = "Field `LTR1` reader - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy)."]
pub type LTR1_R = crate::FieldReader<u32>;
#[doc = "Field `LTR1` writer - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy)."]
pub type LTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy)."]
    #[inline(always)]
    pub fn ltr1(&self) -> LTR1_R {
        LTR1_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy)."]
    #[inline(always)]
    #[must_use]
    pub fn ltr1(&mut self) -> LTR1_W<ADC_LTR1rs> {
        LTR1_W::new(self, 0)
    }
}
#[doc = "ADC watchdog threshold register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ltr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ltr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_LTR1rs;
impl crate::RegisterSpec for ADC_LTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ltr1::R`](R) reader structure"]
impl crate::Readable for ADC_LTR1rs {}
#[doc = "`write(|w| ..)` method takes [`adc_ltr1::W`](W) writer structure"]
impl crate::Writable for ADC_LTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_LTR1 to value 0"]
impl crate::Resettable for ADC_LTR1rs {
    const RESET_VALUE: u32 = 0;
}
