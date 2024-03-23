#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Field `TIM8_ETR_ADC2_RMP` reader - TIM8_ETR_ADC2 remapping capability"]
pub type TIM8_ETR_ADC2_RMP_R = crate::FieldReader;
#[doc = "Field `TIM8_ETR_ADC2_RMP` writer - TIM8_ETR_ADC2 remapping capability"]
pub type TIM8_ETR_ADC2_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM8_ETR_ADC3_RMP` reader - TIM8_ETR_ADC3 remapping capability"]
pub type TIM8_ETR_ADC3_RMP_R = crate::FieldReader;
#[doc = "Field `TIM8_ETR_ADC3_RMP` writer - TIM8_ETR_ADC3 remapping capability"]
pub type TIM8_ETR_ADC3_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - TIM8_ETR_ADC2 remapping capability"]
    #[inline(always)]
    pub fn tim8_etr_adc2_rmp(&self) -> TIM8_ETR_ADC2_RMP_R {
        TIM8_ETR_ADC2_RMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TIM8_ETR_ADC3 remapping capability"]
    #[inline(always)]
    pub fn tim8_etr_adc3_rmp(&self) -> TIM8_ETR_ADC3_RMP_R {
        TIM8_ETR_ADC3_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TIM8_ETR_ADC2 remapping capability"]
    #[inline(always)]
    #[must_use]
    pub fn tim8_etr_adc2_rmp(&mut self) -> TIM8_ETR_ADC2_RMP_W<ORrs> {
        TIM8_ETR_ADC2_RMP_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - TIM8_ETR_ADC3 remapping capability"]
    #[inline(always)]
    #[must_use]
    pub fn tim8_etr_adc3_rmp(&mut self) -> TIM8_ETR_ADC3_RMP_W<ORrs> {
        TIM8_ETR_ADC3_RMP_W::new(self, 2)
    }
}
#[doc = "option registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for ORrs {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for ORrs {
    const RESET_VALUE: u32 = 0;
}
