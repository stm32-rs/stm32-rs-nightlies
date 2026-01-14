///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `TIM8_ETR_ADC2_RMP` reader - TIM8_ETR_ADC2 remapping capability
pub type TIM8_ETR_ADC2_RMP_R = crate::FieldReader;
///Field `TIM8_ETR_ADC2_RMP` writer - TIM8_ETR_ADC2 remapping capability
pub type TIM8_ETR_ADC2_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIM8_ETR_ADC3_RMP` reader - TIM8_ETR_ADC3 remapping capability
pub type TIM8_ETR_ADC3_RMP_R = crate::FieldReader;
///Field `TIM8_ETR_ADC3_RMP` writer - TIM8_ETR_ADC3 remapping capability
pub type TIM8_ETR_ADC3_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - TIM8_ETR_ADC2 remapping capability
    #[inline(always)]
    pub fn tim8_etr_adc2_rmp(&self) -> TIM8_ETR_ADC2_RMP_R {
        TIM8_ETR_ADC2_RMP_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - TIM8_ETR_ADC3 remapping capability
    #[inline(always)]
    pub fn tim8_etr_adc3_rmp(&self) -> TIM8_ETR_ADC3_RMP_R {
        TIM8_ETR_ADC3_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("tim8_etr_adc2_rmp", &self.tim8_etr_adc2_rmp())
            .field("tim8_etr_adc3_rmp", &self.tim8_etr_adc3_rmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TIM8_ETR_ADC2 remapping capability
    #[inline(always)]
    pub fn tim8_etr_adc2_rmp(&mut self) -> TIM8_ETR_ADC2_RMP_W<'_, ORrs> {
        TIM8_ETR_ADC2_RMP_W::new(self, 0)
    }
    ///Bits 2:3 - TIM8_ETR_ADC3 remapping capability
    #[inline(always)]
    pub fn tim8_etr_adc3_rmp(&mut self) -> TIM8_ETR_ADC3_RMP_W<'_, ORrs> {
        TIM8_ETR_ADC3_RMP_W::new(self, 2)
    }
}
/**option registers

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#TIM8:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
