#[doc = "Reader of register USBPHYC_MISC"]
pub type R = crate::R<u32, super::USBPHYC_MISC>;
#[doc = "Writer for register USBPHYC_MISC"]
pub type W = crate::W<u32, super::USBPHYC_MISC>;
#[doc = "Register USBPHYC_MISC `reset()`'s with value 0"]
impl crate::ResetValue for super::USBPHYC_MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWITHOST`"]
pub type SWITHOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWITHOST`"]
pub struct SWITHOST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITHOST_W<'a> {
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
#[doc = "Reader of field `PPCKDIS`"]
pub type PPCKDIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PPCKDIS`"]
pub struct PPCKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCKDIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SWITHOST"]
    #[inline(always)]
    pub fn swithost(&self) -> SWITHOST_R {
        SWITHOST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - PPCKDIS"]
    #[inline(always)]
    pub fn ppckdis(&self) -> PPCKDIS_R {
        PPCKDIS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SWITHOST"]
    #[inline(always)]
    pub fn swithost(&mut self) -> SWITHOST_W {
        SWITHOST_W { w: self }
    }
    #[doc = "Bits 1:2 - PPCKDIS"]
    #[inline(always)]
    pub fn ppckdis(&mut self) -> PPCKDIS_W {
        PPCKDIS_W { w: self }
    }
}
