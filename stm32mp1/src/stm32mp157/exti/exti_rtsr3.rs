#[doc = "Reader of register EXTI_RTSR3"]
pub type R = crate::R<u32, super::EXTI_RTSR3>;
#[doc = "Writer for register EXTI_RTSR3"]
pub type W = crate::W<u32, super::EXTI_RTSR3>;
#[doc = "Register EXTI_RTSR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_RTSR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RT65`"]
pub type RT65_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT65`"]
pub struct RT65_W<'a> {
    w: &'a mut W,
}
impl<'a> RT65_W<'a> {
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
#[doc = "Reader of field `RT66`"]
pub type RT66_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT66`"]
pub struct RT66_W<'a> {
    w: &'a mut W,
}
impl<'a> RT66_W<'a> {
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
#[doc = "Reader of field `RT68`"]
pub type RT68_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT68`"]
pub struct RT68_W<'a> {
    w: &'a mut W,
}
impl<'a> RT68_W<'a> {
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
#[doc = "Reader of field `RT73`"]
pub type RT73_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT73`"]
pub struct RT73_W<'a> {
    w: &'a mut W,
}
impl<'a> RT73_W<'a> {
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
#[doc = "Reader of field `RT74`"]
pub type RT74_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RT74`"]
pub struct RT74_W<'a> {
    w: &'a mut W,
}
impl<'a> RT74_W<'a> {
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
impl R {
    #[doc = "Bit 1 - RT65"]
    #[inline(always)]
    pub fn rt65(&self) -> RT65_R {
        RT65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RT66"]
    #[inline(always)]
    pub fn rt66(&self) -> RT66_R {
        RT66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RT68"]
    #[inline(always)]
    pub fn rt68(&self) -> RT68_R {
        RT68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RT73"]
    #[inline(always)]
    pub fn rt73(&self) -> RT73_R {
        RT73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RT74"]
    #[inline(always)]
    pub fn rt74(&self) -> RT74_R {
        RT74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RT65"]
    #[inline(always)]
    pub fn rt65(&mut self) -> RT65_W {
        RT65_W { w: self }
    }
    #[doc = "Bit 2 - RT66"]
    #[inline(always)]
    pub fn rt66(&mut self) -> RT66_W {
        RT66_W { w: self }
    }
    #[doc = "Bit 4 - RT68"]
    #[inline(always)]
    pub fn rt68(&mut self) -> RT68_W {
        RT68_W { w: self }
    }
    #[doc = "Bit 9 - RT73"]
    #[inline(always)]
    pub fn rt73(&mut self) -> RT73_W {
        RT73_W { w: self }
    }
    #[doc = "Bit 10 - RT74"]
    #[inline(always)]
    pub fn rt74(&mut self) -> RT74_W {
        RT74_W { w: self }
    }
}
