#[doc = "Reader of register PDMCR"]
pub type R = crate::R<u32, super::PDMCR>;
#[doc = "Writer for register PDMCR"]
pub type W = crate::W<u32, super::PDMCR>;
#[doc = "Register PDMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDMEN`"]
pub type PDMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDMEN`"]
pub struct PDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMEN_W<'a> {
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
#[doc = "Reader of field `MICNBR`"]
pub type MICNBR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MICNBR`"]
pub struct MICNBR_W<'a> {
    w: &'a mut W,
}
impl<'a> MICNBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CKEN1`"]
pub type CKEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKEN1`"]
pub struct CKEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEN1_W<'a> {
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
#[doc = "Reader of field `CKEN2`"]
pub type CKEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKEN2`"]
pub struct CKEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEN2_W<'a> {
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
#[doc = "Reader of field `CKEN3`"]
pub type CKEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKEN3`"]
pub struct CKEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEN3_W<'a> {
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
#[doc = "Reader of field `CKEN4`"]
pub type CKEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKEN4`"]
pub struct CKEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEN4_W<'a> {
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
    #[doc = "Bit 0 - PDMEN"]
    #[inline(always)]
    pub fn pdmen(&self) -> PDMEN_R {
        PDMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - MICNBR"]
    #[inline(always)]
    pub fn micnbr(&self) -> MICNBR_R {
        MICNBR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - CKEN1"]
    #[inline(always)]
    pub fn cken1(&self) -> CKEN1_R {
        CKEN1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CKEN2"]
    #[inline(always)]
    pub fn cken2(&self) -> CKEN2_R {
        CKEN2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CKEN3"]
    #[inline(always)]
    pub fn cken3(&self) -> CKEN3_R {
        CKEN3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CKEN4"]
    #[inline(always)]
    pub fn cken4(&self) -> CKEN4_R {
        CKEN4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDMEN"]
    #[inline(always)]
    pub fn pdmen(&mut self) -> PDMEN_W {
        PDMEN_W { w: self }
    }
    #[doc = "Bits 4:5 - MICNBR"]
    #[inline(always)]
    pub fn micnbr(&mut self) -> MICNBR_W {
        MICNBR_W { w: self }
    }
    #[doc = "Bit 8 - CKEN1"]
    #[inline(always)]
    pub fn cken1(&mut self) -> CKEN1_W {
        CKEN1_W { w: self }
    }
    #[doc = "Bit 9 - CKEN2"]
    #[inline(always)]
    pub fn cken2(&mut self) -> CKEN2_W {
        CKEN2_W { w: self }
    }
    #[doc = "Bit 10 - CKEN3"]
    #[inline(always)]
    pub fn cken3(&mut self) -> CKEN3_W {
        CKEN3_W { w: self }
    }
    #[doc = "Bit 11 - CKEN4"]
    #[inline(always)]
    pub fn cken4(&mut self) -> CKEN4_W {
        CKEN4_W { w: self }
    }
}
