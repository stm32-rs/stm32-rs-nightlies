#[doc = "Reader of register C2PR3"]
pub type R = crate::R<u32, super::C2PR3>;
#[doc = "Writer for register C2PR3"]
pub type W = crate::W<u32, super::C2PR3>;
#[doc = "Register C2PR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::C2PR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CPU2 configurable event inputs x+64 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR82_A {
    #[doc = "0: No trigger request occurred"]
    NOTPENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<PR82_A> for bool {
    #[inline(always)]
    fn from(variant: PR82_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PR82`"]
pub type PR82_R = crate::R<bool, PR82_A>;
impl PR82_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR82_A {
        match self.bits {
            false => PR82_A::NOTPENDING,
            true => PR82_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR82_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR82_A::PENDING
    }
}
#[doc = "CPU2 configurable event inputs x+64 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR82_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<PR82_AW> for bool {
    #[inline(always)]
    fn from(variant: PR82_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PR82`"]
pub struct PR82_W<'a> {
    w: &'a mut W,
}
impl<'a> PR82_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR82_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR82_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "CPU2 configurable event inputs x+64 Pending bit"]
pub type PR84_A = PR82_A;
#[doc = "Reader of field `PR84`"]
pub type PR84_R = crate::R<bool, PR82_A>;
#[doc = "CPU2 configurable event inputs x+64 Pending bit"]
pub type PR84_AW = PR82_AW;
#[doc = "Write proxy for field `PR84`"]
pub struct PR84_W<'a> {
    w: &'a mut W,
}
impl<'a> PR84_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR84_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR82_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "CPU2 configurable event inputs x+64 Pending bit"]
pub type PR85_A = PR82_A;
#[doc = "Reader of field `PR85`"]
pub type PR85_R = crate::R<bool, PR82_A>;
#[doc = "CPU2 configurable event inputs x+64 Pending bit"]
pub type PR85_AW = PR82_AW;
#[doc = "Write proxy for field `PR85`"]
pub struct PR85_W<'a> {
    w: &'a mut W,
}
impl<'a> PR85_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR85_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR82_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "CPU2 configurable event inputs x+64 Pending bit"]
pub type PR86_A = PR82_A;
#[doc = "Reader of field `PR86`"]
pub type PR86_R = crate::R<bool, PR82_A>;
#[doc = "CPU2 configurable event inputs x+64 Pending bit"]
pub type PR86_AW = PR82_AW;
#[doc = "Write proxy for field `PR86`"]
pub struct PR86_W<'a> {
    w: &'a mut W,
}
impl<'a> PR86_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR86_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR82_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr82(&self) -> PR82_R {
        PR82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr84(&self) -> PR84_R {
        PR84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr85(&self) -> PR85_R {
        PR85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr86(&self) -> PR86_R {
        PR86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr82(&mut self) -> PR82_W {
        PR82_W { w: self }
    }
    #[doc = "Bit 20 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr84(&mut self) -> PR84_W {
        PR84_W { w: self }
    }
    #[doc = "Bit 21 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr85(&mut self) -> PR85_W {
        PR85_W { w: self }
    }
    #[doc = "Bit 22 - CPU2 configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr86(&mut self) -> PR86_W {
        PR86_W { w: self }
    }
}
