#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR_CRCRDERR` reader - Bank 1 CRCRDERR1 flag clear bit"]
pub struct CLR_CRCRDERR_R(crate::FieldReader<bool, bool>);
impl CLR_CRCRDERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_CRCRDERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_CRCRDERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_CRCRDERR` writer - Bank 1 CRCRDERR1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CLR_CRCEND` reader - Bank 1 CRCEND1 flag clear bit"]
pub struct CLR_CRCEND_R(crate::FieldReader<bool, bool>);
impl CLR_CRCEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_CRCEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_CRCEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_CRCEND` writer - Bank 1 CRCEND1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `CLR_DBECCERR` reader - Bank 1 DBECCERR1 flag clear bit"]
pub struct CLR_DBECCERR_R(crate::FieldReader<bool, bool>);
impl CLR_DBECCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_DBECCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_DBECCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_DBECCERR` writer - Bank 1 DBECCERR1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CLR_SNECCERR` reader - Bank 1 SNECCERR1 flag clear bit"]
pub struct CLR_SNECCERR_R(crate::FieldReader<bool, bool>);
impl CLR_SNECCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_SNECCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_SNECCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_SNECCERR` writer - Bank 1 SNECCERR1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `CLR_RDSERR` reader - Bank 1 RDSERR1 flag clear bit"]
pub struct CLR_RDSERR_R(crate::FieldReader<bool, bool>);
impl CLR_RDSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_RDSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_RDSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_RDSERR` writer - Bank 1 RDSERR1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CLR_RDPERR` reader - Bank 1 RDPERR1 flag clear bit"]
pub struct CLR_RDPERR_R(crate::FieldReader<bool, bool>);
impl CLR_RDPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_RDPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_RDPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_RDPERR` writer - Bank 1 RDPERR1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `CLR_OPERR` reader - Bank 1 OPERR1 flag clear bit"]
pub struct CLR_OPERR_R(crate::FieldReader<bool, bool>);
impl CLR_OPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_OPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_OPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_OPERR` writer - Bank 1 OPERR1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CLR_INCERR` reader - Bank 1 INCERR1 flag clear bit"]
pub struct CLR_INCERR_R(crate::FieldReader<bool, bool>);
impl CLR_INCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_INCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_INCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_INCERR` writer - Bank 1 INCERR1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CLR_STRBERR` reader - Bank 1 STRBERR1 flag clear bit"]
pub struct CLR_STRBERR_R(crate::FieldReader<bool, bool>);
impl CLR_STRBERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_STRBERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_STRBERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_STRBERR` writer - Bank 1 STRBERR1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CLR_PGSERR` reader - Bank 1 PGSERR1 flag clear bit"]
pub struct CLR_PGSERR_R(crate::FieldReader<bool, bool>);
impl CLR_PGSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_PGSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_PGSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_PGSERR` writer - Bank 1 PGSERR1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CLR_WRPERR` reader - Bank 1 WRPERR1 flag clear bit"]
pub struct CLR_WRPERR_R(crate::FieldReader<bool, bool>);
impl CLR_WRPERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_WRPERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_WRPERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_WRPERR` writer - Bank 1 WRPERR1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CLR_EOP` reader - Bank 1 EOP1 flag clear bit"]
pub struct CLR_EOP_R(crate::FieldReader<bool, bool>);
impl CLR_EOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_EOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_EOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_EOP` writer - Bank 1 EOP1 flag clear bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH clear control register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
