#[doc = "Register `RXF1S` reader"]
pub struct R(crate::R<RXF1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF1S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F1FL` reader - F1FL"]
pub struct F1FL_R(crate::FieldReader<u8, u8>);
impl F1FL_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1FL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1GI` reader - F1GI"]
pub struct F1GI_R(crate::FieldReader<u8, u8>);
impl F1GI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1GI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1GI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1PI` reader - F1PI"]
pub struct F1PI_R(crate::FieldReader<u8, u8>);
impl F1PI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1PI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1PI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1F` reader - F1F"]
pub struct F1F_R(crate::FieldReader<bool, bool>);
impl F1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        F1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1L` reader - RF1L"]
pub struct RF1L_R(crate::FieldReader<bool, bool>);
impl RF1L_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMS` reader - DMS"]
pub struct DMS_R(crate::FieldReader<u8, u8>);
impl DMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - F1FL"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - F1GI"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - F1PI"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - F1F"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RF1L"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - DMS"]
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
#[doc = "FDCAN Rx FIFO 1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1s](index.html) module"]
pub struct RXF1S_SPEC;
impl crate::RegisterSpec for RXF1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf1s::R](R) reader structure"]
impl crate::Readable for RXF1S_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXF1S to value 0"]
impl crate::Resettable for RXF1S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
