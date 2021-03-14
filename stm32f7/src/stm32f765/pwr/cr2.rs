#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Writer for register CR2"]
pub type W = crate::W<u32, super::CR2>;
#[doc = "Register CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CWUPF1`"]
pub type CWUPF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CWUPF2`"]
pub type CWUPF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CWUPF3`"]
pub type CWUPF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CWUPF4`"]
pub type CWUPF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CWUPF5`"]
pub type CWUPF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CWUPF6`"]
pub type CWUPF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUPP1`"]
pub type WUPP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPP1`"]
pub struct WUPP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP1_W<'a> {
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
#[doc = "Reader of field `WUPP2`"]
pub type WUPP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPP2`"]
pub struct WUPP2_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP2_W<'a> {
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
#[doc = "Reader of field `WUPP3`"]
pub type WUPP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPP3`"]
pub struct WUPP3_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `WUPP4`"]
pub type WUPP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPP4`"]
pub struct WUPP4_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `WUPP5`"]
pub type WUPP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPP5`"]
pub struct WUPP5_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `WUPP6`"]
pub type WUPP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUPP6`"]
pub struct WUPP6_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPP6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clear Wakeup Pin flag for PA0"]
    #[inline(always)]
    pub fn cwupf1(&self) -> CWUPF1_R {
        CWUPF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear Wakeup Pin flag for PA2"]
    #[inline(always)]
    pub fn cwupf2(&self) -> CWUPF2_R {
        CWUPF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear Wakeup Pin flag for PC1"]
    #[inline(always)]
    pub fn cwupf3(&self) -> CWUPF3_R {
        CWUPF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear Wakeup Pin flag for PC13"]
    #[inline(always)]
    pub fn cwupf4(&self) -> CWUPF4_R {
        CWUPF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear Wakeup Pin flag for PI8"]
    #[inline(always)]
    pub fn cwupf5(&self) -> CWUPF5_R {
        CWUPF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear Wakeup Pin flag for PI11"]
    #[inline(always)]
    pub fn cwupf6(&self) -> CWUPF6_R {
        CWUPF6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wakeup pin polarity bit for PA0"]
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for PA2"]
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for PC1"]
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for PC13"]
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for PI8"]
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for PI11"]
    #[inline(always)]
    pub fn wupp6(&self) -> WUPP6_R {
        WUPP6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Wakeup pin polarity bit for PA0"]
    #[inline(always)]
    pub fn wupp1(&mut self) -> WUPP1_W {
        WUPP1_W { w: self }
    }
    #[doc = "Bit 9 - Wakeup pin polarity bit for PA2"]
    #[inline(always)]
    pub fn wupp2(&mut self) -> WUPP2_W {
        WUPP2_W { w: self }
    }
    #[doc = "Bit 10 - Wakeup pin polarity bit for PC1"]
    #[inline(always)]
    pub fn wupp3(&mut self) -> WUPP3_W {
        WUPP3_W { w: self }
    }
    #[doc = "Bit 11 - Wakeup pin polarity bit for PC13"]
    #[inline(always)]
    pub fn wupp4(&mut self) -> WUPP4_W {
        WUPP4_W { w: self }
    }
    #[doc = "Bit 12 - Wakeup pin polarity bit for PI8"]
    #[inline(always)]
    pub fn wupp5(&mut self) -> WUPP5_W {
        WUPP5_W { w: self }
    }
    #[doc = "Bit 13 - Wakeup pin polarity bit for PI11"]
    #[inline(always)]
    pub fn wupp6(&mut self) -> WUPP6_W {
        WUPP6_W { w: self }
    }
}
