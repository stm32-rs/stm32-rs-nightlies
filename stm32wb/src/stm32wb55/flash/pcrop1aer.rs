#[doc = "Reader of register PCROP1AER"]
pub type R = crate::R<u32, super::PCROP1AER>;
#[doc = "Writer for register PCROP1AER"]
pub type W = crate::W<u32, super::PCROP1AER>;
#[doc = "Register PCROP1AER `reset()`'s with value 0x7fff_fe00"]
impl crate::ResetValue for super::PCROP1AER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fff_fe00
    }
}
#[doc = "Reader of field `PCROP1A_END`"]
pub type PCROP1A_END_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCROP1A_END`"]
pub struct PCROP1A_END_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1A_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `PCROP_RDP`"]
pub type PCROP_RDP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCROP_RDP`"]
pub struct PCROP_RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP_RDP_W<'a> {
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
    #[doc = "Bits 0:8 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1a_end(&self) -> PCROP1A_END_R {
        PCROP1A_END_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1a_end(&mut self) -> PCROP1A_END_W {
        PCROP1A_END_W { w: self }
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W {
        PCROP_RDP_W { w: self }
    }
}
