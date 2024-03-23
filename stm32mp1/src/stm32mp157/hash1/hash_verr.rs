#[doc = "Register `HASH_VERR` reader"]
pub type R = crate::R<HASH_VERRrs>;
#[doc = "Field `VER` reader - VER"]
pub type VER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - VER"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "HASH Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_verr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_VERRrs;
impl crate::RegisterSpec for HASH_VERRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_verr::R`](R) reader structure"]
impl crate::Readable for HASH_VERRrs {}
#[doc = "`reset()` method sets HASH_VERR to value 0x23"]
impl crate::Resettable for HASH_VERRrs {
    const RESET_VALUE: u32 = 0x23;
}
