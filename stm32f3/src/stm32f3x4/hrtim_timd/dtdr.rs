#[doc = "Reader of register DTDR"]
pub type R = crate::R<u32, super::DTDR>;
#[doc = "Writer for register DTDR"]
pub type W = crate::W<u32, super::DTDR>;
#[doc = "Register DTDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DTDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `DTFLKx`"]
pub type DTFLKX_R = crate::R<bool, DTFLKX_A>;
impl DTFLKX_R {
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
        *self == DTFLKX_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFLKX_A::LOCKED
    }
}
#[doc = "Write proxy for field `DTFLKx`"]
pub struct DTFLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFLKX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTFLKX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
#[doc = "Reader of field `DTFSLKx`"]
pub type DTFSLKX_R = crate::R<bool, DTFSLKX_A>;
impl DTFSLKX_R {
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
        *self == DTFSLKX_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFSLKX_A::LOCKED
    }
}
#[doc = "Write proxy for field `DTFSLKx`"]
pub struct DTFSLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFSLKX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTFSLKX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
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
#[doc = "Reader of field `SDTFx`"]
pub type SDTFX_R = crate::R<bool, SDTFX_A>;
impl SDTFX_R {
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
        *self == SDTFX_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTFX_A::NEGATIVE
    }
}
#[doc = "Write proxy for field `SDTFx`"]
pub struct SDTFX_W<'a> {
    w: &'a mut W,
}
impl<'a> SDTFX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDTFX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DTFx`"]
pub type DTFX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DTFx`"]
pub struct DTFX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
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
#[doc = "Reader of field `DTRLKx`"]
pub type DTRLKX_R = crate::R<bool, DTRLKX_A>;
impl DTRLKX_R {
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
        *self == DTRLKX_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRLKX_A::LOCKED
    }
}
#[doc = "Write proxy for field `DTRLKx`"]
pub struct DTRLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRLKX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTRLKX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
#[doc = "Reader of field `DTRSLKx`"]
pub type DTRSLKX_R = crate::R<bool, DTRSLKX_A>;
impl DTRSLKX_R {
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
        *self == DTRSLKX_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRSLKX_A::LOCKED
    }
}
#[doc = "Write proxy for field `DTRSLKx`"]
pub struct DTRSLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRSLKX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTRSLKX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DTPRSC`"]
pub type DTPRSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTPRSC`"]
pub struct DTPRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
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
#[doc = "Reader of field `SDTRx`"]
pub type SDTRX_R = crate::R<bool, SDTRX_A>;
impl SDTRX_R {
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
        *self == SDTRX_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTRX_A::NEGATIVE
    }
}
#[doc = "Write proxy for field `SDTRx`"]
pub struct SDTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> SDTRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDTRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DTRx`"]
pub type DTRX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DTRx`"]
pub struct DTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
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
}
