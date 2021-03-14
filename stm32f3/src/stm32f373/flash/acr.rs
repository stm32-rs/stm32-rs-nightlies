#[doc = "Reader of register ACR"]
pub type R = crate::R<u32, super::ACR>;
#[doc = "Writer for register ACR"]
pub type W = crate::W<u32, super::ACR>;
#[doc = "Register ACR `reset()`'s with value 0x30"]
impl crate::ResetValue for super::ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
    }
}
#[doc = "LATENCY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LATENCY_A {
    #[doc = "0: 0 wait states, if 0 < HCLK <= 24 MHz"]
    WS0 = 0,
    #[doc = "1: 1 wait state, if 24 < HCLK <= 48 MHz"]
    WS1 = 1,
    #[doc = "2: 2 wait states, if 48 < HCLK <= 72 MHz"]
    WS2 = 2,
}
impl From<LATENCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LATENCY`"]
pub type LATENCY_R = crate::R<u8, LATENCY_A>;
impl LATENCY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LATENCY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LATENCY_A::WS0),
            1 => Val(LATENCY_A::WS1),
            2 => Val(LATENCY_A::WS2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WS0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY_A::WS0
    }
    #[doc = "Checks if the value of the field is `WS1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY_A::WS1
    }
    #[doc = "Checks if the value of the field is `WS2`"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY_A::WS2
    }
}
#[doc = "Write proxy for field `LATENCY`"]
pub struct LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> LATENCY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LATENCY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 wait states, if 0 < HCLK <= 24 MHz"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::WS0)
    }
    #[doc = "1 wait state, if 24 < HCLK <= 48 MHz"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::WS1)
    }
    #[doc = "2 wait states, if 48 < HCLK <= 72 MHz"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(LATENCY_A::WS2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "PRFTBE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTBE_A {
    #[doc = "0: Prefetch is disabled"]
    DISABLED = 0,
    #[doc = "1: Prefetch is enabled"]
    ENABLED = 1,
}
impl From<PRFTBE_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRFTBE`"]
pub type PRFTBE_R = crate::R<bool, PRFTBE_A>;
impl PRFTBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTBE_A {
        match self.bits {
            false => PRFTBE_A::DISABLED,
            true => PRFTBE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTBE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTBE_A::ENABLED
    }
}
#[doc = "Write proxy for field `PRFTBE`"]
pub struct PRFTBE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRFTBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRFTBE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTBE_A::DISABLED)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTBE_A::ENABLED)
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
#[doc = "PRFTBS\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTBS_A {
    #[doc = "0: Prefetch buffer is disabled"]
    DISABLED = 0,
    #[doc = "1: Prefetch buffer is enabled"]
    ENABLED = 1,
}
impl From<PRFTBS_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRFTBS`"]
pub type PRFTBS_R = crate::R<bool, PRFTBS_A>;
impl PRFTBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTBS_A {
        match self.bits {
            false => PRFTBS_A::DISABLED,
            true => PRFTBS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTBS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTBS_A::ENABLED
    }
}
impl R {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline(always)]
    pub fn prftbe(&self) -> PRFTBE_R {
        PRFTBE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRFTBS"]
    #[inline(always)]
    pub fn prftbs(&self) -> PRFTBS_R {
        PRFTBS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W {
        LATENCY_W { w: self }
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline(always)]
    pub fn prftbe(&mut self) -> PRFTBE_W {
        PRFTBE_W { w: self }
    }
}
