#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAMP1F` reader - TAMP1F"]
pub struct TAMP1F_R(crate::FieldReader<bool, bool>);
impl TAMP1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2F` reader - TAMP2F"]
pub struct TAMP2F_R(crate::FieldReader<bool, bool>);
impl TAMP2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP1F` reader - ITAMP1F"]
pub struct ITAMP1F_R(crate::FieldReader<bool, bool>);
impl ITAMP1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP3F` reader - ITAMP3F"]
pub struct ITAMP3F_R(crate::FieldReader<bool, bool>);
impl ITAMP3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP4F` reader - ITAMP4F"]
pub struct ITAMP4F_R(crate::FieldReader<bool, bool>);
impl ITAMP4F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP4F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP4F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP5F` reader - ITAMP5F"]
pub struct ITAMP5F_R(crate::FieldReader<bool, bool>);
impl ITAMP5F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP5F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP5F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP6F` reader - ITAMP6F"]
pub struct ITAMP6F_R(crate::FieldReader<bool, bool>);
impl ITAMP6F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP6F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP6F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP7F` reader - ITAMP7F"]
pub struct ITAMP7F_R(crate::FieldReader<bool, bool>);
impl ITAMP7F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP7F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP7F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TAMP1F"]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2F"]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ITAMP1F"]
    #[inline(always)]
    pub fn itamp1f(&self) -> ITAMP1F_R {
        ITAMP1F_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ITAMP3F"]
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ITAMP4F"]
    #[inline(always)]
    pub fn itamp4f(&self) -> ITAMP4F_R {
        ITAMP4F_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ITAMP5F"]
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ITAMP6F"]
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ITAMP7F"]
    #[inline(always)]
    pub fn itamp7f(&self) -> ITAMP7F_R {
        ITAMP7F_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
#[doc = "TAMP status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
