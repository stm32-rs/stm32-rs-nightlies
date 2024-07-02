///Register `DCACHE_WHMONR` reader
pub type R = crate::R<DCACHE_WHMONRrs>;
///Field `WHITMON` reader - WHITMON
pub type WHITMON_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - WHITMON
    #[inline(always)]
    pub fn whitmon(&self) -> WHITMON_R {
        WHITMON_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_WHMONR")
            .field("whitmon", &self.whitmon())
            .finish()
    }
}
/**write-hit monitor register

You can [`read`](crate::Reg::read) this register and get [`dcache_whmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DCACHE1:DCACHE_WHMONR)*/
pub struct DCACHE_WHMONRrs;
impl crate::RegisterSpec for DCACHE_WHMONRrs {
    type Ux = u32;
}
///`read()` method returns [`dcache_whmonr::R`](R) reader structure
impl crate::Readable for DCACHE_WHMONRrs {}
///`reset()` method sets DCACHE_WHMONR to value 0
impl crate::Resettable for DCACHE_WHMONRrs {
    const RESET_VALUE: u32 = 0;
}
