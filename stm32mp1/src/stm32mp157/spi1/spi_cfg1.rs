#[doc = "Register `SPI_CFG1` reader"]
pub struct R(crate::R<SPI_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CFG1` writer"]
pub struct W(crate::W<SPI_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CFG1_SPEC>;
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
impl From<crate::W<SPI_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSIZE` reader - DSIZE"]
pub struct DSIZE_R(crate::FieldReader<u8, u8>);
impl DSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSIZE` writer - DSIZE"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `FTHLV` reader - FTHLV"]
pub struct FTHLV_R(crate::FieldReader<u8, u8>);
impl FTHLV_R {
    pub(crate) fn new(bits: u8) -> Self {
        FTHLV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTHLV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTHLV` writer - FTHLV"]
pub struct FTHLV_W<'a> {
    w: &'a mut W,
}
impl<'a> FTHLV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `UDRCFG` reader - UDRCFG"]
pub struct UDRCFG_R(crate::FieldReader<u8, u8>);
impl UDRCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        UDRCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDRCFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDRCFG` writer - UDRCFG"]
pub struct UDRCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `UDRDET` reader - UDRDET"]
pub struct UDRDET_R(crate::FieldReader<u8, u8>);
impl UDRDET_R {
    pub(crate) fn new(bits: u8) -> Self {
        UDRDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDRDET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDRDET` writer - UDRDET"]
pub struct UDRDET_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRDET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `RXDMAEN` reader - RXDMAEN"]
pub struct RXDMAEN_R(crate::FieldReader<bool, bool>);
impl RXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMAEN` writer - RXDMAEN"]
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
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
#[doc = "Field `TXDMAEN` reader - TXDMAEN"]
pub struct TXDMAEN_R(crate::FieldReader<bool, bool>);
impl TXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMAEN` writer - TXDMAEN"]
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
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
#[doc = "Field `CRCSIZE` reader - CRCSIZE"]
pub struct CRCSIZE_R(crate::FieldReader<u8, u8>);
impl CRCSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRCSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCSIZE` writer - CRCSIZE"]
pub struct CRCSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `CRCEN` reader - CRCEN"]
pub struct CRCEN_R(crate::FieldReader<bool, bool>);
impl CRCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCEN` writer - CRCEN"]
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
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
#[doc = "Field `MBR` reader - MBR"]
pub struct MBR_R(crate::FieldReader<u8, u8>);
impl MBR_R {
    pub(crate) fn new(bits: u8) -> Self {
        MBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MBR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MBR` writer - MBR"]
pub struct MBR_W<'a> {
    w: &'a mut W,
}
impl<'a> MBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DSIZE"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - FTHLV"]
    #[inline(always)]
    pub fn fthlv(&self) -> FTHLV_R {
        FTHLV_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10 - UDRCFG"]
    #[inline(always)]
    pub fn udrcfg(&self) -> UDRCFG_R {
        UDRCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - UDRDET"]
    #[inline(always)]
    pub fn udrdet(&self) -> UDRDET_R {
        UDRDET_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 14 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - CRCSIZE"]
    #[inline(always)]
    pub fn crcsize(&self) -> CRCSIZE_R {
        CRCSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - CRCEN"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - MBR"]
    #[inline(always)]
    pub fn mbr(&self) -> MBR_R {
        MBR_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DSIZE"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bits 5:8 - FTHLV"]
    #[inline(always)]
    pub fn fthlv(&mut self) -> FTHLV_W {
        FTHLV_W { w: self }
    }
    #[doc = "Bits 9:10 - UDRCFG"]
    #[inline(always)]
    pub fn udrcfg(&mut self) -> UDRCFG_W {
        UDRCFG_W { w: self }
    }
    #[doc = "Bits 11:12 - UDRDET"]
    #[inline(always)]
    pub fn udrdet(&mut self) -> UDRDET_W {
        UDRDET_W { w: self }
    }
    #[doc = "Bit 14 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    #[doc = "Bit 15 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    #[doc = "Bits 16:20 - CRCSIZE"]
    #[inline(always)]
    pub fn crcsize(&mut self) -> CRCSIZE_W {
        CRCSIZE_W { w: self }
    }
    #[doc = "Bit 22 - CRCEN"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    #[doc = "Bits 28:30 - MBR"]
    #[inline(always)]
    pub fn mbr(&mut self) -> MBR_W {
        MBR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Content of this register is write protected when SPI is enabled\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cfg1](index.html) module"]
pub struct SPI_CFG1_SPEC;
impl crate::RegisterSpec for SPI_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_cfg1::R](R) reader structure"]
impl crate::Readable for SPI_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_cfg1::W](W) writer structure"]
impl crate::Writable for SPI_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CFG1 to value 0x0007_0007"]
impl crate::Resettable for SPI_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0007_0007
    }
}
