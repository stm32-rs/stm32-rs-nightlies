#[doc = "Register `LTDC_IDR` reader"]
pub struct R(crate::R<LTDC_IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_IDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REV` reader - REV"]
pub struct REV_R(crate::FieldReader<u8, u8>);
impl REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINVER` reader - MINVER"]
pub struct MINVER_R(crate::FieldReader<u8, u8>);
impl MINVER_R {
    pub(crate) fn new(bits: u8) -> Self {
        MINVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINVER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAJVER` reader - MAJVER"]
pub struct MAJVER_R(crate::FieldReader<u8, u8>);
impl MAJVER_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAJVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJVER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - REV"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MINVER"]
    #[inline(always)]
    pub fn minver(&self) -> MINVER_R {
        MINVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MAJVER"]
    #[inline(always)]
    pub fn majver(&self) -> MAJVER_R {
        MAJVER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "LTDC identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_idr](index.html) module"]
pub struct LTDC_IDR_SPEC;
impl crate::RegisterSpec for LTDC_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_idr::R](R) reader structure"]
impl crate::Readable for LTDC_IDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTDC_IDR to value 0x0001_0300"]
impl crate::Resettable for LTDC_IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0300
    }
}
