///Register `RF_FSM10_TIMEOUT` reader
pub type R = crate::R<RF_FSM10_TIMEOUTrs>;
///Register `RF_FSM10_TIMEOUT` writer
pub type W = crate::W<RF_FSM10_TIMEOUTrs>;
///Field `END_TX_TIMER` reader - Timeout management for the RF regulator to stabilize after clock stops on the analog PA block
pub type END_TX_TIMER_R = crate::FieldReader;
///Field `END_TX_TIMER` writer - Timeout management for the RF regulator to stabilize after clock stops on the analog PA block
pub type END_TX_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Timeout management for the RF regulator to stabilize after clock stops on the analog PA block
    #[inline(always)]
    pub fn end_tx_timer(&self) -> END_TX_TIMER_R {
        END_TX_TIMER_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_FSM10_TIMEOUT")
            .field("end_tx_timer", &self.end_tx_timer())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Timeout management for the RF regulator to stabilize after clock stops on the analog PA block
    #[inline(always)]
    pub fn end_tx_timer(&mut self) -> END_TX_TIMER_W<'_, RF_FSM10_TIMEOUTrs> {
        END_TX_TIMER_W::new(self, 0)
    }
}
/**RF_FSM10_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm10_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm10_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM10_TIMEOUT)*/
pub struct RF_FSM10_TIMEOUTrs;
impl crate::RegisterSpec for RF_FSM10_TIMEOUTrs {
    type Ux = u32;
}
///`read()` method returns [`rf_fsm10_timeout::R`](R) reader structure
impl crate::Readable for RF_FSM10_TIMEOUTrs {}
///`write(|w| ..)` method takes [`rf_fsm10_timeout::W`](W) writer structure
impl crate::Writable for RF_FSM10_TIMEOUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RF_FSM10_TIMEOUT to value 0x06
impl crate::Resettable for RF_FSM10_TIMEOUTrs {
    const RESET_VALUE: u32 = 0x06;
}
