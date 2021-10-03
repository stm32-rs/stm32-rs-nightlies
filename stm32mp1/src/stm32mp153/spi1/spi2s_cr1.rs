#[doc = "Register `SPI2S_CR1` reader"]
pub struct R(crate::R<SPI2S_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI2S_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI2S_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI2S_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI2S_CR1` writer"]
pub struct W(crate::W<SPI2S_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI2S_CR1_SPEC>;
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
impl From<crate::W<SPI2S_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI2S_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPE` reader - SPE"]
pub struct SPE_R(crate::FieldReader<bool, bool>);
impl SPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPE` writer - SPE"]
pub struct SPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPE_W<'a> {
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
#[doc = "Field `MASRX` reader - MASRX"]
pub struct MASRX_R(crate::FieldReader<bool, bool>);
impl MASRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASRX` writer - MASRX"]
pub struct MASRX_W<'a> {
    w: &'a mut W,
}
impl<'a> MASRX_W<'a> {
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
#[doc = "Field `CSTART` reader - CSTART"]
pub struct CSTART_R(crate::FieldReader<bool, bool>);
impl CSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSTART` writer - CSTART"]
pub struct CSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTART_W<'a> {
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
#[doc = "Field `CSUSP` writer - CSUSP"]
pub struct CSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSUSP_W<'a> {
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
#[doc = "Field `HDDIR` reader - HDDIR"]
pub struct HDDIR_R(crate::FieldReader<bool, bool>);
impl HDDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDDIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDDIR` writer - HDDIR"]
pub struct HDDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> HDDIR_W<'a> {
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
#[doc = "Field `SSI` reader - SSI"]
pub struct SSI_R(crate::FieldReader<bool, bool>);
impl SSI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSI` writer - SSI"]
pub struct SSI_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_W<'a> {
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
#[doc = "Field `CRC33_17` reader - CRC33_17"]
pub struct CRC33_17_R(crate::FieldReader<bool, bool>);
impl CRC33_17_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC33_17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC33_17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC33_17` writer - CRC33_17"]
pub struct CRC33_17_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC33_17_W<'a> {
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
#[doc = "Field `RCRCINI` reader - RCRCINI"]
pub struct RCRCINI_R(crate::FieldReader<bool, bool>);
impl RCRCINI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCRCINI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCRCINI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRCINI` writer - RCRCINI"]
pub struct RCRCINI_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRCINI_W<'a> {
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
#[doc = "Field `TCRCINI` reader - TCRCINI"]
pub struct TCRCINI_R(crate::FieldReader<bool, bool>);
impl TCRCINI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCRCINI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCRCINI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCRCINI` writer - TCRCINI"]
pub struct TCRCINI_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRCINI_W<'a> {
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
#[doc = "Field `IOLOCK` reader - IOLOCK"]
pub struct IOLOCK_R(crate::FieldReader<bool, bool>);
impl IOLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOLOCK` writer - IOLOCK"]
pub struct IOLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLOCK_W<'a> {
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
    #[doc = "Bit 0 - SPE"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - MASRX"]
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CSTART"]
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HDDIR"]
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SSI"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CRC33_17"]
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RCRCINI"]
    #[inline(always)]
    pub fn rcrcini(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TCRCINI"]
    #[inline(always)]
    pub fn tcrcini(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IOLOCK"]
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPE"]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W {
        SPE_W { w: self }
    }
    #[doc = "Bit 8 - MASRX"]
    #[inline(always)]
    pub fn masrx(&mut self) -> MASRX_W {
        MASRX_W { w: self }
    }
    #[doc = "Bit 9 - CSTART"]
    #[inline(always)]
    pub fn cstart(&mut self) -> CSTART_W {
        CSTART_W { w: self }
    }
    #[doc = "Bit 10 - CSUSP"]
    #[inline(always)]
    pub fn csusp(&mut self) -> CSUSP_W {
        CSUSP_W { w: self }
    }
    #[doc = "Bit 11 - HDDIR"]
    #[inline(always)]
    pub fn hddir(&mut self) -> HDDIR_W {
        HDDIR_W { w: self }
    }
    #[doc = "Bit 12 - SSI"]
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W {
        SSI_W { w: self }
    }
    #[doc = "Bit 13 - CRC33_17"]
    #[inline(always)]
    pub fn crc33_17(&mut self) -> CRC33_17_W {
        CRC33_17_W { w: self }
    }
    #[doc = "Bit 14 - RCRCINI"]
    #[inline(always)]
    pub fn rcrcini(&mut self) -> RCRCINI_W {
        RCRCINI_W { w: self }
    }
    #[doc = "Bit 15 - TCRCINI"]
    #[inline(always)]
    pub fn tcrcini(&mut self) -> TCRCINI_W {
        TCRCINI_W { w: self }
    }
    #[doc = "Bit 16 - IOLOCK"]
    #[inline(always)]
    pub fn iolock(&mut self) -> IOLOCK_W {
        IOLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI/I2S control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_cr1](index.html) module"]
pub struct SPI2S_CR1_SPEC;
impl crate::RegisterSpec for SPI2S_CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi2s_cr1::R](R) reader structure"]
impl crate::Readable for SPI2S_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi2s_cr1::W](W) writer structure"]
impl crate::Writable for SPI2S_CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI2S_CR1 to value 0"]
impl crate::Resettable for SPI2S_CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
