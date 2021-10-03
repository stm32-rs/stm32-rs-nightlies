#[doc = "Register `BCDR` reader"]
pub struct R(crate::R<BCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCDR` writer"]
pub struct W(crate::W<BCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCDR_SPEC>;
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
impl From<crate::W<BCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Battery charging detector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCDEN_A {
    #[doc = "0: disable the BCD support"]
    DISABLED = 0,
    #[doc = "1: enable the BCD support within the USB device"]
    ENABLED = 1,
}
impl From<BCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: BCDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCDEN` reader - Battery charging detector"]
pub struct BCDEN_R(crate::FieldReader<bool, BCDEN_A>);
impl BCDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCDEN_A {
        match self.bits {
            false => BCDEN_A::DISABLED,
            true => BCDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BCDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BCDEN_A::ENABLED
    }
}
impl core::ops::Deref for BCDEN_R {
    type Target = crate::FieldReader<bool, BCDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCDEN` writer - Battery charging detector"]
pub struct BCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable the BCD support"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BCDEN_A::DISABLED)
    }
    #[doc = "enable the BCD support within the USB device"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BCDEN_A::ENABLED)
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
#[doc = "Data contact detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDEN_A {
    #[doc = "0: Data contact detection (DCD) mode disabled"]
    DISABLED = 0,
    #[doc = "1: Data contact detection (DCD) mode enabled"]
    ENABLED = 1,
}
impl From<DCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDEN` reader - Data contact detection"]
pub struct DCDEN_R(crate::FieldReader<bool, DCDEN_A>);
impl DCDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDEN_A {
        match self.bits {
            false => DCDEN_A::DISABLED,
            true => DCDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DCDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DCDEN_A::ENABLED
    }
}
impl core::ops::Deref for DCDEN_R {
    type Target = crate::FieldReader<bool, DCDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDEN` writer - Data contact detection"]
pub struct DCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data contact detection (DCD) mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCDEN_A::DISABLED)
    }
    #[doc = "Data contact detection (DCD) mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCDEN_A::ENABLED)
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
#[doc = "Primary detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDEN_A {
    #[doc = "0: Primary detection (PD) mode disabled"]
    DISABLED = 0,
    #[doc = "1: Primary detection (PD) mode enabled"]
    ENABLED = 1,
}
impl From<PDEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDEN` reader - Primary detection"]
pub struct PDEN_R(crate::FieldReader<bool, PDEN_A>);
impl PDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_A {
        match self.bits {
            false => PDEN_A::DISABLED,
            true => PDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PDEN_A::ENABLED
    }
}
impl core::ops::Deref for PDEN_R {
    type Target = crate::FieldReader<bool, PDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEN` writer - Primary detection"]
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Primary detection (PD) mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PDEN_A::DISABLED)
    }
    #[doc = "Primary detection (PD) mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PDEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Secondary detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDEN_A {
    #[doc = "0: Secondary detection (SD) mode disabled"]
    DISABLED = 0,
    #[doc = "1: Secondary detection (SD) mode enabled"]
    ENABLED = 1,
}
impl From<SDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDEN` reader - Secondary detection"]
pub struct SDEN_R(crate::FieldReader<bool, SDEN_A>);
impl SDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDEN_A {
        match self.bits {
            false => SDEN_A::DISABLED,
            true => SDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SDEN_A::ENABLED
    }
}
impl core::ops::Deref for SDEN_R {
    type Target = crate::FieldReader<bool, SDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDEN` writer - Secondary detection"]
pub struct SDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Secondary detection (SD) mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDEN_A::DISABLED)
    }
    #[doc = "Secondary detection (SD) mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Data contact detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDET_A {
    #[doc = "0: data lines contact not detected"]
    NOTDETECTED = 0,
    #[doc = "1: data lines contact detected"]
    DETECTED = 1,
}
impl From<DCDET_A> for bool {
    #[inline(always)]
    fn from(variant: DCDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDET` reader - Data contact detection"]
pub struct DCDET_R(crate::FieldReader<bool, DCDET_A>);
impl DCDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDET_A {
        match self.bits {
            false => DCDET_A::NOTDETECTED,
            true => DCDET_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        **self == DCDET_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        **self == DCDET_A::DETECTED
    }
}
impl core::ops::Deref for DCDET_R {
    type Target = crate::FieldReader<bool, DCDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Primary detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDET_A {
    #[doc = "0: no BCD support detected"]
    NOBCD = 0,
    #[doc = "1: BCD support detected"]
    BCD = 1,
}
impl From<PDET_A> for bool {
    #[inline(always)]
    fn from(variant: PDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDET` reader - Primary detection"]
pub struct PDET_R(crate::FieldReader<bool, PDET_A>);
impl PDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDET_A {
        match self.bits {
            false => PDET_A::NOBCD,
            true => PDET_A::BCD,
        }
    }
    #[doc = "Checks if the value of the field is `NOBCD`"]
    #[inline(always)]
    pub fn is_no_bcd(&self) -> bool {
        **self == PDET_A::NOBCD
    }
    #[doc = "Checks if the value of the field is `BCD`"]
    #[inline(always)]
    pub fn is_bcd(&self) -> bool {
        **self == PDET_A::BCD
    }
}
impl core::ops::Deref for PDET_R {
    type Target = crate::FieldReader<bool, PDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Secondary detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDET_A {
    #[doc = "0: CDP detected"]
    CDP = 0,
    #[doc = "1: DCP detected"]
    DCP = 1,
}
impl From<SDET_A> for bool {
    #[inline(always)]
    fn from(variant: SDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDET` reader - Secondary detection"]
pub struct SDET_R(crate::FieldReader<bool, SDET_A>);
impl SDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDET_A {
        match self.bits {
            false => SDET_A::CDP,
            true => SDET_A::DCP,
        }
    }
    #[doc = "Checks if the value of the field is `CDP`"]
    #[inline(always)]
    pub fn is_cdp(&self) -> bool {
        **self == SDET_A::CDP
    }
    #[doc = "Checks if the value of the field is `DCP`"]
    #[inline(always)]
    pub fn is_dcp(&self) -> bool {
        **self == SDET_A::DCP
    }
}
impl core::ops::Deref for SDET_R {
    type Target = crate::FieldReader<bool, SDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DM pull-up detection status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS2DET_A {
    #[doc = "0: Normal port detected"]
    NORMAL = 0,
    #[doc = "1: PS2 port or proprietary charger detected"]
    PS2 = 1,
}
impl From<PS2DET_A> for bool {
    #[inline(always)]
    fn from(variant: PS2DET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS2DET` reader - DM pull-up detection status"]
pub struct PS2DET_R(crate::FieldReader<bool, PS2DET_A>);
impl PS2DET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS2DET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS2DET_A {
        match self.bits {
            false => PS2DET_A::NORMAL,
            true => PS2DET_A::PS2,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == PS2DET_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `PS2`"]
    #[inline(always)]
    pub fn is_ps2(&self) -> bool {
        **self == PS2DET_A::PS2
    }
}
impl core::ops::Deref for PS2DET_R {
    type Target = crate::FieldReader<bool, PS2DET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DP pull-up control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPU_A {
    #[doc = "0: signalize disconnect to the host when needed by the user software"]
    DISABLED = 0,
    #[doc = "1: enable the embedded pull-up on the DP line"]
    ENABLED = 1,
}
impl From<DPPU_A> for bool {
    #[inline(always)]
    fn from(variant: DPPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPPU` reader - DP pull-up control"]
pub struct DPPU_R(crate::FieldReader<bool, DPPU_A>);
impl DPPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPPU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPPU_A {
        match self.bits {
            false => DPPU_A::DISABLED,
            true => DPPU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DPPU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DPPU_A::ENABLED
    }
}
impl core::ops::Deref for DPPU_R {
    type Target = crate::FieldReader<bool, DPPU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPPU` writer - DP pull-up control"]
pub struct DPPU_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPPU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "signalize disconnect to the host when needed by the user software"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DPPU_A::DISABLED)
    }
    #[doc = "enable the embedded pull-up on the DP line"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DPPU_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Battery charging detector"]
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data contact detection"]
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Primary detection"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Secondary detection"]
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data contact detection"]
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Primary detection"]
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Secondary detection"]
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DM pull-up detection status"]
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DP pull-up control"]
    #[inline(always)]
    pub fn dppu(&self) -> DPPU_R {
        DPPU_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Battery charging detector"]
    #[inline(always)]
    pub fn bcden(&mut self) -> BCDEN_W {
        BCDEN_W { w: self }
    }
    #[doc = "Bit 1 - Data contact detection"]
    #[inline(always)]
    pub fn dcden(&mut self) -> DCDEN_W {
        DCDEN_W { w: self }
    }
    #[doc = "Bit 2 - Primary detection"]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
    #[doc = "Bit 3 - Secondary detection"]
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W {
        SDEN_W { w: self }
    }
    #[doc = "Bit 15 - DP pull-up control"]
    #[inline(always)]
    pub fn dppu(&mut self) -> DPPU_W {
        DPPU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Battery charging detector\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcdr](index.html) module"]
pub struct BCDR_SPEC;
impl crate::RegisterSpec for BCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcdr::R](R) reader structure"]
impl crate::Readable for BCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcdr::W](W) writer structure"]
impl crate::Writable for BCDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCDR to value 0"]
impl crate::Resettable for BCDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
