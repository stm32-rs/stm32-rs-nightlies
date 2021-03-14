#[doc = "Reader of register ETH_MTLOMR"]
pub type R = crate::R<u32, super::ETH_MTLOMR>;
#[doc = "Writer for register ETH_MTLOMR"]
pub type W = crate::W<u32, super::ETH_MTLOMR>;
#[doc = "Register ETH_MTLOMR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MTLOMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTXSTS`"]
pub type DTXSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTXSTS`"]
pub struct DTXSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTXSTS_W<'a> {
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
#[doc = "Reader of field `RAA`"]
pub type RAA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAA`"]
pub struct RAA_W<'a> {
    w: &'a mut W,
}
impl<'a> RAA_W<'a> {
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
#[doc = "Reader of field `SCHALG`"]
pub type SCHALG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCHALG`"]
pub struct SCHALG_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHALG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `CNTPRST`"]
pub type CNTPRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTPRST`"]
pub struct CNTPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTPRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CNTCLR`"]
pub type CNTCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTCLR`"]
pub struct CNTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - DTXSTS"]
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RAA"]
    #[inline(always)]
    pub fn raa(&self) -> RAA_R {
        RAA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - SCHALG"]
    #[inline(always)]
    pub fn schalg(&self) -> SCHALG_R {
        SCHALG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 8 - CNTPRST"]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CNTCLR"]
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DTXSTS"]
    #[inline(always)]
    pub fn dtxsts(&mut self) -> DTXSTS_W {
        DTXSTS_W { w: self }
    }
    #[doc = "Bit 2 - RAA"]
    #[inline(always)]
    pub fn raa(&mut self) -> RAA_W {
        RAA_W { w: self }
    }
    #[doc = "Bits 5:6 - SCHALG"]
    #[inline(always)]
    pub fn schalg(&mut self) -> SCHALG_W {
        SCHALG_W { w: self }
    }
    #[doc = "Bit 8 - CNTPRST"]
    #[inline(always)]
    pub fn cntprst(&mut self) -> CNTPRST_W {
        CNTPRST_W { w: self }
    }
    #[doc = "Bit 9 - CNTCLR"]
    #[inline(always)]
    pub fn cntclr(&mut self) -> CNTCLR_W {
        CNTCLR_W { w: self }
    }
}
