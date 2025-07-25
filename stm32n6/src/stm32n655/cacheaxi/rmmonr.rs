///Register `RMMONR` reader
pub type R = crate::R<RMMONRrs>;
///Field `RMISSMON` reader - cache read-miss monitor counter
pub type RMISSMON_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - cache read-miss monitor counter
    #[inline(always)]
    pub fn rmissmon(&self) -> RMISSMON_R {
        RMISSMON_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMMONR")
            .field("rmissmon", &self.rmissmon())
            .finish()
    }
}
/**CACHEAXI read-miss monitor register

You can [`read`](crate::Reg::read) this register and get [`rmmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:RMMONR)*/
pub struct RMMONRrs;
impl crate::RegisterSpec for RMMONRrs {
    type Ux = u32;
}
///`read()` method returns [`rmmonr::R`](R) reader structure
impl crate::Readable for RMMONRrs {}
///`reset()` method sets RMMONR to value 0
impl crate::Resettable for RMMONRrs {}
