///Register `EVIMONR` reader
pub type R = crate::R<EVIMONRrs>;
///Field `EVIMON` reader - cache eviction monitor counter
pub type EVIMON_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - cache eviction monitor counter
    #[inline(always)]
    pub fn evimon(&self) -> EVIMON_R {
        EVIMON_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVIMONR")
            .field("evimon", &self.evimon())
            .finish()
    }
}
/**CACHEAXI eviction monitor register

You can [`read`](crate::Reg::read) this register and get [`evimonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#CACHEAXI:EVIMONR)*/
pub struct EVIMONRrs;
impl crate::RegisterSpec for EVIMONRrs {
    type Ux = u32;
}
///`read()` method returns [`evimonr::R`](R) reader structure
impl crate::Readable for EVIMONRrs {}
///`reset()` method sets EVIMONR to value 0
impl crate::Resettable for EVIMONRrs {}
