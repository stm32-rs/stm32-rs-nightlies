#[doc = "Register `ADC_LTR2` reader"]
pub type R = crate::R<ADC_LTR2rs>;
#[doc = "Register `ADC_LTR2` writer"]
pub type W = crate::W<ADC_LTR2rs>;
#[doc = "Field `LTR2` reader - LTR2"]
pub type LTR2_R = crate::FieldReader<u32>;
#[doc = "Field `LTR2` writer - LTR2"]
pub type LTR2_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - LTR2"]
    #[inline(always)]
    pub fn ltr2(&self) -> LTR2_R {
        LTR2_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - LTR2"]
    #[inline(always)]
    #[must_use]
    pub fn ltr2(&mut self) -> LTR2_W<ADC_LTR2rs> {
        LTR2_W::new(self, 0)
    }
}
#[doc = "ADC watchdog lower threshold register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ltr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ltr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_LTR2rs;
impl crate::RegisterSpec for ADC_LTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ltr2::R`](R) reader structure"]
impl crate::Readable for ADC_LTR2rs {}
#[doc = "`write(|w| ..)` method takes [`adc_ltr2::W`](W) writer structure"]
impl crate::Writable for ADC_LTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_LTR2 to value 0"]
impl crate::Resettable for ADC_LTR2rs {
    const RESET_VALUE: u32 = 0;
}
