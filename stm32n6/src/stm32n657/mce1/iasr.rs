///Register `IASR` reader
pub type R = crate::R<IASRrs>;
///Field `IAEF` reader - Illegal access error flag
pub type IAEF_R = crate::BitReader;
impl R {
    ///Bit 1 - Illegal access error flag
    #[inline(always)]
    pub fn iaef(&self) -> IAEF_R {
        IAEF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IASR").field("iaef", &self.iaef()).finish()
    }
}
/**MCE illegal access status register

You can [`read`](crate::Reg::read) this register and get [`iasr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:IASR)*/
pub struct IASRrs;
impl crate::RegisterSpec for IASRrs {
    type Ux = u32;
}
///`read()` method returns [`iasr::R`](R) reader structure
impl crate::Readable for IASRrs {}
///`reset()` method sets IASR to value 0
impl crate::Resettable for IASRrs {}
