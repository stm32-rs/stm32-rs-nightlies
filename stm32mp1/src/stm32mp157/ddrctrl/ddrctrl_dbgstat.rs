#[doc = "Register `DDRCTRL_DBGSTAT` reader"]
pub struct R(crate::R<DDRCTRL_DBGSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DBGSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DBGSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DBGSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RANK0_REFRESH_BUSY` reader - RANK0_REFRESH_BUSY"]
pub struct RANK0_REFRESH_BUSY_R(crate::FieldReader<bool, bool>);
impl RANK0_REFRESH_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RANK0_REFRESH_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANK0_REFRESH_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZQ_CALIB_SHORT_BUSY` reader - ZQ_CALIB_SHORT_BUSY"]
pub struct ZQ_CALIB_SHORT_BUSY_R(crate::FieldReader<bool, bool>);
impl ZQ_CALIB_SHORT_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZQ_CALIB_SHORT_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZQ_CALIB_SHORT_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLUPD_BUSY` reader - CTRLUPD_BUSY"]
pub struct CTRLUPD_BUSY_R(crate::FieldReader<bool, bool>);
impl CTRLUPD_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLUPD_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLUPD_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RANK0_REFRESH_BUSY"]
    #[inline(always)]
    pub fn rank0_refresh_busy(&self) -> RANK0_REFRESH_BUSY_R {
        RANK0_REFRESH_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - ZQ_CALIB_SHORT_BUSY"]
    #[inline(always)]
    pub fn zq_calib_short_busy(&self) -> ZQ_CALIB_SHORT_BUSY_R {
        ZQ_CALIB_SHORT_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CTRLUPD_BUSY"]
    #[inline(always)]
    pub fn ctrlupd_busy(&self) -> CTRLUPD_BUSY_R {
        CTRLUPD_BUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "DDRCTRL status debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dbgstat](index.html) module"]
pub struct DDRCTRL_DBGSTAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_DBGSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dbgstat::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DBGSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_DBGSTAT to value 0"]
impl crate::Resettable for DDRCTRL_DBGSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
