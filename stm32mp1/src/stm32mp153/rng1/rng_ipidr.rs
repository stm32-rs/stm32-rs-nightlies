#[doc = "Register `RNG_IPIDR` reader"]
pub type R = crate::R<RNG_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "RNG identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rng_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_IPIDRrs;
impl crate::RegisterSpec for RNG_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_ipidr::R`](R) reader structure"]
impl crate::Readable for RNG_IPIDRrs {}
#[doc = "`reset()` method sets RNG_IPIDR to value 0x0017_0041"]
impl crate::Resettable for RNG_IPIDRrs {
    const RESET_VALUE: u32 = 0x0017_0041;
}
