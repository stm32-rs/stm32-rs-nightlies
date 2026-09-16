///Register `RF_FSM0_TIMEOUT` reader
pub type R = crate::R<RF_FSM0_TIMEOUTrs>;
///Register `RF_FSM0_TIMEOUT` writer
pub type W = crate::W<RF_FSM0_TIMEOUTrs>;
///Field `ENA_RFREG_TIMER` reader - Timeout for the RF regulator startup (duration in ENA_RF_REG state)
pub type ENA_RFREG_TIMER_R = crate::FieldReader;
///Field `ENA_RFREG_TIMER` writer - Timeout for the RF regulator startup (duration in ENA_RF_REG state)
pub type ENA_RFREG_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Timeout for the RF regulator startup (duration in ENA_RF_REG state)
    #[inline(always)]
    pub fn ena_rfreg_timer(&self) -> ENA_RFREG_TIMER_R {
        ENA_RFREG_TIMER_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_FSM0_TIMEOUT")
            .field("ena_rfreg_timer", &self.ena_rfreg_timer())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Timeout for the RF regulator startup (duration in ENA_RF_REG state)
    #[inline(always)]
    pub fn ena_rfreg_timer(&mut self) -> ENA_RFREG_TIMER_W<'_, RF_FSM0_TIMEOUTrs> {
        ENA_RFREG_TIMER_W::new(self, 0)
    }
}
/**RF_FSM0_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm0_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm0_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM0_TIMEOUT)*/
pub struct RF_FSM0_TIMEOUTrs;
impl crate::RegisterSpec for RF_FSM0_TIMEOUTrs {
    type Ux = u32;
}
///`read()` method returns [`rf_fsm0_timeout::R`](R) reader structure
impl crate::Readable for RF_FSM0_TIMEOUTrs {}
///`write(|w| ..)` method takes [`rf_fsm0_timeout::W`](W) writer structure
impl crate::Writable for RF_FSM0_TIMEOUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RF_FSM0_TIMEOUT to value 0
impl crate::Resettable for RF_FSM0_TIMEOUTrs {}
