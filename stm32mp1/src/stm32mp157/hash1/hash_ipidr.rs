#[doc = "Register `HASH_IPIDR` reader"]
pub type R = crate::R<HASH_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "HASH Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_IPIDRrs;
impl crate::RegisterSpec for HASH_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_ipidr::R`](R) reader structure"]
impl crate::Readable for HASH_IPIDRrs {}
#[doc = "`reset()` method sets HASH_IPIDR to value 0x0017_0031"]
impl crate::Resettable for HASH_IPIDRrs {
    const RESET_VALUE: u32 = 0x0017_0031;
}
