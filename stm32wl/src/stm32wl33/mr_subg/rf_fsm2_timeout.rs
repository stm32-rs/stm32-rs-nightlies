///Register `RF_FSM2_TIMEOUT` reader
pub type R = crate::R<RF_FSM2_TIMEOUTrs>;
///Register `RF_FSM2_TIMEOUT` writer
pub type W = crate::W<RF_FSM2_TIMEOUTrs>;
///Field `VCO_CALIB_LOCK_TIMER` reader - Timeout for the RF PLL calibration + RF PLL lock (duration in CALIB_VCO+LOCKRXTX state)
pub type VCO_CALIB_LOCK_TIMER_R = crate::FieldReader;
///Field `VCO_CALIB_LOCK_TIMER` writer - Timeout for the RF PLL calibration + RF PLL lock (duration in CALIB_VCO+LOCKRXTX state)
pub type VCO_CALIB_LOCK_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Timeout for the RF PLL calibration + RF PLL lock (duration in CALIB_VCO+LOCKRXTX state)
    #[inline(always)]
    pub fn vco_calib_lock_timer(&self) -> VCO_CALIB_LOCK_TIMER_R {
        VCO_CALIB_LOCK_TIMER_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_FSM2_TIMEOUT")
            .field("vco_calib_lock_timer", &self.vco_calib_lock_timer())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Timeout for the RF PLL calibration + RF PLL lock (duration in CALIB_VCO+LOCKRXTX state)
    #[inline(always)]
    pub fn vco_calib_lock_timer(&mut self) -> VCO_CALIB_LOCK_TIMER_W<'_, RF_FSM2_TIMEOUTrs> {
        VCO_CALIB_LOCK_TIMER_W::new(self, 0)
    }
}
/**RF_FSM2_TIMEOUT register

You can [`read`](crate::Reg::read) this register and get [`rf_fsm2_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_fsm2_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:RF_FSM2_TIMEOUT)*/
pub struct RF_FSM2_TIMEOUTrs;
impl crate::RegisterSpec for RF_FSM2_TIMEOUTrs {
    type Ux = u32;
}
///`read()` method returns [`rf_fsm2_timeout::R`](R) reader structure
impl crate::Readable for RF_FSM2_TIMEOUTrs {}
///`write(|w| ..)` method takes [`rf_fsm2_timeout::W`](W) writer structure
impl crate::Writable for RF_FSM2_TIMEOUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RF_FSM2_TIMEOUT to value 0x50
impl crate::Resettable for RF_FSM2_TIMEOUTrs {
    const RESET_VALUE: u32 = 0x50;
}
