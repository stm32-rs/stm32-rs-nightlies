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
#[doc = "OVSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVSE_A {
    #[doc = "0: Oversampler disabled"]
    DISABLED = 0,
    #[doc = "1: Oversampler enabled"]
    ENABLED = 1,
}
impl From<OVSE_A> for bool {
    #[inline(always)]
    fn from(variant: OVSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVSE` reader - OVSE"]
pub struct OVSE_R(crate::FieldReader<bool, OVSE_A>);
impl OVSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSE_A {
        match self.bits {
            false => OVSE_A::DISABLED,
            true => OVSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OVSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OVSE_A::ENABLED
    }
}
impl core::ops::Deref for OVSE_R {
    type Target = crate::FieldReader<bool, OVSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSE` writer - OVSE"]
pub struct OVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Oversampler disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVSE_A::DISABLED)
    }
    #[doc = "Oversampler enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVSE_A::ENABLED)
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
#[doc = "TOVS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOVS_A {
    #[doc = "0: All oversampled conversions for a channel are done consecutively after a trigger"]
    TRIGGERALL = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a trigger"]
    TRIGGEREACH = 1,
}
impl From<TOVS_A> for bool {
    #[inline(always)]
    fn from(variant: TOVS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOVS` reader - TOVS"]
pub struct TOVS_R(crate::FieldReader<bool, TOVS_A>);
impl TOVS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOVS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOVS_A {
        match self.bits {
            false => TOVS_A::TRIGGERALL,
            true => TOVS_A::TRIGGEREACH,
        }
    }
    #[doc = "Checks if the value of the field is `TRIGGERALL`"]
    #[inline(always)]
    pub fn is_trigger_all(&self) -> bool {
        **self == TOVS_A::TRIGGERALL
    }
    #[doc = "Checks if the value of the field is `TRIGGEREACH`"]
    #[inline(always)]
    pub fn is_trigger_each(&self) -> bool {
        **self == TOVS_A::TRIGGEREACH
    }
}
impl core::ops::Deref for TOVS_R {
    type Target = crate::FieldReader<bool, TOVS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOVS` writer - TOVS"]
pub struct TOVS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOVS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All oversampled conversions for a channel are done consecutively after a trigger"]
    #[inline(always)]
    pub fn trigger_all(self) -> &'a mut W {
        self.variant(TOVS_A::TRIGGERALL)
    }
    #[doc = "Each oversampled conversion for a channel needs a trigger"]
    #[inline(always)]
    pub fn trigger_each(self) -> &'a mut W {
        self.variant(TOVS_A::TRIGGEREACH)
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
#[doc = "LFTRIG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFTRIG_A {
    #[doc = "0: Low Frequency Trigger Mode disabled"]
    DISABLED = 0,
    #[doc = "1: Low Frequency Trigger Mode enabled"]
    ENABLED = 1,
}
impl From<LFTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: LFTRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFTRIG` reader - LFTRIG"]
pub struct LFTRIG_R(crate::FieldReader<bool, LFTRIG_A>);
impl LFTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFTRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFTRIG_A {
        match self.bits {
            false => LFTRIG_A::DISABLED,
            true => LFTRIG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LFTRIG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LFTRIG_A::ENABLED
    }
}
impl core::ops::Deref for LFTRIG_R {
    type Target = crate::FieldReader<bool, LFTRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFTRIG` writer - LFTRIG"]
