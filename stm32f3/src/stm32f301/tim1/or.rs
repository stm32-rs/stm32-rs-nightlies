///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `IM1_ETR_ADC1_RMP` reader - TIM1_ETR_ADC1 remapping capability
pub type IM1_ETR_ADC1_RMP_R = crate::FieldReader;
///Field `IM1_ETR_ADC1_RMP` writer - TIM1_ETR_ADC1 remapping capability
pub type IM1_ETR_ADC1_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - TIM1_ETR_ADC1 remapping capability
    #[inline(always)]
    pub fn im1_etr_adc1_rmp(&self) -> IM1_ETR_ADC1_RMP_R {
        IM1_ETR_ADC1_RMP_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("im1_etr_adc1_rmp", &self.im1_etr_adc1_rmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TIM1_ETR_ADC1 remapping capability
    #[inline(always)]
    pub fn im1_etr_adc1_rmp(&mut self) -> IM1_ETR_ADC1_RMP_W<'_, ORrs> {
        IM1_ETR_ADC1_RMP_W::new(self, 0)
    }
}
/**Option registers

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#TIM1:OR)*/
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
