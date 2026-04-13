///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `TIM1_ETR_ADC1_RMP` reader - TIM1_ETR_ADC1 remapping capability
pub type TIM1_ETR_ADC1_RMP_R = crate::FieldReader;
///Field `TIM1_ETR_ADC1_RMP` writer - TIM1_ETR_ADC1 remapping capability
pub type TIM1_ETR_ADC1_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TI1_RMP` reader - Input Capture 1 remap
pub type TI1_RMP_R = crate::BitReader;
///Field `TI1_RMP` writer - Input Capture 1 remap
pub type TI1_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - TIM1_ETR_ADC1 remapping capability
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&self) -> TIM1_ETR_ADC1_RMP_R {
        TIM1_ETR_ADC1_RMP_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - Input Capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("tim1_etr_adc1_rmp", &self.tim1_etr_adc1_rmp())
            .field("ti1_rmp", &self.ti1_rmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TIM1_ETR_ADC1 remapping capability
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&mut self) -> TIM1_ETR_ADC1_RMP_W<'_, ORrs> {
        TIM1_ETR_ADC1_RMP_W::new(self, 0)
    }
    ///Bit 4 - Input Capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<'_, ORrs> {
        TI1_RMP_W::new(self, 4)
    }
}
/**DMA address for full transfer

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TIM1:OR)*/
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
