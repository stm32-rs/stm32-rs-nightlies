///Register `PVTTMR_SR` reader
pub type R = crate::R<PVTTMR_SRrs>;
///Field `TMR_BUSY` reader - Counter busy flag
pub type TMR_BUSY_R = crate::BitReader;
///Field `TMR_DONE` reader - Counter delay expiration flag
pub type TMR_DONE_R = crate::BitReader;
impl R {
    ///Bit 0 - Counter busy flag
    #[inline(always)]
    pub fn tmr_busy(&self) -> TMR_BUSY_R {
        TMR_BUSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Counter delay expiration flag
    #[inline(always)]
    pub fn tmr_done(&self) -> TMR_DONE_R {
        TMR_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVTTMR_SR")
            .field("tmr_busy", &self.tmr_busy())
            .field("tmr_done", &self.tmr_done())
            .finish()
    }
}
/**DTS PVT timer status register

You can [`read`](crate::Reg::read) this register and get [`pvttmr_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:PVTTMR_SR)*/
pub struct PVTTMR_SRrs;
impl crate::RegisterSpec for PVTTMR_SRrs {
    type Ux = u32;
}
///`read()` method returns [`pvttmr_sr::R`](R) reader structure
impl crate::Readable for PVTTMR_SRrs {}
///`reset()` method sets PVTTMR_SR to value 0
impl crate::Resettable for PVTTMR_SRrs {}
