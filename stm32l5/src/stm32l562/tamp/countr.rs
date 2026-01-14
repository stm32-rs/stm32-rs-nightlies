///Register `COUNTR` reader
pub type R = crate::R<COUNTRrs>;
///Field `COUNT` reader - COUNT
pub type COUNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - COUNT
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNTR")
            .field("count", &self.count())
            .finish()
    }
}
/**TAMP monotonic counter register

You can [`read`](crate::Reg::read) this register and get [`countr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#TAMP:COUNTR)*/
pub struct COUNTRrs;
impl crate::RegisterSpec for COUNTRrs {
    type Ux = u32;
}
///`read()` method returns [`countr::R`](R) reader structure
impl crate::Readable for COUNTRrs {}
///`reset()` method sets COUNTR to value 0
impl crate::Resettable for COUNTRrs {}
