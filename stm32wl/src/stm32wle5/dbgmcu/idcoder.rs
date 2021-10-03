#[doc = "Register `IDCODER` reader"]
pub struct R(crate::R<IDCODER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDCODER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDCODER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDCODER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEV_ID` reader - Device ID"]
pub struct DEV_ID_R(crate::FieldReader<u16, u16>);
impl DEV_ID_R {
    pub(crate) fn new(bits: u16) -> Self {
        DEV_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV_ID` reader - Revision"]
pub struct REV_ID_R(crate::FieldReader<u16, u16>);
impl REV_ID_R {
    pub(crate) fn new(bits: u16) -> Self {
        REV_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Device ID"]
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - Revision"]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "DBGMCU Identity Code Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idcoder](index.html) module"]
pub struct IDCODER_SPEC;
impl crate::RegisterSpec for IDCODER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idcoder::R](R) reader structure"]
impl crate::Readable for IDCODER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDCODER to value 0x1000_6497"]
impl crate::Resettable for IDCODER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_6497
    }
}
