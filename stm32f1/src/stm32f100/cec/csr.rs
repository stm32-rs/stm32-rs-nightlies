#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSOM`"]
pub type TSOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSOM`"]
pub struct TSOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TSOM_W<'a> {
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
#[doc = "Reader of field `TEOM`"]
pub type TEOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEOM`"]
pub struct TEOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TEOM_W<'a> {
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
#[doc = "Reader of field `TERR`"]
pub type TERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TERR`"]
pub struct TERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TERR_W<'a> {
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
#[doc = "Reader of field `TBTRF`"]
pub type TBTRF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBTRF`"]
pub struct TBTRF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTRF_W<'a> {
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
#[doc = "Reader of field `RSOM`"]
pub type RSOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSOM`"]
pub struct RSOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RSOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `REOM`"]
pub type REOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REOM`"]
pub struct REOM_W<'a> {
    w: &'a mut W,
}
impl<'a> REOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RERR`"]
pub type RERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RERR`"]
pub struct RERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RBTF`"]
pub type RBTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RBTF`"]
pub struct RBTF_W<'a> {
    w: &'a mut W,
}
impl<'a> RBTF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Tx start of message"]
    #[inline(always)]
    pub fn tsom(&self) -> TSOM_R {
        TSOM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tx end of message"]
    #[inline(always)]
    pub fn teom(&self) -> TEOM_R {
        TEOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tx error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Tx byte transfer request or block transfer finished"]
    #[inline(always)]
    pub fn tbtrf(&self) -> TBTRF_R {
        TBTRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx start of message"]
    #[inline(always)]
    pub fn rsom(&self) -> RSOM_R {
        RSOM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx end of message"]
    #[inline(always)]
    pub fn reom(&self) -> REOM_R {
        REOM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx error"]
    #[inline(always)]
    pub fn rerr(&self) -> RERR_R {
        RERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx byte/block transfer finished"]
    #[inline(always)]
    pub fn rbtf(&self) -> RBTF_R {
        RBTF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tx start of message"]
    #[inline(always)]
    pub fn tsom(&mut self) -> TSOM_W {
        TSOM_W { w: self }
    }
    #[doc = "Bit 1 - Tx end of message"]
    #[inline(always)]
    pub fn teom(&mut self) -> TEOM_W {
        TEOM_W { w: self }
    }
    #[doc = "Bit 2 - Tx error"]
    #[inline(always)]
    pub fn terr(&mut self) -> TERR_W {
        TERR_W { w: self }
    }
    #[doc = "Bit 3 - Tx byte transfer request or block transfer finished"]
    #[inline(always)]
    pub fn tbtrf(&mut self) -> TBTRF_W {
        TBTRF_W { w: self }
    }
    #[doc = "Bit 4 - Rx start of message"]
    #[inline(always)]
    pub fn rsom(&mut self) -> RSOM_W {
        RSOM_W { w: self }
    }
    #[doc = "Bit 5 - Rx end of message"]
    #[inline(always)]
    pub fn reom(&mut self) -> REOM_W {
        REOM_W { w: self }
    }
    #[doc = "Bit 6 - Rx error"]
    #[inline(always)]
    pub fn rerr(&mut self) -> RERR_W {
        RERR_W { w: self }
    }
    #[doc = "Bit 7 - Rx byte/block transfer finished"]
    #[inline(always)]
    pub fn rbtf(&mut self) -> RBTF_W {
        RBTF_W { w: self }
    }
}
