#[doc = "Register `ADC_LTR3` reader"]
pub type R = crate::R<ADC_LTR3rs>;
#[doc = "Register `ADC_LTR3` writer"]
pub type W = crate::W<ADC_LTR3rs>;
#[doc = "Field `LTR3` reader - LTR3"]
pub type LTR3_R = crate::FieldReader<u32>;
#[doc = "Field `LTR3` writer - LTR3"]
pub type LTR3_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - LTR3"]
    #[inline(always)]
    pub fn ltr3(&self) -> LTR3_R {
        LTR3_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - LTR3"]
    #[inline(always)]
    #[must_use]
    pub fn ltr3(&mut self) -> LTR3_W<ADC_LTR3rs> {
        LTR3_W::new(self, 0)
    }
}
#[doc = "ADC watchdog lower threshold register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ltr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ltr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_LTR3rs;
impl crate::RegisterSpec for ADC_LTR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ltr3::R`](R) reader structure"]
impl crate::Readable for ADC_LTR3rs {}
#[doc = "`write(|w| ..)` method takes [`adc_ltr3::W`](W) writer structure"]
impl crate::Writable for ADC_LTR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_LTR3 to value 0"]
impl crate::Resettable for ADC_LTR3rs {
    const RESET_VALUE: u32 = 0;
}
