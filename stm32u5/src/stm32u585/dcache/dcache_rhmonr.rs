///Register `DCACHE_RHMONR` reader
pub type R = crate::R<DCACHE_RHMONRrs>;
///Field `RHITMON` reader - RHITMON
pub type RHITMON_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RHITMON
    #[inline(always)]
    pub fn rhitmon(&self) -> RHITMON_R {
        RHITMON_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_RHMONR")
            .field("rhitmon", &self.rhitmon())
            .finish()
    }
}
/**DCACHE read-hit monitor register

You can [`read`](crate::Reg::read) this register and get [`dcache_rhmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_RHMONR)*/
pub struct DCACHE_RHMONRrs;
impl crate::RegisterSpec for DCACHE_RHMONRrs {
    type Ux = u32;
}
///`read()` method returns [`dcache_rhmonr::R`](R) reader structure
impl crate::Readable for DCACHE_RHMONRrs {}
///`reset()` method sets DCACHE_RHMONR to value 0
impl crate::Resettable for DCACHE_RHMONRrs {
    const RESET_VALUE: u32 = 0;
}
