#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Tamper pin enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPE_A {
    #[doc = "0: The TAMPER pin is free for general purpose I/O"]
    GENERAL = 0,
    #[doc = "1: Tamper alternate I/O function is activated"]
    ALTERNATE = 1,
}
impl From<TPE_A> for bool {
    #[inline(always)]
    fn from(variant: TPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPE`"]
pub type TPE_R = crate::R<bool, TPE_A>;
impl TPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPE_A {
        match self.bits {
            false => TPE_A::GENERAL,
            true => TPE_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `GENERAL`"]
    #[inline(always)]
    pub fn is_general(&self) -> bool {
        *self == TPE_A::GENERAL
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TPE_A::ALTERNATE
    }
}
#[doc = "Write proxy for field `TPE`"]
pub struct TPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The TAMPER pin is free for general purpose I/O"]
    #[inline(always)]
    pub fn general(self) -> &'a mut W {
        self.variant(TPE_A::GENERAL)
    }
    #[doc = "Tamper alternate I/O function is activated"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(TPE_A::ALTERNATE)
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
#[doc = "Tamper pin active level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPAL_A {
    #[doc = "0: A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    HIGH = 0,
    #[doc = "1: A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    LOW = 1,
}
impl From<TPAL_A> for bool {
    #[inline(always)]
    fn from(variant: TPAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPAL`"]
pub type TPAL_R = crate::R<bool, TPAL_A>;
impl TPAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPAL_A {
        match self.bits {
            false => TPAL_A::HIGH,
            true => TPAL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TPAL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TPAL_A::LOW
    }
}
#[doc = "Write proxy for field `TPAL`"]
pub struct TPAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TPAL_A::HIGH)
    }
    #[doc = "A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TPAL_A::LOW)
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
impl R {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpe(&mut self) -> TPE_W {
        TPE_W { w: self }
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    pub fn tpal(&mut self) -> TPAL_W {
        TPAL_W { w: self }
    }
}
