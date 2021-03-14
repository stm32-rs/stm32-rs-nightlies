#[doc = "Reader of register CLRFR"]
pub type R = crate::R<u32, super::CLRFR>;
#[doc = "Writer for register CLRFR"]
pub type W = crate::W<u32, super::CLRFR>;
#[doc = "Register CLRFR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLRFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPERF`"]
pub type CPERF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPERF`"]
pub struct CPERF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPERF_W<'a> {
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
#[doc = "Reader of field `CSERF`"]
pub type CSERF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSERF`"]
pub struct CSERF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSERF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CTERF`"]
pub type CTERF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTERF`"]
pub struct CTERF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTERF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    pub fn cperf(&self) -> CPERF_R {
        CPERF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    pub fn cserf(&self) -> CSERF_R {
        CSERF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    pub fn cterf(&self) -> CTERF_R {
        CTERF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    pub fn cperf(&mut self) -> CPERF_W {
        CPERF_W { w: self }
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    pub fn cserf(&mut self) -> CSERF_W {
        CSERF_W { w: self }
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    pub fn cterf(&mut self) -> CTERF_W {
        CTERF_W { w: self }
    }
}
