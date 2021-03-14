#[doc = "Reader of register EXTI_FPR3"]
pub type R = crate::R<u32, super::EXTI_FPR3>;
#[doc = "Writer for register EXTI_FPR3"]
pub type W = crate::W<u32, super::EXTI_FPR3>;
#[doc = "Register EXTI_FPR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_FPR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPIF65`"]
pub type FPIF65_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF65`"]
pub struct FPIF65_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF65_W<'a> {
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
#[doc = "Reader of field `FPIF66`"]
pub type FPIF66_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF66`"]
pub struct FPIF66_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF66_W<'a> {
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
#[doc = "Reader of field `FPIF68`"]
pub type FPIF68_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF68`"]
pub struct FPIF68_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF68_W<'a> {
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
#[doc = "Reader of field `FPIF73`"]
pub type FPIF73_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF73`"]
pub struct FPIF73_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF73_W<'a> {
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
#[doc = "Reader of field `FPIF74`"]
pub type FPIF74_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF74`"]
pub struct FPIF74_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF74_W<'a> {
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
    #[doc = "Bit 1 - FPIF65"]
    #[inline(always)]
    pub fn fpif65(&self) -> FPIF65_R {
        FPIF65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FPIF66"]
    #[inline(always)]
    pub fn fpif66(&self) -> FPIF66_R {
        FPIF66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FPIF68"]
    #[inline(always)]
    pub fn fpif68(&self) -> FPIF68_R {
        FPIF68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FPIF73"]
    #[inline(always)]
    pub fn fpif73(&self) -> FPIF73_R {
        FPIF73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FPIF74"]
    #[inline(always)]
    pub fn fpif74(&self) -> FPIF74_R {
        FPIF74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FPIF65"]
    #[inline(always)]
    pub fn fpif65(&mut self) -> FPIF65_W {
        FPIF65_W { w: self }
    }
    #[doc = "Bit 2 - FPIF66"]
    #[inline(always)]
    pub fn fpif66(&mut self) -> FPIF66_W {
        FPIF66_W { w: self }
    }
    #[doc = "Bit 4 - FPIF68"]
    #[inline(always)]
    pub fn fpif68(&mut self) -> FPIF68_W {
        FPIF68_W { w: self }
    }
    #[doc = "Bit 9 - FPIF73"]
    #[inline(always)]
    pub fn fpif73(&mut self) -> FPIF73_W {
        FPIF73_W { w: self }
    }
    #[doc = "Bit 10 - FPIF74"]
    #[inline(always)]
    pub fn fpif74(&mut self) -> FPIF74_W {
        FPIF74_W { w: self }
    }
}
