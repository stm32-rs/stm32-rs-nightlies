///Register `WAMMONR` reader
pub type R = crate::R<WAMMONRrs>;
///Field `WAMMON` reader - cache write-allocate miss monitor counter
pub type WAMMON_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - cache write-allocate miss monitor counter
    #[inline(always)]
    pub fn wammon(&self) -> WAMMON_R {
        WAMMON_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAMMONR")
            .field("wammon", &self.wammon())
            .finish()
    }
}
/**CACHEAXI write-allocate miss monitor register

You can [`read`](crate::Reg::read) this register and get [`wammonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#CACHEAXI:WAMMONR)*/
pub struct WAMMONRrs;
impl crate::RegisterSpec for WAMMONRrs {
    type Ux = u32;
}
///`read()` method returns [`wammonr::R`](R) reader structure
impl crate::Readable for WAMMONRrs {}
///`reset()` method sets WAMMONR to value 0
impl crate::Resettable for WAMMONRrs {}
