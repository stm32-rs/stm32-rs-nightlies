#[doc = "Reader of register PCSCNTR"]
pub type R = crate::R<u32, super::PCSCNTR>;
#[doc = "Writer for register PCSCNTR"]
pub type W = crate::W<u32, super::PCSCNTR>;
#[doc = "Register PCSCNTR `reset()`'s with value 0"]
impl crate::ResetValue for super::PCSCNTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSCOUNT`"]
pub type CSCOUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CSCOUNT`"]
pub struct CSCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `CNTB1EN`"]
pub type CNTB1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTB1EN`"]
pub struct CNTB1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTB1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CNTB2EN`"]
pub type CNTB2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTB2EN`"]
pub struct CNTB2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTB2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CNTB3EN`"]
pub type CNTB3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTB3EN`"]
pub struct CNTB3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTB3EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CNTB4EN`"]
pub type CNTB4EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTB4EN`"]
pub struct CNTB4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTB4EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CSCOUNT"]
    #[inline(always)]
    pub fn cscount(&self) -> CSCOUNT_R {
        CSCOUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - CNTB1EN"]
    #[inline(always)]
    pub fn cntb1en(&self) -> CNTB1EN_R {
        CNTB1EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CNTB2EN"]
    #[inline(always)]
    pub fn cntb2en(&self) -> CNTB2EN_R {
        CNTB2EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CNTB3EN"]
    #[inline(always)]
    pub fn cntb3en(&self) -> CNTB3EN_R {
        CNTB3EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CNTB4EN"]
    #[inline(always)]
    pub fn cntb4en(&self) -> CNTB4EN_R {
        CNTB4EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CSCOUNT"]
    #[inline(always)]
    pub fn cscount(&mut self) -> CSCOUNT_W {
        CSCOUNT_W { w: self }
    }
    #[doc = "Bit 16 - CNTB1EN"]
    #[inline(always)]
    pub fn cntb1en(&mut self) -> CNTB1EN_W {
        CNTB1EN_W { w: self }
    }
    #[doc = "Bit 17 - CNTB2EN"]
    #[inline(always)]
    pub fn cntb2en(&mut self) -> CNTB2EN_W {
        CNTB2EN_W { w: self }
    }
    #[doc = "Bit 18 - CNTB3EN"]
    #[inline(always)]
    pub fn cntb3en(&mut self) -> CNTB3EN_W {
        CNTB3EN_W { w: self }
    }
    #[doc = "Bit 19 - CNTB4EN"]
    #[inline(always)]
    pub fn cntb4en(&mut self) -> CNTB4EN_W {
        CNTB4EN_W { w: self }
    }
}
