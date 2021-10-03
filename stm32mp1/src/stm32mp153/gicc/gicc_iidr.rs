#[doc = "Register `GICC_IIDR` reader"]
pub struct R(crate::R<GICC_IIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_IIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_IIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_IIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IMPLEMENTER` reader - IMPLEMENTER"]
pub struct IMPLEMENTER_R(crate::FieldReader<u16, u16>);
impl IMPLEMENTER_R {
    pub(crate) fn new(bits: u16) -> Self {
        IMPLEMENTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMPLEMENTER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVISION` reader - REVISION"]
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARCH` reader - ARCH"]
pub struct ARCH_R(crate::FieldReader<u8, u8>);
impl ARCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        ARCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRODUCTID` reader - PRODUCTID"]
pub struct PRODUCTID_R(crate::FieldReader<u16, u16>);
impl PRODUCTID_R {
    pub(crate) fn new(bits: u16) -> Self {
        PRODUCTID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRODUCTID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - IMPLEMENTER"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - REVISION"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ARCH"]
    #[inline(always)]
    pub fn arch(&self) -> ARCH_R {
        ARCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - PRODUCTID"]
    #[inline(always)]
    pub fn productid(&self) -> PRODUCTID_R {
        PRODUCTID_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "GICC interface identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_iidr](index.html) module"]
pub struct GICC_IIDR_SPEC;
impl crate::RegisterSpec for GICC_IIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_iidr::R](R) reader structure"]
impl crate::Readable for GICC_IIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICC_IIDR to value 0x0102_143b"]
impl crate::Resettable for GICC_IIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0102_143b
    }
}
