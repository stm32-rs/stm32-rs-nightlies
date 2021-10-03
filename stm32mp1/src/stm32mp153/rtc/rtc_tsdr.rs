#[doc = "Register `RTC_TSDR` reader"]
pub struct R(crate::R<RTC_TSDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TSDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TSDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TSDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DU` reader - DU"]
pub struct DU_R(crate::FieldReader<u8, u8>);
impl DU_R {
    pub(crate) fn new(bits: u8) -> Self {
        DU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT` reader - DT"]
pub struct DT_R(crate::FieldReader<u8, u8>);
impl DT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MU` reader - MU"]
pub struct MU_R(crate::FieldReader<u8, u8>);
impl MU_R {
    pub(crate) fn new(bits: u8) -> Self {
        MU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MT` reader - MT"]
pub struct MT_R(crate::FieldReader<bool, bool>);
impl MT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDU` reader - WDU"]
pub struct WDU_R(crate::FieldReader<u8, u8>);
impl WDU_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - DU"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - DT"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - MU"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MT"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - WDU"]
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tsdr](index.html) module"]
pub struct RTC_TSDR_SPEC;
impl crate::RegisterSpec for RTC_TSDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_tsdr::R](R) reader structure"]
impl crate::Readable for RTC_TSDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_TSDR to value 0"]
impl crate::Resettable for RTC_TSDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
