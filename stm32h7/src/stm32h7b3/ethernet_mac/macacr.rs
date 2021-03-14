#[doc = "Reader of register MACACR"]
pub type R = crate::R<u32, super::MACACR>;
#[doc = "Writer for register MACACR"]
pub type W = crate::W<u32, super::MACACR>;
#[doc = "Register MACACR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ATSFC`"]
pub type ATSFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATSFC`"]
pub struct ATSFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSFC_W<'a> {
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
#[doc = "Reader of field `ATSEN0`"]
pub type ATSEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATSEN0`"]
pub struct ATSEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN0_W<'a> {
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
#[doc = "Reader of field `ATSEN1`"]
pub type ATSEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATSEN1`"]
pub struct ATSEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN1_W<'a> {
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
#[doc = "Reader of field `ATSEN2`"]
pub type ATSEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATSEN2`"]
pub struct ATSEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN2_W<'a> {
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
#[doc = "Reader of field `ATSEN3`"]
pub type ATSEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATSEN3`"]
pub struct ATSEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ATSFC"]
    #[inline(always)]
    pub fn atsfc(&self) -> ATSFC_R {
        ATSFC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - ATSEN0"]
    #[inline(always)]
    pub fn atsen0(&self) -> ATSEN0_R {
        ATSEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ATSEN1"]
    #[inline(always)]
    pub fn atsen1(&self) -> ATSEN1_R {
        ATSEN1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ATSEN2"]
    #[inline(always)]
    pub fn atsen2(&self) -> ATSEN2_R {
        ATSEN2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ATSEN3"]
    #[inline(always)]
    pub fn atsen3(&self) -> ATSEN3_R {
        ATSEN3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ATSFC"]
    #[inline(always)]
    pub fn atsfc(&mut self) -> ATSFC_W {
        ATSFC_W { w: self }
    }
    #[doc = "Bit 4 - ATSEN0"]
    #[inline(always)]
    pub fn atsen0(&mut self) -> ATSEN0_W {
        ATSEN0_W { w: self }
    }
    #[doc = "Bit 5 - ATSEN1"]
    #[inline(always)]
    pub fn atsen1(&mut self) -> ATSEN1_W {
        ATSEN1_W { w: self }
    }
    #[doc = "Bit 6 - ATSEN2"]
    #[inline(always)]
    pub fn atsen2(&mut self) -> ATSEN2_W {
        ATSEN2_W { w: self }
    }
    #[doc = "Bit 7 - ATSEN3"]
    #[inline(always)]
    pub fn atsen3(&mut self) -> ATSEN3_W {
        ATSEN3_W { w: self }
    }
}
