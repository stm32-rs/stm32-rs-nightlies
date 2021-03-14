#[doc = "Reader of register SMCR"]
pub type R = crate::R<u32, super::SMCR>;
#[doc = "Writer for register SMCR"]
pub type W = crate::W<u32, super::SMCR>;
#[doc = "Register SMCR `reset()`'s with value 0xe00f"]
impl crate::ResetValue for super::SMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xe00f
    }
}
#[doc = "Reader of field `DECPROT`"]
pub type DECPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DECPROT`"]
pub struct DECPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> DECPROT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `INITDPROT`"]
pub type INITDPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INITDPROT`"]
pub struct INITDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> INITDPROT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CALDPROT`"]
pub type CALDPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALDPROT`"]
pub struct CALDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALDPROT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TSDPROT`"]
pub type TSDPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSDPROT`"]
pub struct TSDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSDPROT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `WUTDPROT`"]
pub type WUTDPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUTDPROT`"]
pub struct WUTDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTDPROT_W<'a> {
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
#[doc = "Reader of field `ALRBDPROT`"]
pub type ALRBDPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALRBDPROT`"]
pub struct ALRBDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBDPROT_W<'a> {
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
#[doc = "Reader of field `ALRADPROT`"]
pub type ALRADPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALRADPROT`"]
pub struct ALRADPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRADPROT_W<'a> {
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
impl R {
    #[doc = "Bit 15 - DECPROT"]
    #[inline(always)]
    pub fn decprot(&self) -> DECPROT_R {
        DECPROT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - INITDPROT"]
    #[inline(always)]
    pub fn initdprot(&self) -> INITDPROT_R {
        INITDPROT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CALDPROT"]
    #[inline(always)]
    pub fn caldprot(&self) -> CALDPROT_R {
        CALDPROT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSDPROT"]
    #[inline(always)]
    pub fn tsdprot(&self) -> TSDPROT_R {
        TSDPROT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WUTDPROT"]
    #[inline(always)]
    pub fn wutdprot(&self) -> WUTDPROT_R {
        WUTDPROT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ALRBDPROT"]
    #[inline(always)]
    pub fn alrbdprot(&self) -> ALRBDPROT_R {
        ALRBDPROT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ALRADPROT"]
    #[inline(always)]
    pub fn alradprot(&self) -> ALRADPROT_R {
        ALRADPROT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - DECPROT"]
    #[inline(always)]
    pub fn decprot(&mut self) -> DECPROT_W {
        DECPROT_W { w: self }
    }
    #[doc = "Bit 14 - INITDPROT"]
    #[inline(always)]
    pub fn initdprot(&mut self) -> INITDPROT_W {
        INITDPROT_W { w: self }
    }
    #[doc = "Bit 13 - CALDPROT"]
    #[inline(always)]
    pub fn caldprot(&mut self) -> CALDPROT_W {
        CALDPROT_W { w: self }
    }
    #[doc = "Bit 3 - TSDPROT"]
    #[inline(always)]
    pub fn tsdprot(&mut self) -> TSDPROT_W {
        TSDPROT_W { w: self }
    }
    #[doc = "Bit 2 - WUTDPROT"]
    #[inline(always)]
    pub fn wutdprot(&mut self) -> WUTDPROT_W {
        WUTDPROT_W { w: self }
    }
    #[doc = "Bit 1 - ALRBDPROT"]
    #[inline(always)]
    pub fn alrbdprot(&mut self) -> ALRBDPROT_W {
        ALRBDPROT_W { w: self }
    }
    #[doc = "Bit 0 - ALRADPROT"]
    #[inline(always)]
    pub fn alradprot(&mut self) -> ALRADPROT_W {
        ALRADPROT_W { w: self }
    }
}
