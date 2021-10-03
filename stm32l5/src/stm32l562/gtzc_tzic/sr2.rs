#[doc = "Register `SR2` reader"]
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR2` writer"]
pub struct W(crate::W<SR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR2_SPEC>;
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
impl From<crate::W<SR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM8F` reader - TIM8F"]
pub struct TIM8F_R(crate::FieldReader<bool, bool>);
impl TIM8F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM8F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8F` writer - TIM8F"]
pub struct TIM8F_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8F_W<'a> {
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
#[doc = "Field `USART1F` reader - USART1F"]
pub struct USART1F_R(crate::FieldReader<bool, bool>);
impl USART1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1F` writer - USART1F"]
pub struct USART1F_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1F_W<'a> {
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
#[doc = "Field `TIM15F` reader - TIM15F"]
pub struct TIM15F_R(crate::FieldReader<bool, bool>);
impl TIM15F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15F` writer - TIM15F"]
pub struct TIM15F_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15F_W<'a> {
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
#[doc = "Field `TIM16F` reader - TIM16F"]
pub struct TIM16F_R(crate::FieldReader<bool, bool>);
impl TIM16F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16F` writer - TIM16F"]
pub struct TIM16F_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16F_W<'a> {
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
#[doc = "Field `TIM17F` reader - TIM17F"]
pub struct TIM17F_R(crate::FieldReader<bool, bool>);
impl TIM17F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17F` writer - TIM17F"]
pub struct TIM17F_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17F_W<'a> {
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
#[doc = "Field `SAI1F` reader - SAI1F"]
pub struct SAI1F_R(crate::FieldReader<bool, bool>);
impl SAI1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1F` writer - SAI1F"]
pub struct SAI1F_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1F_W<'a> {
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
#[doc = "Field `SAI2F` reader - SAI2F"]
pub struct SAI2F_R(crate::FieldReader<bool, bool>);
impl SAI2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2F` writer - SAI2F"]
pub struct SAI2F_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2F_W<'a> {
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
#[doc = "Field `DFSDM1F` reader - DFSDM1F"]
pub struct DFSDM1F_R(crate::FieldReader<bool, bool>);
impl DFSDM1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDM1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDM1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDM1F` writer - DFSDM1F"]
pub struct DFSDM1F_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1F_W<'a> {
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
#[doc = "Field `CRCF` reader - CRCF"]
pub struct CRCF_R(crate::FieldReader<bool, bool>);
impl CRCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCF` writer - CRCF"]
pub struct CRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCF_W<'a> {
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
#[doc = "Field `TSCF` reader - TSCF"]
pub struct TSCF_R(crate::FieldReader<bool, bool>);
impl TSCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCF` writer - TSCF"]
pub struct TSCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCF_W<'a> {
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
#[doc = "Field `ICACHEF` reader - ICACHEF"]
pub struct ICACHEF_R(crate::FieldReader<bool, bool>);
impl ICACHEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHEF` writer - ICACHEF"]
pub struct ICACHEF_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEF_W<'a> {
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
#[doc = "Field `ADCF` reader - ADCF"]
pub struct ADCF_R(crate::FieldReader<bool, bool>);
impl ADCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCF` writer - ADCF"]
pub struct ADCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCF_W<'a> {
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
#[doc = "Field `AESF` reader - AESF"]
pub struct AESF_R(crate::FieldReader<bool, bool>);
impl AESF_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESF` writer - AESF"]
pub struct AESF_W<'a> {
    w: &'a mut W,
}
impl<'a> AESF_W<'a> {
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
#[doc = "Field `HASHF` reader - HASHF"]
pub struct HASHF_R(crate::FieldReader<bool, bool>);
impl HASHF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASHF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASHF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASHF` writer - HASHF"]
pub struct HASHF_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHF_W<'a> {
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
#[doc = "Field `RNGF` reader - RNGF"]
pub struct RNGF_R(crate::FieldReader<bool, bool>);
impl RNGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGF` writer - RNGF"]
pub struct RNGF_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGF_W<'a> {
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
#[doc = "Field `PKAF` reader - PKAF"]
pub struct PKAF_R(crate::FieldReader<bool, bool>);
impl PKAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKAF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKAF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKAF` writer - PKAF"]
pub struct PKAF_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAF_W<'a> {
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
#[doc = "Field `SDMMC1F` reader - SDMMC1F"]
pub struct SDMMC1F_R(crate::FieldReader<bool, bool>);
impl SDMMC1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC1F` writer - SDMMC1F"]
pub struct SDMMC1F_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1F_W<'a> {
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
#[doc = "Field `FMC_REGF` reader - FMC_REGF"]
pub struct FMC_REGF_R(crate::FieldReader<bool, bool>);
impl FMC_REGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMC_REGF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMC_REGF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMC_REGF` writer - FMC_REGF"]
pub struct FMC_REGF_W<'a> {
    w: &'a mut W,
}
impl<'a> FMC_REGF_W<'a> {
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
#[doc = "Field `OCTOSPI1_REGF` reader - OCTOSPI1_REGF"]
pub struct OCTOSPI1_REGF_R(crate::FieldReader<bool, bool>);
impl OCTOSPI1_REGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCTOSPI1_REGF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCTOSPI1_REGF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCTOSPI1_REGF` writer - OCTOSPI1_REGF"]
pub struct OCTOSPI1_REGF_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1_REGF_W<'a> {
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
#[doc = "Field `RTCF` reader - RTCF"]
pub struct RTCF_R(crate::FieldReader<bool, bool>);
impl RTCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCF` writer - RTCF"]
pub struct RTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCF_W<'a> {
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
#[doc = "Field `PWRF` reader - PWRF"]
pub struct PWRF_R(crate::FieldReader<bool, bool>);
impl PWRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRF` writer - PWRF"]
pub struct PWRF_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRF_W<'a> {
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
#[doc = "Field `SYSCFGF` reader - SYSCFGF"]
pub struct SYSCFGF_R(crate::FieldReader<bool, bool>);
impl SYSCFGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCFGF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGF` writer - SYSCFGF"]
pub struct SYSCFGF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGF_W<'a> {
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
#[doc = "Field `DMA1F` reader - DMA1F"]
pub struct DMA1F_R(crate::FieldReader<bool, bool>);
impl DMA1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1F` writer - DMA1F"]
pub struct DMA1F_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1F_W<'a> {
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
#[doc = "Field `DMA2F` reader - DMA2F"]
pub struct DMA2F_R(crate::FieldReader<bool, bool>);
impl DMA2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2F` writer - DMA2F"]
pub struct DMA2F_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2F_W<'a> {
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
#[doc = "Field `DMAMUX1F` reader - DMAMUX1F"]
pub struct DMAMUX1F_R(crate::FieldReader<bool, bool>);
impl DMAMUX1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAMUX1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAMUX1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAMUX1F` writer - DMAMUX1F"]
pub struct DMAMUX1F_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1F_W<'a> {
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
#[doc = "Field `RCCF` reader - RCCF"]
pub struct RCCF_R(crate::FieldReader<bool, bool>);
impl RCCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCCF` writer - RCCF"]
pub struct RCCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCF_W<'a> {
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
#[doc = "Field `FLASHF` reader - FLASHF"]
pub struct FLASHF_R(crate::FieldReader<bool, bool>);
impl FLASHF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHF` writer - FLASHF"]
pub struct FLASHF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHF_W<'a> {
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
#[doc = "Field `FLASH_REGF` reader - FLASH_REGF"]
pub struct FLASH_REGF_R(crate::FieldReader<bool, bool>);
impl FLASH_REGF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_REGF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_REGF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_REGF` writer - FLASH_REGF"]
pub struct FLASH_REGF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_REGF_W<'a> {
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
#[doc = "Field `EXTIF` reader - EXTIF"]
pub struct EXTIF_R(crate::FieldReader<bool, bool>);
impl EXTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIF` writer - EXTIF"]
pub struct EXTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIF_W<'a> {
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
#[doc = "Field `OTFDEC1F` reader - OTFDEC1F"]
pub struct OTFDEC1F_R(crate::FieldReader<bool, bool>);
impl OTFDEC1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTFDEC1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTFDEC1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTFDEC1F` writer - OTFDEC1F"]
pub struct OTFDEC1F_W<'a> {
    w: &'a mut W,
}
impl<'a> OTFDEC1F_W<'a> {
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
    #[doc = "Bit 0 - TIM8F"]
    #[inline(always)]
    pub fn tim8f(&self) -> TIM8F_R {
        TIM8F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USART1F"]
    #[inline(always)]
    pub fn usart1f(&self) -> USART1F_R {
        USART1F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15F"]
    #[inline(always)]
    pub fn tim15f(&self) -> TIM15F_R {
        TIM15F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16F"]
    #[inline(always)]
    pub fn tim16f(&self) -> TIM16F_R {
        TIM16F_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17F"]
    #[inline(always)]
    pub fn tim17f(&self) -> TIM17F_R {
        TIM17F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SAI1F"]
    #[inline(always)]
    pub fn sai1f(&self) -> SAI1F_R {
        SAI1F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAI2F"]
    #[inline(always)]
    pub fn sai2f(&self) -> SAI2F_R {
        SAI2F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFSDM1F"]
    #[inline(always)]
    pub fn dfsdm1f(&self) -> DFSDM1F_R {
        DFSDM1F_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CRCF"]
    #[inline(always)]
    pub fn crcf(&self) -> CRCF_R {
        CRCF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSCF"]
    #[inline(always)]
    pub fn tscf(&self) -> TSCF_R {
        TSCF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ICACHEF"]
    #[inline(always)]
    pub fn icachef(&self) -> ICACHEF_R {
        ICACHEF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADCF"]
    #[inline(always)]
    pub fn adcf(&self) -> ADCF_R {
        ADCF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AESF"]
    #[inline(always)]
    pub fn aesf(&self) -> AESF_R {
        AESF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HASHF"]
    #[inline(always)]
    pub fn hashf(&self) -> HASHF_R {
        HASHF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RNGF"]
    #[inline(always)]
    pub fn rngf(&self) -> RNGF_R {
        RNGF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PKAF"]
    #[inline(always)]
    pub fn pkaf(&self) -> PKAF_R {
        PKAF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1F"]
    #[inline(always)]
    pub fn sdmmc1f(&self) -> SDMMC1F_R {
        SDMMC1F_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FMC_REGF"]
    #[inline(always)]
    pub fn fmc_regf(&self) -> FMC_REGF_R {
        FMC_REGF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGF"]
    #[inline(always)]
    pub fn octospi1_regf(&self) -> OCTOSPI1_REGF_R {
        OCTOSPI1_REGF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RTCF"]
    #[inline(always)]
    pub fn rtcf(&self) -> RTCF_R {
        RTCF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PWRF"]
    #[inline(always)]
    pub fn pwrf(&self) -> PWRF_R {
        PWRF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SYSCFGF"]
    #[inline(always)]
    pub fn syscfgf(&self) -> SYSCFGF_R {
        SYSCFGF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DMA1F"]
    #[inline(always)]
    pub fn dma1f(&self) -> DMA1F_R {
        DMA1F_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DMA2F"]
    #[inline(always)]
    pub fn dma2f(&self) -> DMA2F_R {
        DMA2F_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMAMUX1F"]
    #[inline(always)]
    pub fn dmamux1f(&self) -> DMAMUX1F_R {
        DMAMUX1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RCCF"]
    #[inline(always)]
    pub fn rccf(&self) -> RCCF_R {
        RCCF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FLASHF"]
    #[inline(always)]
    pub fn flashf(&self) -> FLASHF_R {
        FLASHF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FLASH_REGF"]
    #[inline(always)]
    pub fn flash_regf(&self) -> FLASH_REGF_R {
        FLASH_REGF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EXTIF"]
    #[inline(always)]
    pub fn extif(&self) -> EXTIF_R {
        EXTIF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - OTFDEC1F"]
    #[inline(always)]
    pub fn otfdec1f(&self) -> OTFDEC1F_R {
        OTFDEC1F_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8F"]
    #[inline(always)]
    pub fn tim8f(&mut self) -> TIM8F_W {
        TIM8F_W { w: self }
    }
    #[doc = "Bit 1 - USART1F"]
    #[inline(always)]
    pub fn usart1f(&mut self) -> USART1F_W {
        USART1F_W { w: self }
    }
    #[doc = "Bit 2 - TIM15F"]
    #[inline(always)]
    pub fn tim15f(&mut self) -> TIM15F_W {
        TIM15F_W { w: self }
    }
    #[doc = "Bit 3 - TIM16F"]
    #[inline(always)]
    pub fn tim16f(&mut self) -> TIM16F_W {
        TIM16F_W { w: self }
    }
    #[doc = "Bit 4 - TIM17F"]
    #[inline(always)]
    pub fn tim17f(&mut self) -> TIM17F_W {
        TIM17F_W { w: self }
    }
    #[doc = "Bit 5 - SAI1F"]
    #[inline(always)]
    pub fn sai1f(&mut self) -> SAI1F_W {
        SAI1F_W { w: self }
    }
    #[doc = "Bit 6 - SAI2F"]
    #[inline(always)]
    pub fn sai2f(&mut self) -> SAI2F_W {
        SAI2F_W { w: self }
    }
    #[doc = "Bit 7 - DFSDM1F"]
    #[inline(always)]
    pub fn dfsdm1f(&mut self) -> DFSDM1F_W {
        DFSDM1F_W { w: self }
    }
    #[doc = "Bit 8 - CRCF"]
    #[inline(always)]
    pub fn crcf(&mut self) -> CRCF_W {
        CRCF_W { w: self }
    }
    #[doc = "Bit 9 - TSCF"]
    #[inline(always)]
    pub fn tscf(&mut self) -> TSCF_W {
        TSCF_W { w: self }
    }
    #[doc = "Bit 10 - ICACHEF"]
    #[inline(always)]
    pub fn icachef(&mut self) -> ICACHEF_W {
        ICACHEF_W { w: self }
    }
    #[doc = "Bit 11 - ADCF"]
    #[inline(always)]
    pub fn adcf(&mut self) -> ADCF_W {
        ADCF_W { w: self }
    }
    #[doc = "Bit 12 - AESF"]
    #[inline(always)]
    pub fn aesf(&mut self) -> AESF_W {
        AESF_W { w: self }
    }
    #[doc = "Bit 13 - HASHF"]
    #[inline(always)]
    pub fn hashf(&mut self) -> HASHF_W {
        HASHF_W { w: self }
    }
    #[doc = "Bit 14 - RNGF"]
    #[inline(always)]
    pub fn rngf(&mut self) -> RNGF_W {
        RNGF_W { w: self }
    }
    #[doc = "Bit 15 - PKAF"]
    #[inline(always)]
    pub fn pkaf(&mut self) -> PKAF_W {
        PKAF_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1F"]
    #[inline(always)]
    pub fn sdmmc1f(&mut self) -> SDMMC1F_W {
        SDMMC1F_W { w: self }
    }
    #[doc = "Bit 17 - FMC_REGF"]
    #[inline(always)]
    pub fn fmc_regf(&mut self) -> FMC_REGF_W {
        FMC_REGF_W { w: self }
    }
    #[doc = "Bit 18 - OCTOSPI1_REGF"]
    #[inline(always)]
    pub fn octospi1_regf(&mut self) -> OCTOSPI1_REGF_W {
        OCTOSPI1_REGF_W { w: self }
    }
    #[doc = "Bit 19 - RTCF"]
    #[inline(always)]
    pub fn rtcf(&mut self) -> RTCF_W {
        RTCF_W { w: self }
    }
    #[doc = "Bit 20 - PWRF"]
    #[inline(always)]
    pub fn pwrf(&mut self) -> PWRF_W {
        PWRF_W { w: self }
    }
    #[doc = "Bit 21 - SYSCFGF"]
    #[inline(always)]
    pub fn syscfgf(&mut self) -> SYSCFGF_W {
        SYSCFGF_W { w: self }
    }
    #[doc = "Bit 22 - DMA1F"]
    #[inline(always)]
    pub fn dma1f(&mut self) -> DMA1F_W {
        DMA1F_W { w: self }
    }
    #[doc = "Bit 23 - DMA2F"]
    #[inline(always)]
    pub fn dma2f(&mut self) -> DMA2F_W {
        DMA2F_W { w: self }
    }
    #[doc = "Bit 24 - DMAMUX1F"]
    #[inline(always)]
    pub fn dmamux1f(&mut self) -> DMAMUX1F_W {
        DMAMUX1F_W { w: self }
    }
    #[doc = "Bit 25 - RCCF"]
    #[inline(always)]
    pub fn rccf(&mut self) -> RCCF_W {
        RCCF_W { w: self }
    }
    #[doc = "Bit 26 - FLASHF"]
    #[inline(always)]
    pub fn flashf(&mut self) -> FLASHF_W {
        FLASHF_W { w: self }
    }
    #[doc = "Bit 27 - FLASH_REGF"]
    #[inline(always)]
    pub fn flash_regf(&mut self) -> FLASH_REGF_W {
        FLASH_REGF_W { w: self }
    }
    #[doc = "Bit 28 - EXTIF"]
    #[inline(always)]
    pub fn extif(&mut self) -> EXTIF_W {
        EXTIF_W { w: self }
    }
    #[doc = "Bit 29 - OTFDEC1F"]
    #[inline(always)]
    pub fn otfdec1f(&mut self) -> OTFDEC1F_W {
        OTFDEC1F_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt status register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](index.html) module"]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr2::R](R) reader structure"]
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr2::W](W) writer structure"]
impl crate::Writable for SR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
