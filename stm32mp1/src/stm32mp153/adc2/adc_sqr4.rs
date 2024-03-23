#[doc = "Register `ADC_SQR4` reader"]
pub type R = crate::R<ADC_SQR4rs>;
#[doc = "Register `ADC_SQR4` writer"]
pub type W = crate::W<ADC_SQR4rs>;
#[doc = "Field `SQ15` reader - SQ15"]
pub type SQ15_R = crate::FieldReader;
#[doc = "Field `SQ15` writer - SQ15"]
pub type SQ15_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQ16` reader - SQ16"]
pub type SQ16_R = crate::FieldReader;
#[doc = "Field `SQ16` writer - SQ16"]
pub type SQ16_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - SQ15"]
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - SQ16"]
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SQ15"]
    #[inline(always)]
    #[must_use]
    pub fn sq15(&mut self) -> SQ15_W<ADC_SQR4rs> {
        SQ15_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - SQ16"]
    #[inline(always)]
    #[must_use]
    pub fn sq16(&mut self) -> SQ16_W<ADC_SQR4rs> {
        SQ16_W::new(self, 6)
    }
}
#[doc = "ADC regular sequence register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_sqr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_sqr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_SQR4rs;
impl crate::RegisterSpec for ADC_SQR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_sqr4::R`](R) reader structure"]
impl crate::Readable for ADC_SQR4rs {}
#[doc = "`write(|w| ..)` method takes [`adc_sqr4::W`](W) writer structure"]
impl crate::Writable for ADC_SQR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_SQR4 to value 0"]
impl crate::Resettable for ADC_SQR4rs {
    const RESET_VALUE: u32 = 0;
}
