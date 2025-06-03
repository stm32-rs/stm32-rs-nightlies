///Register `PVTTMRRAW_ISR` reader
pub type R = crate::R<PVTTMRRAW_ISRrs>;
///Field `TMR_IRQ_RAW_STATUS` reader - TMR IRQ status bit before masking
pub type TMR_IRQ_RAW_STATUS_R = crate::BitReader;
impl R {
    ///Bit 0 - TMR IRQ status bit before masking
    #[inline(always)]
    pub fn tmr_irq_raw_status(&self) -> TMR_IRQ_RAW_STATUS_R {
        TMR_IRQ_RAW_STATUS_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVTTMRRAW_ISR")
            .field("tmr_irq_raw_status", &self.tmr_irq_raw_status())
            .finish()
    }
}
/**DTS PVT IRQ timer raw status register

You can [`read`](crate::Reg::read) this register and get [`pvttmrraw_isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DTS:PVTTMRRAW_ISR)*/
pub struct PVTTMRRAW_ISRrs;
impl crate::RegisterSpec for PVTTMRRAW_ISRrs {
    type Ux = u32;
}
///`read()` method returns [`pvttmrraw_isr::R`](R) reader structure
impl crate::Readable for PVTTMRRAW_ISRrs {}
///`reset()` method sets PVTTMRRAW_ISR to value 0
impl crate::Resettable for PVTTMRRAW_ISRrs {}
