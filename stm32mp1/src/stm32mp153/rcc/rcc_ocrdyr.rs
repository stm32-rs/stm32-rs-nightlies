#[doc = "Register `RCC_OCRDYR` reader"]
pub struct R(crate::R<RCC_OCRDYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_OCRDYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_OCRDYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_OCRDYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HSIRDY` reader - HSIRDY"]
pub struct HSIRDY_R(crate::FieldReader<bool, bool>);
impl HSIRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIDIVRDY` reader - HSIDIVRDY"]
pub struct HSIDIVRDY_R(crate::FieldReader<bool, bool>);
impl HSIDIVRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIDIVRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIDIVRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSIRDY` reader - CSIRDY"]
pub struct CSIRDY_R(crate::FieldReader<bool, bool>);
impl CSIRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSIRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSIRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSERDY` reader - HSERDY"]
pub struct HSERDY_R(crate::FieldReader<bool, bool>);
impl HSERDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSERDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSERDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUCKRDY` reader - MPUCKRDY"]
pub struct MPUCKRDY_R(crate::FieldReader<bool, bool>);
impl MPUCKRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUCKRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUCKRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXICKRDY` reader - AXICKRDY"]
pub struct AXICKRDY_R(crate::FieldReader<bool, bool>);
impl AXICKRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXICKRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXICKRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKREST` reader - CKREST"]
pub struct CKREST_R(crate::FieldReader<bool, bool>);
impl CKREST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKREST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKREST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSIDIVRDY"]
    #[inline(always)]
    pub fn hsidivrdy(&self) -> HSIDIVRDY_R {
        HSIDIVRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSIRDY"]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HSERDY"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MPUCKRDY"]
    #[inline(always)]
    pub fn mpuckrdy(&self) -> MPUCKRDY_R {
        MPUCKRDY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - AXICKRDY"]
    #[inline(always)]
    pub fn axickrdy(&self) -> AXICKRDY_R {
        AXICKRDY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CKREST"]
    #[inline(always)]
    pub fn ckrest(&self) -> CKREST_R {
        CKREST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
#[doc = "This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ocrdyr](index.html) module"]
pub struct RCC_OCRDYR_SPEC;
impl crate::RegisterSpec for RCC_OCRDYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ocrdyr::R](R) reader structure"]
impl crate::Readable for RCC_OCRDYR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCC_OCRDYR to value 0"]
impl crate::Resettable for RCC_OCRDYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
