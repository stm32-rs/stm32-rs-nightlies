///Register `OR1` reader
pub type R = crate::R<OR1rs>;
///Register `OR1` writer
pub type W = crate::W<OR1rs>;
///Field `TI1_RMP` reader - TI1_RMP\[1:0\]: Timer 17 input 1 connection This bit is set and cleared by software. 00: TIM17 TI1 is connected to GPIO 01: TIM17 TI1 is connected to LCO 1x: TIM17 TI1 is connected to MCO
pub type TI1_RMP_R = crate::FieldReader;
///Field `TI1_RMP` writer - TI1_RMP\[1:0\]: Timer 17 input 1 connection This bit is set and cleared by software. 00: TIM17 TI1 is connected to GPIO 01: TIM17 TI1 is connected to LCO 1x: TIM17 TI1 is connected to MCO
pub type TI1_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - TI1_RMP\[1:0\]: Timer 17 input 1 connection This bit is set and cleared by software. 00: TIM17 TI1 is connected to GPIO 01: TIM17 TI1 is connected to LCO 1x: TIM17 TI1 is connected to MCO
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("ti1_rmp", &self.ti1_rmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TI1_RMP\[1:0\]: Timer 17 input 1 connection This bit is set and cleared by software. 00: TIM17 TI1 is connected to GPIO 01: TIM17 TI1 is connected to LCO 1x: TIM17 TI1 is connected to MCO
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<OR1rs> {
        TI1_RMP_W::new(self, 0)
    }
}
/**OR1 register

You can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TIM17:OR1)*/
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
