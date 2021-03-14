#[doc = "Reader of register ETH_DMAC1TxCR"]
pub type R = crate::R<u32, super::ETH_DMAC1TXCR>;
#[doc = "Writer for register ETH_DMAC1TxCR"]
pub type W = crate::W<u32, super::ETH_DMAC1TXCR>;
#[doc = "Register ETH_DMAC1TxCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_DMAC1TXCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ST`"]
pub type ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ST`"]
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
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
#[doc = "Reader of field `TCW`"]
pub type TCW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCW`"]
pub struct TCW_W<'a> {
    w: &'a mut W,
}
impl<'a> TCW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `OSF`"]
pub type OSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSF`"]
pub struct OSF_W<'a> {
    w: &'a mut W,
}
impl<'a> OSF_W<'a> {
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
#[doc = "Reader of field `TSE`"]
pub type TSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSE`"]
pub struct TSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TXPBL`"]
pub type TXPBL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXPBL`"]
pub struct TXPBL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TQOS`"]
pub type TQOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TQOS`"]
pub struct TQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TQOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - TCW"]
    #[inline(always)]
    pub fn tcw(&self) -> TCW_R {
        TCW_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - OSF"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - TXPBL"]
    #[inline(always)]
    pub fn txpbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - TQOS"]
    #[inline(always)]
    pub fn tqos(&self) -> TQOS_R {
        TQOS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ST"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    #[doc = "Bits 1:3 - TCW"]
    #[inline(always)]
    pub fn tcw(&mut self) -> TCW_W {
        TCW_W { w: self }
    }
    #[doc = "Bit 4 - OSF"]
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W {
        OSF_W { w: self }
    }
    #[doc = "Bit 12 - TSE"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    #[doc = "Bits 16:21 - TXPBL"]
    #[inline(always)]
    pub fn txpbl(&mut self) -> TXPBL_W {
        TXPBL_W { w: self }
    }
    #[doc = "Bits 24:27 - TQOS"]
    #[inline(always)]
    pub fn tqos(&mut self) -> TQOS_W {
        TQOS_W { w: self }
    }
}
