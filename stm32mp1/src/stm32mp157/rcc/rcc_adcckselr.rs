#[doc = "Register `RCC_ADCCKSELR` reader"]
pub type R = crate::R<RCC_ADCCKSELRrs>;
#[doc = "Register `RCC_ADCCKSELR` writer"]
pub type W = crate::W<RCC_ADCCKSELRrs>;
#[doc = "Field `ADCSRC` reader - ADCSRC"]
pub type ADCSRC_R = crate::FieldReader;
#[doc = "Field `ADCSRC` writer - ADCSRC"]
pub type ADCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - ADCSRC"]
    #[inline(always)]
    pub fn adcsrc(&self) -> ADCSRC_R {
        ADCSRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADCSRC"]
    #[inline(always)]
    #[must_use]
    pub fn adcsrc(&mut self) -> ADCSRC_W<RCC_ADCCKSELRrs> {
        ADCSRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the ADC block.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_adcckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_adcckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_ADCCKSELRrs;
impl crate::RegisterSpec for RCC_ADCCKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_adcckselr::R`](R) reader structure"]
impl crate::Readable for RCC_ADCCKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_adcckselr::W`](W) writer structure"]
impl crate::Writable for RCC_ADCCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_ADCCKSELR to value 0"]
impl crate::Resettable for RCC_ADCCKSELRrs {
    const RESET_VALUE: u32 = 0;
}
