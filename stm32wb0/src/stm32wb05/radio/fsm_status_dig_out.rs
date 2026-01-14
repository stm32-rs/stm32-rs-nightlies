///Register `FSM_STATUS_DIG_OUT` reader
pub type R = crate::R<FSM_STATUS_DIG_OUTrs>;
///Field `STATUS` reader - RF FSM state:
pub type STATUS_R = crate::FieldReader;
///Field `SYNTH_CAL_ERROR` reader - PLL calibration error
pub type SYNTH_CAL_ERROR_R = crate::BitReader;
impl R {
    ///Bits 0:4 - RF FSM state:
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 7 - PLL calibration error
    #[inline(always)]
    pub fn synth_cal_error(&self) -> SYNTH_CAL_ERROR_R {
        SYNTH_CAL_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_STATUS_DIG_OUT")
            .field("status", &self.status())
            .field("synth_cal_error", &self.synth_cal_error())
            .finish()
    }
}
/**FSM_STATUS_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`fsm_status_dig_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RADIO:FSM_STATUS_DIG_OUT)*/
pub struct FSM_STATUS_DIG_OUTrs;
impl crate::RegisterSpec for FSM_STATUS_DIG_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`fsm_status_dig_out::R`](R) reader structure
impl crate::Readable for FSM_STATUS_DIG_OUTrs {}
///`reset()` method sets FSM_STATUS_DIG_OUT to value 0
impl crate::Resettable for FSM_STATUS_DIG_OUTrs {}
