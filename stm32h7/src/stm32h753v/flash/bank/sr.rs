#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BSY`"]
pub type BSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSY`"]
pub struct BSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BSY_W<'a> {
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
#[doc = "Reader of field `WBNE`"]
pub type WBNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WBNE`"]
pub struct WBNE_W<'a> {
    w: &'a mut W,
}
impl<'a> WBNE_W<'a> {
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
#[doc = "Reader of field `QW`"]
pub type QW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QW`"]
pub struct QW_W<'a> {
    w: &'a mut W,
}
impl<'a> QW_W<'a> {
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
#[doc = "Reader of field `CRC_BUSY`"]
pub type CRC_BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC_BUSY`"]
pub struct CRC_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_BUSY_W<'a> {
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
#[doc = "Reader of field `EOP`"]
pub type EOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOP`"]
pub struct EOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EOP_W<'a> {
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
#[doc = "Reader of field `WRPERR`"]
pub type WRPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRPERR`"]
pub struct WRPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPERR_W<'a> {
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
#[doc = "Reader of field `PGSERR`"]
pub type PGSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGSERR`"]
pub struct PGSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PGSERR_W<'a> {
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
#[doc = "Reader of field `STRBERR`"]
pub type STRBERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STRBERR`"]
pub struct STRBERR_W<'a> {
    w: &'a mut W,
}
impl<'a> STRBERR_W<'a> {
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
#[doc = "Reader of field `INCERR`"]
pub type INCERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCERR`"]
pub struct INCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> INCERR_W<'a> {
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
#[doc = "Reader of field `OPERR`"]
pub type OPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPERR`"]
pub struct OPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERR_W<'a> {
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
#[doc = "Reader of field `RDPERR`"]
pub type RDPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDPERR`"]
pub struct RDPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDPERR_W<'a> {
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
#[doc = "Reader of field `RDSERR`"]
pub type RDSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDSERR`"]
pub struct RDSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSERR_W<'a> {
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
#[doc = "Reader of field `SNECCERR1`"]
pub type SNECCERR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SNECCERR1`"]
pub struct SNECCERR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SNECCERR1_W<'a> {
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
#[doc = "Reader of field `DBECCERR`"]
pub type DBECCERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBECCERR`"]
pub struct DBECCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBECCERR_W<'a> {
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
#[doc = "Reader of field `CRCEND`"]
pub type CRCEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCEND`"]
pub struct CRCEND_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEND_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Bank 1 ongoing program flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bank 1 write buffer not empty flag"]
    #[inline(always)]
    pub fn wbne(&self) -> WBNE_R {
        WBNE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bank 1 wait queue flag"]
    #[inline(always)]
    pub fn qw(&self) -> QW_R {
        QW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bank 1 CRC busy flag"]
    #[inline(always)]
    pub fn crc_busy(&self) -> CRC_BUSY_R {
        CRC_BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program flag"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Bank 1 write protection error flag"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error flag"]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Bank 1 strobe error flag"]
    #[inline(always)]
    pub fn strberr(&self) -> STRBERR_R {
        STRBERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error flag"]
    #[inline(always)]
    pub fn incerr(&self) -> INCERR_R {
        INCERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Bank 1 write/erase error flag"]
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Bank 1 read protection error flag"]
    #[inline(always)]
    pub fn rdperr(&self) -> RDPERR_R {
        RDPERR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Bank 1 secure error flag"]
    #[inline(always)]
    pub fn rdserr(&self) -> RDSERR_R {
        RDSERR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bank 1 single correction error flag"]
    #[inline(always)]
    pub fn sneccerr1(&self) -> SNECCERR1_R {
        SNECCERR1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error flag"]
    #[inline(always)]
    pub fn dbeccerr(&self) -> DBECCERR_R {
        DBECCERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Bank 1 CRC-complete flag"]
    #[inline(always)]
    pub fn crcend(&self) -> CRCEND_R {
        CRCEND_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 ongoing program flag"]
    #[inline(always)]
    pub fn bsy(&mut self) -> BSY_W {
        BSY_W { w: self }
    }
    #[doc = "Bit 1 - Bank 1 write buffer not empty flag"]
    #[inline(always)]
    pub fn wbne(&mut self) -> WBNE_W {
        WBNE_W { w: self }
    }
    #[doc = "Bit 2 - Bank 1 wait queue flag"]
    #[inline(always)]
    pub fn qw(&mut self) -> QW_W {
        QW_W { w: self }
    }
    #[doc = "Bit 3 - Bank 1 CRC busy flag"]
    #[inline(always)]
    pub fn crc_busy(&mut self) -> CRC_BUSY_W {
        CRC_BUSY_W { w: self }
    }
    #[doc = "Bit 16 - Bank 1 end-of-program flag"]
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W {
        EOP_W { w: self }
    }
    #[doc = "Bit 17 - Bank 1 write protection error flag"]
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W {
        WRPERR_W { w: self }
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error flag"]
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W {
        PGSERR_W { w: self }
    }
    #[doc = "Bit 19 - Bank 1 strobe error flag"]
    #[inline(always)]
    pub fn strberr(&mut self) -> STRBERR_W {
        STRBERR_W { w: self }
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error flag"]
    #[inline(always)]
    pub fn incerr(&mut self) -> INCERR_W {
        INCERR_W { w: self }
    }
    #[doc = "Bit 22 - Bank 1 write/erase error flag"]
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W {
        OPERR_W { w: self }
    }
    #[doc = "Bit 23 - Bank 1 read protection error flag"]
    #[inline(always)]
    pub fn rdperr(&mut self) -> RDPERR_W {
        RDPERR_W { w: self }
    }
    #[doc = "Bit 24 - Bank 1 secure error flag"]
    #[inline(always)]
    pub fn rdserr(&mut self) -> RDSERR_W {
        RDSERR_W { w: self }
    }
    #[doc = "Bit 25 - Bank 1 single correction error flag"]
    #[inline(always)]
    pub fn sneccerr1(&mut self) -> SNECCERR1_W {
        SNECCERR1_W { w: self }
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error flag"]
    #[inline(always)]
    pub fn dbeccerr(&mut self) -> DBECCERR_W {
        DBECCERR_W { w: self }
    }
    #[doc = "Bit 27 - Bank 1 CRC-complete flag"]
    #[inline(always)]
    pub fn crcend(&mut self) -> CRCEND_W {
        CRCEND_W { w: self }
    }
}
