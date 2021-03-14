#[doc = "Reader of register MTLTxQUR"]
pub type R = crate::R<u32, super::MTLTXQUR>;
#[doc = "Writer for register MTLTxQUR"]
pub type W = crate::W<u32, super::MTLTXQUR>;
#[doc = "Register MTLTxQUR `reset()`'s with value 0"]
impl crate::ResetValue for super::MTLTXQUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UFCNTOVF`"]
pub type UFCNTOVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UFCNTOVF`"]
pub struct UFCNTOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> UFCNTOVF_W<'a> {
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
#[doc = "Reader of field `UFFRMCNT`"]
pub type UFFRMCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UFFRMCNT`"]
pub struct UFFRMCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UFFRMCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter"]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 0:10 - Underflow Packet Counter"]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter"]
    #[inline(always)]
    pub fn ufcntovf(&mut self) -> UFCNTOVF_W {
        UFCNTOVF_W { w: self }
    }
    #[doc = "Bits 0:10 - Underflow Packet Counter"]
    #[inline(always)]
    pub fn uffrmcnt(&mut self) -> UFFRMCNT_W {
        UFFRMCNT_W { w: self }
    }
}
