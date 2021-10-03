#[doc = "Register `CRYP_SR` reader"]
pub struct R(crate::R<CRYP_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IFEM` reader - IFEM"]
pub struct IFEM_R(crate::FieldReader<bool, bool>);
impl IFEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFNF` reader - IFNF"]
pub struct IFNF_R(crate::FieldReader<bool, bool>);
impl IFNF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFNE` reader - OFNE"]
pub struct OFNE_R(crate::FieldReader<bool, bool>);
impl OFNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFU` reader - OFFU"]
pub struct OFFU_R(crate::FieldReader<bool, bool>);
impl OFFU_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFFU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - BUSY"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - IFEM"]
    #[inline(always)]
    pub fn ifem(&self) -> IFEM_R {
        IFEM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IFNF"]
    #[inline(always)]
    pub fn ifnf(&self) -> IFNF_R {
        IFNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OFNE"]
    #[inline(always)]
    pub fn ofne(&self) -> OFNE_R {
        OFNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OFFU"]
    #[inline(always)]
    pub fn offu(&self) -> OFFU_R {
        OFFU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "CRYP status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_sr](index.html) module"]
pub struct CRYP_SR_SPEC;
impl crate::RegisterSpec for CRYP_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_sr::R](R) reader structure"]
impl crate::Readable for CRYP_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRYP_SR to value 0x03"]
impl crate::Resettable for CRYP_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
