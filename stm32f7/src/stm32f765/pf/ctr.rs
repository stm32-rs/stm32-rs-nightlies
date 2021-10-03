#[doc = "Register `CTR` reader"]
pub struct R(crate::R<CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `_IminLine` reader - IminLine"]
pub struct _IMINLINE_R(crate::FieldReader<u8, u8>);
impl _IMINLINE_R {
    pub(crate) fn new(bits: u8) -> Self {
        _IMINLINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for _IMINLINE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMinLine` reader - DMinLine"]
pub struct DMINLINE_R(crate::FieldReader<u8, u8>);
impl DMINLINE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMINLINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMINLINE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERG` reader - ERG"]
pub struct ERG_R(crate::FieldReader<u8, u8>);
impl ERG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ERG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWG` reader - CWG"]
pub struct CWG_R(crate::FieldReader<u8, u8>);
impl CWG_R {
    pub(crate) fn new(bits: u8) -> Self {
        CWG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Format` reader - Format"]
pub struct FORMAT_R(crate::FieldReader<u8, u8>);
impl FORMAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FORMAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORMAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - IminLine"]
    #[inline(always)]
    pub fn _imin_line(&self) -> _IMINLINE_R {
        _IMINLINE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMinLine"]
    #[inline(always)]
    pub fn dmin_line(&self) -> DMINLINE_R {
        DMINLINE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - ERG"]
    #[inline(always)]
    pub fn erg(&self) -> ERG_R {
        ERG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CWG"]
    #[inline(always)]
    pub fn cwg(&self) -> CWG_R {
        CWG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - Format"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
#[doc = "Cache Type register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](index.html) module"]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr::R](R) reader structure"]
impl crate::Readable for CTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTR to value 0x8303_c003"]
impl crate::Resettable for CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8303_c003
    }
}
