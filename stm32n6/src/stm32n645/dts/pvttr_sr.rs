///Register `PVTTR_SR` reader
pub type R = crate::R<PVTTR_SRrs>;
///Field `TMR_IRQ_STATUS` reader - Timer IRQ status bit after masking
pub type TMR_IRQ_STATUS_R = crate::BitReader;
impl R {
    ///Bit 0 - Timer IRQ status bit after masking
    #[inline(always)]
    pub fn tmr_irq_status(&self) -> TMR_IRQ_STATUS_R {
        TMR_IRQ_STATUS_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVTTR_SR")
            .field("tmr_irq_status", &self.tmr_irq_status())
            .finish()
    }
}
/**DTS PVT IRQ timer status register

You can [`read`](crate::Reg::read) this register and get [`pvttr_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DTS:PVTTR_SR)*/
pub struct PVTTR_SRrs;
impl crate::RegisterSpec for PVTTR_SRrs {
    type Ux = u32;
}
///`read()` method returns [`pvttr_sr::R`](R) reader structure
impl crate::Readable for PVTTR_SRrs {}
///`reset()` method sets PVTTR_SR to value 0
impl crate::Resettable for PVTTR_SRrs {}
