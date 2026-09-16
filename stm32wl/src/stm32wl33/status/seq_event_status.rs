///Register `SEQ_EVENT_STATUS` reader
pub type R = crate::R<SEQ_EVENT_STATUSrs>;
///Field `SEQ_EVENT_STATUS` reader - Current value of the seq_event_status used by the Sequencer for next action mask comparison.
pub type SEQ_EVENT_STATUS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Current value of the seq_event_status used by the Sequencer for next action mask comparison.
    #[inline(always)]
    pub fn seq_event_status(&self) -> SEQ_EVENT_STATUS_R {
        SEQ_EVENT_STATUS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ_EVENT_STATUS")
            .field("seq_event_status", &self.seq_event_status())
            .finish()
    }
}
/**SEQ_EVENT_STATUS register

You can [`read`](crate::Reg::read) this register and get [`seq_event_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATUS:SEQ_EVENT_STATUS)*/
pub struct SEQ_EVENT_STATUSrs;
impl crate::RegisterSpec for SEQ_EVENT_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`seq_event_status::R`](R) reader structure
impl crate::Readable for SEQ_EVENT_STATUSrs {}
///`reset()` method sets SEQ_EVENT_STATUS to value 0
impl crate::Resettable for SEQ_EVENT_STATUSrs {}
