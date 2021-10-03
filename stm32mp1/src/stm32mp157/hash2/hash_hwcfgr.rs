#[doc = "Register `HASH_HWCFGR` reader"]
pub struct R(crate::R<HASH_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFG1` reader - CFG1"]
pub struct CFG1_R(crate::FieldReader<u8, u8>);
impl CFG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "HASH Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hwcfgr](index.html) module"]
pub struct HASH_HWCFGR_SPEC;
impl crate::RegisterSpec for HASH_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_hwcfgr::R](R) reader structure"]
impl crate::Readable for HASH_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HASH_HWCFGR to value 0x01"]
impl crate::Resettable for HASH_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
