#[doc = "Reader of register FLTINR2"]
pub type R = crate::R<u32, super::FLTINR2>;
#[doc = "Writer for register FLTINR2"]
pub type W = crate::W<u32, super::FLTINR2>;
#[doc = "Register FLTINR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLTINR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLTSD`"]
pub type FLTSD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLTSD`"]
pub struct FLTSD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTSD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `FLT5LCK`"]
pub type FLT5LCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT5LCK`"]
pub struct FLT5LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5LCK_W<'a> {
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
#[doc = "Reader of field `FLT5F`"]
pub type FLT5F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLT5F`"]
pub struct FLT5F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `FLT5SRC`"]
pub type FLT5SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT5SRC`"]
pub struct FLT5SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5SRC_W<'a> {
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
#[doc = "Reader of field `FLT5P`"]
pub type FLT5P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT5P`"]
pub struct FLT5P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5P_W<'a> {
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
#[doc = "Reader of field `FLT5E`"]
pub type FLT5E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT5E`"]
pub struct FLT5E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5E_W<'a> {
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
impl R {
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&self) -> FLTSD_R {
        FLTSD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&self) -> FLT5LCK_R {
        FLT5LCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&self) -> FLT5F_R {
        FLT5F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&self) -> FLT5SRC_R {
        FLT5SRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&self) -> FLT5P_R {
        FLT5P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&self) -> FLT5E_R {
        FLT5E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&mut self) -> FLTSD_W {
        FLTSD_W { w: self }
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&mut self) -> FLT5LCK_W {
        FLT5LCK_W { w: self }
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&mut self) -> FLT5F_W {
        FLT5F_W { w: self }
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&mut self) -> FLT5SRC_W {
        FLT5SRC_W { w: self }
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&mut self) -> FLT5P_W {
        FLT5P_W { w: self }
    }
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&mut self) -> FLT5E_W {
        FLT5E_W { w: self }
    }
}
