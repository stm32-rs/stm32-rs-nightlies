///Register `RADIO_FSM_INFO` reader
pub type R = crate::R<RADIO_FSM_INFOrs>;
///Field `RADIO_FSM_STATE` reader - State of the Radio FSM
pub type RADIO_FSM_STATE_R = crate::FieldReader;
impl R {
    ///Bits 0:4 - State of the Radio FSM
    #[inline(always)]
    pub fn radio_fsm_state(&self) -> RADIO_FSM_STATE_R {
        RADIO_FSM_STATE_R::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RADIO_FSM_INFO")
            .field("radio_fsm_state", &self.radio_fsm_state())
            .finish()
    }
}
/**RADIO_FSM_INFO register

You can [`read`](crate::Reg::read) this register and get [`radio_fsm_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:RADIO_FSM_INFO)*/
pub struct RADIO_FSM_INFOrs;
impl crate::RegisterSpec for RADIO_FSM_INFOrs {
    type Ux = u32;
}
///`read()` method returns [`radio_fsm_info::R`](R) reader structure
impl crate::Readable for RADIO_FSM_INFOrs {}
///`reset()` method sets RADIO_FSM_INFO to value 0
impl crate::Resettable for RADIO_FSM_INFOrs {}
