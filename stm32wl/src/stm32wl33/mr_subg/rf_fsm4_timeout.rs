///Register `RF_FSM4_TIMEOUT` reader
pub type R = crate::R<RF_FSM4_TIMEOUTrs>;
///Register `RF_FSM4_TIMEOUT` writer
pub type W = crate::W<RF_FSM4_TIMEOUTrs>;
///Field `EN_RX_TIMER` reader - Timeout for the analog RX chain setup (duration in EN_RX state)
pub type EN_RX_TIMER_R = crate::FieldReader;
///Field `EN_RX_TIMER` writer - Timeout for the analog RX chain setup (duration in EN_RX state)
pub type EN_RX_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Timeout for the analog RX chain setup (duration in EN_RX state)
    #[inline(always)]
    pub fn en_rx_timer(&self) -> EN_RX_TIMER_R {
        EN_RX_TIMER_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_FSM4_TIMEOUT")
            .field("en_rx_timer", &self.en_rx_timer())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Timeout for the analog RX chain setup (duration in EN_RX state)
    #[inline(always)]
    pub fn en_rx_timer(&mut self) -> EN_RX_TIMER_W<'_, RF_FSM4_TIMEOUTrs> {
        EN_RX_TIMER_W::new(self, 0)
    }
}
/**RF_FSM4_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm4_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm4_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM4_TIMEOUT)*/
pub struct RF_FSM4_TIMEOUTrs;
impl crate::RegisterSpec for RF_FSM4_TIMEOUTrs {
    type Ux = u32;
}
///`read()` method returns [`rf_fsm4_timeout::R`](R) reader structure
impl crate::Readable for RF_FSM4_TIMEOUTrs {}
///`write(|w| ..)` method takes [`rf_fsm4_timeout::W`](W) writer structure
impl crate::Writable for RF_FSM4_TIMEOUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RF_FSM4_TIMEOUT to value 0x0f
impl crate::Resettable for RF_FSM4_TIMEOUTrs {
    const RESET_VALUE: u32 = 0x0f;
}
