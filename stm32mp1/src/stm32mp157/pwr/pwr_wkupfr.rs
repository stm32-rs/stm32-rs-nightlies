#[doc = "Register `PWR_WKUPFR` reader"]
pub struct R(crate::R<PWR_WKUPFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_WKUPFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_WKUPFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_WKUPFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WKUPF1` reader - WKUPF1"]
pub struct WKUPF1_R(crate::FieldReader<bool, bool>);
impl WKUPF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPF2` reader - WKUPF2"]
pub struct WKUPF2_R(crate::FieldReader<bool, bool>);
impl WKUPF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPF3` reader - WKUPF3"]
pub struct WKUPF3_R(crate::FieldReader<bool, bool>);
impl WKUPF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPF4` reader - WKUPF4"]
pub struct WKUPF4_R(crate::FieldReader<bool, bool>);
impl WKUPF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPF5` reader - WKUPF5"]
pub struct WKUPF5_R(crate::FieldReader<bool, bool>);
impl WKUPF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPF6` reader - WKUPF6"]
pub struct WKUPF6_R(crate::FieldReader<bool, bool>);
impl WKUPF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - WKUPF1"]
    #[inline(always)]
    pub fn wkupf1(&self) -> WKUPF1_R {
        WKUPF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WKUPF2"]
    #[inline(always)]
    pub fn wkupf2(&self) -> WKUPF2_R {
        WKUPF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WKUPF3"]
    #[inline(always)]
    pub fn wkupf3(&self) -> WKUPF3_R {
        WKUPF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WKUPF4"]
    #[inline(always)]
    pub fn wkupf4(&self) -> WKUPF4_R {
        WKUPF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WKUPF5"]
    #[inline(always)]
    pub fn wkupf5(&self) -> WKUPF5_R {
        WKUPF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WKUPF6"]
    #[inline(always)]
    pub fn wkupf6(&self) -> WKUPF6_R {
        WKUPF6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_wkupfr](index.html) module"]
pub struct PWR_WKUPFR_SPEC;
impl crate::RegisterSpec for PWR_WKUPFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_wkupfr::R](R) reader structure"]
impl crate::Readable for PWR_WKUPFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWR_WKUPFR to value 0"]
impl crate::Resettable for PWR_WKUPFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
