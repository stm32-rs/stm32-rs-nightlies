#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TAMP1NOER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1NOER_A {
    #[doc = "0: Tamper x event erases the backup registers"]
    ERASE = 0,
    #[doc = "1: Tamper x event does not erase the backup registers"]
    NOTERASE = 1,
}
impl From<TAMP1NOER_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1NOER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1NOER` reader - TAMP1NOER"]
pub struct TAMP1NOER_R(crate::FieldReader<bool, TAMP1NOER_A>);
impl TAMP1NOER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1NOER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1NOER_A {
        match self.bits {
            false => TAMP1NOER_A::ERASE,
            true => TAMP1NOER_A::NOTERASE,
        }
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        **self == TAMP1NOER_A::ERASE
    }
    #[doc = "Checks if the value of the field is `NOTERASE`"]
    #[inline(always)]
    pub fn is_not_erase(&self) -> bool {
        **self == TAMP1NOER_A::NOTERASE
    }
}
impl core::ops::Deref for TAMP1NOER_R {
    type Target = crate::FieldReader<bool, TAMP1NOER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1NOER` writer - TAMP1NOER"]
pub struct TAMP1NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1NOER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP1NOER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(TAMP1NOER_A::ERASE)
    }
    #[doc = "Tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn not_erase(self) -> &'a mut W {
        self.variant(TAMP1NOER_A::NOTERASE)
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
#[doc = "TAMP2NOER"]
pub type TAMP2NOER_A = TAMP1NOER_A;
#[doc = "Field `TAMP2NOER` reader - TAMP2NOER"]
pub type TAMP2NOER_R = TAMP1NOER_R;
#[doc = "Field `TAMP2NOER` writer - TAMP2NOER"]
pub struct TAMP2NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2NOER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP2NOER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(TAMP2NOER_A::ERASE)
    }
    #[doc = "Tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn not_erase(self) -> &'a mut W {
        self.variant(TAMP2NOER_A::NOTERASE)
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
#[doc = "TAMP3NOER"]
pub type TAMP3NOER_A = TAMP1NOER_A;
#[doc = "Field `TAMP3NOER` reader - TAMP3NOER"]
pub type TAMP3NOER_R = TAMP1NOER_R;
#[doc = "Field `TAMP3NOER` writer - TAMP3NOER"]
pub struct TAMP3NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3NOER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP3NOER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(TAMP3NOER_A::ERASE)
    }
    #[doc = "Tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn not_erase(self) -> &'a mut W {
        self.variant(TAMP3NOER_A::NOTERASE)
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
#[doc = "TAMP1MSK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1MSK_A {
    #[doc = "0: Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    RESETBYSOFTWARE = 0,
    #[doc = "1: Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased. The tamper x interrupt must not be enabled when TAMP3MSK is set"]
    RESETBYHARDWARE = 1,
}
impl From<TAMP1MSK_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1MSK` reader - TAMP1MSK"]
pub struct TAMP1MSK_R(crate::FieldReader<bool, TAMP1MSK_A>);
impl TAMP1MSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1MSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1MSK_A {
        match self.bits {
            false => TAMP1MSK_A::RESETBYSOFTWARE,
            true => TAMP1MSK_A::RESETBYHARDWARE,
        }
    }
    #[doc = "Checks if the value of the field is `RESETBYSOFTWARE`"]
    #[inline(always)]
    pub fn is_reset_by_software(&self) -> bool {
        **self == TAMP1MSK_A::RESETBYSOFTWARE
    }
    #[doc = "Checks if the value of the field is `RESETBYHARDWARE`"]
    #[inline(always)]
    pub fn is_reset_by_hardware(&self) -> bool {
        **self == TAMP1MSK_A::RESETBYHARDWARE
    }
}
impl core::ops::Deref for TAMP1MSK_R {
    type Target = crate::FieldReader<bool, TAMP1MSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1MSK` writer - TAMP1MSK"]
