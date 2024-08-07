///Register `RNG_HWCFGR` reader
pub type R = crate::R<RNG_HWCFGRrs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**RNG hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`rng_hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RNG1:RNG_HWCFGR)*/
pub struct RNG_HWCFGRrs;
impl crate::RegisterSpec for RNG_HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`rng_hwcfgr::R`](R) reader structure
impl crate::Readable for RNG_HWCFGRrs {}
///`reset()` method sets RNG_HWCFGR to value 0x06
impl crate::Resettable for RNG_HWCFGRrs {
    const RESET_VALUE: u32 = 0x06;
}
