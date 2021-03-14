#[doc = "Reader of register MTLRxQMPOCR"]
pub type R = crate::R<u32, super::MTLRXQMPOCR>;
#[doc = "Writer for register MTLRxQMPOCR"]
pub type W = crate::W<u32, super::MTLRXQMPOCR>;
#[doc = "Register MTLRxQMPOCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MTLRXQMPOCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MISCNTOVF`"]
pub type MISCNTOVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MISCNTOVF`"]
pub struct MISCNTOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> MISCNTOVF_W<'a> {
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
#[doc = "Reader of field `MISPKTCNT`"]
pub type MISPKTCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MISPKTCNT`"]
pub struct MISPKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MISPKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `OVFCNTOVF`"]
pub type OVFCNTOVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVFCNTOVF`"]
pub struct OVFCNTOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFCNTOVF_W<'a> {
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
#[doc = "Reader of field `OVFPKTCNT`"]
pub type OVFPKTCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OVFPKTCNT`"]
pub struct OVFPKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFPKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - Missed Packet Counter Overflow Bit"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Missed Packet Counter"]
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Counter Overflow Bit"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 0:10 - Overflow Packet Counter"]
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 27 - Missed Packet Counter Overflow Bit"]
    #[inline(always)]
    pub fn miscntovf(&mut self) -> MISCNTOVF_W {
        MISCNTOVF_W { w: self }
    }
    #[doc = "Bits 16:26 - Missed Packet Counter"]
    #[inline(always)]
    pub fn mispktcnt(&mut self) -> MISPKTCNT_W {
        MISPKTCNT_W { w: self }
    }
    #[doc = "Bit 11 - Overflow Counter Overflow Bit"]
    #[inline(always)]
    pub fn ovfcntovf(&mut self) -> OVFCNTOVF_W {
        OVFCNTOVF_W { w: self }
    }
    #[doc = "Bits 0:10 - Overflow Packet Counter"]
    #[inline(always)]
    pub fn ovfpktcnt(&mut self) -> OVFPKTCNT_W {
        OVFPKTCNT_W { w: self }
    }
}
