#[doc = "Reader of register EXTI_C2IMR3"]
pub type R = crate::R<u32, super::EXTI_C2IMR3>;
#[doc = "Writer for register EXTI_C2IMR3"]
pub type W = crate::W<u32, super::EXTI_C2IMR3>;
#[doc = "Register EXTI_C2IMR3 `reset()`'s with value 0x0de9"]
impl crate::ResetValue for super::EXTI_C2IMR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0de9
    }
}
#[doc = "Reader of field `IM64`"]
pub type IM64_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM64`"]
pub struct IM64_W<'a> {
    w: &'a mut W,
}
impl<'a> IM64_W<'a> {
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
#[doc = "Reader of field `IM65`"]
pub type IM65_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM65`"]
pub struct IM65_W<'a> {
    w: &'a mut W,
}
impl<'a> IM65_W<'a> {
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
#[doc = "Reader of field `IM66`"]
pub type IM66_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM66`"]
pub struct IM66_W<'a> {
    w: &'a mut W,
}
impl<'a> IM66_W<'a> {
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
#[doc = "Reader of field `IM67`"]
pub type IM67_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM67`"]
pub struct IM67_W<'a> {
    w: &'a mut W,
}
impl<'a> IM67_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `IM68`"]
pub type IM68_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM68`"]
pub struct IM68_W<'a> {
    w: &'a mut W,
}
impl<'a> IM68_W<'a> {
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
#[doc = "Reader of field `IM69`"]
pub type IM69_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM69`"]
pub struct IM69_W<'a> {
    w: &'a mut W,
}
impl<'a> IM69_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `IM70`"]
pub type IM70_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM70`"]
pub struct IM70_W<'a> {
    w: &'a mut W,
}
impl<'a> IM70_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `IM71`"]
pub type IM71_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM71`"]
pub struct IM71_W<'a> {
    w: &'a mut W,
}
impl<'a> IM71_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `IM72`"]
pub type IM72_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM72`"]
pub struct IM72_W<'a> {
    w: &'a mut W,
}
impl<'a> IM72_W<'a> {
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
#[doc = "Reader of field `IM73`"]
pub type IM73_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM73`"]
pub struct IM73_W<'a> {
    w: &'a mut W,
}
impl<'a> IM73_W<'a> {
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
#[doc = "Reader of field `IM74`"]
pub type IM74_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM74`"]
pub struct IM74_W<'a> {
    w: &'a mut W,
}
impl<'a> IM74_W<'a> {
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
#[doc = "Reader of field `IM75`"]
pub type IM75_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM75`"]
pub struct IM75_W<'a> {
    w: &'a mut W,
}
impl<'a> IM75_W<'a> {
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
impl R {
    #[doc = "Bit 0 - IM64"]
    #[inline(always)]
    pub fn im64(&self) -> IM64_R {
        IM64_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IM65"]
    #[inline(always)]
    pub fn im65(&self) -> IM65_R {
        IM65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IM66"]
    #[inline(always)]
    pub fn im66(&self) -> IM66_R {
        IM66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IM67"]
    #[inline(always)]
    pub fn im67(&self) -> IM67_R {
        IM67_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IM68"]
    #[inline(always)]
    pub fn im68(&self) -> IM68_R {
        IM68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IM69"]
    #[inline(always)]
    pub fn im69(&self) -> IM69_R {
        IM69_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IM70"]
    #[inline(always)]
    pub fn im70(&self) -> IM70_R {
        IM70_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IM71"]
    #[inline(always)]
    pub fn im71(&self) -> IM71_R {
        IM71_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IM72"]
    #[inline(always)]
    pub fn im72(&self) -> IM72_R {
        IM72_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IM73"]
    #[inline(always)]
    pub fn im73(&self) -> IM73_R {
        IM73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IM74"]
    #[inline(always)]
    pub fn im74(&self) -> IM74_R {
        IM74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IM75"]
    #[inline(always)]
    pub fn im75(&self) -> IM75_R {
        IM75_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IM64"]
    #[inline(always)]
    pub fn im64(&mut self) -> IM64_W {
        IM64_W { w: self }
    }
    #[doc = "Bit 1 - IM65"]
    #[inline(always)]
    pub fn im65(&mut self) -> IM65_W {
        IM65_W { w: self }
    }
    #[doc = "Bit 2 - IM66"]
    #[inline(always)]
    pub fn im66(&mut self) -> IM66_W {
        IM66_W { w: self }
    }
    #[doc = "Bit 3 - IM67"]
    #[inline(always)]
    pub fn im67(&mut self) -> IM67_W {
        IM67_W { w: self }
    }
    #[doc = "Bit 4 - IM68"]
    #[inline(always)]
    pub fn im68(&mut self) -> IM68_W {
        IM68_W { w: self }
    }
    #[doc = "Bit 5 - IM69"]
    #[inline(always)]
    pub fn im69(&mut self) -> IM69_W {
        IM69_W { w: self }
    }
    #[doc = "Bit 6 - IM70"]
    #[inline(always)]
    pub fn im70(&mut self) -> IM70_W {
        IM70_W { w: self }
    }
    #[doc = "Bit 7 - IM71"]
    #[inline(always)]
    pub fn im71(&mut self) -> IM71_W {
        IM71_W { w: self }
    }
    #[doc = "Bit 8 - IM72"]
    #[inline(always)]
    pub fn im72(&mut self) -> IM72_W {
        IM72_W { w: self }
    }
    #[doc = "Bit 9 - IM73"]
    #[inline(always)]
    pub fn im73(&mut self) -> IM73_W {
        IM73_W { w: self }
    }
    #[doc = "Bit 10 - IM74"]
    #[inline(always)]
    pub fn im74(&mut self) -> IM74_W {
        IM74_W { w: self }
    }
    #[doc = "Bit 11 - IM75"]
    #[inline(always)]
    pub fn im75(&mut self) -> IM75_W {
        IM75_W { w: self }
    }
}
