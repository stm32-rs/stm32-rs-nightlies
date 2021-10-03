#[doc = "Register `SPI2S_IER` reader"]
pub struct R(crate::R<SPI2S_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI2S_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI2S_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI2S_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI2S_IER` writer"]
pub struct W(crate::W<SPI2S_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI2S_IER_SPEC>;
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
impl From<crate::W<SPI2S_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI2S_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPIE` reader - RXPIE"]
pub struct RXPIE_R(crate::FieldReader<bool, bool>);
impl RXPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPIE` writer - RXPIE"]
pub struct RXPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPIE_W<'a> {
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
#[doc = "Field `TXPIE` reader - TXPIE"]
pub struct TXPIE_R(crate::FieldReader<bool, bool>);
impl TXPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPIE` writer - TXPIE"]
pub struct TXPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPIE_W<'a> {
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
#[doc = "Field `DXPIE` reader - DXPIE"]
pub struct DXPIE_R(crate::FieldReader<bool, bool>);
impl DXPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DXPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DXPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DXPIE` writer - DXPIE"]
pub struct DXPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DXPIE_W<'a> {
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
#[doc = "Field `EOTIE` reader - EOTIE"]
pub struct EOTIE_R(crate::FieldReader<bool, bool>);
impl EOTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOTIE` writer - EOTIE"]
pub struct EOTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTIE_W<'a> {
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
#[doc = "Field `TXTFIE` reader - TXTFIE"]
pub struct TXTFIE_R(crate::FieldReader<bool, bool>);
impl TXTFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXTFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTFIE` writer - TXTFIE"]
pub struct TXTFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTFIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `UDRIE` reader - UDRIE"]
pub struct UDRIE_R(crate::FieldReader<bool, bool>);
impl UDRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UDRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDRIE` writer - UDRIE"]
pub struct UDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `OVRIE` reader - OVRIE"]
pub struct OVRIE_R(crate::FieldReader<bool, bool>);
impl OVRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRIE` writer - OVRIE"]
pub struct OVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CRCEIE` reader - CRCEIE"]
pub struct CRCEIE_R(crate::FieldReader<bool, bool>);
impl CRCEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCEIE` writer - CRCEIE"]
pub struct CRCEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEIE_W<'a> {
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
#[doc = "Field `TIFREIE` reader - TIFREIE"]
pub struct TIFREIE_R(crate::FieldReader<bool, bool>);
impl TIFREIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIFREIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIFREIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIFREIE` writer - TIFREIE"]
pub struct TIFREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIFREIE_W<'a> {
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
#[doc = "Field `MODFIE` reader - MODFIE"]
pub struct MODFIE_R(crate::FieldReader<bool, bool>);
impl MODFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODFIE` writer - MODFIE"]
pub struct MODFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODFIE_W<'a> {
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
#[doc = "Field `TSERFIE` reader - TSERFIE"]
pub struct TSERFIE_R(crate::FieldReader<bool, bool>);
impl TSERFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSERFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSERFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSERFIE` writer - TSERFIE"]
pub struct TSERFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSERFIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RXPIE"]
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXPIE"]
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DXPIE"]
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EOTIE"]
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXTFIE"]
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UDRIE"]
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRCEIE"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIFREIE"]
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MODFIE"]
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TSERFIE"]
    #[inline(always)]
    pub fn tserfie(&self) -> TSERFIE_R {
        TSERFIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXPIE"]
    #[inline(always)]
    pub fn rxpie(&mut self) -> RXPIE_W {
        RXPIE_W { w: self }
    }
    #[doc = "Bit 1 - TXPIE"]
    #[inline(always)]
    pub fn txpie(&mut self) -> TXPIE_W {
        TXPIE_W { w: self }
    }
    #[doc = "Bit 2 - DXPIE"]
    #[inline(always)]
    pub fn dxpie(&mut self) -> DXPIE_W {
        DXPIE_W { w: self }
    }
    #[doc = "Bit 3 - EOTIE"]
    #[inline(always)]
    pub fn eotie(&mut self) -> EOTIE_W {
        EOTIE_W { w: self }
    }
    #[doc = "Bit 4 - TXTFIE"]
    #[inline(always)]
    pub fn txtfie(&mut self) -> TXTFIE_W {
        TXTFIE_W { w: self }
    }
    #[doc = "Bit 5 - UDRIE"]
    #[inline(always)]
    pub fn udrie(&mut self) -> UDRIE_W {
        UDRIE_W { w: self }
    }
    #[doc = "Bit 6 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W {
        OVRIE_W { w: self }
    }
    #[doc = "Bit 7 - CRCEIE"]
    #[inline(always)]
    pub fn crceie(&mut self) -> CRCEIE_W {
        CRCEIE_W { w: self }
    }
    #[doc = "Bit 8 - TIFREIE"]
    #[inline(always)]
    pub fn tifreie(&mut self) -> TIFREIE_W {
        TIFREIE_W { w: self }
    }
    #[doc = "Bit 9 - MODFIE"]
    #[inline(always)]
    pub fn modfie(&mut self) -> MODFIE_W {
        MODFIE_W { w: self }
    }
    #[doc = "Bit 10 - TSERFIE"]
    #[inline(always)]
    pub fn tserfie(&mut self) -> TSERFIE_W {
        TSERFIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI/I2S interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_ier](index.html) module"]
pub struct SPI2S_IER_SPEC;
impl crate::RegisterSpec for SPI2S_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi2s_ier::R](R) reader structure"]
impl crate::Readable for SPI2S_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi2s_ier::W](W) writer structure"]
impl crate::Writable for SPI2S_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI2S_IER to value 0"]
impl crate::Resettable for SPI2S_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
