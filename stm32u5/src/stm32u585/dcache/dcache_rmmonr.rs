///Register `DCACHE_RMMONR` reader
pub type R = crate::R<DCACHE_RMMONRrs>;
///Field `MRISSMON` reader - RMISSMON
pub type MRISSMON_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - RMISSMON
    #[inline(always)]
    pub fn mrissmon(&self) -> MRISSMON_R {
        MRISSMON_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_RMMONR")
            .field("mrissmon", &self.mrissmon())
            .finish()
    }
}
/**DCACHE read-miss monitor register

You can [`read`](crate::Reg::read) this register and get [`dcache_rmmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_RMMONR)*/
pub struct DCACHE_RMMONRrs;
impl crate::RegisterSpec for DCACHE_RMMONRrs {
    type Ux = u32;
}
///`read()` method returns [`dcache_rmmonr::R`](R) reader structure
impl crate::Readable for DCACHE_RMMONRrs {}
///`reset()` method sets DCACHE_RMMONR to value 0
impl crate::Resettable for DCACHE_RMMONRrs {
    const RESET_VALUE: u32 = 0;
}
