#[doc = "Register `DTS_SR` reader"]
pub struct R(crate::R<DTS_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TS1_ITEF` reader - TS1_ITEF"]
pub struct TS1_ITEF_R(crate::FieldReader<bool, bool>);
impl TS1_ITEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_ITEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_ITEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_ITLF` reader - TS1_ITLF"]
pub struct TS1_ITLF_R(crate::FieldReader<bool, bool>);
impl TS1_ITLF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_ITLF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_ITLF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_ITHF` reader - TS1_ITHF"]
pub struct TS1_ITHF_R(crate::FieldReader<bool, bool>);
impl TS1_ITHF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_ITHF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_ITHF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_AITEF` reader - TS1_AITEF"]
pub struct TS1_AITEF_R(crate::FieldReader<bool, bool>);
impl TS1_AITEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_AITEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_AITEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_AITLF` reader - TS1_AITLF"]
pub struct TS1_AITLF_R(crate::FieldReader<bool, bool>);
impl TS1_AITLF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_AITLF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_AITLF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_AITHF` reader - TS1_AITHF"]
pub struct TS1_AITHF_R(crate::FieldReader<bool, bool>);
impl TS1_AITHF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_AITHF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_AITHF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_RDY` reader - TS1_RDY"]
pub struct TS1_RDY_R(crate::FieldReader<bool, bool>);
impl TS1_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TS1_ITEF"]
    #[inline(always)]
    pub fn ts1_itef(&self) -> TS1_ITEF_R {
        TS1_ITEF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TS1_ITLF"]
    #[inline(always)]
    pub fn ts1_itlf(&self) -> TS1_ITLF_R {
        TS1_ITLF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TS1_ITHF"]
    #[inline(always)]
    pub fn ts1_ithf(&self) -> TS1_ITHF_R {
        TS1_ITHF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TS1_AITEF"]
    #[inline(always)]
    pub fn ts1_aitef(&self) -> TS1_AITEF_R {
        TS1_AITEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TS1_AITLF"]
    #[inline(always)]
    pub fn ts1_aitlf(&self) -> TS1_AITLF_R {
        TS1_AITLF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TS1_AITHF"]
    #[inline(always)]
    pub fn ts1_aithf(&self) -> TS1_AITHF_R {
        TS1_AITHF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TS1_RDY"]
    #[inline(always)]
    pub fn ts1_rdy(&self) -> TS1_RDY_R {
        TS1_RDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Temperature sensor status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_sr](index.html) module"]
pub struct DTS_SR_SPEC;
impl crate::RegisterSpec for DTS_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_sr::R](R) reader structure"]
impl crate::Readable for DTS_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTS_SR to value 0"]
impl crate::Resettable for DTS_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
