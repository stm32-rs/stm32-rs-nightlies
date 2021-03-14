#[doc = "Reader of register USBPHYC_PLL"]
pub type R = crate::R<u32, super::USBPHYC_PLL>;
#[doc = "Writer for register USBPHYC_PLL"]
pub type W = crate::W<u32, super::USBPHYC_PLL>;
#[doc = "Register USBPHYC_PLL `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::USBPHYC_PLL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Reader of field `PLLNDIV`"]
pub type PLLNDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLNDIV`"]
pub struct PLLNDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLNDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `PLLODF`"]
pub type PLLODF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLODF`"]
pub struct PLLODF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLODF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `PLLFRACIN`"]
pub type PLLFRACIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PLLFRACIN`"]
pub struct PLLFRACIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFRACIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 10)) | (((value as u32) & 0xffff) << 10);
        self.w
    }
}
#[doc = "Reader of field `PLLEN`"]
pub type PLLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLEN`"]
pub struct PLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `PLLSTRB`"]
pub type PLLSTRB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSTRB`"]
pub struct PLLSTRB_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSTRB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `PLLSTRBYP`"]
pub type PLLSTRBYP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSTRBYP`"]
pub struct PLLSTRBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSTRBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `PLLFRACCTL`"]
pub type PLLFRACCTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLFRACCTL`"]
pub struct PLLFRACCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFRACCTL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `PLLDITHEN0`"]
pub type PLLDITHEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLDITHEN0`"]
pub struct PLLDITHEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLDITHEN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `PLLDITHEN1`"]
pub type PLLDITHEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLDITHEN1`"]
pub struct PLLDITHEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLDITHEN1_W<'a> {
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
    #[doc = "Bits 0:6 - PLLNDIV"]
    #[inline(always)]
    pub fn pllndiv(&self) -> PLLNDIV_R {
        PLLNDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:9 - PLLODF"]
    #[inline(always)]
    pub fn pllodf(&self) -> PLLODF_R {
        PLLODF_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 10:25 - PLLFRACIN"]
    #[inline(always)]
    pub fn pllfracin(&self) -> PLLFRACIN_R {
        PLLFRACIN_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bit 26 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PLLSTRB"]
    #[inline(always)]
    pub fn pllstrb(&self) -> PLLSTRB_R {
        PLLSTRB_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PLLSTRBYP"]
    #[inline(always)]
    pub fn pllstrbyp(&self) -> PLLSTRBYP_R {
        PLLSTRBYP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - PLLFRACCTL"]
    #[inline(always)]
    pub fn pllfracctl(&self) -> PLLFRACCTL_R {
        PLLFRACCTL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - PLLDITHEN0"]
    #[inline(always)]
    pub fn plldithen0(&self) -> PLLDITHEN0_R {
        PLLDITHEN0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - PLLDITHEN1"]
    #[inline(always)]
    pub fn plldithen1(&self) -> PLLDITHEN1_R {
        PLLDITHEN1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PLLNDIV"]
    #[inline(always)]
    pub fn pllndiv(&mut self) -> PLLNDIV_W {
        PLLNDIV_W { w: self }
    }
    #[doc = "Bits 7:9 - PLLODF"]
    #[inline(always)]
    pub fn pllodf(&mut self) -> PLLODF_W {
        PLLODF_W { w: self }
    }
    #[doc = "Bits 10:25 - PLLFRACIN"]
    #[inline(always)]
    pub fn pllfracin(&mut self) -> PLLFRACIN_W {
        PLLFRACIN_W { w: self }
    }
    #[doc = "Bit 26 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W {
        PLLEN_W { w: self }
    }
    #[doc = "Bit 27 - PLLSTRB"]
    #[inline(always)]
    pub fn pllstrb(&mut self) -> PLLSTRB_W {
        PLLSTRB_W { w: self }
    }
    #[doc = "Bit 28 - PLLSTRBYP"]
    #[inline(always)]
    pub fn pllstrbyp(&mut self) -> PLLSTRBYP_W {
        PLLSTRBYP_W { w: self }
    }
    #[doc = "Bit 29 - PLLFRACCTL"]
    #[inline(always)]
    pub fn pllfracctl(&mut self) -> PLLFRACCTL_W {
        PLLFRACCTL_W { w: self }
    }
    #[doc = "Bit 30 - PLLDITHEN0"]
    #[inline(always)]
    pub fn plldithen0(&mut self) -> PLLDITHEN0_W {
        PLLDITHEN0_W { w: self }
    }
    #[doc = "Bit 31 - PLLDITHEN1"]
    #[inline(always)]
    pub fn plldithen1(&mut self) -> PLLDITHEN1_W {
        PLLDITHEN1_W { w: self }
    }
}
