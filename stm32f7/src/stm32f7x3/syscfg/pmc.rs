#[doc = "Register `PMC` reader"]
pub struct R(crate::R<PMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC` writer"]
pub struct W(crate::W<PMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_SPEC>;
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
impl From<crate::W<PMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB7_FMP` reader - PB7_FMP Fast Mode + Enable"]
pub struct PB7_FMP_R(crate::FieldReader<bool, bool>);
impl PB7_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB7_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB7_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB7_FMP` writer - PB7_FMP Fast Mode + Enable"]
pub struct PB7_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB7_FMP_W<'a> {
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
#[doc = "Field `PB8_FMP` reader - PB8_FMP Fast Mode + Enable"]
pub struct PB8_FMP_R(crate::FieldReader<bool, bool>);
impl PB8_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB8_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB8_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB8_FMP` writer - PB8_FMP Fast Mode + Enable"]
pub struct PB8_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB8_FMP_W<'a> {
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
#[doc = "Field `PB9_FMP` reader - Fast Mode + Enable"]
pub struct PB9_FMP_R(crate::FieldReader<bool, bool>);
impl PB9_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB9_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB9_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB9_FMP` writer - Fast Mode + Enable"]
pub struct PB9_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB9_FMP_W<'a> {
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
#[doc = "Field `ADC1DC2` reader - ADC3DC2"]
pub struct ADC1DC2_R(crate::FieldReader<bool, bool>);
impl ADC1DC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC1DC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1DC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1DC2` writer - ADC3DC2"]
pub struct ADC1DC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1DC2_W<'a> {
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
#[doc = "Field `PB6_FMP` reader - PB6_FMP Fast Mode"]
pub struct PB6_FMP_R(crate::FieldReader<bool, bool>);
impl PB6_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB6_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB6_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB6_FMP` writer - PB6_FMP Fast Mode"]
pub struct PB6_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB6_FMP_W<'a> {
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
#[doc = "Field `I2C3_FMP` reader - I2C3_FMP I2C3 Fast Mode + Enable"]
pub struct I2C3_FMP_R(crate::FieldReader<bool, bool>);
impl I2C3_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3_FMP` writer - I2C3_FMP I2C3 Fast Mode + Enable"]
pub struct I2C3_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_FMP_W<'a> {
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
#[doc = "Field `I2C2_FMP` reader - I2C2_FMP I2C2 Fast Mode + Enable"]
pub struct I2C2_FMP_R(crate::FieldReader<bool, bool>);
impl I2C2_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2_FMP` writer - I2C2_FMP I2C2 Fast Mode + Enable"]
pub struct I2C2_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_FMP_W<'a> {
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
#[doc = "Field `I2C1_FMP` reader - I2C1_FMP I2C1 Fast Mode + Enable"]
pub struct I2C1_FMP_R(crate::FieldReader<bool, bool>);
impl I2C1_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1_FMP` writer - I2C1_FMP I2C1 Fast Mode + Enable"]
pub struct I2C1_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_FMP_W<'a> {
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
#[doc = "Field `ADC3DC2` reader - ADC3DC2"]
pub struct ADC3DC2_R(crate::FieldReader<bool, bool>);
impl ADC3DC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC3DC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC3DC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC3DC2` writer - ADC3DC2"]
pub struct ADC3DC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC3DC2_W<'a> {
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
#[doc = "Field `ADC2DC2` reader - ADC2DC2"]
pub struct ADC2DC2_R(crate::FieldReader<bool, bool>);
impl ADC2DC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC2DC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2DC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2DC2` writer - ADC2DC2"]
pub struct ADC2DC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2DC2_W<'a> {
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
impl R {
    #[doc = "Bit 5 - PB7_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb7_fmp(&self) -> PB7_FMP_R {
        PB7_FMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PB8_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb8_fmp(&self) -> PB8_FMP_R {
        PB8_FMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb9_fmp(&self) -> PB9_FMP_R {
        PB9_FMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC3DC2"]
    #[inline(always)]
    pub fn adc1dc2(&self) -> ADC1DC2_R {
        ADC1DC2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PB6_FMP Fast Mode"]
    #[inline(always)]
    pub fn pb6_fmp(&self) -> PB6_FMP_R {
        PB6_FMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C3_FMP I2C3 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C2_FMP I2C2 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - I2C1_FMP I2C1 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    pub fn adc3dc2(&self) -> ADC3DC2_R {
        ADC3DC2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    pub fn adc2dc2(&self) -> ADC2DC2_R {
        ADC2DC2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - PB7_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb7_fmp(&mut self) -> PB7_FMP_W {
        PB7_FMP_W { w: self }
    }
    #[doc = "Bit 6 - PB8_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb8_fmp(&mut self) -> PB8_FMP_W {
        PB8_FMP_W { w: self }
    }
    #[doc = "Bit 7 - Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb9_fmp(&mut self) -> PB9_FMP_W {
        PB9_FMP_W { w: self }
    }
    #[doc = "Bit 16 - ADC3DC2"]
    #[inline(always)]
    pub fn adc1dc2(&mut self) -> ADC1DC2_W {
        ADC1DC2_W { w: self }
    }
    #[doc = "Bit 4 - PB6_FMP Fast Mode"]
    #[inline(always)]
    pub fn pb6_fmp(&mut self) -> PB6_FMP_W {
        PB6_FMP_W { w: self }
    }
    #[doc = "Bit 2 - I2C3_FMP I2C3 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W {
        I2C3_FMP_W { w: self }
    }
    #[doc = "Bit 1 - I2C2_FMP I2C2 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W {
        I2C2_FMP_W { w: self }
    }
    #[doc = "Bit 0 - I2C1_FMP I2C1 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W {
        I2C1_FMP_W { w: self }
    }
    #[doc = "Bit 18 - ADC3DC2"]
    #[inline(always)]
    pub fn adc3dc2(&mut self) -> ADC3DC2_W {
        ADC3DC2_W { w: self }
    }
    #[doc = "Bit 17 - ADC2DC2"]
    #[inline(always)]
    pub fn adc2dc2(&mut self) -> ADC2DC2_W {
        ADC2DC2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc](index.html) module"]
pub struct PMC_SPEC;
impl crate::RegisterSpec for PMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc::R](R) reader structure"]
impl crate::Readable for PMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc::W](W) writer structure"]
impl crate::Writable for PMC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC to value 0"]
impl crate::Resettable for PMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
