///Register `IRQ_ENABLE` reader
pub type R = crate::R<IRQ_ENABLErs>;
///Register `IRQ_ENABLE` writer
pub type W = crate::W<IRQ_ENABLErs>;
///Field `SLOW_CLK_IRQ_MASK` reader - mask slow clock measurement interrupt
pub type SLOW_CLK_IRQ_MASK_R = crate::BitReader;
///Field `SLOW_CLK_IRQ_MASK` writer - mask slow clock measurement interrupt
pub type SLOW_CLK_IRQ_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RADIO_FSM_IRQ_MASK` reader - mask for each RfFsm_event (Radio FSM) interrupt.
pub type RADIO_FSM_IRQ_MASK_R = crate::FieldReader;
///Field `RADIO_FSM_IRQ_MASK` writer - mask for each RfFsm_event (Radio FSM) interrupt.
pub type RADIO_FSM_IRQ_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - mask slow clock measurement interrupt
    #[inline(always)]
    pub fn slow_clk_irq_mask(&self) -> SLOW_CLK_IRQ_MASK_R {
        SLOW_CLK_IRQ_MASK_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:13 - mask for each RfFsm_event (Radio FSM) interrupt.
    #[inline(always)]
    pub fn radio_fsm_irq_mask(&self) -> RADIO_FSM_IRQ_MASK_R {
        RADIO_FSM_IRQ_MASK_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_ENABLE")
            .field("slow_clk_irq_mask", &self.slow_clk_irq_mask())
            .field("radio_fsm_irq_mask", &self.radio_fsm_irq_mask())
            .finish()
    }
}
impl W {
    ///Bit 0 - mask slow clock measurement interrupt
    #[inline(always)]
    pub fn slow_clk_irq_mask(&mut self) -> SLOW_CLK_IRQ_MASK_W<'_, IRQ_ENABLErs> {
        SLOW_CLK_IRQ_MASK_W::new(self, 0)
    }
    ///Bits 8:13 - mask for each RfFsm_event (Radio FSM) interrupt.
    #[inline(always)]
    pub fn radio_fsm_irq_mask(&mut self) -> RADIO_FSM_IRQ_MASK_W<'_, IRQ_ENABLErs> {
        RADIO_FSM_IRQ_MASK_W::new(self, 8)
    }
}
/**RADIO_CONTROL_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RADIO_CONTROL:IRQ_ENABLE)*/
pub struct IRQ_ENABLErs;
impl crate::RegisterSpec for IRQ_ENABLErs {
    type Ux = u32;
}
///`read()` method returns [`irq_enable::R`](R) reader structure
impl crate::Readable for IRQ_ENABLErs {}
///`write(|w| ..)` method takes [`irq_enable::W`](W) writer structure
impl crate::Writable for IRQ_ENABLErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IRQ_ENABLE to value 0
impl crate::Resettable for IRQ_ENABLErs {}
