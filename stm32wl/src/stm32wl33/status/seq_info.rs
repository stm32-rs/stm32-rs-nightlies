///Register `SEQ_INFO` reader
pub type R = crate::R<SEQ_INFOrs>;
///Field `SEQ_FSM_STATE` reader - Current state of the Sequencer
pub type SEQ_FSM_STATE_R = crate::FieldReader;
impl R {
    ///Bits 0:4 - Current state of the Sequencer
    #[inline(always)]
    pub fn seq_fsm_state(&self) -> SEQ_FSM_STATE_R {
        SEQ_FSM_STATE_R::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ_INFO")
            .field("seq_fsm_state", &self.seq_fsm_state())
            .finish()
    }
}
/**SEQ_INFO register

You can [`read`](crate::Reg::read) this register and get [`seq_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:SEQ_INFO)*/
pub struct SEQ_INFOrs;
impl crate::RegisterSpec for SEQ_INFOrs {
    type Ux = u32;
}
///`read()` method returns [`seq_info::R`](R) reader structure
impl crate::Readable for SEQ_INFOrs {}
///`reset()` method sets SEQ_INFO to value 0
impl crate::Resettable for SEQ_INFOrs {}
