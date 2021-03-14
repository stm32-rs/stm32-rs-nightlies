#[doc = "Reader of register EXTI_RPR3"]
pub type R = crate::R<u32, super::EXTI_RPR3>;
#[doc = "Writer for register EXTI_RPR3"]
pub type W = crate::W<u32, super::EXTI_RPR3>;
#[doc = "Register EXTI_RPR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_RPR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPIF65`"]
pub type RPIF65_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF65`"]
pub struct RPIF65_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF65_W<'a> {
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
#[doc = "Reader of field `RPIF66`"]
pub type RPIF66_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF66`"]
pub struct RPIF66_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF66_W<'a> {
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
#[doc = "Reader of field `RPIF68`"]
pub type RPIF68_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF68`"]
pub struct RPIF68_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF68_W<'a> {
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
#[doc = "Reader of field `RPIF73`"]
pub type RPIF73_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF73`"]
pub struct RPIF73_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF73_W<'a> {
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
#[doc = "Reader of field `RPIF74`"]
pub type RPIF74_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPIF74`"]
pub struct RPIF74_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF74_W<'a> {
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
    #[doc = "Bit 1 - RPIF65"]
    #[inline(always)]
    pub fn rpif65(&self) -> RPIF65_R {
        RPIF65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RPIF66"]
    #[inline(always)]
    pub fn rpif66(&self) -> RPIF66_R {
        RPIF66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RPIF68"]
    #[inline(always)]
    pub fn rpif68(&self) -> RPIF68_R {
        RPIF68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RPIF73"]
    #[inline(always)]
    pub fn rpif73(&self) -> RPIF73_R {
        RPIF73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RPIF74"]
    #[inline(always)]
    pub fn rpif74(&self) -> RPIF74_R {
        RPIF74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RPIF65"]
    #[inline(always)]
    pub fn rpif65(&mut self) -> RPIF65_W {
        RPIF65_W { w: self }
    }
    #[doc = "Bit 2 - RPIF66"]
    #[inline(always)]
    pub fn rpif66(&mut self) -> RPIF66_W {
        RPIF66_W { w: self }
    }
    #[doc = "Bit 4 - RPIF68"]
    #[inline(always)]
    pub fn rpif68(&mut self) -> RPIF68_W {
        RPIF68_W { w: self }
    }
    #[doc = "Bit 9 - RPIF73"]
    #[inline(always)]
    pub fn rpif73(&mut self) -> RPIF73_W {
        RPIF73_W { w: self }
    }
    #[doc = "Bit 10 - RPIF74"]
    #[inline(always)]
    pub fn rpif74(&mut self) -> RPIF74_W {
        RPIF74_W { w: self }
    }
}
