///Register `RHMONR` reader
pub type R = crate::R<RHMONRrs>;
///Field `RHITMON` reader - cache read-hit monitor counter
pub type RHITMON_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - cache read-hit monitor counter
    #[inline(always)]
    pub fn rhitmon(&self) -> RHITMON_R {
        RHITMON_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RHMONR")
            .field("rhitmon", &self.rhitmon())
            .finish()
    }
}
/**CACHEAXI read-hit monitor register

You can [`read`](crate::Reg::read) this register and get [`rhmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CACHEAXI:RHMONR)*/
pub struct RHMONRrs;
impl crate::RegisterSpec for RHMONRrs {
    type Ux = u32;
}
///`read()` method returns [`rhmonr::R`](R) reader structure
impl crate::Readable for RHMONRrs {}
///`reset()` method sets RHMONR to value 0
impl crate::Resettable for RHMONRrs {}
