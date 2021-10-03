#[doc = "Register `SMISR` reader"]
pub struct R(crate::R<SMISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAMP1MF` reader - TAMP1MF:"]
pub struct TAMP1MF_R(crate::FieldReader<bool, bool>);
impl TAMP1MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2MF` reader - TAMP2MF"]
pub struct TAMP2MF_R(crate::FieldReader<bool, bool>);
impl TAMP2MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP3MF` reader - TAMP3MF"]
pub struct TAMP3MF_R(crate::FieldReader<bool, bool>);
impl TAMP3MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP3MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP3MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP4MF` reader - TAMP4MF"]
pub struct TAMP4MF_R(crate::FieldReader<bool, bool>);
impl TAMP4MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP4MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP4MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP5MF` reader - TAMP5MF"]
pub struct TAMP5MF_R(crate::FieldReader<bool, bool>);
impl TAMP5MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP5MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP5MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP6MF` reader - TAMP6MF"]
pub struct TAMP6MF_R(crate::FieldReader<bool, bool>);
impl TAMP6MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP6MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP6MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP7MF` reader - TAMP7MF:"]
pub struct TAMP7MF_R(crate::FieldReader<bool, bool>);
impl TAMP7MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP7MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP7MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP8MF` reader - TAMP8MF"]
pub struct TAMP8MF_R(crate::FieldReader<bool, bool>);
impl TAMP8MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP8MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP8MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP1MF` reader - ITAMP1MF"]
pub struct ITAMP1MF_R(crate::FieldReader<bool, bool>);
impl ITAMP1MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP1MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP1MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP2MF` reader - ITAMP2MF"]
pub struct ITAMP2MF_R(crate::FieldReader<bool, bool>);
impl ITAMP2MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP2MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP2MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP3MF` reader - ITAMP3MF"]
pub struct ITAMP3MF_R(crate::FieldReader<bool, bool>);
impl ITAMP3MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP3MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP5MF` reader - ITAMP5MF"]
pub struct ITAMP5MF_R(crate::FieldReader<bool, bool>);
impl ITAMP5MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP5MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP5MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP8MF` reader - ITAMP8MF"]
pub struct ITAMP8MF_R(crate::FieldReader<bool, bool>);
impl ITAMP8MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP8MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP8MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TAMP1MF:"]
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2MF"]
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TAMP3MF"]
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TAMP4MF"]
    #[inline(always)]
    pub fn tamp4mf(&self) -> TAMP4MF_R {
        TAMP4MF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TAMP5MF"]
    #[inline(always)]
    pub fn tamp5mf(&self) -> TAMP5MF_R {
        TAMP5MF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TAMP6MF"]
    #[inline(always)]
    pub fn tamp6mf(&self) -> TAMP6MF_R {
        TAMP6MF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TAMP7MF:"]
    #[inline(always)]
    pub fn tamp7mf(&self) -> TAMP7MF_R {
        TAMP7MF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TAMP8MF"]
    #[inline(always)]
    pub fn tamp8mf(&self) -> TAMP8MF_R {
        TAMP8MF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ITAMP1MF"]
    #[inline(always)]
    pub fn itamp1mf(&self) -> ITAMP1MF_R {
        ITAMP1MF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ITAMP2MF"]
    #[inline(always)]
    pub fn itamp2mf(&self) -> ITAMP2MF_R {
        ITAMP2MF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ITAMP3MF"]
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ITAMP5MF"]
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ITAMP8MF"]
    #[inline(always)]
    pub fn itamp8mf(&self) -> ITAMP8MF_R {
        ITAMP8MF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "TAMP secure masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smisr](index.html) module"]
pub struct SMISR_SPEC;
impl crate::RegisterSpec for SMISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smisr::R](R) reader structure"]
impl crate::Readable for SMISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMISR to value 0"]
impl crate::Resettable for SMISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
