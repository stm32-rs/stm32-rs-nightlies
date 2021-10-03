#[doc = "Register `STGENC_PIDR3` reader"]
pub struct R(crate::R<STGENC_PIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_PIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_PIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_PIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMOD` reader - CMOD"]
pub struct CMOD_R(crate::FieldReader<u8, u8>);
impl CMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVAND` reader - REVAND"]
pub struct REVAND_R(crate::FieldReader<u8, u8>);
impl REVAND_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVAND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - CMOD"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - REVAND"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "STGENC peripheral ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr3](index.html) module"]
pub struct STGENC_PIDR3_SPEC;
impl crate::RegisterSpec for STGENC_PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_pidr3::R](R) reader structure"]
impl crate::Readable for STGENC_PIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENC_PIDR3 to value 0"]
impl crate::Resettable for STGENC_PIDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
