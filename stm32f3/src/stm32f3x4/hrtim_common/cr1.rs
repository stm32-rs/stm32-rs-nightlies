#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC Trigger 4 Update Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AD4USRC_A {
    #[doc = "0: ADC trigger update from master timer"]
    MASTER = 0,
    #[doc = "1: ADC trigger update from timer A"]
    TIMERA = 1,
    #[doc = "2: ADC trigger update from timer B"]
    TIMERB = 2,
    #[doc = "3: ADC trigger update from timer C"]
    TIMERC = 3,
    #[doc = "4: ADC trigger update from timer D"]
    TIMERD = 4,
    #[doc = "5: ADC trigger update from timer E"]
    TIMERE = 5,
}
impl From<AD4USRC_A> for u8 {
    #[inline(always)]
    fn from(variant: AD4USRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AD4USRC`"]
pub type AD4USRC_R = crate::R<u8, AD4USRC_A>;
impl AD4USRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AD4USRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AD4USRC_A::MASTER),
            1 => Val(AD4USRC_A::TIMERA),
            2 => Val(AD4USRC_A::TIMERB),
            3 => Val(AD4USRC_A::TIMERC),
            4 => Val(AD4USRC_A::TIMERD),
            5 => Val(AD4USRC_A::TIMERE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == AD4USRC_A::MASTER
    }
    #[doc = "Checks if the value of the field is `TIMERA`"]
    #[inline(always)]
    pub fn is_timer_a(&self) -> bool {
        *self == AD4USRC_A::TIMERA
    }
    #[doc = "Checks if the value of the field is `TIMERB`"]
    #[inline(always)]
    pub fn is_timer_b(&self) -> bool {
        *self == AD4USRC_A::TIMERB
    }
    #[doc = "Checks if the value of the field is `TIMERC`"]
    #[inline(always)]
    pub fn is_timer_c(&self) -> bool {
        *self == AD4USRC_A::TIMERC
    }
    #[doc = "Checks if the value of the field is `TIMERD`"]
    #[inline(always)]
    pub fn is_timer_d(&self) -> bool {
        *self == AD4USRC_A::TIMERD
    }
    #[doc = "Checks if the value of the field is `TIMERE`"]
    #[inline(always)]
    pub fn is_timer_e(&self) -> bool {
        *self == AD4USRC_A::TIMERE
    }
}
#[doc = "Write proxy for field `AD4USRC`"]
pub struct AD4USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD4USRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD4USRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC trigger update from master timer"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(AD4USRC_A::MASTER)
    }
    #[doc = "ADC trigger update from timer A"]
    #[inline(always)]
    pub fn timer_a(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERA)
    }
    #[doc = "ADC trigger update from timer B"]
    #[inline(always)]
    pub fn timer_b(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERB)
    }
    #[doc = "ADC trigger update from timer C"]
    #[inline(always)]
    pub fn timer_c(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERC)
    }
    #[doc = "ADC trigger update from timer D"]
    #[inline(always)]
    pub fn timer_d(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERD)
    }
    #[doc = "ADC trigger update from timer E"]
    #[inline(always)]
    pub fn timer_e(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "ADC Trigger 3 Update Source"]
pub type AD3USRC_A = AD4USRC_A;
#[doc = "Reader of field `AD3USRC`"]
pub type AD3USRC_R = crate::R<u8, AD4USRC_A>;
#[doc = "Write proxy for field `AD3USRC`"]
pub struct AD3USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD3USRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD3USRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC trigger update from master timer"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(AD4USRC_A::MASTER)
    }
    #[doc = "ADC trigger update from timer A"]
    #[inline(always)]
    pub fn timer_a(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERA)
    }
    #[doc = "ADC trigger update from timer B"]
    #[inline(always)]
    pub fn timer_b(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERB)
    }
    #[doc = "ADC trigger update from timer C"]
    #[inline(always)]
    pub fn timer_c(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERC)
    }
    #[doc = "ADC trigger update from timer D"]
    #[inline(always)]
    pub fn timer_d(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERD)
    }
    #[doc = "ADC trigger update from timer E"]
    #[inline(always)]
    pub fn timer_e(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "ADC Trigger 2 Update Source"]
pub type AD2USRC_A = AD4USRC_A;
#[doc = "Reader of field `AD2USRC`"]
pub type AD2USRC_R = crate::R<u8, AD4USRC_A>;
#[doc = "Write proxy for field `AD2USRC`"]
pub struct AD2USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2USRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD2USRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC trigger update from master timer"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(AD4USRC_A::MASTER)
    }
    #[doc = "ADC trigger update from timer A"]
    #[inline(always)]
    pub fn timer_a(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERA)
    }
    #[doc = "ADC trigger update from timer B"]
    #[inline(always)]
    pub fn timer_b(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERB)
    }
    #[doc = "ADC trigger update from timer C"]
    #[inline(always)]
    pub fn timer_c(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERC)
    }
    #[doc = "ADC trigger update from timer D"]
    #[inline(always)]
    pub fn timer_d(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERD)
    }
    #[doc = "ADC trigger update from timer E"]
    #[inline(always)]
    pub fn timer_e(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "ADC Trigger 1 Update Source"]
pub type AD1USRC_A = AD4USRC_A;
#[doc = "Reader of field `AD1USRC`"]
pub type AD1USRC_R = crate::R<u8, AD4USRC_A>;
#[doc = "Write proxy for field `AD1USRC`"]
pub struct AD1USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1USRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AD1USRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC trigger update from master timer"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(AD4USRC_A::MASTER)
    }
    #[doc = "ADC trigger update from timer A"]
    #[inline(always)]
    pub fn timer_a(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERA)
    }
    #[doc = "ADC trigger update from timer B"]
    #[inline(always)]
    pub fn timer_b(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERB)
    }
    #[doc = "ADC trigger update from timer C"]
    #[inline(always)]
    pub fn timer_c(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERC)
    }
    #[doc = "ADC trigger update from timer D"]
    #[inline(always)]
    pub fn timer_d(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERD)
    }
    #[doc = "ADC trigger update from timer E"]
    #[inline(always)]
    pub fn timer_e(self) -> &'a mut W {
        self.variant(AD4USRC_A::TIMERE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Timer E Update Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEUDIS_A {
    #[doc = "0: Timer update enabled"]
    ENABLED = 0,
    #[doc = "1: Timer update disabled"]
    DISABLED = 1,
}
impl From<TEUDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TEUDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEUDIS`"]
pub type TEUDIS_R = crate::R<bool, TEUDIS_A>;
impl TEUDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEUDIS_A {
        match self.bits {
            false => TEUDIS_A::ENABLED,
            true => TEUDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEUDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEUDIS_A::DISABLED
    }
}
#[doc = "Write proxy for field `TEUDIS`"]
pub struct TEUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TEUDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEUDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer update enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::ENABLED)
    }
    #[doc = "Timer update disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Timer D Update Disable"]
