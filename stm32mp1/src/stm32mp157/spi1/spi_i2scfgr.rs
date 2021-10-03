#[doc = "Register `SPI_I2SCFGR` reader"]
pub struct R(crate::R<SPI_I2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_I2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_I2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_I2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_I2SCFGR` writer"]
pub struct W(crate::W<SPI_I2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_I2SCFGR_SPEC>;
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
impl From<crate::W<SPI_I2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_I2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2SMOD` reader - I2SMOD"]
pub struct I2SMOD_R(crate::FieldReader<bool, bool>);
impl I2SMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2SMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2SMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SMOD` writer - I2SMOD"]
pub struct I2SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SMOD_W<'a> {
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
#[doc = "Field `I2SCFG` reader - I2SCFG"]
pub struct I2SCFG_R(crate::FieldReader<u8, u8>);
impl I2SCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2SCFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SCFG` writer - I2SCFG"]
pub struct I2SCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `I2SSTD` reader - I2SSTD"]
pub struct I2SSTD_R(crate::FieldReader<u8, u8>);
impl I2SSTD_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SSTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2SSTD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SSTD` writer - I2SSTD"]
pub struct I2SSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SSTD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `PCMSYNC` reader - PCMSYNC"]
pub struct PCMSYNC_R(crate::FieldReader<bool, bool>);
impl PCMSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCMSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCMSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCMSYNC` writer - PCMSYNC"]
pub struct PCMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMSYNC_W<'a> {
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
#[doc = "Field `DATLEN` reader - DATLEN"]
pub struct DATLEN_R(crate::FieldReader<u8, u8>);
impl DATLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATLEN` writer - DATLEN"]
pub struct DATLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CHLEN` reader - CHLEN"]
pub struct CHLEN_R(crate::FieldReader<bool, bool>);
impl CHLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHLEN` writer - CHLEN"]
pub struct CHLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHLEN_W<'a> {
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
#[doc = "Field `CKPOL` reader - CKPOL"]
pub struct CKPOL_R(crate::FieldReader<bool, bool>);
impl CKPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKPOL` writer - CKPOL"]
pub struct CKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `FIXCH` reader - FIXCH"]
pub struct FIXCH_R(crate::FieldReader<bool, bool>);
impl FIXCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIXCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIXCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIXCH` writer - FIXCH"]
pub struct FIXCH_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `WSINV` reader - WSINV"]
pub struct WSINV_R(crate::FieldReader<bool, bool>);
impl WSINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        WSINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WSINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WSINV` writer - WSINV"]
pub struct WSINV_W<'a> {
    w: &'a mut W,
}
impl<'a> WSINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `DATFMT` reader - DATFMT"]
pub struct DATFMT_R(crate::FieldReader<bool, bool>);
impl DATFMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATFMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATFMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATFMT` writer - DATFMT"]
pub struct DATFMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATFMT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `I2SDIV` reader - I2SDIV"]
pub struct I2SDIV_R(crate::FieldReader<u8, u8>);
impl I2SDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2SDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SDIV` writer - I2SDIV"]
pub struct I2SDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ODD` reader - ODD"]
pub struct ODD_R(crate::FieldReader<bool, bool>);
impl ODD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODD` writer - ODD"]
pub struct ODD_W<'a> {
    w: &'a mut W,
}
impl<'a> ODD_W<'a> {
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
#[doc = "Field `MCKOE` reader - MCKOE"]
pub struct MCKOE_R(crate::FieldReader<bool, bool>);
impl MCKOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCKOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCKOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKOE` writer - MCKOE"]
pub struct MCKOE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKOE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2SMOD"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - I2SCFG"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - I2SSTD"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - PCMSYNC"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - DATLEN"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - CHLEN"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CKPOL"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FIXCH"]
    #[inline(always)]
    pub fn fixch(&self) -> FIXCH_R {
        FIXCH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WSINV"]
    #[inline(always)]
    pub fn wsinv(&self) -> WSINV_R {
        WSINV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DATFMT"]
    #[inline(always)]
    pub fn datfmt(&self) -> DATFMT_R {
        DATFMT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - I2SDIV"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - ODD"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MCKOE"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2SMOD"]
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W {
        I2SMOD_W { w: self }
    }
    #[doc = "Bits 1:3 - I2SCFG"]
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W {
        I2SCFG_W { w: self }
    }
    #[doc = "Bits 4:5 - I2SSTD"]
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W {
        I2SSTD_W { w: self }
    }
    #[doc = "Bit 7 - PCMSYNC"]
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W {
        PCMSYNC_W { w: self }
    }
    #[doc = "Bits 8:9 - DATLEN"]
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W {
        DATLEN_W { w: self }
    }
    #[doc = "Bit 10 - CHLEN"]
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W {
        CHLEN_W { w: self }
    }
    #[doc = "Bit 11 - CKPOL"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W {
        CKPOL_W { w: self }
    }
    #[doc = "Bit 12 - FIXCH"]
    #[inline(always)]
    pub fn fixch(&mut self) -> FIXCH_W {
        FIXCH_W { w: self }
    }
    #[doc = "Bit 13 - WSINV"]
    #[inline(always)]
    pub fn wsinv(&mut self) -> WSINV_W {
        WSINV_W { w: self }
    }
    #[doc = "Bit 14 - DATFMT"]
    #[inline(always)]
    pub fn datfmt(&mut self) -> DATFMT_W {
        DATFMT_W { w: self }
    }
    #[doc = "Bits 16:23 - I2SDIV"]
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W {
        I2SDIV_W { w: self }
    }
    #[doc = "Bit 24 - ODD"]
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W {
        ODD_W { w: self }
    }
    #[doc = "Bit 25 - MCKOE"]
    #[inline(always)]
    pub fn mckoe(&mut self) -> MCKOE_W {
        MCKOE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "All documented bits in this register must be configured when the I2S is disabled (SPE = 0).These bits are not used in SPI mode except for I2SMOD which needs to be set to 0 in SPI mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_i2scfgr](index.html) module"]
pub struct SPI_I2SCFGR_SPEC;
impl crate::RegisterSpec for SPI_I2SCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_i2scfgr::R](R) reader structure"]
impl crate::Readable for SPI_I2SCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_i2scfgr::W](W) writer structure"]
impl crate::Writable for SPI_I2SCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_I2SCFGR to value 0"]
impl crate::Resettable for SPI_I2SCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
