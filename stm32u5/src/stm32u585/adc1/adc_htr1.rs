#[doc = "Register `ADC_HTR1` reader"]
pub type R = crate::R<ADC_HTR1rs>;
#[doc = "Register `ADC_HTR1` writer"]
pub type W = crate::W<ADC_HTR1rs>;
#[doc = "Field `HTR1` reader - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy)."]
pub type HTR1_R = crate::FieldReader<u32>;
#[doc = "Field `HTR1` writer - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy)."]
pub type HTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
#[doc = "Field `AWDFILT1` reader - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWDFILT1_R = crate::FieldReader;
#[doc = "Field `AWDFILT1` writer - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
pub type AWDFILT1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:24 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy)."]
    #[inline(always)]
    pub fn htr1(&self) -> HTR1_R {
        HTR1_R::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bits 29:31 - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awdfilt1(&self) -> AWDFILT1_R {
        AWDFILT1_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:24 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy)."]
    #[inline(always)]
    #[must_use]
    pub fn htr1(&mut self) -> HTR1_W<ADC_HTR1rs> {
        HTR1_W::new(self, 0)
    }
    #[doc = "Bits 29:31 - Analog watchdog filtering parameter This bit is set and cleared by software. ... Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awdfilt1(&mut self) -> AWDFILT1_W<ADC_HTR1rs> {
        AWDFILT1_W::new(self, 29)
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
#[doc = "`reset()` method sets ADC_HTR1 to value 0x01ff_ffff"]
impl crate::Resettable for ADC_HTR1rs {
    const RESET_VALUE: u32 = 0x01ff_ffff;
}
