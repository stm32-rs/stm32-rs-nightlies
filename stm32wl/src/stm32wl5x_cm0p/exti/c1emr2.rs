#[doc = "Reader of register C1EMR2"]
pub type R = crate::R<u32, super::C1EMR2>;
#[doc = "Writer for register C1EMR2"]
pub type W = crate::W<u32, super::C1EMR2>;
#[doc = "Register C1EMR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::C1EMR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EM40`"]
pub type EM40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM40`"]
pub struct EM40_W<'a> {
    w: &'a mut W,
}
impl<'a> EM40_W<'a> {
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
#[doc = "Reader of field `EM41`"]
pub type EM41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM41`"]
pub struct EM41_W<'a> {
    w: &'a mut W,
}
impl<'a> EM41_W<'a> {
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
impl R {
    #[doc = "Bit 8 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em40(&mut self) -> EM40_W {
        EM40_W { w: self }
    }
    #[doc = "Bit 9 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em41(&mut self) -> EM41_W {
        EM41_W { w: self }
    }
}
