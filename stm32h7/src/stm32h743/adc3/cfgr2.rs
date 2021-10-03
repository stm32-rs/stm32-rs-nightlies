#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC oversampler enable on scope ADC group regular\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVSE_A {
    #[doc = "0: Regular oversampling disabled"]
    DISABLED = 0,
    #[doc = "1: Regular oversampling enabled"]
    ENABLED = 1,
}
impl From<ROVSE_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVSE` reader - ADC oversampler enable on scope ADC group regular"]
pub struct ROVSE_R(crate::FieldReader<bool, ROVSE_A>);
impl ROVSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROVSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVSE_A {
        match self.bits {
            false => ROVSE_A::DISABLED,
            true => ROVSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ROVSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ROVSE_A::ENABLED
    }
}
impl core::ops::Deref for ROVSE_R {
    type Target = crate::FieldReader<bool, ROVSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROVSE` writer - ADC oversampler enable on scope ADC group regular"]
pub struct ROVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROVSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Regular oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROVSE_A::DISABLED)
    }
    #[doc = "Regular oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROVSE_A::ENABLED)
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
#[doc = "ADC oversampler enable on scope ADC group injected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JOVSE_A {
    #[doc = "0: Injected oversampling disabled"]
    DISABLED = 0,
    #[doc = "1: Injected oversampling enabled"]
    ENABLED = 1,
}
impl From<JOVSE_A> for bool {
    #[inline(always)]
    fn from(variant: JOVSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JOVSE` reader - ADC oversampler enable on scope ADC group injected"]
pub struct JOVSE_R(crate::FieldReader<bool, JOVSE_A>);
impl JOVSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JOVSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JOVSE_A {
        match self.bits {
            false => JOVSE_A::DISABLED,
            true => JOVSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == JOVSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == JOVSE_A::ENABLED
    }
}
impl core::ops::Deref for JOVSE_R {
    type Target = crate::FieldReader<bool, JOVSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JOVSE` writer - ADC oversampler enable on scope ADC group injected"]
pub struct JOVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> JOVSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JOVSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Injected oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JOVSE_A::DISABLED)
    }
    #[doc = "Injected oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JOVSE_A::ENABLED)
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
#[doc = "Field `OVSS` reader - ADC oversampling shift"]
pub struct OVSS_R(crate::FieldReader<u8, u8>);
impl OVSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OVSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSS` writer - ADC oversampling shift"]
pub struct OVSS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "ADC oversampling discontinuous mode (triggered mode) for ADC group regular\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TROVS_A {
    #[doc = "0: All oversampled conversions for a channel are run following a trigger"]
    AUTOMATIC = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a new trigger"]
    TRIGGERED = 1,
}
impl From<TROVS_A> for bool {
    #[inline(always)]
    fn from(variant: TROVS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TROVS` reader - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
pub struct TROVS_R(crate::FieldReader<bool, TROVS_A>);
impl TROVS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TROVS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TROVS_A {
        match self.bits {
            false => TROVS_A::AUTOMATIC,
            true => TROVS_A::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        **self == TROVS_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        **self == TROVS_A::TRIGGERED
    }
}
impl core::ops::Deref for TROVS_R {
    type Target = crate::FieldReader<bool, TROVS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TROVS` writer - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
pub struct TROVS_W<'a> {
    w: &'a mut W,
}
impl<'a> TROVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TROVS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All oversampled conversions for a channel are run following a trigger"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(TROVS_A::AUTOMATIC)
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn triggered(self) -> &'a mut W {
        self.variant(TROVS_A::TRIGGERED)
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
#[doc = "Regular Oversampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVSM_A {
    #[doc = "0: Oversampling is temporary stopped and continued after injection sequence"]
    CONTINUED = 0,
    #[doc = "1: Oversampling is aborted and resumed from start after injection sequence"]
    RESUMED = 1,
}
impl From<ROVSM_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVSM` reader - Regular Oversampling mode"]
pub struct ROVSM_R(crate::FieldReader<bool, ROVSM_A>);
impl ROVSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROVSM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVSM_A {
        match self.bits {
            false => ROVSM_A::CONTINUED,
            true => ROVSM_A::RESUMED,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUED`"]
    #[inline(always)]
    pub fn is_continued(&self) -> bool {
        **self == ROVSM_A::CONTINUED
    }
    #[doc = "Checks if the value of the field is `RESUMED`"]
    #[inline(always)]
    pub fn is_resumed(&self) -> bool {
        **self == ROVSM_A::RESUMED
    }
}
impl core::ops::Deref for ROVSM_R {
    type Target = crate::FieldReader<bool, ROVSM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROVSM` writer - Regular Oversampling mode"]
pub struct ROVSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROVSM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Oversampling is temporary stopped and continued after injection sequence"]
    #[inline(always)]
    pub fn continued(self) -> &'a mut W {
        self.variant(ROVSM_A::CONTINUED)
    }
    #[doc = "Oversampling is aborted and resumed from start after injection sequence"]
    #[inline(always)]
    pub fn resumed(self) -> &'a mut W {
        self.variant(ROVSM_A::RESUMED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Right-shift data after Offset 1 correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSHIFT1_A {
    #[doc = "0: Right-shifting disabled"]
    DISABLED = 0,
    #[doc = "1: Data is right-shifted 1-bit"]
    ENABLED = 1,
}
impl From<RSHIFT1_A> for bool {
    #[inline(always)]
    fn from(variant: RSHIFT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSHIFT1` reader - Right-shift data after Offset 1 correction"]
pub struct RSHIFT1_R(crate::FieldReader<bool, RSHIFT1_A>);
impl RSHIFT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSHIFT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSHIFT1_A {
        match self.bits {
            false => RSHIFT1_A::DISABLED,
            true => RSHIFT1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RSHIFT1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RSHIFT1_A::ENABLED
    }
}
impl core::ops::Deref for RSHIFT1_R {
    type Target = crate::FieldReader<bool, RSHIFT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSHIFT1` writer - Right-shift data after Offset 1 correction"]
pub struct RSHIFT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSHIFT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSHIFT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Right-shifting disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSHIFT1_A::DISABLED)
    }
    #[doc = "Data is right-shifted 1-bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSHIFT1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Right-shift data after Offset 2 correction"]
pub type RSHIFT2_A = RSHIFT1_A;
#[doc = "Field `RSHIFT2` reader - Right-shift data after Offset 2 correction"]
pub type RSHIFT2_R = RSHIFT1_R;
#[doc = "Field `RSHIFT2` writer - Right-shift data after Offset 2 correction"]
pub struct RSHIFT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSHIFT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSHIFT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Right-shifting disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSHIFT2_A::DISABLED)
    }
    #[doc = "Data is right-shifted 1-bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSHIFT2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Right-shift data after Offset 3 correction"]
pub type RSHIFT3_A = RSHIFT1_A;
#[doc = "Field `RSHIFT3` reader - Right-shift data after Offset 3 correction"]
pub type RSHIFT3_R = RSHIFT1_R;
#[doc = "Field `RSHIFT3` writer - Right-shift data after Offset 3 correction"]
pub struct RSHIFT3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSHIFT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSHIFT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Right-shifting disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSHIFT3_A::DISABLED)
    }
    #[doc = "Data is right-shifted 1-bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSHIFT3_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Right-shift data after Offset 4 correction"]
