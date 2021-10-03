#[doc = "Register `IER2` reader"]
pub struct R(crate::R<IER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER2` writer"]
pub struct W(crate::W<IER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER2_SPEC>;
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
impl From<crate::W<IER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM8IE` reader - TIM8IE"]
pub struct TIM8IE_R(crate::FieldReader<bool, bool>);
impl TIM8IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM8IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8IE` writer - TIM8IE"]
pub struct TIM8IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8IE_W<'a> {
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
#[doc = "Field `USART1IE` reader - USART1IE"]
pub struct USART1IE_R(crate::FieldReader<bool, bool>);
impl USART1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1IE` writer - USART1IE"]
pub struct USART1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1IE_W<'a> {
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
#[doc = "Field `TIM15IE` reader - TIM15IE"]
pub struct TIM15IE_R(crate::FieldReader<bool, bool>);
impl TIM15IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15IE` writer - TIM15IE"]
pub struct TIM15IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15IE_W<'a> {
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
#[doc = "Field `TIM16IE` reader - TIM16IE"]
pub struct TIM16IE_R(crate::FieldReader<bool, bool>);
impl TIM16IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16IE` writer - TIM16IE"]
pub struct TIM16IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16IE_W<'a> {
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
#[doc = "Field `TIM17IE` reader - TIM17IE"]
pub struct TIM17IE_R(crate::FieldReader<bool, bool>);
impl TIM17IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17IE` writer - TIM17IE"]
pub struct TIM17IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17IE_W<'a> {
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
#[doc = "Field `SAI1IE` reader - SAI1IE"]
pub struct SAI1IE_R(crate::FieldReader<bool, bool>);
impl SAI1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1IE` writer - SAI1IE"]
pub struct SAI1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1IE_W<'a> {
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
#[doc = "Field `SAI2IE` reader - SAI2IE"]
pub struct SAI2IE_R(crate::FieldReader<bool, bool>);
impl SAI2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2IE` writer - SAI2IE"]
pub struct SAI2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2IE_W<'a> {
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
#[doc = "Field `DFSDM1IE` reader - DFSDM1IE"]
pub struct DFSDM1IE_R(crate::FieldReader<bool, bool>);
impl DFSDM1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDM1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDM1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDM1IE` writer - DFSDM1IE"]
pub struct DFSDM1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1IE_W<'a> {
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
#[doc = "Field `CRCIE` reader - CRCIE"]
pub struct CRCIE_R(crate::FieldReader<bool, bool>);
impl CRCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCIE` writer - CRCIE"]
pub struct CRCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCIE_W<'a> {
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
#[doc = "Field `TSCIE` reader - TSCIE"]
pub struct TSCIE_R(crate::FieldReader<bool, bool>);
impl TSCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCIE` writer - TSCIE"]
pub struct TSCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCIE_W<'a> {
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
#[doc = "Field `ICACHEIE` reader - ICACHEIE"]
pub struct ICACHEIE_R(crate::FieldReader<bool, bool>);
impl ICACHEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHEIE` writer - ICACHEIE"]
pub struct ICACHEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEIE_W<'a> {
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
#[doc = "Field `ADCIE` reader - ADCIE"]
pub struct ADCIE_R(crate::FieldReader<bool, bool>);
impl ADCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCIE` writer - ADCIE"]
pub struct ADCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIE_W<'a> {
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
#[doc = "Field `AESIE` reader - AESIE"]
pub struct AESIE_R(crate::FieldReader<bool, bool>);
impl AESIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESIE` writer - AESIE"]
pub struct AESIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AESIE_W<'a> {
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
#[doc = "Field `HASHIE` reader - HASHIE"]
pub struct HASHIE_R(crate::FieldReader<bool, bool>);
impl HASHIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASHIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASHIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASHIE` writer - HASHIE"]
pub struct HASHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHIE_W<'a> {
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
#[doc = "Field `RNGIE` reader - RNGIE"]
pub struct RNGIE_R(crate::FieldReader<bool, bool>);
impl RNGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGIE` writer - RNGIE"]
pub struct RNGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGIE_W<'a> {
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
#[doc = "Field `PKAIE` reader - PKAIE"]
pub struct PKAIE_R(crate::FieldReader<bool, bool>);
impl PKAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKAIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKAIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKAIE` writer - PKAIE"]
pub struct PKAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAIE_W<'a> {
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
#[doc = "Field `SDMMC1IE` reader - SDMMC1IE"]
pub struct SDMMC1IE_R(crate::FieldReader<bool, bool>);
impl SDMMC1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC1IE` writer - SDMMC1IE"]
pub struct SDMMC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1IE_W<'a> {
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
#[doc = "Field `FMC_REGIE` reader - FMC_REGIE"]
pub struct FMC_REGIE_R(crate::FieldReader<bool, bool>);
impl FMC_REGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMC_REGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMC_REGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMC_REGIE` writer - FMC_REGIE"]
pub struct FMC_REGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FMC_REGIE_W<'a> {
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
#[doc = "Field `OCTOSPI1_REGIE` reader - OCTOSPI1_REGIE"]
pub struct OCTOSPI1_REGIE_R(crate::FieldReader<bool, bool>);
impl OCTOSPI1_REGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCTOSPI1_REGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCTOSPI1_REGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCTOSPI1_REGIE` writer - OCTOSPI1_REGIE"]
pub struct OCTOSPI1_REGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1_REGIE_W<'a> {
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
#[doc = "Field `RTCIE` reader - RTCIE"]
pub struct RTCIE_R(crate::FieldReader<bool, bool>);
impl RTCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCIE` writer - RTCIE"]
pub struct RTCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCIE_W<'a> {
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
#[doc = "Field `PWRIE` reader - PWRIE"]
pub struct PWRIE_R(crate::FieldReader<bool, bool>);
impl PWRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRIE` writer - PWRIE"]
pub struct PWRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `SYSCFGIE` reader - SYSCFGIE"]
pub struct SYSCFGIE_R(crate::FieldReader<bool, bool>);
impl SYSCFGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCFGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGIE` writer - SYSCFGIE"]
pub struct SYSCFGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGIE_W<'a> {
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
#[doc = "Field `DMA1IE` reader - DMA1IE"]
pub struct DMA1IE_R(crate::FieldReader<bool, bool>);
impl DMA1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1IE` writer - DMA1IE"]
pub struct DMA1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1IE_W<'a> {
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
#[doc = "Field `DMA2IE` reader - DMA2IE"]
pub struct DMA2IE_R(crate::FieldReader<bool, bool>);
impl DMA2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2IE` writer - DMA2IE"]
pub struct DMA2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2IE_W<'a> {
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
#[doc = "Field `DMAMUX1IE` reader - DMAMUX1IE"]
pub struct DMAMUX1IE_R(crate::FieldReader<bool, bool>);
impl DMAMUX1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAMUX1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAMUX1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAMUX1IE` writer - DMAMUX1IE"]
pub struct DMAMUX1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1IE_W<'a> {
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
#[doc = "Field `RCCIE` reader - RCCIE"]
pub struct RCCIE_R(crate::FieldReader<bool, bool>);
impl RCCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCCIE` writer - RCCIE"]
pub struct RCCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCIE_W<'a> {
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
#[doc = "Field `FLASHIE` reader - FLASHIE"]
pub struct FLASHIE_R(crate::FieldReader<bool, bool>);
impl FLASHIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHIE` writer - FLASHIE"]
pub struct FLASHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHIE_W<'a> {
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
#[doc = "Field `FLASH_REGIE` reader - FLASH_REGIE"]
pub struct FLASH_REGIE_R(crate::FieldReader<bool, bool>);
impl FLASH_REGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_REGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_REGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_REGIE` writer - FLASH_REGIE"]
pub struct FLASH_REGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_REGIE_W<'a> {
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
#[doc = "Field `EXTIIE` reader - EXTIIE"]
pub struct EXTIIE_R(crate::FieldReader<bool, bool>);
impl EXTIIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTIIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTIIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIIE` writer - EXTIIE"]
pub struct EXTIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIIE_W<'a> {
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
#[doc = "Field `OTFDEC1IE` reader - OTFDEC1IE"]
pub struct OTFDEC1IE_R(crate::FieldReader<bool, bool>);
impl OTFDEC1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTFDEC1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTFDEC1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTFDEC1IE` writer - OTFDEC1IE"]
pub struct OTFDEC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OTFDEC1IE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TIM8IE"]
    #[inline(always)]
    pub fn tim8ie(&self) -> TIM8IE_R {
        TIM8IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USART1IE"]
    #[inline(always)]
    pub fn usart1ie(&self) -> USART1IE_R {
        USART1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15IE"]
    #[inline(always)]
    pub fn tim15ie(&self) -> TIM15IE_R {
        TIM15IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16IE"]
    #[inline(always)]
    pub fn tim16ie(&self) -> TIM16IE_R {
        TIM16IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17IE"]
    #[inline(always)]
    pub fn tim17ie(&self) -> TIM17IE_R {
        TIM17IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SAI1IE"]
    #[inline(always)]
    pub fn sai1ie(&self) -> SAI1IE_R {
        SAI1IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAI2IE"]
    #[inline(always)]
    pub fn sai2ie(&self) -> SAI2IE_R {
        SAI2IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFSDM1IE"]
    #[inline(always)]
    pub fn dfsdm1ie(&self) -> DFSDM1IE_R {
        DFSDM1IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CRCIE"]
    #[inline(always)]
    pub fn crcie(&self) -> CRCIE_R {
        CRCIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSCIE"]
    #[inline(always)]
    pub fn tscie(&self) -> TSCIE_R {
        TSCIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ICACHEIE"]
    #[inline(always)]
    pub fn icacheie(&self) -> ICACHEIE_R {
        ICACHEIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADCIE"]
    #[inline(always)]
    pub fn adcie(&self) -> ADCIE_R {
        ADCIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AESIE"]
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HASHIE"]
    #[inline(always)]
    pub fn hashie(&self) -> HASHIE_R {
        HASHIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RNGIE"]
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PKAIE"]
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1IE"]
    #[inline(always)]
    pub fn sdmmc1ie(&self) -> SDMMC1IE_R {
        SDMMC1IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FMC_REGIE"]
    #[inline(always)]
    pub fn fmc_regie(&self) -> FMC_REGIE_R {
        FMC_REGIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGIE"]
    #[inline(always)]
    pub fn octospi1_regie(&self) -> OCTOSPI1_REGIE_R {
        OCTOSPI1_REGIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RTCIE"]
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWRIE"]
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SYSCFGIE"]
    #[inline(always)]
    pub fn syscfgie(&self) -> SYSCFGIE_R {
        SYSCFGIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DMA1IE"]
    #[inline(always)]
    pub fn dma1ie(&self) -> DMA1IE_R {
        DMA1IE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DMA2IE"]
    #[inline(always)]
    pub fn dma2ie(&self) -> DMA2IE_R {
        DMA2IE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMAMUX1IE"]
    #[inline(always)]
    pub fn dmamux1ie(&self) -> DMAMUX1IE_R {
        DMAMUX1IE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RCCIE"]
    #[inline(always)]
    pub fn rccie(&self) -> RCCIE_R {
        RCCIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FLASHIE"]
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FLASH_REGIE"]
    #[inline(always)]
    pub fn flash_regie(&self) -> FLASH_REGIE_R {
        FLASH_REGIE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EXTIIE"]
    #[inline(always)]
    pub fn extiie(&self) -> EXTIIE_R {
        EXTIIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - OTFDEC1IE"]
    #[inline(always)]
    pub fn otfdec1ie(&self) -> OTFDEC1IE_R {
        OTFDEC1IE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8IE"]
    #[inline(always)]
    pub fn tim8ie(&mut self) -> TIM8IE_W {
        TIM8IE_W { w: self }
    }
    #[doc = "Bit 1 - USART1IE"]
    #[inline(always)]
    pub fn usart1ie(&mut self) -> USART1IE_W {
        USART1IE_W { w: self }
    }
    #[doc = "Bit 2 - TIM15IE"]
    #[inline(always)]
    pub fn tim15ie(&mut self) -> TIM15IE_W {
        TIM15IE_W { w: self }
    }
    #[doc = "Bit 3 - TIM16IE"]
    #[inline(always)]
    pub fn tim16ie(&mut self) -> TIM16IE_W {
        TIM16IE_W { w: self }
    }
    #[doc = "Bit 4 - TIM17IE"]
    #[inline(always)]
    pub fn tim17ie(&mut self) -> TIM17IE_W {
        TIM17IE_W { w: self }
    }
    #[doc = "Bit 5 - SAI1IE"]
    #[inline(always)]
    pub fn sai1ie(&mut self) -> SAI1IE_W {
        SAI1IE_W { w: self }
    }
    #[doc = "Bit 6 - SAI2IE"]
    #[inline(always)]
    pub fn sai2ie(&mut self) -> SAI2IE_W {
        SAI2IE_W { w: self }
    }
    #[doc = "Bit 7 - DFSDM1IE"]
    #[inline(always)]
    pub fn dfsdm1ie(&mut self) -> DFSDM1IE_W {
        DFSDM1IE_W { w: self }
    }
    #[doc = "Bit 8 - CRCIE"]
    #[inline(always)]
    pub fn crcie(&mut self) -> CRCIE_W {
        CRCIE_W { w: self }
    }
    #[doc = "Bit 9 - TSCIE"]
    #[inline(always)]
    pub fn tscie(&mut self) -> TSCIE_W {
        TSCIE_W { w: self }
    }
    #[doc = "Bit 10 - ICACHEIE"]
    #[inline(always)]
    pub fn icacheie(&mut self) -> ICACHEIE_W {
        ICACHEIE_W { w: self }
    }
    #[doc = "Bit 11 - ADCIE"]
    #[inline(always)]
    pub fn adcie(&mut self) -> ADCIE_W {
        ADCIE_W { w: self }
    }
    #[doc = "Bit 12 - AESIE"]
    #[inline(always)]
    pub fn aesie(&mut self) -> AESIE_W {
        AESIE_W { w: self }
    }
    #[doc = "Bit 13 - HASHIE"]
    #[inline(always)]
    pub fn hashie(&mut self) -> HASHIE_W {
        HASHIE_W { w: self }
    }
    #[doc = "Bit 14 - RNGIE"]
    #[inline(always)]
    pub fn rngie(&mut self) -> RNGIE_W {
        RNGIE_W { w: self }
    }
    #[doc = "Bit 15 - PKAIE"]
    #[inline(always)]
    pub fn pkaie(&mut self) -> PKAIE_W {
        PKAIE_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1IE"]
    #[inline(always)]
    pub fn sdmmc1ie(&mut self) -> SDMMC1IE_W {
        SDMMC1IE_W { w: self }
    }
    #[doc = "Bit 17 - FMC_REGIE"]
    #[inline(always)]
    pub fn fmc_regie(&mut self) -> FMC_REGIE_W {
        FMC_REGIE_W { w: self }
    }
    #[doc = "Bit 18 - OCTOSPI1_REGIE"]
    #[inline(always)]
    pub fn octospi1_regie(&mut self) -> OCTOSPI1_REGIE_W {
        OCTOSPI1_REGIE_W { w: self }
    }
    #[doc = "Bit 19 - RTCIE"]
    #[inline(always)]
    pub fn rtcie(&mut self) -> RTCIE_W {
        RTCIE_W { w: self }
    }
    #[doc = "Bit 20 - PWRIE"]
    #[inline(always)]
    pub fn pwrie(&mut self) -> PWRIE_W {
        PWRIE_W { w: self }
    }
    #[doc = "Bit 21 - SYSCFGIE"]
    #[inline(always)]
    pub fn syscfgie(&mut self) -> SYSCFGIE_W {
        SYSCFGIE_W { w: self }
    }
    #[doc = "Bit 22 - DMA1IE"]
    #[inline(always)]
    pub fn dma1ie(&mut self) -> DMA1IE_W {
        DMA1IE_W { w: self }
    }
    #[doc = "Bit 23 - DMA2IE"]
    #[inline(always)]
    pub fn dma2ie(&mut self) -> DMA2IE_W {
        DMA2IE_W { w: self }
    }
    #[doc = "Bit 24 - DMAMUX1IE"]
    #[inline(always)]
    pub fn dmamux1ie(&mut self) -> DMAMUX1IE_W {
        DMAMUX1IE_W { w: self }
    }
    #[doc = "Bit 25 - RCCIE"]
    #[inline(always)]
    pub fn rccie(&mut self) -> RCCIE_W {
        RCCIE_W { w: self }
    }
    #[doc = "Bit 26 - FLASHIE"]
    #[inline(always)]
    pub fn flashie(&mut self) -> FLASHIE_W {
        FLASHIE_W { w: self }
    }
    #[doc = "Bit 27 - FLASH_REGIE"]
    #[inline(always)]
    pub fn flash_regie(&mut self) -> FLASH_REGIE_W {
        FLASH_REGIE_W { w: self }
    }
    #[doc = "Bit 28 - EXTIIE"]
    #[inline(always)]
    pub fn extiie(&mut self) -> EXTIIE_W {
        EXTIIE_W { w: self }
    }
    #[doc = "Bit 29 - OTFDEC1IE"]
    #[inline(always)]
    pub fn otfdec1ie(&mut self) -> OTFDEC1IE_W {
        OTFDEC1IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier2](index.html) module"]
pub struct IER2_SPEC;
impl crate::RegisterSpec for IER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier2::R](R) reader structure"]
impl crate::Readable for IER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier2::W](W) writer structure"]
impl crate::Writable for IER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER2 to value 0"]
impl crate::Resettable for IER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
