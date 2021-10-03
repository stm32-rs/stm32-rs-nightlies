#[doc = "Register `DTER` reader"]
pub struct R(crate::R<DTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTER` writer"]
pub struct W(crate::W<DTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTER_SPEC>;
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
impl From<crate::W<DTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Deadtime Falling Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTFLKX_A {
    #[doc = "0: Deadtime falling value and sign is writable"]
    UNLOCKED = 0,
    #[doc = "1: Deadtime falling value and sign is read-only"]
    LOCKED = 1,
}
impl From<DTFLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTFLKX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTFLKx` reader - Deadtime Falling Lock"]
pub struct DTFLKX_R(crate::FieldReader<bool, DTFLKX_A>);
impl DTFLKX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTFLKX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTFLKX_A {
        match self.bits {
            false => DTFLKX_A::UNLOCKED,
            true => DTFLKX_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == DTFLKX_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == DTFLKX_A::LOCKED
    }
}
impl core::ops::Deref for DTFLKX_R {
    type Target = crate::FieldReader<bool, DTFLKX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTFLKx` writer - Deadtime Falling Lock"]
pub struct DTFLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFLKX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTFLKX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Deadtime falling value and sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTFLKX_A::UNLOCKED)
    }
    #[doc = "Deadtime falling value and sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTFLKX_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Deadtime Falling Sign Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTFSLKX_A {
    #[doc = "0: Deadtime falling sign is writable"]
    UNLOCKED = 0,
    #[doc = "1: Deadtime falling sign is read-only"]
    LOCKED = 1,
}
impl From<DTFSLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTFSLKX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTFSLKx` reader - Deadtime Falling Sign Lock"]
pub struct DTFSLKX_R(crate::FieldReader<bool, DTFSLKX_A>);
impl DTFSLKX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTFSLKX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTFSLKX_A {
        match self.bits {
            false => DTFSLKX_A::UNLOCKED,
            true => DTFSLKX_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == DTFSLKX_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == DTFSLKX_A::LOCKED
    }
}
impl core::ops::Deref for DTFSLKX_R {
    type Target = crate::FieldReader<bool, DTFSLKX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTFSLKx` writer - Deadtime Falling Sign Lock"]
pub struct DTFSLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFSLKX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTFSLKX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Deadtime falling sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTFSLKX_A::UNLOCKED)
    }
    #[doc = "Deadtime falling sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTFSLKX_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Sign Deadtime Falling value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDTFX_A {
    #[doc = "0: Positive deadtime on falling edge"]
    POSITIVE = 0,
    #[doc = "1: Negative deadtime on falling edge"]
    NEGATIVE = 1,
}
impl From<SDTFX_A> for bool {
    #[inline(always)]
    fn from(variant: SDTFX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDTFx` reader - Sign Deadtime Falling value"]
pub struct SDTFX_R(crate::FieldReader<bool, SDTFX_A>);
impl SDTFX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDTFX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDTFX_A {
        match self.bits {
            false => SDTFX_A::POSITIVE,
            true => SDTFX_A::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        **self == SDTFX_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        **self == SDTFX_A::NEGATIVE
    }
}
impl core::ops::Deref for SDTFX_R {
    type Target = crate::FieldReader<bool, SDTFX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDTFx` writer - Sign Deadtime Falling value"]
pub struct SDTFX_W<'a> {
    w: &'a mut W,
}
impl<'a> SDTFX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDTFX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Positive deadtime on falling edge"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(SDTFX_A::POSITIVE)
    }
    #[doc = "Negative deadtime on falling edge"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(SDTFX_A::NEGATIVE)
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
#[doc = "Field `DTFx` reader - Deadtime Falling value"]
pub struct DTFX_R(crate::FieldReader<u16, u16>);
impl DTFX_R {
    pub(crate) fn new(bits: u16) -> Self {
        DTFX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTFX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTFx` writer - Deadtime Falling value"]
pub struct DTFX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Deadtime Rising Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTRLKX_A {
    #[doc = "0: Deadtime rising value and sign is writable"]
    UNLOCKED = 0,
    #[doc = "1: Deadtime rising value and sign is read-only"]
    LOCKED = 1,
}
impl From<DTRLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTRLKX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTRLKx` reader - Deadtime Rising Lock"]
pub struct DTRLKX_R(crate::FieldReader<bool, DTRLKX_A>);
impl DTRLKX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTRLKX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTRLKX_A {
        match self.bits {
            false => DTRLKX_A::UNLOCKED,
            true => DTRLKX_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == DTRLKX_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == DTRLKX_A::LOCKED
    }
}
impl core::ops::Deref for DTRLKX_R {
    type Target = crate::FieldReader<bool, DTRLKX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTRLKx` writer - Deadtime Rising Lock"]
pub struct DTRLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRLKX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTRLKX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Deadtime rising value and sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTRLKX_A::UNLOCKED)
    }
    #[doc = "Deadtime rising value and sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTRLKX_A::LOCKED)
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
#[doc = "Deadtime Rising Sign Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTRSLKX_A {
    #[doc = "0: Deadtime rising sign is writable"]
    UNLOCKED = 0,
    #[doc = "1: Deadtime rising sign is read-only"]
    LOCKED = 1,
}
impl From<DTRSLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTRSLKX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTRSLKx` reader - Deadtime Rising Sign Lock"]
pub struct DTRSLKX_R(crate::FieldReader<bool, DTRSLKX_A>);
impl DTRSLKX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTRSLKX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTRSLKX_A {
        match self.bits {
            false => DTRSLKX_A::UNLOCKED,
            true => DTRSLKX_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == DTRSLKX_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == DTRSLKX_A::LOCKED
    }
}
impl core::ops::Deref for DTRSLKX_R {
    type Target = crate::FieldReader<bool, DTRSLKX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTRSLKx` writer - Deadtime Rising Sign Lock"]
pub struct DTRSLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRSLKX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTRSLKX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Deadtime rising sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTRSLKX_A::UNLOCKED)
    }
    #[doc = "Deadtime rising sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTRSLKX_A::LOCKED)
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
#[doc = "Field `DTPRSC` reader - Deadtime Prescaler"]
pub struct DTPRSC_R(crate::FieldReader<u8, u8>);
impl DTPRSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTPRSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTPRSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTPRSC` writer - Deadtime Prescaler"]
pub struct DTPRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | ((value as u32 & 0x07) << 10);
        self.w
    }
}
#[doc = "Sign Deadtime Rising value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDTRX_A {
    #[doc = "0: Positive deadtime on rising edge"]
    POSITIVE = 0,
    #[doc = "1: Negative deadtime on rising edge"]
    NEGATIVE = 1,
}
impl From<SDTRX_A> for bool {
    #[inline(always)]
    fn from(variant: SDTRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDTRx` reader - Sign Deadtime Rising value"]
