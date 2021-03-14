#[doc = "Reader of register DDRPHYC_DTAR"]
pub type R = crate::R<u32, super::DDRPHYC_DTAR>;
#[doc = "Writer for register DDRPHYC_DTAR"]
pub type W = crate::W<u32, super::DDRPHYC_DTAR>;
#[doc = "Register DDRPHYC_DTAR `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_DTAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTCOL`"]
pub type DTCOL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DTCOL`"]
pub struct DTCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `DTROW`"]
pub type DTROW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DTROW`"]
pub struct DTROW_W<'a> {
    w: &'a mut W,
}
impl<'a> DTROW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 12)) | (((value as u32) & 0xffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `DTBANK`"]
pub type DTBANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTBANK`"]
pub struct DTBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `DTMPR`"]
pub type DTMPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTMPR`"]
pub struct DTMPR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTMPR_W<'a> {
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
    #[doc = "Bits 0:11 - DTCOL"]
    #[inline(always)]
    pub fn dtcol(&self) -> DTCOL_R {
        DTCOL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:27 - DTROW"]
    #[inline(always)]
    pub fn dtrow(&self) -> DTROW_R {
        DTROW_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:30 - DTBANK"]
    #[inline(always)]
    pub fn dtbank(&self) -> DTBANK_R {
        DTBANK_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - DTMPR"]
    #[inline(always)]
    pub fn dtmpr(&self) -> DTMPR_R {
        DTMPR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - DTCOL"]
    #[inline(always)]
    pub fn dtcol(&mut self) -> DTCOL_W {
        DTCOL_W { w: self }
    }
    #[doc = "Bits 12:27 - DTROW"]
    #[inline(always)]
    pub fn dtrow(&mut self) -> DTROW_W {
        DTROW_W { w: self }
    }
    #[doc = "Bits 28:30 - DTBANK"]
    #[inline(always)]
    pub fn dtbank(&mut self) -> DTBANK_W {
        DTBANK_W { w: self }
    }
    #[doc = "Bit 31 - DTMPR"]
    #[inline(always)]
    pub fn dtmpr(&mut self) -> DTMPR_W {
        DTMPR_W { w: self }
    }
}
