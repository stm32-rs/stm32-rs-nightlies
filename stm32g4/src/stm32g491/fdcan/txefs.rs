#[doc = "Register `TXEFS` reader"]
pub struct R(crate::R<TXEFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EFFL` reader - EFFL"]
pub struct EFFL_R(crate::FieldReader<u8, u8>);
impl EFFL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFGI` reader - EFGI"]
pub struct EFGI_R(crate::FieldReader<u8, u8>);
impl EFGI_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFGI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFGI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFPI` reader - EFPI"]
pub struct EFPI_R(crate::FieldReader<u8, u8>);
impl EFPI_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFPI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFF` reader - EFF"]
pub struct EFF_R(crate::FieldReader<bool, bool>);
impl EFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFL` reader - TEFL"]
pub struct TEFL_R(crate::FieldReader<bool, bool>);
impl TEFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - EFFL"]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - EFGI"]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - EFPI"]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 24 - EFF"]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TEFL"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
#[doc = "FDCAN Tx Event FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefs](index.html) module"]
pub struct TXEFS_SPEC;
impl crate::RegisterSpec for TXEFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txefs::R](R) reader structure"]
impl crate::Readable for TXEFS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXEFS to value 0"]
impl crate::Resettable for TXEFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