pub struct SDTRX_R(crate::FieldReader<bool, SDTRX_A>);
impl SDTRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDTRX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDTRX_A {
        match self.bits {
            false => SDTRX_A::POSITIVE,
            true => SDTRX_A::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        **self == SDTRX_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        **self == SDTRX_A::NEGATIVE
    }
}
impl core::ops::Deref for SDTRX_R {
    type Target = crate::FieldReader<bool, SDTRX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDTRx` writer - Sign Deadtime Rising value"]
pub struct SDTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> SDTRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDTRX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Positive deadtime on rising edge"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(SDTRX_A::POSITIVE)
    }
    #[doc = "Negative deadtime on rising edge"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(SDTRX_A::NEGATIVE)
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
#[doc = "Field `DTRx` reader - Deadtime Rising value"]
pub struct DTRX_R(crate::FieldReader<u16, u16>);
impl DTRX_R {
    pub(crate) fn new(bits: u16) -> Self {
        DTRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTRX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTRx` writer - Deadtime Rising value"]
pub struct DTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    pub fn dtflkx(&self) -> DTFLKX_R {
        DTFLKX_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    pub fn dtfslkx(&self) -> DTFSLKX_R {
        DTFSLKX_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    pub fn sdtfx(&self) -> SDTFX_R {
        SDTFX_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    pub fn dtfx(&self) -> DTFX_R {
        DTFX_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    pub fn dtrlkx(&self) -> DTRLKX_R {
        DTRLKX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    pub fn dtrslkx(&self) -> DTRSLKX_R {
        DTRSLKX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    pub fn dtprsc(&self) -> DTPRSC_R {
        DTPRSC_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    pub fn sdtrx(&self) -> SDTRX_R {
        SDTRX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    pub fn dtrx(&self) -> DTRX_R {
        DTRX_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    pub fn dtflkx(&mut self) -> DTFLKX_W {
        DTFLKX_W { w: self }
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    pub fn dtfslkx(&mut self) -> DTFSLKX_W {
        DTFSLKX_W { w: self }
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    pub fn sdtfx(&mut self) -> SDTFX_W {
        SDTFX_W { w: self }
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    pub fn dtfx(&mut self) -> DTFX_W {
        DTFX_W { w: self }
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    pub fn dtrlkx(&mut self) -> DTRLKX_W {
        DTRLKX_W { w: self }
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    pub fn dtrslkx(&mut self) -> DTRSLKX_W {
        DTRSLKX_W { w: self }
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    pub fn dtprsc(&mut self) -> DTPRSC_W {
        DTPRSC_W { w: self }
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    pub fn sdtrx(&mut self) -> SDTRX_W {
        SDTRX_W { w: self }
    }
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    pub fn dtrx(&mut self) -> DTRX_W {
        DTRX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Deadtime Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dter](index.html) module"]
pub struct DTER_SPEC;
impl crate::RegisterSpec for DTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dter::R](R) reader structure"]
impl crate::Readable for DTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dter::W](W) writer structure"]
impl crate::Writable for DTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTER to value 0"]
impl crate::Resettable for DTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
