#[doc = "Register `RNG_HWCFGR` reader"]
pub type R = crate::R<RNG_HWCFGRrs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RNG_HWCFGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RNG hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_HWCFGRrs;
impl crate::RegisterSpec for RNG_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_hwcfgr::R`](R) reader structure"]
impl crate::Readable for RNG_HWCFGRrs {}
#[doc = "`reset()` method sets RNG_HWCFGR to value 0x06"]
impl crate::Resettable for RNG_HWCFGRrs {
    const RESET_VALUE: u32 = 0x06;
}
