#[doc = "Register `ADC_CALFACT` reader"]
pub type R = crate::R<ADC_CALFACTrs>;
#[doc = "Register `ADC_CALFACT` writer"]
pub type W = crate::W<ADC_CALFACTrs>;
#[doc = "Field `CALFACT_S` reader - CALFACT_S"]
pub type CALFACT_S_R = crate::FieldReader<u16>;
#[doc = "Field `CALFACT_S` writer - CALFACT_S"]
pub type CALFACT_S_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CALFACT_D` reader - CALFACT_D"]
pub type CALFACT_D_R = crate::FieldReader<u16>;
#[doc = "Field `CALFACT_D` writer - CALFACT_D"]
pub type CALFACT_D_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - CALFACT_S"]
    #[inline(always)]
    pub fn calfact_s(&self) -> CALFACT_S_R {
        CALFACT_S_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - CALFACT_D"]
    #[inline(always)]
    pub fn calfact_d(&self) -> CALFACT_D_R {
        CALFACT_D_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - CALFACT_S"]
    #[inline(always)]
    #[must_use]
    pub fn calfact_s(&mut self) -> CALFACT_S_W<ADC_CALFACTrs> {
        CALFACT_S_W::new(self, 0)
    }
    #[doc = "Bits 16:26 - CALFACT_D"]
    #[inline(always)]
    #[must_use]
    pub fn calfact_d(&mut self) -> CALFACT_D_W<ADC_CALFACTrs> {
        CALFACT_D_W::new(self, 16)
    }
}
#[doc = "ADC calibration factors register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_calfact::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_calfact::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CALFACTrs;
impl crate::RegisterSpec for ADC_CALFACTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_calfact::R`](R) reader structure"]
impl crate::Readable for ADC_CALFACTrs {}
#[doc = "`write(|w| ..)` method takes [`adc_calfact::W`](W) writer structure"]
impl crate::Writable for ADC_CALFACTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CALFACT to value 0"]
impl crate::Resettable for ADC_CALFACTrs {
    const RESET_VALUE: u32 = 0;
}
