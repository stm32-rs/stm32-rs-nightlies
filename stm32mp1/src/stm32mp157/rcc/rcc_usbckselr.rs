#[doc = "Reader of register RCC_USBCKSELR"]
pub type R = crate::R<u32, super::RCC_USBCKSELR>;
#[doc = "Writer for register RCC_USBCKSELR"]
pub type W = crate::W<u32, super::RCC_USBCKSELR>;
#[doc = "Register RCC_USBCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_USBCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBPHYSRC`"]
pub type USBPHYSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBPHYSRC`"]
pub struct USBPHYSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `USBOSRC`"]
pub type USBOSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBOSRC`"]
pub struct USBOSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOSRC_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - USBPHYSRC"]
    #[inline(always)]
    pub fn usbphysrc(&self) -> USBPHYSRC_R {
        USBPHYSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - USBOSRC"]
    #[inline(always)]
    pub fn usbosrc(&self) -> USBOSRC_R {
        USBOSRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USBPHYSRC"]
    #[inline(always)]
    pub fn usbphysrc(&mut self) -> USBPHYSRC_W {
        USBPHYSRC_W { w: self }
    }
    #[doc = "Bit 4 - USBOSRC"]
    #[inline(always)]
    pub fn usbosrc(&mut self) -> USBOSRC_W {
        USBOSRC_W { w: self }
    }
}
