///Register `MESR` reader
pub type R = crate::R<MESRrs>;
///Field `MEF` reader - memory erase flag This bit is set by hardware when BKPRAM and PKA SRAM erase is ongoing after a POWER ON reset or one tamper event (see Section 50: Tamper and backup registers (TAMP) for details). This bit is cleared when the erase is done.
pub type MEF_R = crate::BitReader;
impl R {
    ///Bit 0 - memory erase flag This bit is set by hardware when BKPRAM and PKA SRAM erase is ongoing after a POWER ON reset or one tamper event (see Section 50: Tamper and backup registers (TAMP) for details). This bit is cleared when the erase is done.
    #[inline(always)]
    pub fn mef(&self) -> MEF_R {
        MEF_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MESR").field("mef", &self.mef()).finish()
    }
}
/**SBS memory erase status register

You can [`read`](crate::Reg::read) this register and get [`mesr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SBS:MESR)*/
pub struct MESRrs;
impl crate::RegisterSpec for MESRrs {
    type Ux = u32;
}
///`read()` method returns [`mesr::R`](R) reader structure
impl crate::Readable for MESRrs {}
///`reset()` method sets MESR to value 0
impl crate::Resettable for MESRrs {}
