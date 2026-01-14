///Register `WTMONR` reader
pub type R = crate::R<WTMONRrs>;
///Field `WTMON` reader - cache write-through monitor counter
pub type WTMON_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - cache write-through monitor counter
    #[inline(always)]
    pub fn wtmon(&self) -> WTMON_R {
        WTMON_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WTMONR")
            .field("wtmon", &self.wtmon())
            .finish()
    }
}
/**CACHEAXI write-through monitor register

You can [`read`](crate::Reg::read) this register and get [`wtmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CACHEAXI:WTMONR)*/
pub struct WTMONRrs;
impl crate::RegisterSpec for WTMONRrs {
    type Ux = u32;
}
///`read()` method returns [`wtmonr::R`](R) reader structure
impl crate::Readable for WTMONRrs {}
///`reset()` method sets WTMONR to value 0
impl crate::Resettable for WTMONRrs {}
