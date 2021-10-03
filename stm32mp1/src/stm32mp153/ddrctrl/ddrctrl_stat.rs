#[doc = "Register `DDRCTRL_STAT` reader"]
pub struct R(crate::R<DDRCTRL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OPERATING_MODE` reader - OPERATING_MODE"]
pub struct OPERATING_MODE_R(crate::FieldReader<u8, u8>);
impl OPERATING_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPERATING_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPERATING_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELFREF_TYPE` reader - SELFREF_TYPE"]
pub struct SELFREF_TYPE_R(crate::FieldReader<u8, u8>);
impl SELFREF_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELFREF_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELFREF_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELFREF_CAM_NOT_EMPTY` reader - SELFREF_CAM_NOT_EMPTY"]
pub struct SELFREF_CAM_NOT_EMPTY_R(crate::FieldReader<bool, bool>);
impl SELFREF_CAM_NOT_EMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELFREF_CAM_NOT_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELFREF_CAM_NOT_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - OPERATING_MODE"]
    #[inline(always)]
    pub fn operating_mode(&self) -> OPERATING_MODE_R {
        OPERATING_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - SELFREF_TYPE"]
    #[inline(always)]
    pub fn selfref_type(&self) -> SELFREF_TYPE_R {
        SELFREF_TYPE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 12 - SELFREF_CAM_NOT_EMPTY"]
    #[inline(always)]
    pub fn selfref_cam_not_empty(&self) -> SELFREF_CAM_NOT_EMPTY_R {
        SELFREF_CAM_NOT_EMPTY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
#[doc = "DDRCTRL operating mode status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_stat](index.html) module"]
pub struct DDRCTRL_STAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_stat::R](R) reader structure"]
impl crate::Readable for DDRCTRL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_STAT to value 0"]
impl crate::Resettable for DDRCTRL_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
