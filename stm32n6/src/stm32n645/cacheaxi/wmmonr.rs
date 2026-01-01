///Register `WMMONR` reader
pub type R = crate::R<WMMONRrs>;
///Field `WMISSMON` reader - cache write-miss monitor counter
pub type WMISSMON_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - cache write-miss monitor counter
    #[inline(always)]
    pub fn wmissmon(&self) -> WMISSMON_R {
        WMISSMON_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WMMONR")
            .field("wmissmon", &self.wmissmon())
            .finish()
    }
}
/**CACHEAXI write-miss monitor register

You can [`read`](crate::Reg::read) this register and get [`wmmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#CACHEAXI:WMMONR)*/
pub struct WMMONRrs;
impl crate::RegisterSpec for WMMONRrs {
    type Ux = u32;
}
///`read()` method returns [`wmmonr::R`](R) reader structure
impl crate::Readable for WMMONRrs {}
///`reset()` method sets WMMONR to value 0
impl crate::Resettable for WMMONRrs {}
