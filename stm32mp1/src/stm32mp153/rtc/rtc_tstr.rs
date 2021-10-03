#[doc = "Register `RTC_TSTR` reader"]
pub struct R(crate::R<RTC_TSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SU` reader - SU"]
pub struct SU_R(crate::FieldReader<u8, u8>);
impl SU_R {
    pub(crate) fn new(bits: u8) -> Self {
        SU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST` reader - ST"]
pub struct ST_R(crate::FieldReader<u8, u8>);
impl ST_R {
    pub(crate) fn new(bits: u8) -> Self {
        ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MNU` reader - MNU"]
pub struct MNU_R(crate::FieldReader<u8, u8>);
impl MNU_R {
    pub(crate) fn new(bits: u8) -> Self {
        MNU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MNU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MNT` reader - MNT"]
pub struct MNT_R(crate::FieldReader<u8, u8>);
impl MNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HU` reader - HU"]
pub struct HU_R(crate::FieldReader<u8, u8>);
impl HU_R {
    pub(crate) fn new(bits: u8) -> Self {
        HU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HT` reader - HT"]
pub struct HT_R(crate::FieldReader<u8, u8>);
impl HT_R {
    pub(crate) fn new(bits: u8) -> Self {
        HT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM` reader - PM"]
pub struct PM_R(crate::FieldReader<bool, bool>);
impl PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - SU"]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - MNU"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - MNT"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - HU"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - HT"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tstr](index.html) module"]
pub struct RTC_TSTR_SPEC;
impl crate::RegisterSpec for RTC_TSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_tstr::R](R) reader structure"]
impl crate::Readable for RTC_TSTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_TSTR to value 0"]
impl crate::Resettable for RTC_TSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
