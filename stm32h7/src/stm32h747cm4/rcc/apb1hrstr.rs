#[doc = "Reader of register APB1HRSTR"]
pub type R = crate::R<u32, super::APB1HRSTR>;
#[doc = "Writer for register APB1HRSTR"]
pub type W = crate::W<u32, super::APB1HRSTR>;
#[doc = "Register APB1HRSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::APB1HRSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Recovery System reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<CRSRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRSRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRSRST`"]
pub type CRSRST_R = crate::R<bool, CRSRST_A>;
impl CRSRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CRSRST_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CRSRST_A::RESET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CRSRST_A::RESET
    }
}
#[doc = "Write proxy for field `CRSRST`"]
pub struct CRSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRSRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRSRST_A::RESET)
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
#[doc = "SWPMI block reset"]
pub type SWPRST_A = CRSRST_A;
#[doc = "Reader of field `SWPRST`"]
pub type SWPRST_R = crate::R<bool, CRSRST_A>;
#[doc = "Write proxy for field `SWPRST`"]
pub struct SWPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRSRST_A::RESET)
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
#[doc = "OPAMP block reset"]
pub type OPAMPRST_A = CRSRST_A;
#[doc = "Reader of field `OPAMPRST`"]
pub type OPAMPRST_R = crate::R<bool, CRSRST_A>;
#[doc = "Write proxy for field `OPAMPRST`"]
pub struct OPAMPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMPRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAMPRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRSRST_A::RESET)
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
#[doc = "MDIOS block reset"]
pub type MDIOSRST_A = CRSRST_A;
#[doc = "Reader of field `MDIOSRST`"]
pub type MDIOSRST_R = crate::R<bool, CRSRST_A>;
#[doc = "Write proxy for field `MDIOSRST`"]
pub struct MDIOSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIOSRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIOSRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRSRST_A::RESET)
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
#[doc = "FDCAN block reset"]
pub type FDCANRST_A = CRSRST_A;
#[doc = "Reader of field `FDCANRST`"]
pub type FDCANRST_R = crate::R<bool, CRSRST_A>;
#[doc = "Write proxy for field `FDCANRST`"]
pub struct FDCANRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRSRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Clock Recovery System reset"]
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWPMI block reset"]
    #[inline(always)]
    pub fn swprst(&self) -> SWPRST_R {
        SWPRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OPAMP block reset"]
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MDIOS block reset"]
    #[inline(always)]
    pub fn mdiosrst(&self) -> MDIOSRST_R {
        MDIOSRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FDCAN block reset"]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System reset"]
    #[inline(always)]
    pub fn crsrst(&mut self) -> CRSRST_W {
        CRSRST_W { w: self }
    }
    #[doc = "Bit 2 - SWPMI block reset"]
    #[inline(always)]
    pub fn swprst(&mut self) -> SWPRST_W {
        SWPRST_W { w: self }
    }
    #[doc = "Bit 4 - OPAMP block reset"]
    #[inline(always)]
    pub fn opamprst(&mut self) -> OPAMPRST_W {
        OPAMPRST_W { w: self }
    }
    #[doc = "Bit 5 - MDIOS block reset"]
    #[inline(always)]
    pub fn mdiosrst(&mut self) -> MDIOSRST_W {
        MDIOSRST_W { w: self }
    }
    #[doc = "Bit 8 - FDCAN block reset"]
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W {
        FDCANRST_W { w: self }
    }
}
