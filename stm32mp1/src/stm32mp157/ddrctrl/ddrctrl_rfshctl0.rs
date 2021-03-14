#[doc = "Reader of register DDRCTRL_RFSHCTL0"]
pub type R = crate::R<u32, super::DDRCTRL_RFSHCTL0>;
#[doc = "Writer for register DDRCTRL_RFSHCTL0"]
pub type W = crate::W<u32, super::DDRCTRL_RFSHCTL0>;
#[doc = "Register DDRCTRL_RFSHCTL0 `reset()`'s with value 0x0021_0000"]
impl crate::ResetValue for super::DDRCTRL_RFSHCTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0021_0000
    }
}
#[doc = "Reader of field `PER_BANK_REFRESH`"]
pub type PER_BANK_REFRESH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER_BANK_REFRESH`"]
pub struct PER_BANK_REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_BANK_REFRESH_W<'a> {
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
#[doc = "Reader of field `REFRESH_BURST`"]
pub type REFRESH_BURST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFRESH_BURST`"]
pub struct REFRESH_BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_BURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Reader of field `REFRESH_TO_X32`"]
pub type REFRESH_TO_X32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFRESH_TO_X32`"]
pub struct REFRESH_TO_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_TO_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `REFRESH_MARGIN`"]
pub type REFRESH_MARGIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFRESH_MARGIN`"]
pub struct REFRESH_MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_MARGIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - PER_BANK_REFRESH"]
    #[inline(always)]
    pub fn per_bank_refresh(&self) -> PER_BANK_REFRESH_R {
        PER_BANK_REFRESH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:8 - REFRESH_BURST"]
    #[inline(always)]
    pub fn refresh_burst(&self) -> REFRESH_BURST_R {
        REFRESH_BURST_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - REFRESH_TO_X32"]
    #[inline(always)]
    pub fn refresh_to_x32(&self) -> REFRESH_TO_X32_R {
        REFRESH_TO_X32_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - REFRESH_MARGIN"]
    #[inline(always)]
    pub fn refresh_margin(&self) -> REFRESH_MARGIN_R {
        REFRESH_MARGIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - PER_BANK_REFRESH"]
    #[inline(always)]
    pub fn per_bank_refresh(&mut self) -> PER_BANK_REFRESH_W {
        PER_BANK_REFRESH_W { w: self }
    }
    #[doc = "Bits 4:8 - REFRESH_BURST"]
    #[inline(always)]
    pub fn refresh_burst(&mut self) -> REFRESH_BURST_W {
        REFRESH_BURST_W { w: self }
    }
    #[doc = "Bits 12:16 - REFRESH_TO_X32"]
    #[inline(always)]
    pub fn refresh_to_x32(&mut self) -> REFRESH_TO_X32_W {
        REFRESH_TO_X32_W { w: self }
    }
    #[doc = "Bits 20:23 - REFRESH_MARGIN"]
    #[inline(always)]
    pub fn refresh_margin(&mut self) -> REFRESH_MARGIN_W {
        REFRESH_MARGIN_W { w: self }
    }
}
