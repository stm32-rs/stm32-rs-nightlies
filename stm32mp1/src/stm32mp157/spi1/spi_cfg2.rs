#[doc = "Register `SPI_CFG2` reader"]
pub struct R(crate::R<SPI_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CFG2` writer"]
pub struct W(crate::W<SPI_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CFG2_SPEC>;
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
impl From<crate::W<SPI_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSSI` reader - MSSI"]
pub struct MSSI_R(crate::FieldReader<u8, u8>);
impl MSSI_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSSI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSSI` writer - MSSI"]
pub struct MSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> MSSI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `MIDI` reader - MIDI"]
pub struct MIDI_R(crate::FieldReader<u8, u8>);
impl MIDI_R {
    pub(crate) fn new(bits: u8) -> Self {
        MIDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIDI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIDI` writer - MIDI"]
pub struct MIDI_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `IOSWP` reader - IOSWP"]
pub struct IOSWP_R(crate::FieldReader<bool, bool>);
impl IOSWP_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOSWP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOSWP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOSWP` writer - IOSWP"]
pub struct IOSWP_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSWP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `COMM` reader - COMM"]
pub struct COMM_R(crate::FieldReader<u8, u8>);
impl COMM_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMM` writer - COMM"]
pub struct COMM_W<'a> {
    w: &'a mut W,
}
impl<'a> COMM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `SP` reader - SP"]
pub struct SP_R(crate::FieldReader<u8, u8>);
impl SP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP` writer - SP"]
pub struct SP_W<'a> {
    w: &'a mut W,
}
impl<'a> SP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `MASTER` reader - MASTER"]
pub struct MASTER_R(crate::FieldReader<bool, bool>);
impl MASTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER` writer - MASTER"]
pub struct MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_W<'a> {
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
#[doc = "Field `LSBFRST` reader - LSBFRST"]
pub struct LSBFRST_R(crate::FieldReader<bool, bool>);
impl LSBFRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSBFRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSBFRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSBFRST` writer - LSBFRST"]
pub struct LSBFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBFRST_W<'a> {
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
#[doc = "Field `CPHA` reader - CPHA"]
pub struct CPHA_R(crate::FieldReader<bool, bool>);
impl CPHA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPHA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPHA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPHA` writer - CPHA"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
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
#[doc = "Field `CPOL` reader - CPOL"]
pub struct CPOL_R(crate::FieldReader<bool, bool>);
impl CPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - CPOL"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
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
#[doc = "Field `SSM` reader - SSM"]
pub struct SSM_R(crate::FieldReader<bool, bool>);
impl SSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSM` writer - SSM"]
pub struct SSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSM_W<'a> {
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
#[doc = "Field `SSIOP` reader - SSIOP"]
pub struct SSIOP_R(crate::FieldReader<bool, bool>);
impl SSIOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSIOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSIOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSIOP` writer - SSIOP"]
pub struct SSIOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIOP_W<'a> {
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
#[doc = "Field `SSOE` reader - SSOE"]
pub struct SSOE_R(crate::FieldReader<bool, bool>);
impl SSOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSOE` writer - SSOE"]
pub struct SSOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `SSOM` reader - SSOM"]
pub struct SSOM_R(crate::FieldReader<bool, bool>);
impl SSOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSOM` writer - SSOM"]
pub struct SSOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `AFCNTR` reader - AFCNTR"]
pub struct AFCNTR_R(crate::FieldReader<bool, bool>);
impl AFCNTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AFCNTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFCNTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFCNTR` writer - AFCNTR"]
pub struct AFCNTR_W<'a> {
    w: &'a mut W,
}
impl<'a> AFCNTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - MSSI"]
    #[inline(always)]
    pub fn mssi(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MIDI"]
    #[inline(always)]
    pub fn midi(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - IOSWP"]
    #[inline(always)]
    pub fn ioswp(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - COMM"]
    #[inline(always)]
    pub fn comm(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:21 - SP"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 22 - MASTER"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LSBFRST"]
    #[inline(always)]
    pub fn lsbfrst(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CPHA"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CPOL"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SSM"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SSIOP"]
    #[inline(always)]
    pub fn ssiop(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SSOE"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SSOM"]
    #[inline(always)]
    pub fn ssom(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - AFCNTR"]
    #[inline(always)]
    pub fn afcntr(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MSSI"]
    #[inline(always)]
    pub fn mssi(&mut self) -> MSSI_W {
        MSSI_W { w: self }
    }
    #[doc = "Bits 4:7 - MIDI"]
    #[inline(always)]
    pub fn midi(&mut self) -> MIDI_W {
        MIDI_W { w: self }
    }
    #[doc = "Bit 15 - IOSWP"]
    #[inline(always)]
    pub fn ioswp(&mut self) -> IOSWP_W {
        IOSWP_W { w: self }
    }
    #[doc = "Bits 17:18 - COMM"]
    #[inline(always)]
    pub fn comm(&mut self) -> COMM_W {
        COMM_W { w: self }
    }
    #[doc = "Bits 19:21 - SP"]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W {
        SP_W { w: self }
    }
    #[doc = "Bit 22 - MASTER"]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W {
        MASTER_W { w: self }
    }
    #[doc = "Bit 23 - LSBFRST"]
    #[inline(always)]
    pub fn lsbfrst(&mut self) -> LSBFRST_W {
        LSBFRST_W { w: self }
    }
    #[doc = "Bit 24 - CPHA"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 25 - CPOL"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 26 - SSM"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W {
        SSM_W { w: self }
    }
    #[doc = "Bit 28 - SSIOP"]
    #[inline(always)]
    pub fn ssiop(&mut self) -> SSIOP_W {
        SSIOP_W { w: self }
    }
    #[doc = "Bit 29 - SSOE"]
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W {
        SSOE_W { w: self }
    }
    #[doc = "Bit 30 - SSOM"]
    #[inline(always)]
    pub fn ssom(&mut self) -> SSOM_W {
        SSOM_W { w: self }
    }
    #[doc = "Bit 31 - AFCNTR"]
    #[inline(always)]
    pub fn afcntr(&mut self) -> AFCNTR_W {
        AFCNTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The content of this register is write protected when SPI is enabled or IOLOCK bit is set at SPI2S_CR1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cfg2](index.html) module"]
pub struct SPI_CFG2_SPEC;
impl crate::RegisterSpec for SPI_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_cfg2::R](R) reader structure"]
impl crate::Readable for SPI_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_cfg2::W](W) writer structure"]
impl crate::Writable for SPI_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CFG2 to value 0"]
impl crate::Resettable for SPI_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
