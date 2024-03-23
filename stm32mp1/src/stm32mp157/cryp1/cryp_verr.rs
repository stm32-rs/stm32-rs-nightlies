#[doc = "Register `CRYP_VERR` reader"]
pub type R = crate::R<CRYP_VERRrs>;
#[doc = "Field `VER` reader - VER"]
pub type VER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - VER"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CRYP HW Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_verr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_VERRrs;
impl crate::RegisterSpec for CRYP_VERRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_verr::R`](R) reader structure"]
impl crate::Readable for CRYP_VERRrs {}
#[doc = "`reset()` method sets CRYP_VERR to value 0x22"]
impl crate::Resettable for CRYP_VERRrs {
    const RESET_VALUE: u32 = 0x22;
}
