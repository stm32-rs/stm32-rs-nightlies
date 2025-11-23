///Register `OR1` reader
pub type R = crate::R<OR1rs>;
///Register `OR1` writer
pub type W = crate::W<OR1rs>;
///Field `OR1_0` reader - Not used in Blue51. Not available in IUM
pub type OR1_0_R = crate::BitReader;
///Field `OR1_0` writer - Not used in Blue51. Not available in IUM
pub type OR1_0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TI1_RMP` reader - TI1_RMP\[1:0\]: Timer 16 input 1 connection This bit is set and cleared by software. 00: TIM16 TI1 is connected to GPIO 01: TIM16 TI1 is connected to LCO 10: TIM16 TI1 is connected to COMP_OUT 11: TIM16 TI1 is connected to MCO
pub type TI1_RMP_R = crate::FieldReader;
///Field `TI1_RMP` writer - TI1_RMP\[1:0\]: Timer 16 input 1 connection This bit is set and cleared by software. 00: TIM16 TI1 is connected to GPIO 01: TIM16 TI1 is connected to LCO 10: TIM16 TI1 is connected to COMP_OUT 11: TIM16 TI1 is connected to MCO
pub type TI1_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Not used in Blue51. Not available in IUM
    #[inline(always)]
    pub fn or1_0(&self) -> OR1_0_R {
        OR1_0_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - TI1_RMP\[1:0\]: Timer 16 input 1 connection This bit is set and cleared by software. 00: TIM16 TI1 is connected to GPIO 01: TIM16 TI1 is connected to LCO 10: TIM16 TI1 is connected to COMP_OUT 11: TIM16 TI1 is connected to MCO
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("or1_0", &self.or1_0())
            .field("ti1_rmp", &self.ti1_rmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Not used in Blue51. Not available in IUM
    #[inline(always)]
    pub fn or1_0(&mut self) -> OR1_0_W<'_, OR1rs> {
        OR1_0_W::new(self, 0)
    }
    ///Bits 1:2 - TI1_RMP\[1:0\]: Timer 16 input 1 connection This bit is set and cleared by software. 00: TIM16 TI1 is connected to GPIO 01: TIM16 TI1 is connected to LCO 10: TIM16 TI1 is connected to COMP_OUT 11: TIM16 TI1 is connected to MCO
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<'_, OR1rs> {
        TI1_RMP_W::new(self, 1)
    }
}
/**OR1 register

You can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM16:OR1)*/
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
