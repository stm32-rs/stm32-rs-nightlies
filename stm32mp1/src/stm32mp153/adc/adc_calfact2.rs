#[doc = "Register `ADC_CALFACT2` reader"]
pub type R = crate::R<ADC_CALFACT2rs>;
#[doc = "Register `ADC_CALFACT2` writer"]
pub type W = crate::W<ADC_CALFACT2rs>;
#[doc = "Field `LINCALFACT` reader - LINCALFACT"]
pub type LINCALFACT_R = crate::FieldReader<u32>;
#[doc = "Field `LINCALFACT` writer - LINCALFACT"]
pub type LINCALFACT_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - LINCALFACT"]
    #[inline(always)]
    pub fn lincalfact(&self) -> LINCALFACT_R {
        LINCALFACT_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - LINCALFACT"]
    #[inline(always)]
    #[must_use]
    pub fn lincalfact(&mut self) -> LINCALFACT_W<ADC_CALFACT2rs> {
        LINCALFACT_W::new(self, 0)
    }
}
#[doc = "ADC calibration factor register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_calfact2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_calfact2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CALFACT2rs;
impl crate::RegisterSpec for ADC_CALFACT2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_calfact2::R`](R) reader structure"]
impl crate::Readable for ADC_CALFACT2rs {}
#[doc = "`write(|w| ..)` method takes [`adc_calfact2::W`](W) writer structure"]
impl crate::Writable for ADC_CALFACT2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CALFACT2 to value 0"]
impl crate::Resettable for ADC_CALFACT2rs {
    const RESET_VALUE: u32 = 0;
}