pub struct TAMP1MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP1MSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    #[inline(always)]
    pub fn reset_by_software(self) -> &'a mut W {
        self.variant(TAMP1MSK_A::RESETBYSOFTWARE)
    }
    #[doc = "Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased. The tamper x interrupt must not be enabled when TAMP3MSK is set"]
    #[inline(always)]
    pub fn reset_by_hardware(self) -> &'a mut W {
        self.variant(TAMP1MSK_A::RESETBYHARDWARE)
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
#[doc = "TAMP2MSK"]
pub type TAMP2MSK_A = TAMP1MSK_A;
#[doc = "Field `TAMP2MSK` reader - TAMP2MSK"]
pub type TAMP2MSK_R = TAMP1MSK_R;
#[doc = "Field `TAMP2MSK` writer - TAMP2MSK"]
pub struct TAMP2MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP2MSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    #[inline(always)]
    pub fn reset_by_software(self) -> &'a mut W {
        self.variant(TAMP2MSK_A::RESETBYSOFTWARE)
    }
    #[doc = "Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased. The tamper x interrupt must not be enabled when TAMP3MSK is set"]
    #[inline(always)]
    pub fn reset_by_hardware(self) -> &'a mut W {
        self.variant(TAMP2MSK_A::RESETBYHARDWARE)
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
#[doc = "TAMP3MSK"]
pub type TAMP3MSK_A = TAMP1MSK_A;
#[doc = "Field `TAMP3MSK` reader - TAMP3MSK"]
pub type TAMP3MSK_R = TAMP1MSK_R;
#[doc = "Field `TAMP3MSK` writer - TAMP3MSK"]
pub struct TAMP3MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP3MSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    #[inline(always)]
    pub fn reset_by_software(self) -> &'a mut W {
        self.variant(TAMP3MSK_A::RESETBYSOFTWARE)
    }
    #[doc = "Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased. The tamper x interrupt must not be enabled when TAMP3MSK is set"]
    #[inline(always)]
    pub fn reset_by_hardware(self) -> &'a mut W {
        self.variant(TAMP3MSK_A::RESETBYHARDWARE)
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
#[doc = "Backup registerserase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKERASE_A {
    #[doc = "1: Reset backup registers"]
    RESET = 1,
}
impl From<BKERASE_A> for bool {
    #[inline(always)]
    fn from(variant: BKERASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKERASE` reader - Backup registerserase"]
pub struct BKERASE_R(crate::FieldReader<bool, BKERASE_A>);
impl BKERASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKERASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BKERASE_A> {
        match self.bits {
            true => Some(BKERASE_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == BKERASE_A::RESET
    }
}
impl core::ops::Deref for BKERASE_R {
    type Target = crate::FieldReader<bool, BKERASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKERASE` writer - Backup registerserase"]
pub struct BKERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> BKERASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKERASE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset backup registers"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BKERASE_A::RESET)
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
#[doc = "TAMP1TRG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1TRG_A {
    #[doc = "0: If TAMPFLT != 00 Tamper x input staying low triggers a tamper detection event. If TAMPFLT = 00 Tamper x input rising edge and high level triggers a tamper detection event"]
    FILTEREDLOWORUNFILTEREDHIGH = 0,
    #[doc = "1: If TAMPFLT != 00 Tamper x input staying high triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge and low level triggers a tamper detection event"]
    FILTEREDHIGHORUNFILTEREDLOW = 1,
}
impl From<TAMP1TRG_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1TRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1TRG` reader - TAMP1TRG"]
pub struct TAMP1TRG_R(crate::FieldReader<bool, TAMP1TRG_A>);
impl TAMP1TRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1TRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1TRG_A {
        match self.bits {
            false => TAMP1TRG_A::FILTEREDLOWORUNFILTEREDHIGH,
            true => TAMP1TRG_A::FILTEREDHIGHORUNFILTEREDLOW,
        }
    }
    #[doc = "Checks if the value of the field is `FILTEREDLOWORUNFILTEREDHIGH`"]
    #[inline(always)]
    pub fn is_filtered_low_or_unfiltered_high(&self) -> bool {
        **self == TAMP1TRG_A::FILTEREDLOWORUNFILTEREDHIGH
    }
    #[doc = "Checks if the value of the field is `FILTEREDHIGHORUNFILTEREDLOW`"]
    #[inline(always)]
    pub fn is_filtered_high_or_unfiltered_low(&self) -> bool {
        **self == TAMP1TRG_A::FILTEREDHIGHORUNFILTEREDLOW
    }
}
impl core::ops::Deref for TAMP1TRG_R {
    type Target = crate::FieldReader<bool, TAMP1TRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1TRG` writer - TAMP1TRG"]
pub struct TAMP1TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1TRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP1TRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If TAMPFLT != 00 Tamper x input staying low triggers a tamper detection event. If TAMPFLT = 00 Tamper x input rising edge and high level triggers a tamper detection event"]
    #[inline(always)]
    pub fn filtered_low_or_unfiltered_high(self) -> &'a mut W {
        self.variant(TAMP1TRG_A::FILTEREDLOWORUNFILTEREDHIGH)
    }
    #[doc = "If TAMPFLT != 00 Tamper x input staying high triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge and low level triggers a tamper detection event"]
    #[inline(always)]
    pub fn filtered_high_or_unfiltered_low(self) -> &'a mut W {
        self.variant(TAMP1TRG_A::FILTEREDHIGHORUNFILTEREDLOW)
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
#[doc = "TAMP2TRG"]
pub type TAMP2TRG_A = TAMP1TRG_A;
#[doc = "Field `TAMP2TRG` reader - TAMP2TRG"]
pub type TAMP2TRG_R = TAMP1TRG_R;
#[doc = "Field `TAMP2TRG` writer - TAMP2TRG"]
pub struct TAMP2TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2TRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP2TRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If TAMPFLT != 00 Tamper x input staying low triggers a tamper detection event. If TAMPFLT = 00 Tamper x input rising edge and high level triggers a tamper detection event"]
    #[inline(always)]
    pub fn filtered_low_or_unfiltered_high(self) -> &'a mut W {
        self.variant(TAMP2TRG_A::FILTEREDLOWORUNFILTEREDHIGH)
    }
    #[doc = "If TAMPFLT != 00 Tamper x input staying high triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge and low level triggers a tamper detection event"]
    #[inline(always)]
    pub fn filtered_high_or_unfiltered_low(self) -> &'a mut W {
        self.variant(TAMP2TRG_A::FILTEREDHIGHORUNFILTEREDLOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "TAMP3TRG"]
