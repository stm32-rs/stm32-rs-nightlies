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
#[doc = "Reader of field `FLT6SRC_1`"]
pub type FLT6SRC_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT6SRC_1`"]
pub struct FLT6SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6SRC_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `FLT5SRC_1`"]
pub type FLT5SRC_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT5SRC_1`"]
pub struct FLT5SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5SRC_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `FLT4SRC_1`"]
pub type FLT4SRC_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT4SRC_1`"]
pub struct FLT4SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4SRC_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `FLT3SRC_1`"]
pub type FLT3SRC_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT3SRC_1`"]
pub struct FLT3SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3SRC_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `FLT2SRC_1`"]
pub type FLT2SRC_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT2SRC_1`"]
pub struct FLT2SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2SRC_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `FLT1SRC_1`"]
pub type FLT1SRC_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT1SRC_1`"]
pub struct FLT1SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1SRC_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `FLT6LCK`"]
pub type FLT6LCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT6LCK`"]
pub struct FLT6LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6LCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `FLT6F`"]
pub type FLT6F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLT6F`"]
pub struct FLT6F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | (((value as u32) & 0x0f) << 11);
        self.w
    }
}
#[doc = "Reader of field `FLT6SRC_0`"]
pub type FLT6SRC_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT6SRC_0`"]
pub struct FLT6SRC_0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6SRC_0_W<'a> {
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
#[doc = "Reader of field `FLT6P`"]
pub type FLT6P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT6P`"]
pub struct FLT6P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6P_W<'a> {
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
#[doc = "Reader of field `FLT6E`"]
pub type FLT6E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT6E`"]
pub struct FLT6E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6E_W<'a> {
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
    #[doc = "Bit 21 - FLT6SRC"]
    #[inline(always)]
    pub fn flt6src_1(&self) -> FLT6SRC_1_R {
        FLT6SRC_1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - FLT5SRC_1"]
    #[inline(always)]
    pub fn flt5src_1(&self) -> FLT5SRC_1_R {
        FLT5SRC_1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - FLT4SRC_1"]
    #[inline(always)]
    pub fn flt4src_1(&self) -> FLT4SRC_1_R {
        FLT4SRC_1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - FLT3SRC_1"]
    #[inline(always)]
    pub fn flt3src_1(&self) -> FLT3SRC_1_R {
        FLT3SRC_1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FLT2SRC_1"]
    #[inline(always)]
    pub fn flt2src_1(&self) -> FLT2SRC_1_R {
        FLT2SRC_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FLT1SRC_1"]
    #[inline(always)]
    pub fn flt1src_1(&self) -> FLT1SRC_1_R {
        FLT1SRC_1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FLT6LCK"]
    #[inline(always)]
    pub fn flt6lck(&self) -> FLT6LCK_R {
        FLT6LCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 11:14 - FLT6F"]
    #[inline(always)]
    pub fn flt6f(&self) -> FLT6F_R {
        FLT6F_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - FLT6F"]
    #[inline(always)]
    pub fn flt6src_0(&self) -> FLT6SRC_0_R {
        FLT6SRC_0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FLT6P"]
    #[inline(always)]
    pub fn flt6p(&self) -> FLT6P_R {
        FLT6P_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FLT6E"]
    #[inline(always)]
    pub fn flt6e(&self) -> FLT6E_R {
        FLT6E_R::new(((self.bits >> 8) & 0x01) != 0)
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
    #[doc = "Bit 21 - FLT6SRC"]
    #[inline(always)]
    pub fn flt6src_1(&mut self) -> FLT6SRC_1_W {
        FLT6SRC_1_W { w: self }
    }
    #[doc = "Bit 20 - FLT5SRC_1"]
    #[inline(always)]
    pub fn flt5src_1(&mut self) -> FLT5SRC_1_W {
        FLT5SRC_1_W { w: self }
    }
    #[doc = "Bit 19 - FLT4SRC_1"]
    #[inline(always)]
    pub fn flt4src_1(&mut self) -> FLT4SRC_1_W {
        FLT4SRC_1_W { w: self }
    }
    #[doc = "Bit 18 - FLT3SRC_1"]
    #[inline(always)]
    pub fn flt3src_1(&mut self) -> FLT3SRC_1_W {
        FLT3SRC_1_W { w: self }
    }
    #[doc = "Bit 17 - FLT2SRC_1"]
    #[inline(always)]
    pub fn flt2src_1(&mut self) -> FLT2SRC_1_W {
        FLT2SRC_1_W { w: self }
    }
    #[doc = "Bit 16 - FLT1SRC_1"]
    #[inline(always)]
    pub fn flt1src_1(&mut self) -> FLT1SRC_1_W {
        FLT1SRC_1_W { w: self }
    }
    #[doc = "Bit 15 - FLT6LCK"]
    #[inline(always)]
    pub fn flt6lck(&mut self) -> FLT6LCK_W {
        FLT6LCK_W { w: self }
    }
    #[doc = "Bits 11:14 - FLT6F"]
    #[inline(always)]
    pub fn flt6f(&mut self) -> FLT6F_W {
        FLT6F_W { w: self }
    }
    #[doc = "Bit 10 - FLT6F"]
    #[inline(always)]
    pub fn flt6src_0(&mut self) -> FLT6SRC_0_W {
        FLT6SRC_0_W { w: self }
    }
    #[doc = "Bit 9 - FLT6P"]
    #[inline(always)]
    pub fn flt6p(&mut self) -> FLT6P_W {
        FLT6P_W { w: self }
    }
    #[doc = "Bit 8 - FLT6E"]
    #[inline(always)]
    pub fn flt6e(&mut self) -> FLT6E_W {
        FLT6E_W { w: self }
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
