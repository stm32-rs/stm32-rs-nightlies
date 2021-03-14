#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOP_A {
    #[doc = "0: No EOP event occurred"]
    NOEVENT = 0,
    #[doc = "1: An EOP event occurred"]
    EVENT = 1,
}
impl From<EOP_A> for bool {
    #[inline(always)]
    fn from(variant: EOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOP`"]
pub type EOP_R = crate::R<bool, EOP_A>;
impl EOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOP_A {
        match self.bits {
            false => EOP_A::NOEVENT,
            true => EOP_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOP_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOP_A::EVENT
    }
}
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOP_AW {
    #[doc = "1: Reset EOP event"]
    RESET = 1,
}
impl From<EOP_AW> for bool {
    #[inline(always)]
    fn from(variant: EOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `EOP`"]
pub struct EOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset EOP event"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(EOP_AW::RESET)
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
#[doc = "Write protection error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPRTERR_A {
    #[doc = "0: No write protection error occurred"]
    NOERROR = 0,
    #[doc = "1: A write protection error occurred"]
    ERROR = 1,
}
impl From<WRPRTERR_A> for bool {
    #[inline(always)]
    fn from(variant: WRPRTERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WRPRTERR`"]
pub type WRPRTERR_R = crate::R<bool, WRPRTERR_A>;
impl WRPRTERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRPRTERR_A {
        match self.bits {
            false => WRPRTERR_A::NOERROR,
            true => WRPRTERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPRTERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPRTERR_A::ERROR
    }
}
#[doc = "Write protection error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPRTERR_AW {
    #[doc = "1: Reset write protection error"]
    RESET = 1,
}
impl From<WRPRTERR_AW> for bool {
    #[inline(always)]
    fn from(variant: WRPRTERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `WRPRTERR`"]
pub struct WRPRTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPRTERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRPRTERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset write protection error"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WRPRTERR_AW::RESET)
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
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGERR_A {
    #[doc = "0: No programming error occurred"]
    NOERROR = 0,
    #[doc = "1: A programming error occurred"]
    ERROR = 1,
}
impl From<PGERR_A> for bool {
    #[inline(always)]
    fn from(variant: PGERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PGERR`"]
pub type PGERR_R = crate::R<bool, PGERR_A>;
impl PGERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGERR_A {
        match self.bits {
            false => PGERR_A::NOERROR,
            true => PGERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGERR_A::ERROR
    }
}
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGERR_AW {
    #[doc = "1: Reset programming error"]
    RESET = 1,
}
impl From<PGERR_AW> for bool {
    #[inline(always)]
    fn from(variant: PGERR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PGERR`"]
pub struct PGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset programming error"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PGERR_AW::RESET)
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
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSY_A {
    #[doc = "0: No write/erase operation is in progress"]
    INACTIVE = 0,
    #[doc = "1: No write/erase operation is in progress"]
    ACTIVE = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BSY`"]
pub type BSY_R = crate::R<bool, BSY_A>;
impl BSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::INACTIVE,
            true => BSY_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BSY_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSY_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrprterr(&self) -> WRPRTERR_R {
        WRPRTERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Programming error"]
    #[inline(always)]
    pub fn pgerr(&self) -> PGERR_R {
        PGERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W {
        EOP_W { w: self }
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrprterr(&mut self) -> WRPRTERR_W {
        WRPRTERR_W { w: self }
    }
    #[doc = "Bit 2 - Programming error"]
    #[inline(always)]
    pub fn pgerr(&mut self) -> PGERR_W {
        PGERR_W { w: self }
    }
}
