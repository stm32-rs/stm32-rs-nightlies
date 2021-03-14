#[doc = "Reader of register OPTCR2"]
pub type R = crate::R<u32, super::OPTCR2>;
#[doc = "Writer for register OPTCR2"]
pub type W = crate::W<u32, super::OPTCR2>;
#[doc = "Register OPTCR2 `reset()`'s with value 0x8000_00ff"]
impl crate::ResetValue for super::OPTCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_00ff
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
#[doc = "Reader of field `PCROPi`"]
pub type PCROPI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCROPi`"]
pub struct PCROPI_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROPI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - PCROP zone preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - PCROP option byte"]
    #[inline(always)]
    pub fn pcropi(&self) -> PCROPI_R {
        PCROPI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - PCROP zone preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W {
        PCROP_RDP_W { w: self }
    }
    #[doc = "Bits 0:7 - PCROP option byte"]
    #[inline(always)]
    pub fn pcropi(&mut self) -> PCROPI_W {
        PCROPI_W { w: self }
    }
}
