#[doc = "Reader of register FTSR1"]
pub type R = crate::R<u32, super::FTSR1>;
#[doc = "Writer for register FTSR1"]
pub type W = crate::W<u32, super::FTSR1>;
#[doc = "Register FTSR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FTSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FT`"]
pub type FT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FT`"]
pub struct FT_W<'a> {
    w: &'a mut W,
}
impl<'a> FT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
#[doc = "Reader of field `FT_31`"]
pub type FT_31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT_31`"]
pub struct FT_31_W<'a> {
    w: &'a mut W,
}
impl<'a> FT_31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 31 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft_31(&self) -> FT_31_R {
        FT_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft(&mut self) -> FT_W {
        FT_W { w: self }
    }
    #[doc = "Bit 31 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft_31(&mut self) -> FT_31_W {
        FT_31_W { w: self }
    }
}
