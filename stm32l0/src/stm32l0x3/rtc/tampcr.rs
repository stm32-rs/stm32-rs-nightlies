#[doc = "Register `TAMPCR` reader"]
pub struct R(crate::R<TAMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMPCR` writer"]
pub struct W(crate::W<TAMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMPCR_SPEC>;
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
impl From<crate::W<TAMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Tamper 2 mask flag"]
pub type TAMP2MF_A = TAMP1MF_A;
#[doc = "Field `TAMP2MF` reader - Tamper 2 mask flag"]
pub type TAMP2MF_R = TAMP1MF_R;
#[doc = "Field `TAMP2MF` writer - Tamper 2 mask flag"]
pub struct TAMP2MF_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2MF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP2MF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TAMP2MF_A::NOTMASKED)
    }
    #[doc = "Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TAMP2MF_A::MASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Tamper 2 no erase"]
pub type TAMP2NOERASE_A = TAMP1NOERASE_A;
#[doc = "Field `TAMP2NOERASE` reader - Tamper 2 no erase"]
pub type TAMP2NOERASE_R = TAMP1NOERASE_R;
#[doc = "Field `TAMP2NOERASE` writer - Tamper 2 no erase"]
pub struct TAMP2NOERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2NOERASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP2NOERASE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(TAMP2NOERASE_A::ERASE)
    }
    #[doc = "Tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut W {
        self.variant(TAMP2NOERASE_A::NOERASE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Tamper 2 interrupt enable"]
pub type TAMP2IE_A = TAMP1IE_A;
#[doc = "Field `TAMP2IE` reader - Tamper 2 interrupt enable"]
pub type TAMP2IE_R = TAMP1IE_R;
#[doc = "Field `TAMP2IE` writer - Tamper 2 interrupt enable"]
pub struct TAMP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP2IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x interrupt is disabled if TAMPIE = 0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP2IE_A::DISABLED)
    }
    #[doc = "Tamper x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP2IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Tamper 1 mask flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1MF_A {
    #[doc = "0: Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    NOTMASKED = 0,
    #[doc = "1: Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased."]
    MASKED = 1,
}
impl From<TAMP1MF_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1MF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1MF` reader - Tamper 1 mask flag"]
pub struct TAMP1MF_R(crate::FieldReader<bool, TAMP1MF_A>);
impl TAMP1MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1MF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1MF_A {
        match self.bits {
            false => TAMP1MF_A::NOTMASKED,
            true => TAMP1MF_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTMASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        **self == TAMP1MF_A::NOTMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == TAMP1MF_A::MASKED
    }
}
impl core::ops::Deref for TAMP1MF_R {
    type Target = crate::FieldReader<bool, TAMP1MF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1MF` writer - Tamper 1 mask flag"]
pub struct TAMP1MF_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1MF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP1MF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TAMP1MF_A::NOTMASKED)
    }
    #[doc = "Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TAMP1MF_A::MASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Tamper 1 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1NOERASE_A {
    #[doc = "0: Tamper x event erases the backup registers"]
    ERASE = 0,
    #[doc = "1: Tamper x event does not erase the backup registers"]
    NOERASE = 1,
}
impl From<TAMP1NOERASE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1NOERASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1NOERASE` reader - Tamper 1 no erase"]
pub struct TAMP1NOERASE_R(crate::FieldReader<bool, TAMP1NOERASE_A>);
impl TAMP1NOERASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1NOERASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1NOERASE_A {
        match self.bits {
            false => TAMP1NOERASE_A::ERASE,
            true => TAMP1NOERASE_A::NOERASE,
        }
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        **self == TAMP1NOERASE_A::ERASE
    }
    #[doc = "Checks if the value of the field is `NOERASE`"]
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        **self == TAMP1NOERASE_A::NOERASE
    }
}
impl core::ops::Deref for TAMP1NOERASE_R {
    type Target = crate::FieldReader<bool, TAMP1NOERASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1NOERASE` writer - Tamper 1 no erase"]
pub struct TAMP1NOERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1NOERASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP1NOERASE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(TAMP1NOERASE_A::ERASE)
    }
    #[doc = "Tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut W {
        self.variant(TAMP1NOERASE_A::NOERASE)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Tamper 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1IE_A {
    #[doc = "0: Tamper x interrupt is disabled if TAMPIE = 0"]
    DISABLED = 0,
    #[doc = "1: Tamper x interrupt enabled"]
    ENABLED = 1,
}
impl From<TAMP1IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1IE` reader - Tamper 1 interrupt enable"]
pub struct TAMP1IE_R(crate::FieldReader<bool, TAMP1IE_A>);
impl TAMP1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1IE_A {
        match self.bits {
            false => TAMP1IE_A::DISABLED,
            true => TAMP1IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TAMP1IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TAMP1IE_A::ENABLED
    }
}
impl core::ops::Deref for TAMP1IE_R {
    type Target = crate::FieldReader<bool, TAMP1IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1IE` writer - Tamper 1 interrupt enable"]
pub struct TAMP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP1IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x interrupt is disabled if TAMPIE = 0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP1IE_A::DISABLED)
    }
    #[doc = "Tamper x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP1IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "RTC_TAMPx pull-up disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPPUDIS_A {
    #[doc = "0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up)"]
    ENABLED = 0,
    #[doc = "1: Disable precharge of RTC_TAMPx pins"]
    DISABLED = 1,
}
impl From<TAMPPUDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPPUDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPPUDIS` reader - RTC_TAMPx pull-up disable"]
pub struct TAMPPUDIS_R(crate::FieldReader<bool, TAMPPUDIS_A>);
impl TAMPPUDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPPUDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPPUDIS_A {
        match self.bits {
            false => TAMPPUDIS_A::ENABLED,
            true => TAMPPUDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TAMPPUDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TAMPPUDIS_A::DISABLED
    }
}
impl core::ops::Deref for TAMPPUDIS_R {
    type Target = crate::FieldReader<bool, TAMPPUDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPPUDIS` writer - RTC_TAMPx pull-up disable"]
pub struct TAMPPUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPUDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPPUDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Precharge RTC_TAMPx pins before sampling (enable internal pull-up)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::ENABLED)
    }
    #[doc = "Disable precharge of RTC_TAMPx pins"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::DISABLED)
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
#[doc = "RTC_TAMPx precharge duration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPPRCH_A {
    #[doc = "0: 1 RTCCLK cycle"]
    CYCLES1 = 0,
    #[doc = "1: 2 RTCCLK cycles"]
    CYCLES2 = 1,
    #[doc = "2: 4 RTCCLK cycles"]
    CYCLES4 = 2,
    #[doc = "3: 8 RTCCLK cycles"]
    CYCLES8 = 3,
}
impl From<TAMPPRCH_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPPRCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TAMPPRCH` reader - RTC_TAMPx precharge duration"]
pub struct TAMPPRCH_R(crate::FieldReader<u8, TAMPPRCH_A>);
impl TAMPPRCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPPRCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPPRCH_A {
        match self.bits {
            0 => TAMPPRCH_A::CYCLES1,
            1 => TAMPPRCH_A::CYCLES2,
            2 => TAMPPRCH_A::CYCLES4,
            3 => TAMPPRCH_A::CYCLES8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES1`"]
    #[inline(always)]
    pub fn is_cycles1(&self) -> bool {
        **self == TAMPPRCH_A::CYCLES1
    }
    #[doc = "Checks if the value of the field is `CYCLES2`"]
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        **self == TAMPPRCH_A::CYCLES2
    }
    #[doc = "Checks if the value of the field is `CYCLES4`"]
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        **self == TAMPPRCH_A::CYCLES4
    }
    #[doc = "Checks if the value of the field is `CYCLES8`"]
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        **self == TAMPPRCH_A::CYCLES8
    }
}
impl core::ops::Deref for TAMPPRCH_R {
    type Target = crate::FieldReader<u8, TAMPPRCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPPRCH` writer - RTC_TAMPx precharge duration"]
pub struct TAMPPRCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPRCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPPRCH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1 RTCCLK cycle"]
    #[inline(always)]
    pub fn cycles1(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::CYCLES1)
    }
    #[doc = "2 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::CYCLES2)
    }
    #[doc = "4 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::CYCLES4)
    }
    #[doc = "8 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::CYCLES8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "RTC_TAMPx filter count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPFLT_A {
    #[doc = "0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)"]
    IMMEDIATE = 0,
    #[doc = "1: Tamper event is activated after 2 consecutive samples at the active level"]
    SAMPLES2 = 1,
    #[doc = "2: Tamper event is activated after 4 consecutive samples at the active level"]
    SAMPLES4 = 2,
    #[doc = "3: Tamper event is activated after 8 consecutive samples at the active level"]
    SAMPLES8 = 3,
}
impl From<TAMPFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFLT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TAMPFLT` reader - RTC_TAMPx filter count"]
pub struct TAMPFLT_R(crate::FieldReader<u8, TAMPFLT_A>);
impl TAMPFLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPFLT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPFLT_A {
        match self.bits {
            0 => TAMPFLT_A::IMMEDIATE,
            1 => TAMPFLT_A::SAMPLES2,
            2 => TAMPFLT_A::SAMPLES4,
            3 => TAMPFLT_A::SAMPLES8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        **self == TAMPFLT_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `SAMPLES2`"]
    #[inline(always)]
    pub fn is_samples2(&self) -> bool {
        **self == TAMPFLT_A::SAMPLES2
    }
    #[doc = "Checks if the value of the field is `SAMPLES4`"]
    #[inline(always)]
    pub fn is_samples4(&self) -> bool {
        **self == TAMPFLT_A::SAMPLES4
    }
    #[doc = "Checks if the value of the field is `SAMPLES8`"]
    #[inline(always)]
    pub fn is_samples8(&self) -> bool {
        **self == TAMPFLT_A::SAMPLES8
    }
}
impl core::ops::Deref for TAMPFLT_R {
    type Target = crate::FieldReader<u8, TAMPFLT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPFLT` writer - RTC_TAMPx filter count"]
pub struct TAMPFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPFLT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)"]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(TAMPFLT_A::IMMEDIATE)
    }
    #[doc = "Tamper event is activated after 2 consecutive samples at the active level"]
    #[inline(always)]
    pub fn samples2(self) -> &'a mut W {
        self.variant(TAMPFLT_A::SAMPLES2)
    }
    #[doc = "Tamper event is activated after 4 consecutive samples at the active level"]
    #[inline(always)]
    pub fn samples4(self) -> &'a mut W {
        self.variant(TAMPFLT_A::SAMPLES4)
    }
    #[doc = "Tamper event is activated after 8 consecutive samples at the active level"]
    #[inline(always)]
    pub fn samples8(self) -> &'a mut W {
        self.variant(TAMPFLT_A::SAMPLES8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Tamper sampling frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPFREQ_A {
    #[doc = "0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    DIV32768 = 0,
    #[doc = "1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    DIV16384 = 1,
    #[doc = "2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    DIV8192 = 2,
    #[doc = "3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    DIV4096 = 3,
    #[doc = "4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    DIV2048 = 4,
    #[doc = "5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    DIV1024 = 5,
    #[doc = "6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    DIV512 = 6,
    #[doc = "7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    DIV256 = 7,
}
impl From<TAMPFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency"]
pub struct TAMPFREQ_R(crate::FieldReader<u8, TAMPFREQ_A>);
impl TAMPFREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPFREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPFREQ_A {
        match self.bits {
            0 => TAMPFREQ_A::DIV32768,
            1 => TAMPFREQ_A::DIV16384,
            2 => TAMPFREQ_A::DIV8192,
            3 => TAMPFREQ_A::DIV4096,
            4 => TAMPFREQ_A::DIV2048,
            5 => TAMPFREQ_A::DIV1024,
            6 => TAMPFREQ_A::DIV512,
            7 => TAMPFREQ_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        **self == TAMPFREQ_A::DIV32768
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        **self == TAMPFREQ_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        **self == TAMPFREQ_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        **self == TAMPFREQ_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        **self == TAMPFREQ_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        **self == TAMPFREQ_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        **self == TAMPFREQ_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == TAMPFREQ_A::DIV256
    }
}
impl core::ops::Deref for TAMPFREQ_R {
    type Target = crate::FieldReader<u8, TAMPFREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency"]
pub struct TAMPFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPFREQ_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::DIV32768)
    }
    #[doc = "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::DIV16384)
    }
    #[doc = "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::DIV8192)
    }
    #[doc = "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::DIV4096)
    }
    #[doc = "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::DIV2048)
    }
    #[doc = "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::DIV1024)
    }
    #[doc = "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::DIV512)
    }
    #[doc = "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::DIV256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Activate timestamp on tamper detection event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPTS_A {
    #[doc = "0: Tamper detection event does not cause a timestamp to be saved"]
    NOSAVE = 0,
    #[doc = "1: Save timestamp on tamper detection event"]
    SAVE = 1,
}
impl From<TAMPTS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event"]
pub struct TAMPTS_R(crate::FieldReader<bool, TAMPTS_A>);
impl TAMPTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPTS_A {
        match self.bits {
            false => TAMPTS_A::NOSAVE,
            true => TAMPTS_A::SAVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOSAVE`"]
    #[inline(always)]
    pub fn is_no_save(&self) -> bool {
        **self == TAMPTS_A::NOSAVE
    }
    #[doc = "Checks if the value of the field is `SAVE`"]
    #[inline(always)]
    pub fn is_save(&self) -> bool {
        **self == TAMPTS_A::SAVE
    }
}
impl core::ops::Deref for TAMPTS_R {
    type Target = crate::FieldReader<bool, TAMPTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event"]
pub struct TAMPTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper detection event does not cause a timestamp to be saved"]
    #[inline(always)]
    pub fn no_save(self) -> &'a mut W {
        self.variant(TAMPTS_A::NOSAVE)
    }
    #[doc = "Save timestamp on tamper detection event"]
    #[inline(always)]
    pub fn save(self) -> &'a mut W {
        self.variant(TAMPTS_A::SAVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Active level for RTC_TAMP2 input"]
pub type TAMP2TRG_A = TAMP1TRG_A;
#[doc = "Field `TAMP2TRG` reader - Active level for RTC_TAMP2 input"]
pub type TAMP2TRG_R = TAMP1TRG_R;
#[doc = "Field `TAMP2TRG` writer - Active level for RTC_TAMP2 input"]
pub struct TAMP2TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2TRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP2TRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TAMP2TRG_A::RISINGEDGE)
    }
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input falling edge triggers a tamper detection event"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TAMP2TRG_A::FALLINGEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "RTC_TAMP2 input detection enable"]
pub type TAMP2E_A = TAMP1E_A;
#[doc = "Field `TAMP2E` reader - RTC_TAMP2 input detection enable"]
pub type TAMP2E_R = TAMP1E_R;
#[doc = "Field `TAMP2E` writer - RTC_TAMP2 input detection enable"]
pub struct TAMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP2E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC_TAMPx input detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP2E_A::DISABLED)
    }
    #[doc = "RTC_TAMPx input detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP2E_A::ENABLED)
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
#[doc = "Tamper interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPIE_A {
    #[doc = "0: Tamper interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Tamper interrupt enabled"]
    ENABLED = 1,
}
impl From<TAMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub struct TAMPIE_R(crate::FieldReader<bool, TAMPIE_A>);
impl TAMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPIE_A {
        match self.bits {
            false => TAMPIE_A::DISABLED,
            true => TAMPIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TAMPIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TAMPIE_A::ENABLED
    }
}
impl core::ops::Deref for TAMPIE_R {
    type Target = crate::FieldReader<bool, TAMPIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub struct TAMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMPIE_A::DISABLED)
    }
    #[doc = "Tamper interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMPIE_A::ENABLED)
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
#[doc = "Active level for RTC_TAMP1 input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1TRG_A {
    #[doc = "0: If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input staying low triggers a tamper detection event."]
    RISINGEDGE = 0,
    #[doc = "1: If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input falling edge triggers a tamper detection event"]
    FALLINGEDGE = 1,
}
impl From<TAMP1TRG_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1TRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1TRG` reader - Active level for RTC_TAMP1 input"]
pub struct TAMP1TRG_R(crate::FieldReader<bool, TAMP1TRG_A>);
impl TAMP1TRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1TRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1TRG_A {
        match self.bits {
            false => TAMP1TRG_A::RISINGEDGE,
            true => TAMP1TRG_A::FALLINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == TAMP1TRG_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == TAMP1TRG_A::FALLINGEDGE
    }
}
impl core::ops::Deref for TAMP1TRG_R {
    type Target = crate::FieldReader<bool, TAMP1TRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1TRG` writer - Active level for RTC_TAMP1 input"]
pub struct TAMP1TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1TRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP1TRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TAMP1TRG_A::RISINGEDGE)
    }
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input falling edge triggers a tamper detection event"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TAMP1TRG_A::FALLINGEDGE)
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
#[doc = "RTC_TAMP1 input detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1E_A {
    #[doc = "0: RTC_TAMPx input detection disabled"]
    DISABLED = 0,
    #[doc = "1: RTC_TAMPx input detection enabled"]
    ENABLED = 1,
}
impl From<TAMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1E` reader - RTC_TAMP1 input detection enable"]
pub struct TAMP1E_R(crate::FieldReader<bool, TAMP1E_A>);
impl TAMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1E_A {
        match self.bits {
            false => TAMP1E_A::DISABLED,
            true => TAMP1E_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TAMP1E_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TAMP1E_A::ENABLED
    }
}
impl core::ops::Deref for TAMP1E_R {
    type Target = crate::FieldReader<bool, TAMP1E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1E` writer - RTC_TAMP1 input detection enable"]
pub struct TAMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP1E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC_TAMPx input detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP1E_A::DISABLED)
    }
    #[doc = "RTC_TAMPx input detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP1E_A::ENABLED)
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
#[doc = "Tamper 3 mask flag"]
pub type TAMP3MF_A = TAMP1MF_A;
#[doc = "Field `TAMP3MF` reader - Tamper 3 mask flag"]
pub type TAMP3MF_R = TAMP1MF_R;
#[doc = "Field `TAMP3MF` writer - Tamper 3 mask flag"]
pub struct TAMP3MF_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3MF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP3MF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TAMP3MF_A::NOTMASKED)
    }
    #[doc = "Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TAMP3MF_A::MASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Tamper 3 no erase"]
pub type TAMP3NOERASE_A = TAMP1NOERASE_A;
#[doc = "Field `TAMP3NOERASE` reader - Tamper 3 no erase"]
pub type TAMP3NOERASE_R = TAMP1NOERASE_R;
#[doc = "Field `TAMP3NOERASE` writer - Tamper 3 no erase"]
pub struct TAMP3NOERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3NOERASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP3NOERASE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(TAMP3NOERASE_A::ERASE)
    }
    #[doc = "Tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut W {
        self.variant(TAMP3NOERASE_A::NOERASE)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Tamper 3 interrupt enable"]
pub type TAMP3IE_A = TAMP1IE_A;
#[doc = "Field `TAMP3IE` reader - Tamper 3 interrupt enable"]
pub type TAMP3IE_R = TAMP1IE_R;
#[doc = "Field `TAMP3IE` writer - Tamper 3 interrupt enable"]
pub struct TAMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP3IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x interrupt is disabled if TAMPIE = 0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP3IE_A::DISABLED)
    }
    #[doc = "Tamper x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP3IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Active level for RTC_TAMP3 input"]
pub type TAMP3TRG_A = TAMP1TRG_A;
#[doc = "Field `TAMP3TRG` reader - Active level for RTC_TAMP3 input"]
pub type TAMP3TRG_R = TAMP1TRG_R;
#[doc = "Field `TAMP3TRG` writer - Active level for RTC_TAMP3 input"]
pub struct TAMP3TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3TRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP3TRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TAMP3TRG_A::RISINGEDGE)
    }
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input falling edge triggers a tamper detection event"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TAMP3TRG_A::FALLINGEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "RTC_TAMP3 detection enable"]
pub type TAMP3E_A = TAMP1E_A;
#[doc = "Field `TAMP3E` reader - RTC_TAMP3 detection enable"]
pub type TAMP3E_R = TAMP1E_R;
#[doc = "Field `TAMP3E` writer - RTC_TAMP3 detection enable"]
pub struct TAMP3E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP3E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC_TAMPx input detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP3E_A::DISABLED)
    }
    #[doc = "RTC_TAMPx input detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP3E_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 21 - Tamper 2 mask flag"]
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noerase(&self) -> TAMP2NOERASE_R {
        TAMP2NOERASE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Tamper 1 mask flag"]
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noerase(&self) -> TAMP1NOERASE_R {
        TAMP1NOERASE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 24 - Tamper 3 mask flag"]
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Tamper 3 no erase"]
    #[inline(always)]
    pub fn tamp3noerase(&self) -> TAMP3NOERASE_R {
        TAMP3NOERASE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - Tamper 2 mask flag"]
    #[inline(always)]
    pub fn tamp2mf(&mut self) -> TAMP2MF_W {
        TAMP2MF_W { w: self }
    }
    #[doc = "Bit 20 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noerase(&mut self) -> TAMP2NOERASE_W {
        TAMP2NOERASE_W { w: self }
    }
    #[doc = "Bit 19 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W {
        TAMP2IE_W { w: self }
    }
    #[doc = "Bit 18 - Tamper 1 mask flag"]
    #[inline(always)]
    pub fn tamp1mf(&mut self) -> TAMP1MF_W {
        TAMP1MF_W { w: self }
    }
    #[doc = "Bit 17 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noerase(&mut self) -> TAMP1NOERASE_W {
        TAMP1NOERASE_W { w: self }
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W {
        TAMP1IE_W { w: self }
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W {
        TAMPPUDIS_W { w: self }
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W {
        TAMPPRCH_W { w: self }
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W {
        TAMPFLT_W { w: self }
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W {
        TAMPFREQ_W { w: self }
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W {
        TAMPTS_W { w: self }
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input"]
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W {
        TAMP2TRG_W { w: self }
    }
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W {
        TAMP2E_W { w: self }
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W {
        TAMPIE_W { w: self }
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W {
        TAMP1TRG_W { w: self }
    }
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W {
        TAMP1E_W { w: self }
    }
    #[doc = "Bit 24 - Tamper 3 mask flag"]
    #[inline(always)]
    pub fn tamp3mf(&mut self) -> TAMP3MF_W {
        TAMP3MF_W { w: self }
    }
    #[doc = "Bit 23 - Tamper 3 no erase"]
    #[inline(always)]
    pub fn tamp3noerase(&mut self) -> TAMP3NOERASE_W {
        TAMP3NOERASE_W { w: self }
    }
    #[doc = "Bit 22 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W {
        TAMP3IE_W { w: self }
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input"]
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W {
        TAMP3TRG_W { w: self }
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W {
        TAMP3E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC tamper configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tampcr](index.html) module"]
pub struct TAMPCR_SPEC;
impl crate::RegisterSpec for TAMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tampcr::R](R) reader structure"]
impl crate::Readable for TAMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tampcr::W](W) writer structure"]
impl crate::Writable for TAMPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMPCR to value 0"]
impl crate::Resettable for TAMPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
