///Register `RAMMONR` reader
pub type R = crate::R<RAMMONRrs>;
///Field `RAMMON` reader - cache read-allocate miss monitor counter
pub type RAMMON_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - cache read-allocate miss monitor counter
    #[inline(always)]
    pub fn rammon(&self) -> RAMMON_R {
        RAMMON_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMMONR")
            .field("rammon", &self.rammon())
            .finish()
    }
}
/**CACHEAXI read-allocate miss monitor register

You can [`read`](crate::Reg::read) this register and get [`rammonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:RAMMONR)*/
pub struct RAMMONRrs;
impl crate::RegisterSpec for RAMMONRrs {
    type Ux = u32;
}
///`read()` method returns [`rammonr::R`](R) reader structure
impl crate::Readable for RAMMONRrs {}
///`reset()` method sets RAMMONR to value 0
impl crate::Resettable for RAMMONRrs {}
