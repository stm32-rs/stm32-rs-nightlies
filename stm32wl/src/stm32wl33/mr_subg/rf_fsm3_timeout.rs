///Register `RF_FSM3_TIMEOUT` reader
pub type R = crate::R<RF_FSM3_TIMEOUTrs>;
///Register `RF_FSM3_TIMEOUT` writer
pub type W = crate::W<RF_FSM3_TIMEOUTrs>;
///Field `VCO_LOCK_TIMER` reader - Timeout for the RF PLL lock event when no calibration is requested (duration in LOCKRXTX state)
pub type VCO_LOCK_TIMER_R = crate::FieldReader;
///Field `VCO_LOCK_TIMER` writer - Timeout for the RF PLL lock event when no calibration is requested (duration in LOCKRXTX state)
pub type VCO_LOCK_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Timeout for the RF PLL lock event when no calibration is requested (duration in LOCKRXTX state)
    #[inline(always)]
    pub fn vco_lock_timer(&self) -> VCO_LOCK_TIMER_R {
        VCO_LOCK_TIMER_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_FSM3_TIMEOUT")
            .field("vco_lock_timer", &self.vco_lock_timer())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Timeout for the RF PLL lock event when no calibration is requested (duration in LOCKRXTX state)
    #[inline(always)]
    pub fn vco_lock_timer(&mut self) -> VCO_LOCK_TIMER_W<'_, RF_FSM3_TIMEOUTrs> {
        VCO_LOCK_TIMER_W::new(self, 0)
    }
}
/**RF_FSM3_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm3_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm3_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM3_TIMEOUT)*/
pub struct RF_FSM3_TIMEOUTrs;
impl crate::RegisterSpec for RF_FSM3_TIMEOUTrs {
    type Ux = u32;
}
///`read()` method returns [`rf_fsm3_timeout::R`](R) reader structure
impl crate::Readable for RF_FSM3_TIMEOUTrs {}
///`write(|w| ..)` method takes [`rf_fsm3_timeout::W`](W) writer structure
impl crate::Writable for RF_FSM3_TIMEOUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RF_FSM3_TIMEOUT to value 0x28
impl crate::Resettable for RF_FSM3_TIMEOUTrs {
    const RESET_VALUE: u32 = 0x28;
}
