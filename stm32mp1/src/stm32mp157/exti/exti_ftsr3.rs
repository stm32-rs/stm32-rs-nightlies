#[doc = "Reader of register EXTI_FTSR3"]
pub type R = crate::R<u32, super::EXTI_FTSR3>;
#[doc = "Writer for register EXTI_FTSR3"]
pub type W = crate::W<u32, super::EXTI_FTSR3>;
#[doc = "Register EXTI_FTSR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_FTSR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FT65`"]
pub type FT65_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT65`"]
pub struct FT65_W<'a> {
    w: &'a mut W,
}
impl<'a> FT65_W<'a> {
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
#[doc = "Reader of field `FT66`"]
pub type FT66_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT66`"]
pub struct FT66_W<'a> {
    w: &'a mut W,
}
impl<'a> FT66_W<'a> {
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
#[doc = "Reader of field `FT68`"]
pub type FT68_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT68`"]
pub struct FT68_W<'a> {
    w: &'a mut W,
}
impl<'a> FT68_W<'a> {
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
#[doc = "Reader of field `FT73`"]
pub type FT73_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT73`"]
pub struct FT73_W<'a> {
    w: &'a mut W,
}
impl<'a> FT73_W<'a> {
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
#[doc = "Reader of field `FT74`"]
pub type FT74_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT74`"]
pub struct FT74_W<'a> {
    w: &'a mut W,
}
impl<'a> FT74_W<'a> {
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
    #[doc = "Bit 1 - FT65"]
    #[inline(always)]
    pub fn ft65(&self) -> FT65_R {
        FT65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FT66"]
    #[inline(always)]
    pub fn ft66(&self) -> FT66_R {
        FT66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FT68"]
    #[inline(always)]
    pub fn ft68(&self) -> FT68_R {
        FT68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FT73"]
    #[inline(always)]
    pub fn ft73(&self) -> FT73_R {
        FT73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FT74"]
    #[inline(always)]
    pub fn ft74(&self) -> FT74_R {
        FT74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FT65"]
    #[inline(always)]
    pub fn ft65(&mut self) -> FT65_W {
        FT65_W { w: self }
    }
    #[doc = "Bit 2 - FT66"]
    #[inline(always)]
    pub fn ft66(&mut self) -> FT66_W {
        FT66_W { w: self }
    }
    #[doc = "Bit 4 - FT68"]
    #[inline(always)]
    pub fn ft68(&mut self) -> FT68_W {
        FT68_W { w: self }
    }
    #[doc = "Bit 9 - FT73"]
    #[inline(always)]
    pub fn ft73(&mut self) -> FT73_W {
        FT73_W { w: self }
    }
    #[doc = "Bit 10 - FT74"]
    #[inline(always)]
    pub fn ft74(&mut self) -> FT74_W {
        FT74_W { w: self }
    }
}
