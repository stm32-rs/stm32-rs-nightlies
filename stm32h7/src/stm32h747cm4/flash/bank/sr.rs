#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCRDERR` reader - Bank 1 CRC read error flag"]
pub struct CRCRDERR_R(crate::FieldReader<bool, bool>);
impl CRCRDERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCRDERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCRDERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCRDERR` writer - Bank 1 CRC read error flag"]
pub struct CRCRDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCRDERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CRCEND` reader - Bank 1 CRC end of calculation flag"]
pub struct CRCEND_R(crate::FieldReader<bool, bool>);
impl CRCEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCEND` writer - Bank 1 CRC end of calculation flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `DBECCERR` reader - Bank 1 ECC double detection error flag"]
pub struct DBECCERR_R(crate::FieldReader<bool, bool>);
impl DBECCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBECCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBECCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBECCERR` writer - Bank 1 ECC double detection error flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `SNECCERR` reader - Bank 1 single correction error flag"]
pub struct SNECCERR_R(crate::FieldReader<bool, bool>);
impl SNECCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SNECCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNECCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNECCERR` writer - Bank 1 single correction error flag"]
pub struct SNECCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SNECCERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `RDSERR` reader - Bank 1 secure error flag"]
pub struct RDSERR_R(crate::FieldReader<bool, bool>);
impl RDSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDSERR` writer - Bank 1 secure error flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `RDPERR` reader - Bank 1 read protection error flag"]
pub struct RDPERR_R(crate::FieldReader<bool, bool>);
impl RDPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDPERR` writer - Bank 1 read protection error flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `OPERR` reader - Bank 1 write"]
pub struct OPERR_R(crate::FieldReader<bool, bool>);
impl OPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPERR` writer - Bank 1 write"]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `INCERR` reader - Bank 1 inconsistency error flag"]
pub struct INCERR_R(crate::FieldReader<bool, bool>);
impl INCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INCERR` writer - Bank 1 inconsistency error flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `STRBERR` reader - Bank 1 strobe error flag"]
pub struct STRBERR_R(crate::FieldReader<bool, bool>);
impl STRBERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        STRBERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRBERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRBERR` writer - Bank 1 strobe error flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PGSERR` reader - Bank 1 programming sequence error flag"]
pub struct PGSERR_R(crate::FieldReader<bool, bool>);
impl PGSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGSERR` writer - Bank 1 programming sequence error flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `WRPERR` reader - Bank 1 write protection error flag"]
pub struct WRPERR_R(crate::FieldReader<bool, bool>);
impl WRPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRPERR` writer - Bank 1 write protection error flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `EOP` reader - Bank 1 end-of-program flag"]
pub struct EOP_R(crate::FieldReader<bool, bool>);
impl EOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOP` writer - Bank 1 end-of-program flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CRC_BUSY` reader - Bank 1 CRC busy flag"]
pub struct CRC_BUSY_R(crate::FieldReader<bool, bool>);
impl CRC_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_BUSY` writer - Bank 1 CRC busy flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `QW` reader - Bank 1 wait queue flag"]
pub struct QW_R(crate::FieldReader<bool, bool>);
impl QW_R {
    pub(crate) fn new(bits: bool) -> Self {
        QW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QW` writer - Bank 1 wait queue flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `WBNE` reader - Bank 1 write buffer not empty flag"]
pub struct WBNE_R(crate::FieldReader<bool, bool>);
impl WBNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WBNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WBNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WBNE` writer - Bank 1 write buffer not empty flag"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `BSY` reader - Bank 1 busy flag"]
pub struct BSY_R(crate::FieldReader<bool, bool>);
impl BSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSY` writer - Bank 1 busy flag"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 28 - Bank 1 CRC read error flag"]
    #[inline(always)]
    pub fn crcrderr(&self) -> CRCRDERR_R {
        CRCRDERR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Bank 1 CRC end of calculation flag"]
    #[inline(always)]
    pub fn crcend(&self) -> CRCEND_R {
        CRCEND_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error flag"]
    #[inline(always)]
    pub fn dbeccerr(&self) -> DBECCERR_R {
        DBECCERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bank 1 single correction error flag"]
    #[inline(always)]
    pub fn sneccerr(&self) -> SNECCERR_R {
        SNECCERR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Bank 1 secure error flag"]
    #[inline(always)]
    pub fn rdserr(&self) -> RDSERR_R {
        RDSERR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Bank 1 read protection error flag"]
    #[inline(always)]
    pub fn rdperr(&self) -> RDPERR_R {
        RDPERR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Bank 1 write"]
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error flag"]
    #[inline(always)]
    pub fn incerr(&self) -> INCERR_R {
        INCERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Bank 1 strobe error flag"]
    #[inline(always)]
    pub fn strberr(&self) -> STRBERR_R {
        STRBERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error flag"]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Bank 1 write protection error flag"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program flag"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bank 1 CRC busy flag"]
    #[inline(always)]
    pub fn crc_busy(&self) -> CRC_BUSY_R {
        CRC_BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bank 1 wait queue flag"]
    #[inline(always)]
    pub fn qw(&self) -> QW_R {
        QW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bank 1 write buffer not empty flag"]
    #[inline(always)]
    pub fn wbne(&self) -> WBNE_R {
        WBNE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Bank 1 busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Bank 1 CRC read error flag"]
    #[inline(always)]
    pub fn crcrderr(&mut self) -> CRCRDERR_W {
        CRCRDERR_W { w: self }
    }
    #[doc = "Bit 27 - Bank 1 CRC end of calculation flag"]
    #[inline(always)]
    pub fn crcend(&mut self) -> CRCEND_W {
        CRCEND_W { w: self }
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error flag"]
    #[inline(always)]
    pub fn dbeccerr(&mut self) -> DBECCERR_W {
        DBECCERR_W { w: self }
    }
    #[doc = "Bit 25 - Bank 1 single correction error flag"]
    #[inline(always)]
    pub fn sneccerr(&mut self) -> SNECCERR_W {
        SNECCERR_W { w: self }
    }
    #[doc = "Bit 24 - Bank 1 secure error flag"]
    #[inline(always)]
    pub fn rdserr(&mut self) -> RDSERR_W {
        RDSERR_W { w: self }
    }
    #[doc = "Bit 23 - Bank 1 read protection error flag"]
    #[inline(always)]
    pub fn rdperr(&mut self) -> RDPERR_W {
        RDPERR_W { w: self }
    }
    #[doc = "Bit 22 - Bank 1 write"]
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W {
        OPERR_W { w: self }
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error flag"]
    #[inline(always)]
    pub fn incerr(&mut self) -> INCERR_W {
        INCERR_W { w: self }
    }
    #[doc = "Bit 19 - Bank 1 strobe error flag"]
    #[inline(always)]
    pub fn strberr(&mut self) -> STRBERR_W {
        STRBERR_W { w: self }
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error flag"]
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W {
        PGSERR_W { w: self }
    }
    #[doc = "Bit 17 - Bank 1 write protection error flag"]
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W {
        WRPERR_W { w: self }
    }
    #[doc = "Bit 16 - Bank 1 end-of-program flag"]
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W {
        EOP_W { w: self }
    }
    #[doc = "Bit 3 - Bank 1 CRC busy flag"]
    #[inline(always)]
    pub fn crc_busy(&mut self) -> CRC_BUSY_W {
        CRC_BUSY_W { w: self }
    }
    #[doc = "Bit 2 - Bank 1 wait queue flag"]
    #[inline(always)]
    pub fn qw(&mut self) -> QW_W {
        QW_W { w: self }
    }
    #[doc = "Bit 1 - Bank 1 write buffer not empty flag"]
    #[inline(always)]
    pub fn wbne(&mut self) -> WBNE_W {
        WBNE_W { w: self }
    }
    #[doc = "Bit 0 - Bank 1 busy flag"]
    #[inline(always)]
    pub fn bsy(&mut self) -> BSY_W {
        BSY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH status register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
