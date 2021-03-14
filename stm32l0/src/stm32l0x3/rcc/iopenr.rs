#[doc = "Reader of register IOPENR"]
pub type R = crate::R<u32, super::IOPENR>;
#[doc = "Writer for register IOPENR"]
pub type W = crate::W<u32, super::IOPENR>;
#[doc = "Register IOPENR `reset()`'s with value 0"]
impl crate::ResetValue for super::IOPENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I/O port H clock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOPHEN_A {
    #[doc = "0: Port clock disabled"]
    DISABLED = 0,
    #[doc = "1: Port clock enabled"]
    ENABLED = 1,
}
impl From<IOPHEN_A> for bool {
    #[inline(always)]
    fn from(variant: IOPHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IOPHEN`"]
pub type IOPHEN_R = crate::R<bool, IOPHEN_A>;
impl IOPHEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOPHEN_A {
        match self.bits {
            false => IOPHEN_A::DISABLED,
            true => IOPHEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOPHEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOPHEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `IOPHEN`"]
pub struct IOPHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPHEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "I/O port D clock enable bit"]
pub type IOPDEN_A = IOPHEN_A;
#[doc = "Reader of field `IOPDEN`"]
pub type IOPDEN_R = crate::R<bool, IOPHEN_A>;
#[doc = "Write proxy for field `IOPDEN`"]
pub struct IOPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::ENABLED)
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
#[doc = "IO port A clock enable bit"]
pub type IOPCEN_A = IOPHEN_A;
#[doc = "Reader of field `IOPCEN`"]
pub type IOPCEN_R = crate::R<bool, IOPHEN_A>;
#[doc = "Write proxy for field `IOPCEN`"]
pub struct IOPCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::ENABLED)
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
#[doc = "IO port B clock enable bit"]
pub type IOPBEN_A = IOPHEN_A;
#[doc = "Reader of field `IOPBEN`"]
pub type IOPBEN_R = crate::R<bool, IOPHEN_A>;
#[doc = "Write proxy for field `IOPBEN`"]
pub struct IOPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPBEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::ENABLED)
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
#[doc = "IO port A clock enable bit"]
pub type IOPAEN_A = IOPHEN_A;
#[doc = "Reader of field `IOPAEN`"]
pub type IOPAEN_R = crate::R<bool, IOPHEN_A>;
#[doc = "Write proxy for field `IOPAEN`"]
pub struct IOPAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::ENABLED)
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
#[doc = "I/O port E clock enable bit"]
pub type IOPEEN_A = IOPHEN_A;
#[doc = "Reader of field `IOPEEN`"]
pub type IOPEEN_R = crate::R<bool, IOPHEN_A>;
#[doc = "Write proxy for field `IOPEEN`"]
pub struct IOPEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOPEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::DISABLED)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPHEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 7 - I/O port H clock enable bit"]
    #[inline(always)]
    pub fn iophen(&self) -> IOPHEN_R {
        IOPHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable bit"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IO port A clock enable bit"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable bit"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IO port A clock enable bit"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - I/O port E clock enable bit"]
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - I/O port H clock enable bit"]
    #[inline(always)]
    pub fn iophen(&mut self) -> IOPHEN_W {
        IOPHEN_W { w: self }
    }
    #[doc = "Bit 3 - I/O port D clock enable bit"]
    #[inline(always)]
    pub fn iopden(&mut self) -> IOPDEN_W {
        IOPDEN_W { w: self }
    }
    #[doc = "Bit 2 - IO port A clock enable bit"]
    #[inline(always)]
    pub fn iopcen(&mut self) -> IOPCEN_W {
        IOPCEN_W { w: self }
    }
    #[doc = "Bit 1 - IO port B clock enable bit"]
    #[inline(always)]
    pub fn iopben(&mut self) -> IOPBEN_W {
        IOPBEN_W { w: self }
    }
    #[doc = "Bit 0 - IO port A clock enable bit"]
    #[inline(always)]
    pub fn iopaen(&mut self) -> IOPAEN_W {
        IOPAEN_W { w: self }
    }
    #[doc = "Bit 4 - I/O port E clock enable bit"]
    #[inline(always)]
    pub fn iopeen(&mut self) -> IOPEEN_W {
        IOPEEN_W { w: self }
    }
}
