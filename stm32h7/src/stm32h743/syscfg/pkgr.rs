#[doc = "Register `PKGR` reader"]
pub struct R(crate::R<PKGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKG` reader - Package"]
pub struct PKG_R(crate::FieldReader<u8, u8>);
impl PKG_R {
    pub(crate) fn new(bits: u8) -> Self {
        PKG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Package"]
    #[inline(always)]
    pub fn pkg(&self) -> PKG_R {
        PKG_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "SYSCFG package register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkgr](index.html) module"]
pub struct PKGR_SPEC;
impl crate::RegisterSpec for PKGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkgr::R](R) reader structure"]
impl crate::Readable for PKGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKGR to value 0"]
impl crate::Resettable for PKGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
