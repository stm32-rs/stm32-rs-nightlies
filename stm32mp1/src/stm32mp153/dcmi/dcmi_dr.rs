#[doc = "Register `DCMI_DR` reader"]
pub struct R(crate::R<DCMI_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Byte0` reader - Byte0"]
pub struct BYTE0_R(crate::FieldReader<u8, u8>);
impl BYTE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        BYTE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Byte1` reader - Byte1"]
pub struct BYTE1_R(crate::FieldReader<u8, u8>);
impl BYTE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        BYTE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Byte2` reader - Byte2"]
pub struct BYTE2_R(crate::FieldReader<u8, u8>);
impl BYTE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        BYTE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Byte3` reader - Byte3"]
pub struct BYTE3_R(crate::FieldReader<u8, u8>);
impl BYTE3_R {
    pub(crate) fn new(bits: u8) -> Self {
        BYTE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Byte0"]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Byte1"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Byte2"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Byte3"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DCMI data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_dr](index.html) module"]
pub struct DCMI_DR_SPEC;
impl crate::RegisterSpec for DCMI_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_dr::R](R) reader structure"]
impl crate::Readable for DCMI_DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCMI_DR to value 0"]
impl crate::Resettable for DCMI_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
