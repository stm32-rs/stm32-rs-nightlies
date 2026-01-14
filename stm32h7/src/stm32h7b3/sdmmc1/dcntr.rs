///Register `DCNTR` reader
pub type R = crate::R<DCNTRrs>;
///Field `DATACOUNT` reader - Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect.
pub type DATACOUNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:24 - Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect.
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCNTR")
            .field("datacount", &self.datacount())
            .finish()
    }
}
/**The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.

You can [`read`](crate::Reg::read) this register and get [`dcntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SDMMC1:DCNTR)*/
pub struct DCNTRrs;
impl crate::RegisterSpec for DCNTRrs {
    type Ux = u32;
}
///`read()` method returns [`dcntr::R`](R) reader structure
impl crate::Readable for DCNTRrs {}
///`reset()` method sets DCNTR to value 0
impl crate::Resettable for DCNTRrs {}
