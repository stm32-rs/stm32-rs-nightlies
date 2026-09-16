///Register `IRQ_STATUS` reader
pub type R = crate::R<IRQ_STATUSrs>;
///Register `IRQ_STATUS` writer
pub type W = crate::W<IRQ_STATUSrs>;
///Field `SLOW_CLK_IRQ` reader - slow clock measurement end of calculation interrupt status
pub type SLOW_CLK_IRQ_R = crate::BitReader;
///Field `SLOW_CLK_IRQ` writer - slow clock measurement end of calculation interrupt status
pub type SLOW_CLK_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RADIO_FSM_IRQ` reader - Radio FSM interrupt status (aka RfFsm_event_irq).
pub type RADIO_FSM_IRQ_R = crate::FieldReader;
///Field `RADIO_FSM_IRQ` writer - Radio FSM interrupt status (aka RfFsm_event_irq).
pub type RADIO_FSM_IRQ_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - slow clock measurement end of calculation interrupt status
    #[inline(always)]
    pub fn slow_clk_irq(&self) -> SLOW_CLK_IRQ_R {
        SLOW_CLK_IRQ_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:13 - Radio FSM interrupt status (aka RfFsm_event_irq).
    #[inline(always)]
    pub fn radio_fsm_irq(&self) -> RADIO_FSM_IRQ_R {
        RADIO_FSM_IRQ_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_STATUS")
            .field("slow_clk_irq", &self.slow_clk_irq())
            .field("radio_fsm_irq", &self.radio_fsm_irq())
            .finish()
    }
}
impl W {
    ///Bit 0 - slow clock measurement end of calculation interrupt status
    #[inline(always)]
    pub fn slow_clk_irq(&mut self) -> SLOW_CLK_IRQ_W<'_, IRQ_STATUSrs> {
        SLOW_CLK_IRQ_W::new(self, 0)
    }
    ///Bits 8:13 - Radio FSM interrupt status (aka RfFsm_event_irq).
    #[inline(always)]
    pub fn radio_fsm_irq(&mut self) -> RADIO_FSM_IRQ_W<'_, IRQ_STATUSrs> {
        RADIO_FSM_IRQ_W::new(self, 8)
    }
}
/**RADIO_CONTROL_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`irq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RADIO_CONTROL:IRQ_STATUS)*/
pub struct IRQ_STATUSrs;
impl crate::RegisterSpec for IRQ_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`irq_status::R`](R) reader structure
impl crate::Readable for IRQ_STATUSrs {}
///`write(|w| ..)` method takes [`irq_status::W`](W) writer structure
impl crate::Writable for IRQ_STATUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQ_STATUS to value 0
impl crate::Resettable for IRQ_STATUSrs {}
