#[doc = "Reader of register C1PR2"]
pub type R = crate::R<u32, super::C1PR2>;
#[doc = "Writer for register C1PR2"]
pub type W = crate::W<u32, super::C1PR2>;
#[doc = "Register C1PR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::C1PR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Configurable event inputs x+32 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR49_A {
    #[doc = "0: No trigger request occurred"]
    NOTPENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<PR49_A> for bool {
    #[inline(always)]
    fn from(variant: PR49_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PR49`"]
pub type PR49_R = crate::R<bool, PR49_A>;
impl PR49_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR49_A {
        match self.bits {
            false => PR49_A::NOTPENDING,
            true => PR49_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR49_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR49_A::PENDING
    }
}
#[doc = "Configurable event inputs x+32 Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR49_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<PR49_AW> for bool {
    #[inline(always)]
    fn from(variant: PR49_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PR49`"]
pub struct PR49_W<'a> {
    w: &'a mut W,
}
impl<'a> PR49_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR49_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR49_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Configurable event inputs x+32 Pending bit"]
pub type PR51_A = PR49_A;
#[doc = "Reader of field `PR51`"]
pub type PR51_R = crate::R<bool, PR49_A>;
#[doc = "Configurable event inputs x+32 Pending bit"]
pub type PR51_AW = PR49_AW;
#[doc = "Write proxy for field `PR51`"]
pub struct PR51_W<'a> {
    w: &'a mut W,
}
impl<'a> PR51_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR51_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR49_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr49(&self) -> PR49_R {
        PR49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr51(&self) -> PR51_R {
        PR51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr49(&mut self) -> PR49_W {
        PR49_W { w: self }
    }
    #[doc = "Bit 19 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr51(&mut self) -> PR51_W {
        PR51_W { w: self }
    }
}
