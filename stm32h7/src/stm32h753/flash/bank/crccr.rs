#[doc = "Register `CRCCR` reader"]
pub struct R(crate::R<CRCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCCR` writer"]
pub struct W(crate::W<CRCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCCR_SPEC>;
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
impl From<crate::W<CRCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_SECT` reader - Bank 1 CRC sector number"]
pub struct CRC_SECT_R(crate::FieldReader<u8, u8>);
impl CRC_SECT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRC_SECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_SECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_SECT` writer - Bank 1 CRC sector number"]
pub struct CRC_SECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_SECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `ALL_BANK` reader - Bank 1 CRC select bit"]
pub struct ALL_BANK_R(crate::FieldReader<bool, bool>);
impl ALL_BANK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALL_BANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALL_BANK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALL_BANK` writer - Bank 1 CRC select bit"]
pub struct ALL_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_BANK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CRC_BY_SECT` reader - Bank 1 CRC sector mode select bit"]
pub struct CRC_BY_SECT_R(crate::FieldReader<bool, bool>);
impl CRC_BY_SECT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_BY_SECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_BY_SECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_BY_SECT` writer - Bank 1 CRC sector mode select bit"]
pub struct CRC_BY_SECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_BY_SECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `ADD_SECT` reader - Bank 1 CRC sector select bit"]
pub struct ADD_SECT_R(crate::FieldReader<bool, bool>);
impl ADD_SECT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD_SECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD_SECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD_SECT` writer - Bank 1 CRC sector select bit"]
pub struct ADD_SECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_SECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CLEAN_SECT` reader - Bank 1 CRC sector list clear bit"]
pub struct CLEAN_SECT_R(crate::FieldReader<bool, bool>);
impl CLEAN_SECT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLEAN_SECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLEAN_SECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLEAN_SECT` writer - Bank 1 CRC sector list clear bit"]
pub struct CLEAN_SECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAN_SECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `START_CRC` reader - Bank 1 CRC start bit"]
pub struct START_CRC_R(crate::FieldReader<bool, bool>);
impl START_CRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_CRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_CRC` writer - Bank 1 CRC start bit"]
pub struct START_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> START_CRC_W<'a> {
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
#[doc = "Field `CLEAN_CRC` reader - Bank 1 CRC clear bit"]
pub struct CLEAN_CRC_R(crate::FieldReader<bool, bool>);
impl CLEAN_CRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLEAN_CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLEAN_CRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLEAN_CRC` writer - Bank 1 CRC clear bit"]
pub struct CLEAN_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAN_CRC_W<'a> {
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
#[doc = "Field `CRC_BURST` reader - Bank 1 CRC burst size"]
pub struct CRC_BURST_R(crate::FieldReader<u8, u8>);
impl CRC_BURST_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRC_BURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_BURST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_BURST` writer - Bank 1 CRC burst size"]
pub struct CRC_BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_BURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Bank 1 CRC sector number"]
    #[inline(always)]
    pub fn crc_sect(&self) -> CRC_SECT_R {
        CRC_SECT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - Bank 1 CRC select bit"]
    #[inline(always)]
    pub fn all_bank(&self) -> ALL_BANK_R {
        ALL_BANK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bank 1 CRC sector mode select bit"]
    #[inline(always)]
    pub fn crc_by_sect(&self) -> CRC_BY_SECT_R {
        CRC_BY_SECT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Bank 1 CRC sector select bit"]
    #[inline(always)]
    pub fn add_sect(&self) -> ADD_SECT_R {
        ADD_SECT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Bank 1 CRC sector list clear bit"]
    #[inline(always)]
    pub fn clean_sect(&self) -> CLEAN_SECT_R {
        CLEAN_SECT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Bank 1 CRC start bit"]
    #[inline(always)]
    pub fn start_crc(&self) -> START_CRC_R {
        START_CRC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Bank 1 CRC clear bit"]
    #[inline(always)]
    pub fn clean_crc(&self) -> CLEAN_CRC_R {
        CLEAN_CRC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Bank 1 CRC burst size"]
    #[inline(always)]
    pub fn crc_burst(&self) -> CRC_BURST_R {
        CRC_BURST_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bank 1 CRC sector number"]
    #[inline(always)]
    pub fn crc_sect(&mut self) -> CRC_SECT_W {
        CRC_SECT_W { w: self }
    }
    #[doc = "Bit 7 - Bank 1 CRC select bit"]
    #[inline(always)]
    pub fn all_bank(&mut self) -> ALL_BANK_W {
        ALL_BANK_W { w: self }
    }
    #[doc = "Bit 8 - Bank 1 CRC sector mode select bit"]
    #[inline(always)]
    pub fn crc_by_sect(&mut self) -> CRC_BY_SECT_W {
        CRC_BY_SECT_W { w: self }
    }
    #[doc = "Bit 9 - Bank 1 CRC sector select bit"]
    #[inline(always)]
    pub fn add_sect(&mut self) -> ADD_SECT_W {
        ADD_SECT_W { w: self }
    }
    #[doc = "Bit 10 - Bank 1 CRC sector list clear bit"]
    #[inline(always)]
    pub fn clean_sect(&mut self) -> CLEAN_SECT_W {
        CLEAN_SECT_W { w: self }
    }
    #[doc = "Bit 16 - Bank 1 CRC start bit"]
    #[inline(always)]
    pub fn start_crc(&mut self) -> START_CRC_W {
        START_CRC_W { w: self }
    }
    #[doc = "Bit 17 - Bank 1 CRC clear bit"]
    #[inline(always)]
    pub fn clean_crc(&mut self) -> CLEAN_CRC_W {
        CLEAN_CRC_W { w: self }
    }
    #[doc = "Bits 20:21 - Bank 1 CRC burst size"]
    #[inline(always)]
    pub fn crc_burst(&mut self) -> CRC_BURST_W {
        CRC_BURST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH CRC control register for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crccr](index.html) module"]
pub struct CRCCR_SPEC;
impl crate::RegisterSpec for CRCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crccr::R](R) reader structure"]
impl crate::Readable for CRCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crccr::W](W) writer structure"]
impl crate::Writable for CRCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCCR to value 0"]
impl crate::Resettable for CRCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
