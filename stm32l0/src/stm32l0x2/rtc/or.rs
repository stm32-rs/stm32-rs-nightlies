#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR` writer"]
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_OUT_RMP` reader - RTC_ALARM on PC13 output type"]
pub struct RTC_OUT_RMP_R(crate::FieldReader<bool, bool>);
impl RTC_OUT_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_OUT_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_OUT_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_OUT_RMP` writer - RTC_ALARM on PC13 output type"]
pub struct RTC_OUT_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_OUT_RMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RTC_ALARM_TYPE` reader - RTC_ALARM on PC13 output type"]
pub struct RTC_ALARM_TYPE_R(crate::FieldReader<bool, bool>);
impl RTC_ALARM_TYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ALARM_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ALARM_TYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ALARM_TYPE` writer - RTC_ALARM on PC13 output type"]
pub struct RTC_ALARM_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALARM_TYPE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - RTC_ALARM on PC13 output type"]
    #[inline(always)]
    pub fn rtc_out_rmp(&self) -> RTC_OUT_RMP_R {
        RTC_OUT_RMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RTC_ALARM on PC13 output type"]
    #[inline(always)]
    pub fn rtc_alarm_type(&self) -> RTC_ALARM_TYPE_R {
        RTC_ALARM_TYPE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RTC_ALARM on PC13 output type"]
    #[inline(always)]
    pub fn rtc_out_rmp(&mut self) -> RTC_OUT_RMP_W {
        RTC_OUT_RMP_W { w: self }
    }
    #[doc = "Bit 0 - RTC_ALARM on PC13 output type"]
    #[inline(always)]
    pub fn rtc_alarm_type(&mut self) -> RTC_ALARM_TYPE_W {
        RTC_ALARM_TYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or::W](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
