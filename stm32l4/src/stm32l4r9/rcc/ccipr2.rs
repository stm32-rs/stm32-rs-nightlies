#[doc = "Register `CCIPR2` reader"]
pub struct R(crate::R<CCIPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR2` writer"]
pub struct W(crate::W<CCIPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR2_SPEC>;
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
impl From<crate::W<CCIPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C4SEL` reader - I2C4 clock source selection"]
pub struct I2C4SEL_R(crate::FieldReader<u8, u8>);
impl I2C4SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C4SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4SEL` writer - I2C4 clock source selection"]
pub struct I2C4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DFSDMSEL` reader - Digital filter for sigma delta modulator kernel clock source selection"]
pub struct DFSDMSEL_R(crate::FieldReader<bool, bool>);
impl DFSDMSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDMSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDMSEL` writer - Digital filter for sigma delta modulator kernel clock source selection"]
pub struct DFSDMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMSEL_W<'a> {
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
#[doc = "Field `ADFSDMSEL` reader - Digital filter for sigma delta modulator audio clock source selection"]
pub struct ADFSDMSEL_R(crate::FieldReader<u8, u8>);
impl ADFSDMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADFSDMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADFSDMSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADFSDMSEL` writer - Digital filter for sigma delta modulator audio clock source selection"]
pub struct ADFSDMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFSDMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `SAI1SEL` reader - SAI1 clock source selection"]
pub struct SAI1SEL_R(crate::FieldReader<u8, u8>);
impl SAI1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAI1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1SEL` writer - SAI1 clock source selection"]
pub struct SAI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `SAI2SEL` reader - SAI2 clock source selection"]
pub struct SAI2SEL_R(crate::FieldReader<u8, u8>);
impl SAI2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAI2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2SEL` writer - SAI2 clock source selection"]
pub struct SAI2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `DSISEL` reader - clock selection"]
pub struct DSISEL_R(crate::FieldReader<bool, bool>);
impl DSISEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSISEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSISEL` writer - clock selection"]
pub struct DSISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSISEL_W<'a> {
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
#[doc = "Field `SDMMCSEL` reader - SDMMC clock selection"]
pub struct SDMMCSEL_R(crate::FieldReader<bool, bool>);
impl SDMMCSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMCSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMCSEL` writer - SDMMC clock selection"]
pub struct SDMMCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMCSEL_W<'a> {
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
#[doc = "Field `PLLSAI2DIVR` reader - division factor for LTDC clock"]
pub struct PLLSAI2DIVR_R(crate::FieldReader<u8, u8>);
impl PLLSAI2DIVR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSAI2DIVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2DIVR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2DIVR` writer - division factor for LTDC clock"]
pub struct PLLSAI2DIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2DIVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `OSPISEL` reader - Octospi clock source selection"]
pub struct OSPISEL_R(crate::FieldReader<u8, u8>);
impl OSPISEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSPISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPISEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSPISEL` writer - Octospi clock source selection"]
pub struct OSPISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Digital filter for sigma delta modulator kernel clock source selection"]
    #[inline(always)]
    pub fn dfsdmsel(&self) -> DFSDMSEL_R {
        DFSDMSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection"]
    #[inline(always)]
    pub fn adfsdmsel(&self) -> ADFSDMSEL_R {
        ADFSDMSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - clock selection"]
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SDMMC clock selection"]
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - division factor for LTDC clock"]
    #[inline(always)]
    pub fn pllsai2divr(&self) -> PLLSAI2DIVR_R {
        PLLSAI2DIVR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    pub fn ospisel(&self) -> OSPISEL_R {
        OSPISEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W {
        I2C4SEL_W { w: self }
    }
    #[doc = "Bit 2 - Digital filter for sigma delta modulator kernel clock source selection"]
    #[inline(always)]
    pub fn dfsdmsel(&mut self) -> DFSDMSEL_W {
        DFSDMSEL_W { w: self }
    }
    #[doc = "Bits 3:4 - Digital filter for sigma delta modulator audio clock source selection"]
    #[inline(always)]
    pub fn adfsdmsel(&mut self) -> ADFSDMSEL_W {
        ADFSDMSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W {
        SAI1SEL_W { w: self }
    }
    #[doc = "Bits 8:10 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W {
        SAI2SEL_W { w: self }
    }
    #[doc = "Bit 12 - clock selection"]
    #[inline(always)]
    pub fn dsisel(&mut self) -> DSISEL_W {
        DSISEL_W { w: self }
    }
    #[doc = "Bit 14 - SDMMC clock selection"]
    #[inline(always)]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W {
        SDMMCSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - division factor for LTDC clock"]
    #[inline(always)]
    pub fn pllsai2divr(&mut self) -> PLLSAI2DIVR_W {
        PLLSAI2DIVR_W { w: self }
    }
    #[doc = "Bits 20:21 - Octospi clock source selection"]
    #[inline(always)]
    pub fn ospisel(&mut self) -> OSPISEL_W {
        OSPISEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripherals independent clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr2](index.html) module"]
pub struct CCIPR2_SPEC;
impl crate::RegisterSpec for CCIPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr2::R](R) reader structure"]
impl crate::Readable for CCIPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr2::W](W) writer structure"]
impl crate::Writable for CCIPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCIPR2 to value 0"]
impl crate::Resettable for CCIPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