pub type TDUDIS_A = TEUDIS_A;
#[doc = "Reader of field `TDUDIS`"]
pub type TDUDIS_R = crate::R<bool, TEUDIS_A>;
#[doc = "Write proxy for field `TDUDIS`"]
pub struct TDUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TDUDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDUDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer update enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::ENABLED)
    }
    #[doc = "Timer update disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Timer C Update Disable"]
pub type TCUDIS_A = TEUDIS_A;
#[doc = "Reader of field `TCUDIS`"]
pub type TCUDIS_R = crate::R<bool, TEUDIS_A>;
#[doc = "Write proxy for field `TCUDIS`"]
pub struct TCUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCUDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCUDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer update enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::ENABLED)
    }
    #[doc = "Timer update disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Timer B Update Disable"]
pub type TBUDIS_A = TEUDIS_A;
#[doc = "Reader of field `TBUDIS`"]
pub type TBUDIS_R = crate::R<bool, TEUDIS_A>;
#[doc = "Write proxy for field `TBUDIS`"]
pub struct TBUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBUDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBUDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer update enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::ENABLED)
    }
    #[doc = "Timer update disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Timer A Update Disable"]
pub type TAUDIS_A = TEUDIS_A;
#[doc = "Reader of field `TAUDIS`"]
pub type TAUDIS_R = crate::R<bool, TEUDIS_A>;
#[doc = "Write proxy for field `TAUDIS`"]
pub struct TAUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAUDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAUDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer update enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::ENABLED)
    }
    #[doc = "Timer update disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Master Update Disable"]
pub type MUDIS_A = TEUDIS_A;
#[doc = "Reader of field `MUDIS`"]
pub type MUDIS_R = crate::R<bool, TEUDIS_A>;
#[doc = "Write proxy for field `MUDIS`"]
pub struct MUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer update enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::ENABLED)
    }
    #[doc = "Timer update disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEUDIS_A::DISABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline(always)]
    pub fn ad4usrc(&self) -> AD4USRC_R {
        AD4USRC_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline(always)]
    pub fn ad3usrc(&self) -> AD3USRC_R {
        AD3USRC_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline(always)]
    pub fn ad2usrc(&self) -> AD2USRC_R {
        AD2USRC_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline(always)]
    pub fn ad1usrc(&self) -> AD1USRC_R {
        AD1USRC_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline(always)]
    pub fn teudis(&self) -> TEUDIS_R {
        TEUDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline(always)]
    pub fn tdudis(&self) -> TDUDIS_R {
        TDUDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline(always)]
    pub fn tcudis(&self) -> TCUDIS_R {
        TCUDIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline(always)]
    pub fn tbudis(&self) -> TBUDIS_R {
        TBUDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline(always)]
    pub fn taudis(&self) -> TAUDIS_R {
        TAUDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline(always)]
    pub fn mudis(&self) -> MUDIS_R {
        MUDIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline(always)]
    pub fn ad4usrc(&mut self) -> AD4USRC_W {
        AD4USRC_W { w: self }
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline(always)]
    pub fn ad3usrc(&mut self) -> AD3USRC_W {
        AD3USRC_W { w: self }
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline(always)]
    pub fn ad2usrc(&mut self) -> AD2USRC_W {
        AD2USRC_W { w: self }
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline(always)]
    pub fn ad1usrc(&mut self) -> AD1USRC_W {
        AD1USRC_W { w: self }
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline(always)]
    pub fn teudis(&mut self) -> TEUDIS_W {
        TEUDIS_W { w: self }
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline(always)]
    pub fn tdudis(&mut self) -> TDUDIS_W {
        TDUDIS_W { w: self }
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline(always)]
    pub fn tcudis(&mut self) -> TCUDIS_W {
        TCUDIS_W { w: self }
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline(always)]
    pub fn tbudis(&mut self) -> TBUDIS_W {
        TBUDIS_W { w: self }
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline(always)]
    pub fn taudis(&mut self) -> TAUDIS_W {
        TAUDIS_W { w: self }
    }
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline(always)]
    pub fn mudis(&mut self) -> MUDIS_W {
        MUDIS_W { w: self }
    }
}
