///Register `OR1` reader
pub type R = crate::R<OR1rs>;
///Register `OR1` writer
pub type W = crate::W<OR1rs>;
///Field `ETR_ADC2_RMP` reader - External trigger remap on ADC2 analog watchdog 00: TIM8_ETR is not connected to ADC2 AWDx. This configuration must be selected when the ETR comes from the I/O.
pub type ETR_ADC2_RMP_R = crate::FieldReader;
///Field `ETR_ADC2_RMP` writer - External trigger remap on ADC2 analog watchdog 00: TIM8_ETR is not connected to ADC2 AWDx. This configuration must be selected when the ETR comes from the I/O.
pub type ETR_ADC2_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TI1_RMP` reader - Input Capture 1 remap 0: TIM8 input capture 1 is connected to I/O 1: TIM8 input capture 1 is connected to COMP2 outpu
pub type TI1_RMP_R = crate::BitReader;
///Field `TI1_RMP` writer - Input Capture 1 remap 0: TIM8 input capture 1 is connected to I/O 1: TIM8 input capture 1 is connected to COMP2 outpu
pub type TI1_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - External trigger remap on ADC2 analog watchdog 00: TIM8_ETR is not connected to ADC2 AWDx. This configuration must be selected when the ETR comes from the I/O.
    #[inline(always)]
    pub fn etr_adc2_rmp(&self) -> ETR_ADC2_RMP_R {
        ETR_ADC2_RMP_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - Input Capture 1 remap 0: TIM8 input capture 1 is connected to I/O 1: TIM8 input capture 1 is connected to COMP2 outpu
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("etr_adc2_rmp", &self.etr_adc2_rmp())
            .field("ti1_rmp", &self.ti1_rmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - External trigger remap on ADC2 analog watchdog 00: TIM8_ETR is not connected to ADC2 AWDx. This configuration must be selected when the ETR comes from the I/O.
    #[inline(always)]
    pub fn etr_adc2_rmp(&mut self) -> ETR_ADC2_RMP_W<'_, OR1rs> {
        ETR_ADC2_RMP_W::new(self, 0)
    }
    ///Bit 4 - Input Capture 1 remap 0: TIM8 input capture 1 is connected to I/O 1: TIM8 input capture 1 is connected to COMP2 outpu
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<'_, OR1rs> {
        TI1_RMP_W::new(self, 4)
    }
}
/**TIM8 option register 1

You can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#TIM8:OR1)*/
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
