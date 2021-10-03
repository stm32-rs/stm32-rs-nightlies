#[doc = "Register `RTCCR` reader"]
pub struct R(crate::R<RTCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCR` writer"]
pub struct W(crate::W<RTCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCR_SPEC>;
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
impl From<crate::W<RTCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL` reader - Calibration value"]
pub struct CAL_R(crate::FieldReader<u8, u8>);
impl CAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL` writer - Calibration value"]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `CCO` reader - Calibration Clock Output"]
pub struct CCO_R(crate::FieldReader<bool, bool>);
impl CCO_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCO` writer - Calibration Clock Output"]
pub struct CCO_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Alarm or second output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASOE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit"]
    ENABLED = 1,
}
impl From<ASOE_A> for bool {
    #[inline(always)]
    fn from(variant: ASOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASOE` reader - Alarm or second output enable"]
pub struct ASOE_R(crate::FieldReader<bool, ASOE_A>);
impl ASOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASOE_A {
        match self.bits {
            false => ASOE_A::DISABLED,
            true => ASOE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ASOE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ASOE_A::ENABLED
    }
}
impl core::ops::Deref for ASOE_R {
    type Target = crate::FieldReader<bool, ASOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASOE` writer - Alarm or second output enable"]
pub struct ASOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ASOE_A::DISABLED)
    }
    #[doc = "Setting this bit outputs either the RTC Alarm pulse signal or the Second pulse signal on the TAMPER pin depending on the ASOS bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ASOE_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Alarm or second output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASOS_A {
    #[doc = "0: RTC Alarm pulse output selected"]
    ALARM = 0,
    #[doc = "1: RTC Second pulse output selected"]
    SECOND = 1,
}
impl From<ASOS_A> for bool {
    #[inline(always)]
    fn from(variant: ASOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASOS` reader - Alarm or second output selection"]
pub struct ASOS_R(crate::FieldReader<bool, ASOS_A>);
impl ASOS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASOS_A {
        match self.bits {
            false => ASOS_A::ALARM,
            true => ASOS_A::SECOND,
        }
    }
    #[doc = "Checks if the value of the field is `ALARM`"]
    #[inline(always)]
    pub fn is_alarm(&self) -> bool {
        **self == ASOS_A::ALARM
    }
    #[doc = "Checks if the value of the field is `SECOND`"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        **self == ASOS_A::SECOND
    }
}
impl core::ops::Deref for ASOS_R {
    type Target = crate::FieldReader<bool, ASOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASOS` writer - Alarm or second output selection"]
pub struct ASOS_W<'a> {
    w: &'a mut W,
}
impl<'a> ASOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASOS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC Alarm pulse output selected"]
    #[inline(always)]
    pub fn alarm(self) -> &'a mut W {
        self.variant(ASOS_A::ALARM)
    }
    #[doc = "RTC Second pulse output selected"]
    #[inline(always)]
    pub fn second(self) -> &'a mut W {
        self.variant(ASOS_A::SECOND)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn cco(&self) -> CCO_R {
        CCO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&self) -> ASOE_R {
        ASOE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&self) -> ASOS_R {
        ASOS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn cco(&mut self) -> CCO_W {
        CCO_W { w: self }
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&mut self) -> ASOE_W {
        ASOE_W { w: self }
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&mut self) -> ASOS_W {
        ASOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC clock calibration register (BKP_RTCCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccr](index.html) module"]
pub struct RTCCR_SPEC;
impl crate::RegisterSpec for RTCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccr::R](R) reader structure"]
impl crate::Readable for RTCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccr::W](W) writer structure"]
impl crate::Writable for RTCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCR to value 0"]
impl crate::Resettable for RTCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