pub struct LFTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> LFTRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFTRIG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low Frequency Trigger Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFTRIG_A::DISABLED)
    }
    #[doc = "Low Frequency Trigger Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LFTRIG_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "CKMODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKMODE_A {
    #[doc = "0: ADCCLK (Asynchronous clock mode)"]
    ADCLK = 0,
    #[doc = "1: PCLK/2 (Synchronous clock mode)"]
    PCLK_DIV2 = 1,
    #[doc = "2: PCLK/4 (Synchronous clock mode)"]
    PCLK_DIV4 = 2,
    #[doc = "3: PCLK (Synchronous clock mode)"]
    PCLK = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKMODE` reader - CKMODE"]
pub struct CKMODE_R(crate::FieldReader<u8, CKMODE_A>);
impl CKMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::ADCLK,
            1 => CKMODE_A::PCLK_DIV2,
            2 => CKMODE_A::PCLK_DIV4,
            3 => CKMODE_A::PCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCLK`"]
    #[inline(always)]
    pub fn is_adclk(&self) -> bool {
        **self == CKMODE_A::ADCLK
    }
    #[doc = "Checks if the value of the field is `PCLK_DIV2`"]
    #[inline(always)]
    pub fn is_pclk_div2(&self) -> bool {
        **self == CKMODE_A::PCLK_DIV2
    }
    #[doc = "Checks if the value of the field is `PCLK_DIV4`"]
    #[inline(always)]
    pub fn is_pclk_div4(&self) -> bool {
        **self == CKMODE_A::PCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == CKMODE_A::PCLK
    }
}
impl core::ops::Deref for CKMODE_R {
    type Target = crate::FieldReader<u8, CKMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKMODE` writer - CKMODE"]
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "ADCCLK (Asynchronous clock mode)"]
    #[inline(always)]
    pub fn adclk(self) -> &'a mut W {
        self.variant(CKMODE_A::ADCLK)
    }
    #[doc = "PCLK/2 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn pclk_div2(self) -> &'a mut W {
        self.variant(CKMODE_A::PCLK_DIV2)
    }
    #[doc = "PCLK/4 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn pclk_div4(self) -> &'a mut W {
        self.variant(CKMODE_A::PCLK_DIV4)
    }
    #[doc = "PCLK (Synchronous clock mode)"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(CKMODE_A::PCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "OVSS0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVSS_A {
    #[doc = "0: No shift"]
    NOSHIFT = 0,
    #[doc = "1: Shift 1-bit"]
    SHIFT1 = 1,
    #[doc = "2: Shift 2-bits"]
    SHIFT2 = 2,
    #[doc = "3: Shift 3-bits"]
    SHIFT3 = 3,
    #[doc = "4: Shift 4-bits"]
    SHIFT4 = 4,
    #[doc = "5: Shift 5-bits"]
    SHIFT5 = 5,
    #[doc = "6: Shift 6-bits"]
    SHIFT6 = 6,
    #[doc = "7: Shift 7-bits"]
    SHIFT7 = 7,
    #[doc = "8: Shift 8-bits"]
    SHIFT8 = 8,
}
impl From<OVSS_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OVSS` reader - OVSS0"]
pub struct OVSS_R(crate::FieldReader<u8, OVSS_A>);
impl OVSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OVSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OVSS_A> {
        match self.bits {
            0 => Some(OVSS_A::NOSHIFT),
            1 => Some(OVSS_A::SHIFT1),
            2 => Some(OVSS_A::SHIFT2),
            3 => Some(OVSS_A::SHIFT3),
            4 => Some(OVSS_A::SHIFT4),
            5 => Some(OVSS_A::SHIFT5),
            6 => Some(OVSS_A::SHIFT6),
            7 => Some(OVSS_A::SHIFT7),
            8 => Some(OVSS_A::SHIFT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOSHIFT`"]
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        **self == OVSS_A::NOSHIFT
    }
    #[doc = "Checks if the value of the field is `SHIFT1`"]
    #[inline(always)]
    pub fn is_shift1(&self) -> bool {
        **self == OVSS_A::SHIFT1
    }
    #[doc = "Checks if the value of the field is `SHIFT2`"]
    #[inline(always)]
    pub fn is_shift2(&self) -> bool {
        **self == OVSS_A::SHIFT2
    }
    #[doc = "Checks if the value of the field is `SHIFT3`"]
    #[inline(always)]
    pub fn is_shift3(&self) -> bool {
        **self == OVSS_A::SHIFT3
    }
    #[doc = "Checks if the value of the field is `SHIFT4`"]
    #[inline(always)]
    pub fn is_shift4(&self) -> bool {
        **self == OVSS_A::SHIFT4
    }
    #[doc = "Checks if the value of the field is `SHIFT5`"]
    #[inline(always)]
    pub fn is_shift5(&self) -> bool {
        **self == OVSS_A::SHIFT5
    }
    #[doc = "Checks if the value of the field is `SHIFT6`"]
    #[inline(always)]
    pub fn is_shift6(&self) -> bool {
        **self == OVSS_A::SHIFT6
    }
    #[doc = "Checks if the value of the field is `SHIFT7`"]
    #[inline(always)]
    pub fn is_shift7(&self) -> bool {
        **self == OVSS_A::SHIFT7
    }
    #[doc = "Checks if the value of the field is `SHIFT8`"]
    #[inline(always)]
    pub fn is_shift8(&self) -> bool {
        **self == OVSS_A::SHIFT8
    }
}
impl core::ops::Deref for OVSS_R {
    type Target = crate::FieldReader<u8, OVSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSS` writer - OVSS0"]
pub struct OVSS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No shift"]
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut W {
        self.variant(OVSS_A::NOSHIFT)
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn shift1(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT1)
    }
    #[doc = "Shift 2-bits"]
    #[inline(always)]
    pub fn shift2(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT2)
    }
    #[doc = "Shift 3-bits"]
    #[inline(always)]
    pub fn shift3(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT3)
    }
    #[doc = "Shift 4-bits"]
    #[inline(always)]
    pub fn shift4(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT4)
    }
    #[doc = "Shift 5-bits"]
    #[inline(always)]
    pub fn shift5(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT5)
    }
    #[doc = "Shift 6-bits"]
    #[inline(always)]
    pub fn shift6(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT6)
    }
    #[doc = "Shift 7-bits"]
    #[inline(always)]
    pub fn shift7(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT7)
    }
    #[doc = "Shift 8-bits"]
    #[inline(always)]
    pub fn shift8(self) -> &'a mut W {
        self.variant(OVSS_A::SHIFT8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "OVSR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVSR_A {
    #[doc = "0: 2x"]
    MUL2 = 0,
    #[doc = "1: 4x"]
    MUL4 = 1,
    #[doc = "2: 8x"]
    MUL8 = 2,
    #[doc = "3: 16x"]
    MUL16 = 3,
    #[doc = "4: 32x"]
    MUL32 = 4,
    #[doc = "5: 64x"]
    MUL64 = 5,
    #[doc = "6: 128x"]
    MUL128 = 6,
    #[doc = "7: 256x"]
    MUL256 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OVSR` reader - OVSR0"]
pub struct OVSR_R(crate::FieldReader<u8, OVSR_A>);
impl OVSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        OVSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::MUL2,
            1 => OVSR_A::MUL4,
            2 => OVSR_A::MUL8,
            3 => OVSR_A::MUL16,
            4 => OVSR_A::MUL32,
            5 => OVSR_A::MUL64,
            6 => OVSR_A::MUL128,
            7 => OVSR_A::MUL256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUL2`"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        **self == OVSR_A::MUL2
    }
    #[doc = "Checks if the value of the field is `MUL4`"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        **self == OVSR_A::MUL4
    }
    #[doc = "Checks if the value of the field is `MUL8`"]
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        **self == OVSR_A::MUL8
    }
    #[doc = "Checks if the value of the field is `MUL16`"]
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        **self == OVSR_A::MUL16
    }
    #[doc = "Checks if the value of the field is `MUL32`"]
    #[inline(always)]
    pub fn is_mul32(&self) -> bool {
        **self == OVSR_A::MUL32
    }
    #[doc = "Checks if the value of the field is `MUL64`"]
    #[inline(always)]
    pub fn is_mul64(&self) -> bool {
        **self == OVSR_A::MUL64
    }
    #[doc = "Checks if the value of the field is `MUL128`"]
    #[inline(always)]
    pub fn is_mul128(&self) -> bool {
        **self == OVSR_A::MUL128
    }
    #[doc = "Checks if the value of the field is `MUL256`"]
    #[inline(always)]
    pub fn is_mul256(&self) -> bool {
        **self == OVSR_A::MUL256
    }
}
impl core::ops::Deref for OVSR_R {
    type Target = crate::FieldReader<u8, OVSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSR` writer - OVSR0"]
pub struct OVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVSR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(OVSR_A::MUL2)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(OVSR_A::MUL4)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(OVSR_A::MUL8)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(OVSR_A::MUL16)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn mul32(self) -> &'a mut W {
        self.variant(OVSR_A::MUL32)
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn mul64(self) -> &'a mut W {
        self.variant(OVSR_A::MUL64)
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn mul128(self) -> &'a mut W {
        self.variant(OVSR_A::MUL128)
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn mul256(self) -> &'a mut W {
        self.variant(OVSR_A::MUL256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OVSE"]
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 9 - TOVS"]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LFTRIG"]
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 5:8 - OVSS0"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 2:4 - OVSR0"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OVSE"]
    #[inline(always)]
    pub fn ovse(&mut self) -> OVSE_W {
        OVSE_W { w: self }
    }
    #[doc = "Bit 9 - TOVS"]
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W {
        TOVS_W { w: self }
    }
    #[doc = "Bit 29 - LFTRIG"]
    #[inline(always)]
    pub fn lftrig(&mut self) -> LFTRIG_W {
        LFTRIG_W { w: self }
    }
    #[doc = "Bits 30:31 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    #[doc = "Bits 5:8 - OVSS0"]
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W {
        OVSS_W { w: self }
    }
    #[doc = "Bits 2:4 - OVSR0"]
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W {
        OVSR_W { w: self }
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
