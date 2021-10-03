#[doc = "Register `FCR2` reader"]
pub struct R(crate::R<FCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR2` writer"]
pub struct W(crate::W<FCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR2_SPEC>;
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
impl From<crate::W<FCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM8FC` reader - TIM8FC"]
pub struct TIM8FC_R(crate::FieldReader<bool, bool>);
impl TIM8FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM8FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8FC` writer - TIM8FC"]
pub struct TIM8FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8FC_W<'a> {
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
#[doc = "Field `USART1FC` reader - USART1FC"]
pub struct USART1FC_R(crate::FieldReader<bool, bool>);
impl USART1FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1FC` writer - USART1FC"]
pub struct USART1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1FC_W<'a> {
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
#[doc = "Field `TIM15FC` reader - TIM15FC"]
pub struct TIM15FC_R(crate::FieldReader<bool, bool>);
impl TIM15FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15FC` writer - TIM15FC"]
pub struct TIM15FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15FC_W<'a> {
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
#[doc = "Field `TIM16FC` reader - TIM16FC"]
pub struct TIM16FC_R(crate::FieldReader<bool, bool>);
impl TIM16FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16FC` writer - TIM16FC"]
pub struct TIM16FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16FC_W<'a> {
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
#[doc = "Field `TIM17FC` reader - TIM17FC"]
pub struct TIM17FC_R(crate::FieldReader<bool, bool>);
impl TIM17FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17FC` writer - TIM17FC"]
pub struct TIM17FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17FC_W<'a> {
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
#[doc = "Field `SAI1FC` reader - SAI1FC"]
pub struct SAI1FC_R(crate::FieldReader<bool, bool>);
impl SAI1FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI1FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1FC` writer - SAI1FC"]
pub struct SAI1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1FC_W<'a> {
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
#[doc = "Field `SAI2FC` reader - SAI2FC"]
pub struct SAI2FC_R(crate::FieldReader<bool, bool>);
impl SAI2FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI2FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2FC` writer - SAI2FC"]
pub struct SAI2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2FC_W<'a> {
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
#[doc = "Field `DFSDM1FC` reader - DFSDM1FC"]
pub struct DFSDM1FC_R(crate::FieldReader<bool, bool>);
impl DFSDM1FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDM1FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDM1FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDM1FC` writer - DFSDM1FC"]
pub struct DFSDM1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1FC_W<'a> {
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
#[doc = "Field `CRCFC` reader - CRCFC"]
pub struct CRCFC_R(crate::FieldReader<bool, bool>);
impl CRCFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCFC` writer - CRCFC"]
pub struct CRCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCFC_W<'a> {
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
#[doc = "Field `TSCFC` reader - TSCFC"]
pub struct TSCFC_R(crate::FieldReader<bool, bool>);
impl TSCFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCFC` writer - TSCFC"]
pub struct TSCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCFC_W<'a> {
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
#[doc = "Field `ICACHEFC` reader - ICACHEFC"]
pub struct ICACHEFC_R(crate::FieldReader<bool, bool>);
impl ICACHEFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHEFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHEFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHEFC` writer - ICACHEFC"]
pub struct ICACHEFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEFC_W<'a> {
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
#[doc = "Field `ADCFC` reader - ADCFC"]
pub struct ADCFC_R(crate::FieldReader<bool, bool>);
impl ADCFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCFC` writer - ADCFC"]
pub struct ADCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCFC_W<'a> {
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
#[doc = "Field `AESFC` reader - AESFC"]
pub struct AESFC_R(crate::FieldReader<bool, bool>);
impl AESFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESFC` writer - AESFC"]
pub struct AESFC_W<'a> {
    w: &'a mut W,
}
impl<'a> AESFC_W<'a> {
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
#[doc = "Field `HASHFC` reader - HASHFC"]
pub struct HASHFC_R(crate::FieldReader<bool, bool>);
impl HASHFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASHFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASHFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASHFC` writer - HASHFC"]
pub struct HASHFC_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHFC_W<'a> {
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
#[doc = "Field `RNGFC` reader - RNGFC"]
pub struct RNGFC_R(crate::FieldReader<bool, bool>);
impl RNGFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGFC` writer - RNGFC"]
pub struct RNGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGFC_W<'a> {
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
#[doc = "Field `PKAFC` reader - PKAFC"]
pub struct PKAFC_R(crate::FieldReader<bool, bool>);
impl PKAFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKAFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKAFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKAFC` writer - PKAFC"]
pub struct PKAFC_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAFC_W<'a> {
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
#[doc = "Field `SDMMC1FC` reader - SDMMC1FC"]
pub struct SDMMC1FC_R(crate::FieldReader<bool, bool>);
impl SDMMC1FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC1FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC1FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC1FC` writer - SDMMC1FC"]
pub struct SDMMC1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1FC_W<'a> {
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
#[doc = "Field `FMC_REGFC` reader - FMC_REGFC"]
pub struct FMC_REGFC_R(crate::FieldReader<bool, bool>);
impl FMC_REGFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMC_REGFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMC_REGFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMC_REGFC` writer - FMC_REGFC"]
pub struct FMC_REGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> FMC_REGFC_W<'a> {
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
#[doc = "Field `OCTOSPI1_REGFC` reader - OCTOSPI1_REGFC"]
pub struct OCTOSPI1_REGFC_R(crate::FieldReader<bool, bool>);
impl OCTOSPI1_REGFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCTOSPI1_REGFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCTOSPI1_REGFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCTOSPI1_REGFC` writer - OCTOSPI1_REGFC"]
pub struct OCTOSPI1_REGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1_REGFC_W<'a> {
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
#[doc = "Field `RTCFC` reader - RTCFC"]
pub struct RTCFC_R(crate::FieldReader<bool, bool>);
impl RTCFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCFC` writer - RTCFC"]
pub struct RTCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCFC_W<'a> {
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
#[doc = "Field `PWRFC` reader - PWRFC"]
pub struct PWRFC_R(crate::FieldReader<bool, bool>);
impl PWRFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRFC` writer - PWRFC"]
pub struct PWRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRFC_W<'a> {
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
#[doc = "Field `SYSCFGFC` reader - SYSCFGFC"]
pub struct SYSCFGFC_R(crate::FieldReader<bool, bool>);
impl SYSCFGFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCFGFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGFC` writer - SYSCFGFC"]
pub struct SYSCFGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGFC_W<'a> {
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
#[doc = "Field `DMA1FC` reader - DMA1FC"]
pub struct DMA1FC_R(crate::FieldReader<bool, bool>);
impl DMA1FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1FC` writer - DMA1FC"]
pub struct DMA1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1FC_W<'a> {
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
#[doc = "Field `DMA2FC` reader - DMA2FC"]
pub struct DMA2FC_R(crate::FieldReader<bool, bool>);
impl DMA2FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2FC` writer - DMA2FC"]
pub struct DMA2FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2FC_W<'a> {
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
#[doc = "Field `DMAMUX1FC` reader - DMAMUX1FC"]
pub struct DMAMUX1FC_R(crate::FieldReader<bool, bool>);
impl DMAMUX1FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAMUX1FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAMUX1FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAMUX1FC` writer - DMAMUX1FC"]
pub struct DMAMUX1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1FC_W<'a> {
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
#[doc = "Field `RCCFC` reader - RCCFC"]
pub struct RCCFC_R(crate::FieldReader<bool, bool>);
impl RCCFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCCFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCCFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCCFC` writer - RCCFC"]
pub struct RCCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCFC_W<'a> {
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
#[doc = "Field `FLASHFC` reader - FLASHFC"]
pub struct FLASHFC_R(crate::FieldReader<bool, bool>);
impl FLASHFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHFC` writer - FLASHFC"]
pub struct FLASHFC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHFC_W<'a> {
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
#[doc = "Field `FLASH_REGFC` reader - FLASH_REGFC"]
pub struct FLASH_REGFC_R(crate::FieldReader<bool, bool>);
impl FLASH_REGFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_REGFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_REGFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_REGFC` writer - FLASH_REGFC"]
pub struct FLASH_REGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_REGFC_W<'a> {
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
#[doc = "Field `EXTIFC` reader - EXTIFC"]
pub struct EXTIFC_R(crate::FieldReader<bool, bool>);
impl EXTIFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTIFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTIFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIFC` writer - EXTIFC"]
pub struct EXTIFC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIFC_W<'a> {
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
#[doc = "Field `OTFDEC1FC` reader - OTFDEC1FC"]
pub struct OTFDEC1FC_R(crate::FieldReader<bool, bool>);
impl OTFDEC1FC_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTFDEC1FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTFDEC1FC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTFDEC1FC` writer - OTFDEC1FC"]
pub struct OTFDEC1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> OTFDEC1FC_W<'a> {
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
    #[doc = "Bit 0 - TIM8FC"]
    #[inline(always)]
    pub fn tim8fc(&self) -> TIM8FC_R {
        TIM8FC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USART1FC"]
    #[inline(always)]
    pub fn usart1fc(&self) -> USART1FC_R {
        USART1FC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15FC"]
    #[inline(always)]
    pub fn tim15fc(&self) -> TIM15FC_R {
        TIM15FC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16FC"]
    #[inline(always)]
    pub fn tim16fc(&self) -> TIM16FC_R {
        TIM16FC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17FC"]
    #[inline(always)]
    pub fn tim17fc(&self) -> TIM17FC_R {
        TIM17FC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SAI1FC"]
    #[inline(always)]
    pub fn sai1fc(&self) -> SAI1FC_R {
        SAI1FC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAI2FC"]
    #[inline(always)]
    pub fn sai2fc(&self) -> SAI2FC_R {
        SAI2FC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFSDM1FC"]
    #[inline(always)]
    pub fn dfsdm1fc(&self) -> DFSDM1FC_R {
        DFSDM1FC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CRCFC"]
    #[inline(always)]
    pub fn crcfc(&self) -> CRCFC_R {
        CRCFC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSCFC"]
    #[inline(always)]
    pub fn tscfc(&self) -> TSCFC_R {
        TSCFC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ICACHEFC"]
    #[inline(always)]
    pub fn icachefc(&self) -> ICACHEFC_R {
        ICACHEFC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADCFC"]
    #[inline(always)]
    pub fn adcfc(&self) -> ADCFC_R {
        ADCFC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AESFC"]
    #[inline(always)]
    pub fn aesfc(&self) -> AESFC_R {
        AESFC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HASHFC"]
    #[inline(always)]
    pub fn hashfc(&self) -> HASHFC_R {
        HASHFC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RNGFC"]
    #[inline(always)]
    pub fn rngfc(&self) -> RNGFC_R {
        RNGFC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PKAFC"]
    #[inline(always)]
    pub fn pkafc(&self) -> PKAFC_R {
        PKAFC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1FC"]
    #[inline(always)]
    pub fn sdmmc1fc(&self) -> SDMMC1FC_R {
        SDMMC1FC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FMC_REGFC"]
    #[inline(always)]
    pub fn fmc_regfc(&self) -> FMC_REGFC_R {
        FMC_REGFC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGFC"]
    #[inline(always)]
    pub fn octospi1_regfc(&self) -> OCTOSPI1_REGFC_R {
        OCTOSPI1_REGFC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RTCFC"]
    #[inline(always)]
    pub fn rtcfc(&self) -> RTCFC_R {
        RTCFC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWRFC"]
    #[inline(always)]
    pub fn pwrfc(&self) -> PWRFC_R {
        PWRFC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SYSCFGFC"]
    #[inline(always)]
    pub fn syscfgfc(&self) -> SYSCFGFC_R {
        SYSCFGFC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DMA1FC"]
    #[inline(always)]
    pub fn dma1fc(&self) -> DMA1FC_R {
        DMA1FC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DMA2FC"]
    #[inline(always)]
    pub fn dma2fc(&self) -> DMA2FC_R {
        DMA2FC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMAMUX1FC"]
    #[inline(always)]
    pub fn dmamux1fc(&self) -> DMAMUX1FC_R {
        DMAMUX1FC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RCCFC"]
    #[inline(always)]
    pub fn rccfc(&self) -> RCCFC_R {
        RCCFC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FLASHFC"]
    #[inline(always)]
    pub fn flashfc(&self) -> FLASHFC_R {
        FLASHFC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FLASH_REGFC"]
    #[inline(always)]
    pub fn flash_regfc(&self) -> FLASH_REGFC_R {
        FLASH_REGFC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EXTIFC"]
    #[inline(always)]
    pub fn extifc(&self) -> EXTIFC_R {
        EXTIFC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - OTFDEC1FC"]
    #[inline(always)]
    pub fn otfdec1fc(&self) -> OTFDEC1FC_R {
        OTFDEC1FC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8FC"]
    #[inline(always)]
    pub fn tim8fc(&mut self) -> TIM8FC_W {
        TIM8FC_W { w: self }
    }
    #[doc = "Bit 1 - USART1FC"]
    #[inline(always)]
    pub fn usart1fc(&mut self) -> USART1FC_W {
        USART1FC_W { w: self }
    }
    #[doc = "Bit 2 - TIM15FC"]
    #[inline(always)]
    pub fn tim15fc(&mut self) -> TIM15FC_W {
        TIM15FC_W { w: self }
    }
    #[doc = "Bit 3 - TIM16FC"]
    #[inline(always)]
    pub fn tim16fc(&mut self) -> TIM16FC_W {
        TIM16FC_W { w: self }
    }
    #[doc = "Bit 4 - TIM17FC"]
    #[inline(always)]
    pub fn tim17fc(&mut self) -> TIM17FC_W {
        TIM17FC_W { w: self }
    }
    #[doc = "Bit 5 - SAI1FC"]
    #[inline(always)]
    pub fn sai1fc(&mut self) -> SAI1FC_W {
        SAI1FC_W { w: self }
    }
    #[doc = "Bit 6 - SAI2FC"]
    #[inline(always)]
    pub fn sai2fc(&mut self) -> SAI2FC_W {
        SAI2FC_W { w: self }
    }
    #[doc = "Bit 7 - DFSDM1FC"]
    #[inline(always)]
    pub fn dfsdm1fc(&mut self) -> DFSDM1FC_W {
        DFSDM1FC_W { w: self }
    }
    #[doc = "Bit 8 - CRCFC"]
    #[inline(always)]
    pub fn crcfc(&mut self) -> CRCFC_W {
        CRCFC_W { w: self }
    }
    #[doc = "Bit 9 - TSCFC"]
    #[inline(always)]
    pub fn tscfc(&mut self) -> TSCFC_W {
        TSCFC_W { w: self }
    }
    #[doc = "Bit 10 - ICACHEFC"]
    #[inline(always)]
    pub fn icachefc(&mut self) -> ICACHEFC_W {
        ICACHEFC_W { w: self }
    }
    #[doc = "Bit 11 - ADCFC"]
    #[inline(always)]
    pub fn adcfc(&mut self) -> ADCFC_W {
        ADCFC_W { w: self }
    }
    #[doc = "Bit 12 - AESFC"]
    #[inline(always)]
    pub fn aesfc(&mut self) -> AESFC_W {
        AESFC_W { w: self }
    }
    #[doc = "Bit 13 - HASHFC"]
    #[inline(always)]
    pub fn hashfc(&mut self) -> HASHFC_W {
        HASHFC_W { w: self }
    }
    #[doc = "Bit 14 - RNGFC"]
    #[inline(always)]
    pub fn rngfc(&mut self) -> RNGFC_W {
        RNGFC_W { w: self }
    }
    #[doc = "Bit 15 - PKAFC"]
    #[inline(always)]
    pub fn pkafc(&mut self) -> PKAFC_W {
        PKAFC_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1FC"]
    #[inline(always)]
    pub fn sdmmc1fc(&mut self) -> SDMMC1FC_W {
        SDMMC1FC_W { w: self }
    }
    #[doc = "Bit 17 - FMC_REGFC"]
    #[inline(always)]
    pub fn fmc_regfc(&mut self) -> FMC_REGFC_W {
        FMC_REGFC_W { w: self }
    }
    #[doc = "Bit 18 - OCTOSPI1_REGFC"]
    #[inline(always)]
    pub fn octospi1_regfc(&mut self) -> OCTOSPI1_REGFC_W {
        OCTOSPI1_REGFC_W { w: self }
    }
    #[doc = "Bit 19 - RTCFC"]
    #[inline(always)]
    pub fn rtcfc(&mut self) -> RTCFC_W {
        RTCFC_W { w: self }
    }
    #[doc = "Bit 20 - PWRFC"]
    #[inline(always)]
    pub fn pwrfc(&mut self) -> PWRFC_W {
        PWRFC_W { w: self }
    }
    #[doc = "Bit 21 - SYSCFGFC"]
    #[inline(always)]
    pub fn syscfgfc(&mut self) -> SYSCFGFC_W {
        SYSCFGFC_W { w: self }
    }
    #[doc = "Bit 22 - DMA1FC"]
    #[inline(always)]
    pub fn dma1fc(&mut self) -> DMA1FC_W {
        DMA1FC_W { w: self }
    }
    #[doc = "Bit 23 - DMA2FC"]
    #[inline(always)]
    pub fn dma2fc(&mut self) -> DMA2FC_W {
        DMA2FC_W { w: self }
    }
    #[doc = "Bit 24 - DMAMUX1FC"]
    #[inline(always)]
    pub fn dmamux1fc(&mut self) -> DMAMUX1FC_W {
        DMAMUX1FC_W { w: self }
    }
    #[doc = "Bit 25 - RCCFC"]
    #[inline(always)]
    pub fn rccfc(&mut self) -> RCCFC_W {
        RCCFC_W { w: self }
    }
    #[doc = "Bit 26 - FLASHFC"]
    #[inline(always)]
    pub fn flashfc(&mut self) -> FLASHFC_W {
        FLASHFC_W { w: self }
    }
    #[doc = "Bit 27 - FLASH_REGFC"]
    #[inline(always)]
    pub fn flash_regfc(&mut self) -> FLASH_REGFC_W {
        FLASH_REGFC_W { w: self }
    }
    #[doc = "Bit 28 - EXTIFC"]
    #[inline(always)]
    pub fn extifc(&mut self) -> EXTIFC_W {
        EXTIFC_W { w: self }
    }
    #[doc = "Bit 29 - OTFDEC1FC"]
    #[inline(always)]
    pub fn otfdec1fc(&mut self) -> OTFDEC1FC_W {
        OTFDEC1FC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt clear register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr2](index.html) module"]
pub struct FCR2_SPEC;
impl crate::RegisterSpec for FCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr2::R](R) reader structure"]
impl crate::Readable for FCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr2::W](W) writer structure"]
impl crate::Writable for FCR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR2 to value 0"]
impl crate::Resettable for FCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