pub type RSHIFT4_A = RSHIFT1_A;
#[doc = "Field `RSHIFT4` reader - Right-shift data after Offset 4 correction"]
pub type RSHIFT4_R = RSHIFT1_R;
#[doc = "Field `RSHIFT4` writer - Right-shift data after Offset 4 correction"]
pub struct RSHIFT4_W<'a> {
    w: &'a mut W,
}
impl<'a> RSHIFT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSHIFT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Right-shifting disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSHIFT4_A::DISABLED)
    }
    #[doc = "Data is right-shifted 1-bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSHIFT4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `OSVR` reader - Oversampling ratio"]
pub struct OSVR_R(crate::FieldReader<u16, u16>);
impl OSVR_R {
    pub(crate) fn new(bits: u16) -> Self {
        OSVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSVR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSVR` writer - Oversampling ratio"]
pub struct OSVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `LSHIFT` reader - Left shift factor"]
pub struct LSHIFT_R(crate::FieldReader<u8, u8>);
impl LSHIFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSHIFT` writer - Left shift factor"]
pub struct LSHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC oversampler enable on scope ADC group regular"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC oversampler enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - ADC oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Right-shift data after Offset 1 correction"]
    #[inline(always)]
    pub fn rshift1(&self) -> RSHIFT1_R {
        RSHIFT1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Right-shift data after Offset 2 correction"]
    #[inline(always)]
    pub fn rshift2(&self) -> RSHIFT2_R {
        RSHIFT2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Right-shift data after Offset 3 correction"]
    #[inline(always)]
    pub fn rshift3(&self) -> RSHIFT3_R {
        RSHIFT3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Right-shift data after Offset 4 correction"]
    #[inline(always)]
    pub fn rshift4(&self) -> RSHIFT4_R {
        RSHIFT4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - Oversampling ratio"]
    #[inline(always)]
    pub fn osvr(&self) -> OSVR_R {
        OSVR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - Left shift factor"]
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC oversampler enable on scope ADC group regular"]
    #[inline(always)]
    pub fn rovse(&mut self) -> ROVSE_W {
        ROVSE_W { w: self }
    }
    #[doc = "Bit 1 - ADC oversampler enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jovse(&mut self) -> JOVSE_W {
        JOVSE_W { w: self }
    }
    #[doc = "Bits 5:8 - ADC oversampling shift"]
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W {
        OVSS_W { w: self }
    }
    #[doc = "Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
    #[inline(always)]
    pub fn trovs(&mut self) -> TROVS_W {
        TROVS_W { w: self }
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&mut self) -> ROVSM_W {
        ROVSM_W { w: self }
    }
    #[doc = "Bit 11 - Right-shift data after Offset 1 correction"]
    #[inline(always)]
    pub fn rshift1(&mut self) -> RSHIFT1_W {
        RSHIFT1_W { w: self }
    }
    #[doc = "Bit 12 - Right-shift data after Offset 2 correction"]
    #[inline(always)]
    pub fn rshift2(&mut self) -> RSHIFT2_W {
        RSHIFT2_W { w: self }
    }
    #[doc = "Bit 13 - Right-shift data after Offset 3 correction"]
    #[inline(always)]
    pub fn rshift3(&mut self) -> RSHIFT3_W {
        RSHIFT3_W { w: self }
    }
    #[doc = "Bit 14 - Right-shift data after Offset 4 correction"]
    #[inline(always)]
    pub fn rshift4(&mut self) -> RSHIFT4_W {
        RSHIFT4_W { w: self }
    }
    #[doc = "Bits 16:25 - Oversampling ratio"]
    #[inline(always)]
    pub fn osvr(&mut self) -> OSVR_W {
        OSVR_W { w: self }
    }
    #[doc = "Bits 28:31 - Left shift factor"]
    #[inline(always)]
    pub fn lshift(&mut self) -> LSHIFT_W {
        LSHIFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
