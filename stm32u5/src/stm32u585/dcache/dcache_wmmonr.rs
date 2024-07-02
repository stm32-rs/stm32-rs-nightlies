///Register `DCACHE_WMMONR` reader
pub type R = crate::R<DCACHE_WMMONRrs>;
///Field `WMISSMON` reader - WMISSMON
pub type WMISSMON_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - WMISSMON
    #[inline(always)]
    pub fn wmissmon(&self) -> WMISSMON_R {
        WMISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_WMMONR")
            .field("wmissmon", &self.wmissmon())
            .finish()
    }
}
/**write-miss monitor register

You can [`read`](crate::Reg::read) this register and get [`dcache_wmmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_WMMONR)*/
pub struct DCACHE_WMMONRrs;
impl crate::RegisterSpec for DCACHE_WMMONRrs {
    type Ux = u32;
}
///`read()` method returns [`dcache_wmmonr::R`](R) reader structure
impl crate::Readable for DCACHE_WMMONRrs {}
///`reset()` method sets DCACHE_WMMONR to value 0
impl crate::Resettable for DCACHE_WMMONRrs {
    const RESET_VALUE: u32 = 0;
}
