#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HSI14 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI14ON_A {
    #[doc = "0: HSI14 oscillator off"]
    OFF = 0,
    #[doc = "1: HSI14 oscillator on"]
    ON = 1,
}
impl From<HSI14ON_A> for bool {
    #[inline(always)]
    fn from(variant: HSI14ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI14ON` reader - HSI14 clock enable"]
pub struct HSI14ON_R(crate::FieldReader<bool, HSI14ON_A>);
impl HSI14ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI14ON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI14ON_A {
        match self.bits {
            false => HSI14ON_A::OFF,
            true => HSI14ON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == HSI14ON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == HSI14ON_A::ON
    }
}
impl core::ops::Deref for HSI14ON_R {
    type Target = crate::FieldReader<bool, HSI14ON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI14ON` writer - HSI14 clock enable"]
pub struct HSI14ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI14ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSI14ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSI14 oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSI14ON_A::OFF)
    }
    #[doc = "HSI14 oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSI14ON_A::ON)
    }
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
#[doc = "HR14 clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI14RDY_A {
    #[doc = "0: HSI14 oscillator not ready"]
    NOTREADY = 0,
    #[doc = "1: HSI14 oscillator ready"]
    READY = 1,
}
impl From<HSI14RDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSI14RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI14RDY` reader - HR14 clock ready flag"]
pub struct HSI14RDY_R(crate::FieldReader<bool, HSI14RDY_A>);
impl HSI14RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI14RDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI14RDY_A {
        match self.bits {
            false => HSI14RDY_A::NOTREADY,
            true => HSI14RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == HSI14RDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == HSI14RDY_A::READY
    }
}
impl core::ops::Deref for HSI14RDY_R {
    type Target = crate::FieldReader<bool, HSI14RDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HSI14 clock request from ADC disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI14DIS_A {
    #[doc = "0: ADC can turn on the HSI14 oscillator"]
    ALLOW = 0,
    #[doc = "1: ADC can not turn on the HSI14 oscillator"]
    DISALLOW = 1,
}
impl From<HSI14DIS_A> for bool {
    #[inline(always)]
    fn from(variant: HSI14DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI14DIS` reader - HSI14 clock request from ADC disable"]
pub struct HSI14DIS_R(crate::FieldReader<bool, HSI14DIS_A>);
impl HSI14DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI14DIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI14DIS_A {
        match self.bits {
            false => HSI14DIS_A::ALLOW,
            true => HSI14DIS_A::DISALLOW,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOW`"]
    #[inline(always)]
    pub fn is_allow(&self) -> bool {
        **self == HSI14DIS_A::ALLOW
    }
    #[doc = "Checks if the value of the field is `DISALLOW`"]
    #[inline(always)]
    pub fn is_disallow(&self) -> bool {
        **self == HSI14DIS_A::DISALLOW
    }
}
impl core::ops::Deref for HSI14DIS_R {
    type Target = crate::FieldReader<bool, HSI14DIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI14DIS` writer - HSI14 clock request from ADC disable"]
pub struct HSI14DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI14DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSI14DIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC can turn on the HSI14 oscillator"]
    #[inline(always)]
    pub fn allow(self) -> &'a mut W {
        self.variant(HSI14DIS_A::ALLOW)
    }
    #[doc = "ADC can not turn on the HSI14 oscillator"]
    #[inline(always)]
    pub fn disallow(self) -> &'a mut W {
        self.variant(HSI14DIS_A::DISALLOW)
    }
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
#[doc = "Field `HSI14TRIM` reader - HSI14 clock trimming"]
pub struct HSI14TRIM_R(crate::FieldReader<u8, u8>);
impl HSI14TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSI14TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI14TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI14TRIM` writer - HSI14 clock trimming"]
pub struct HSI14TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI14TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `HSI14CAL` reader - HSI14 clock calibration"]
pub struct HSI14CAL_R(crate::FieldReader<u8, u8>);
impl HSI14CAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSI14CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI14CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HSI48 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI48ON_A {
    #[doc = "0: HSI48 oscillator off"]
    OFF = 0,
    #[doc = "1: HSI48 oscillator on"]
    ON = 1,
}
impl From<HSI48ON_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48ON` reader - HSI48 clock enable"]
pub struct HSI48ON_R(crate::FieldReader<bool, HSI48ON_A>);
impl HSI48ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48ON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48ON_A {
        match self.bits {
            false => HSI48ON_A::OFF,
            true => HSI48ON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == HSI48ON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == HSI48ON_A::ON
    }
}
impl core::ops::Deref for HSI48ON_R {
    type Target = crate::FieldReader<bool, HSI48ON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI48ON` writer - HSI48 clock enable"]
pub struct HSI48ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSI48ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSI48 oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSI48ON_A::OFF)
    }
    #[doc = "HSI48 oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSI48ON_A::ON)
    }
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
#[doc = "HSI48 clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSI48RDY_A {
    #[doc = "0: HSI48 oscillator ready"]
    NOTREADY = 0,
    #[doc = "1: HSI48 oscillator ready"]
    READY = 1,
}
impl From<HSI48RDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDY` reader - HSI48 clock ready flag"]
pub struct HSI48RDY_R(crate::FieldReader<bool, HSI48RDY_A>);
impl HSI48RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48RDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48RDY_A {
        match self.bits {
            false => HSI48RDY_A::NOTREADY,
            true => HSI48RDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == HSI48RDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == HSI48RDY_A::READY
    }
}
impl core::ops::Deref for HSI48RDY_R {
    type Target = crate::FieldReader<bool, HSI48RDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI48CAL` reader - HSI48 factory clock calibration"]
pub struct HSI48CAL_R(crate::FieldReader<u8, u8>);
impl HSI48CAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSI48CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - HSI14 clock enable"]
    #[inline(always)]
    pub fn hsi14on(&self) -> HSI14ON_R {
        HSI14ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HR14 clock ready flag"]
    #[inline(always)]
    pub fn hsi14rdy(&self) -> HSI14RDY_R {
        HSI14RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI14 clock request from ADC disable"]
    #[inline(always)]
    pub fn hsi14dis(&self) -> HSI14DIS_R {
        HSI14DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - HSI14 clock trimming"]
    #[inline(always)]
    pub fn hsi14trim(&self) -> HSI14TRIM_R {
        HSI14TRIM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - HSI14 clock calibration"]
    #[inline(always)]
    pub fn hsi14cal(&self) -> HSI14CAL_R {
        HSI14CAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - HSI48 clock enable"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HSI48 clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - HSI48 factory clock calibration"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HSI14 clock enable"]
    #[inline(always)]
    pub fn hsi14on(&mut self) -> HSI14ON_W {
        HSI14ON_W { w: self }
    }
    #[doc = "Bit 2 - HSI14 clock request from ADC disable"]
    #[inline(always)]
    pub fn hsi14dis(&mut self) -> HSI14DIS_W {
        HSI14DIS_W { w: self }
    }
    #[doc = "Bits 3:7 - HSI14 clock trimming"]
    #[inline(always)]
    pub fn hsi14trim(&mut self) -> HSI14TRIM_W {
        HSI14TRIM_W { w: self }
    }
    #[doc = "Bit 16 - HSI48 clock enable"]
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W {
        HSI48ON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0x80"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
