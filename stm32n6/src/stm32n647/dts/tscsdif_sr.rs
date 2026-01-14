///Register `TSCSDIF_SR` reader
pub type R = crate::R<TSCSDIF_SRrs>;
///Field `SDIF_BUSY` reader - SDIF busy flag
pub type SDIF_BUSY_R = crate::BitReader;
///Field `SDIF_LOCK` reader - SDIF locked flag
pub type SDIF_LOCK_R = crate::BitReader;
impl R {
    ///Bit 0 - SDIF busy flag
    #[inline(always)]
    pub fn sdif_busy(&self) -> SDIF_BUSY_R {
        SDIF_BUSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SDIF locked flag
    #[inline(always)]
    pub fn sdif_lock(&self) -> SDIF_LOCK_R {
        SDIF_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCSDIF_SR")
            .field("sdif_busy", &self.sdif_busy())
            .field("sdif_lock", &self.sdif_lock())
            .finish()
    }
}
/**DTS TSC SDIF status register

You can [`read`](crate::Reg::read) this register and get [`tscsdif_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSDIF_SR)*/
pub struct TSCSDIF_SRrs;
impl crate::RegisterSpec for TSCSDIF_SRrs {
    type Ux = u32;
}
///`read()` method returns [`tscsdif_sr::R`](R) reader structure
impl crate::Readable for TSCSDIF_SRrs {}
///`reset()` method sets TSCSDIF_SR to value 0
impl crate::Resettable for TSCSDIF_SRrs {}
