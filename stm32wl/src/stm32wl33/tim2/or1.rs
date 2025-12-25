///Register `OR1` reader
pub type R = crate::R<OR1rs>;
///Register `OR1` writer
pub type W = crate::W<OR1rs>;
///Field `ETR_RMP` reader - ETR_RMP: ETR remapping capability 0: TIMx_ETR is not connected to ADC AWD (must be selected when the ETR comes from the ETR input pin) 1: TIMx_ETR is connected to ADC AWD Note: ADC AWD source is 'ORed' with the TIMx_ETR input signals. When ADC AWD is used, it is necessary to make sure that the corresponding TIMx_ETR input pin is not enabled in the alternate function controller.
pub type ETR_RMP_R = crate::BitReader;
///Field `ETR_RMP` writer - ETR_RMP: ETR remapping capability 0: TIMx_ETR is not connected to ADC AWD (must be selected when the ETR comes from the ETR input pin) 1: TIMx_ETR is connected to ADC AWD Note: ADC AWD source is 'ORed' with the TIMx_ETR input signals. When ADC AWD is used, it is necessary to make sure that the corresponding TIMx_ETR input pin is not enabled in the alternate function controller.
pub type ETR_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OR1_1` reader - This field is not used in Blue51. Not available in IUM
pub type OR1_1_R = crate::BitReader;
///Field `OR1_1` writer - This field is not used in Blue51. Not available in IUM
pub type OR1_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TI4_RMP` reader - TI4_RMP: Input capture 4 remap 0: TIM2 input capture 4 is connected to I/O 1: TIM2 input capture 4 is connected to COMP1-OUT
pub type TI4_RMP_R = crate::BitReader;
///Field `TI4_RMP` writer - TI4_RMP: Input capture 4 remap 0: TIM2 input capture 4 is connected to I/O 1: TIM2 input capture 4 is connected to COMP1-OUT
pub type TI4_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ETR_RMP: ETR remapping capability 0: TIMx_ETR is not connected to ADC AWD (must be selected when the ETR comes from the ETR input pin) 1: TIMx_ETR is connected to ADC AWD Note: ADC AWD source is 'ORed' with the TIMx_ETR input signals. When ADC AWD is used, it is necessary to make sure that the corresponding TIMx_ETR input pin is not enabled in the alternate function controller.
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - This field is not used in Blue51. Not available in IUM
    #[inline(always)]
    pub fn or1_1(&self) -> OR1_1_R {
        OR1_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TI4_RMP: Input capture 4 remap 0: TIM2 input capture 4 is connected to I/O 1: TIM2 input capture 4 is connected to COMP1-OUT
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("etr_rmp", &self.etr_rmp())
            .field("or1_1", &self.or1_1())
            .field("ti4_rmp", &self.ti4_rmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - ETR_RMP: ETR remapping capability 0: TIMx_ETR is not connected to ADC AWD (must be selected when the ETR comes from the ETR input pin) 1: TIMx_ETR is connected to ADC AWD Note: ADC AWD source is 'ORed' with the TIMx_ETR input signals. When ADC AWD is used, it is necessary to make sure that the corresponding TIMx_ETR input pin is not enabled in the alternate function controller.
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<'_, OR1rs> {
        ETR_RMP_W::new(self, 0)
    }
    ///Bit 1 - This field is not used in Blue51. Not available in IUM
    #[inline(always)]
    pub fn or1_1(&mut self) -> OR1_1_W<'_, OR1rs> {
        OR1_1_W::new(self, 1)
    }
    ///Bit 2 - TI4_RMP: Input capture 4 remap 0: TIM2 input capture 4 is connected to I/O 1: TIM2 input capture 4 is connected to COMP1-OUT
    #[inline(always)]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W<'_, OR1rs> {
        TI4_RMP_W::new(self, 2)
    }
}
/**OR1 register

You can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM2:OR1)*/
pub struct OR1rs;
impl crate::RegisterSpec for OR1rs {
    type Ux = u32;
}
///`read()` method returns [`or1::R`](R) reader structure
impl crate::Readable for OR1rs {}
///`write(|w| ..)` method takes [`or1::W`](W) writer structure
impl crate::Writable for OR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR1 to value 0
impl crate::Resettable for OR1rs {}
