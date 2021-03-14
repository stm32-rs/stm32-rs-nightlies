#[doc = "Reader of register TIM8_AF2"]
pub type R = crate::R<u32, super::TIM8_AF2>;
#[doc = "Writer for register TIM8_AF2"]
pub type W = crate::W<u32, super::TIM8_AF2>;
#[doc = "Register TIM8_AF2 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::TIM8_AF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `BK2INE`"]
pub type BK2INE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BK2INE`"]
pub struct BK2INE_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `BK2DF1BK3E`"]
pub type BK2DF1BK3E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BK2DF1BK3E`"]
pub struct BK2DF1BK3E_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2DF1BK3E_W<'a> {
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
#[doc = "Reader of field `BK2INP`"]
pub type BK2INP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BK2INP`"]
pub struct BK2INP_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INP_W<'a> {
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
    #[doc = "Bit 0 - BK2INE"]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - BK2DF1BK3E"]
    #[inline(always)]
    pub fn bk2df1bk3e(&self) -> BK2DF1BK3E_R {
        BK2DF1BK3E_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BK2INP"]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BK2INE"]
    #[inline(always)]
    pub fn bk2ine(&mut self) -> BK2INE_W {
        BK2INE_W { w: self }
    }
    #[doc = "Bit 8 - BK2DF1BK3E"]
    #[inline(always)]
    pub fn bk2df1bk3e(&mut self) -> BK2DF1BK3E_W {
        BK2DF1BK3E_W { w: self }
    }
    #[doc = "Bit 9 - BK2INP"]
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W {
        BK2INP_W { w: self }
    }
}
