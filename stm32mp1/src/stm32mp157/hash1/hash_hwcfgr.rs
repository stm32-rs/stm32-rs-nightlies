#[doc = "Register `HASH_HWCFGR` reader"]
pub type R = crate::R<HASH_HWCFGRrs>;
#[doc = "Field `CFG1` reader - CFG1"]
pub type CFG1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "HASH Hardware Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_HWCFGRrs;
impl crate::RegisterSpec for HASH_HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hwcfgr::R`](R) reader structure"]
impl crate::Readable for HASH_HWCFGRrs {}
#[doc = "`reset()` method sets HASH_HWCFGR to value 0x01"]
impl crate::Resettable for HASH_HWCFGRrs {
    const RESET_VALUE: u32 = 0x01;
}
