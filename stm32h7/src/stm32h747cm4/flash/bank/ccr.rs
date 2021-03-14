#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLR_CRCRDERR`"]
pub type CLR_CRCRDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_CRCRDERR`"]
pub struct CLR_CRCRDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_CRCRDERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `CLR_CRCEND`"]
pub type CLR_CRCEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_CRCEND`"]
pub struct CLR_CRCEND_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_CRCEND_W<'a> {
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
#[doc = "Reader of field `CLR_DBECCERR`"]
pub type CLR_DBECCERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_DBECCERR`"]
pub struct CLR_DBECCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_DBECCERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CLR_SNECCERR`"]
pub type CLR_SNECCERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_SNECCERR`"]
pub struct CLR_SNECCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_SNECCERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `CLR_RDSERR`"]
pub type CLR_RDSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_RDSERR`"]
pub struct CLR_RDSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_RDSERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CLR_RDPERR`"]
pub type CLR_RDPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_RDPERR`"]
pub struct CLR_RDPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_RDPERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `CLR_OPERR`"]
pub type CLR_OPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_OPERR`"]
pub struct CLR_OPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_OPERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CLR_INCERR`"]
pub type CLR_INCERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_INCERR`"]
pub struct CLR_INCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_INCERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `CLR_STRBERR`"]
pub type CLR_STRBERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_STRBERR`"]
pub struct CLR_STRBERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_STRBERR_W<'a> {
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
#[doc = "Reader of field `CLR_PGSERR`"]
pub type CLR_PGSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_PGSERR`"]
pub struct CLR_PGSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_PGSERR_W<'a> {
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
#[doc = "Reader of field `CLR_WRPERR`"]
pub type CLR_WRPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_WRPERR`"]
pub struct CLR_WRPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_WRPERR_W<'a> {
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
#[doc = "Reader of field `CLR_EOP`"]
pub type CLR_EOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_EOP`"]
pub struct CLR_EOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_EOP_W<'a> {
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
impl R {
    #[doc = "Bit 28 - Bank 1 CRCRDERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_crcrderr(&self) -> CLR_CRCRDERR_R {
        CLR_CRCRDERR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Bank 1 CRCEND1 flag clear bit"]
    #[inline(always)]
    pub fn clr_crcend(&self) -> CLR_CRCEND_R {
        CLR_CRCEND_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_dbeccerr(&self) -> CLR_DBECCERR_R {
        CLR_DBECCERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bank 1 SNECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_sneccerr(&self) -> CLR_SNECCERR_R {
        CLR_SNECCERR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdserr(&self) -> CLR_RDSERR_R {
        CLR_RDSERR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Bank 1 RDPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdperr(&self) -> CLR_RDPERR_R {
        CLR_RDPERR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Bank 1 OPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_operr(&self) -> CLR_OPERR_R {
        CLR_OPERR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bank 1 INCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_incerr(&self) -> CLR_INCERR_R {
        CLR_INCERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Bank 1 STRBERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_strberr(&self) -> CLR_STRBERR_R {
        CLR_STRBERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Bank 1 PGSERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_pgserr(&self) -> CLR_PGSERR_R {
        CLR_PGSERR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Bank 1 WRPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_wrperr(&self) -> CLR_WRPERR_R {
        CLR_WRPERR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    pub fn clr_eop(&self) -> CLR_EOP_R {
        CLR_EOP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Bank 1 CRCRDERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_crcrderr(&mut self) -> CLR_CRCRDERR_W {
        CLR_CRCRDERR_W { w: self }
    }
    #[doc = "Bit 27 - Bank 1 CRCEND1 flag clear bit"]
    #[inline(always)]
    pub fn clr_crcend(&mut self) -> CLR_CRCEND_W {
        CLR_CRCEND_W { w: self }
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_dbeccerr(&mut self) -> CLR_DBECCERR_W {
        CLR_DBECCERR_W { w: self }
    }
    #[doc = "Bit 25 - Bank 1 SNECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_sneccerr(&mut self) -> CLR_SNECCERR_W {
        CLR_SNECCERR_W { w: self }
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdserr(&mut self) -> CLR_RDSERR_W {
        CLR_RDSERR_W { w: self }
    }
    #[doc = "Bit 23 - Bank 1 RDPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdperr(&mut self) -> CLR_RDPERR_W {
        CLR_RDPERR_W { w: self }
    }
    #[doc = "Bit 22 - Bank 1 OPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_operr(&mut self) -> CLR_OPERR_W {
        CLR_OPERR_W { w: self }
    }
    #[doc = "Bit 21 - Bank 1 INCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_incerr(&mut self) -> CLR_INCERR_W {
        CLR_INCERR_W { w: self }
    }
    #[doc = "Bit 19 - Bank 1 STRBERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_strberr(&mut self) -> CLR_STRBERR_W {
        CLR_STRBERR_W { w: self }
    }
    #[doc = "Bit 18 - Bank 1 PGSERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_pgserr(&mut self) -> CLR_PGSERR_W {
        CLR_PGSERR_W { w: self }
    }
    #[doc = "Bit 17 - Bank 1 WRPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_wrperr(&mut self) -> CLR_WRPERR_W {
        CLR_WRPERR_W { w: self }
    }
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    pub fn clr_eop(&mut self) -> CLR_EOP_W {
        CLR_EOP_W { w: self }
    }
}