pub type TAMP3TRG_A = TAMP1TRG_A;
#[doc = "Field `TAMP3TRG` reader - TAMP3TRG"]
pub type TAMP3TRG_R = TAMP1TRG_R;
#[doc = "Field `TAMP3TRG` writer - TAMP3TRG"]
pub struct TAMP3TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3TRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP3TRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If TAMPFLT != 00 Tamper x input staying low triggers a tamper detection event. If TAMPFLT = 00 Tamper x input rising edge and high level triggers a tamper detection event"]
    #[inline(always)]
    pub fn filtered_low_or_unfiltered_high(self) -> &'a mut W {
        self.variant(TAMP3TRG_A::FILTEREDLOWORUNFILTEREDHIGH)
    }
    #[doc = "If TAMPFLT != 00 Tamper x input staying high triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge and low level triggers a tamper detection event"]
    #[inline(always)]
    pub fn filtered_high_or_unfiltered_low(self) -> &'a mut W {
        self.variant(TAMP3TRG_A::FILTEREDHIGHORUNFILTEREDLOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    pub fn tamp1noer(&self) -> TAMP1NOER_R {
        TAMP1NOER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    pub fn tamp2noer(&self) -> TAMP2NOER_R {
        TAMP2NOER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    pub fn tamp3noer(&self) -> TAMP3NOER_R {
        TAMP3NOER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    pub fn tamp3msk(&self) -> TAMP3MSK_R {
        TAMP3MSK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Backup registerserase"]
    #[inline(always)]
    pub fn bkerase(&self) -> BKERASE_R {
        BKERASE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    pub fn tamp1noer(&mut self) -> TAMP1NOER_W {
        TAMP1NOER_W { w: self }
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    pub fn tamp2noer(&mut self) -> TAMP2NOER_W {
        TAMP2NOER_W { w: self }
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    pub fn tamp3noer(&mut self) -> TAMP3NOER_W {
        TAMP3NOER_W { w: self }
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W {
        TAMP1MSK_W { w: self }
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W {
        TAMP2MSK_W { w: self }
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    pub fn tamp3msk(&mut self) -> TAMP3MSK_W {
        TAMP3MSK_W { w: self }
    }
    #[doc = "Bit 23 - Backup registerserase"]
    #[inline(always)]
    pub fn bkerase(&mut self) -> BKERASE_W {
        BKERASE_W { w: self }
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W {
        TAMP1TRG_W { w: self }
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W {
        TAMP2TRG_W { w: self }
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W {
        TAMP3TRG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
