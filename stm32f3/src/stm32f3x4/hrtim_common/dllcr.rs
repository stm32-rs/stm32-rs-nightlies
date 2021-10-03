#[doc = "Register `DLLCR` reader"]
pub struct R(crate::R<DLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLLCR` writer"]
pub struct W(crate::W<DLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLLCR_SPEC>;
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
impl From<crate::W<DLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DLL Calibration rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CALRTE_A {
    #[doc = "0: 1048576*t_HRTIM (7.3ms)"]
    MILLIS7_3 = 0,
    #[doc = "1: 131072*t_HRTIM (910µs)"]
    MICROS910 = 1,
    #[doc = "2: 16384*t_HRTIM (114µs)"]
    MICROS114 = 2,
    #[doc = "3: 2048*t_HRTIM (14µs)"]
    MICROS14 = 3,
}
impl From<CALRTE_A> for u8 {
    #[inline(always)]
    fn from(variant: CALRTE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CALRTE` reader - DLL Calibration rate"]
pub struct CALRTE_R(crate::FieldReader<u8, CALRTE_A>);
impl CALRTE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALRTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALRTE_A {
        match self.bits {
            0 => CALRTE_A::MILLIS7_3,
            1 => CALRTE_A::MICROS910,
            2 => CALRTE_A::MICROS114,
            3 => CALRTE_A::MICROS14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MILLIS7_3`"]
    #[inline(always)]
    pub fn is_millis7_3(&self) -> bool {
        **self == CALRTE_A::MILLIS7_3
    }
    #[doc = "Checks if the value of the field is `MICROS910`"]
    #[inline(always)]
    pub fn is_micros910(&self) -> bool {
        **self == CALRTE_A::MICROS910
    }
    #[doc = "Checks if the value of the field is `MICROS114`"]
    #[inline(always)]
    pub fn is_micros114(&self) -> bool {
        **self == CALRTE_A::MICROS114
    }
    #[doc = "Checks if the value of the field is `MICROS14`"]
    #[inline(always)]
    pub fn is_micros14(&self) -> bool {
        **self == CALRTE_A::MICROS14
    }
}
impl core::ops::Deref for CALRTE_R {
    type Target = crate::FieldReader<u8, CALRTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALRTE` writer - DLL Calibration rate"]
pub struct CALRTE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALRTE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1048576*t_HRTIM (7.3ms)"]
    #[inline(always)]
    pub fn millis7_3(self) -> &'a mut W {
        self.variant(CALRTE_A::MILLIS7_3)
    }
    #[doc = "131072*t_HRTIM (910µs)"]
    #[inline(always)]
    pub fn micros910(self) -> &'a mut W {
        self.variant(CALRTE_A::MICROS910)
    }
    #[doc = "16384*t_HRTIM (114µs)"]
    #[inline(always)]
    pub fn micros114(self) -> &'a mut W {
        self.variant(CALRTE_A::MICROS114)
    }
    #[doc = "2048*t_HRTIM (14µs)"]
    #[inline(always)]
    pub fn micros14(self) -> &'a mut W {
        self.variant(CALRTE_A::MICROS14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "DLL Calibration Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALEN_A {
    #[doc = "0: Periodic calibration disabled"]
    DISABLED = 0,
    #[doc = "1: Calibration is performed periodically, as per CALRTE setting"]
    ENABLED = 1,
}
impl From<CALEN_A> for bool {
    #[inline(always)]
    fn from(variant: CALEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALEN` reader - DLL Calibration Enable"]
pub struct CALEN_R(crate::FieldReader<bool, CALEN_A>);
impl CALEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALEN_A {
        match self.bits {
            false => CALEN_A::DISABLED,
            true => CALEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CALEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CALEN_A::ENABLED
    }
}
impl core::ops::Deref for CALEN_R {
    type Target = crate::FieldReader<bool, CALEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALEN` writer - DLL Calibration Enable"]
pub struct CALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CALEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Periodic calibration disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CALEN_A::DISABLED)
    }
    #[doc = "Calibration is performed periodically, as per CALRTE setting"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CALEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "DLL Calibration Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_A {
    #[doc = "1: Calibration start"]
    START = 1,
}
impl From<CAL_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL` reader - DLL Calibration Start"]
pub struct CAL_R(crate::FieldReader<bool, CAL_A>);
impl CAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAL_A> {
        match self.bits {
            true => Some(CAL_A::START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == CAL_A::START
    }
}
impl core::ops::Deref for CAL_R {
    type Target = crate::FieldReader<bool, CAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL` writer - DLL Calibration Start"]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calibration start"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CAL_A::START)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    pub fn calrte(&self) -> CALRTE_R {
        CALRTE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    pub fn calrte(&mut self) -> CALRTE_W {
        CALRTE_W { w: self }
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    pub fn calen(&mut self) -> CALEN_W {
        CALEN_W { w: self }
    }
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dllcr](index.html) module"]
pub struct DLLCR_SPEC;
impl crate::RegisterSpec for DLLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dllcr::R](R) reader structure"]
impl crate::Readable for DLLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dllcr::W](W) writer structure"]
impl crate::Writable for DLLCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLLCR to value 0"]
impl crate::Resettable for DLLCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
