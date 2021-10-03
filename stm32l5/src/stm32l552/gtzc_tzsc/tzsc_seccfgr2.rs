#[doc = "Register `TZSC_SECCFGR2` reader"]
pub struct R(crate::R<TZSC_SECCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_SECCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_SECCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_SECCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZSC_SECCFGR2` writer"]
pub struct W(crate::W<TZSC_SECCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_SECCFGR2_SPEC>;
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
impl From<crate::W<TZSC_SECCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_SECCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM8SEC` reader - TIM8SEC"]
pub struct TIM8SEC_R(crate::FieldReader<bool, bool>);
impl TIM8SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM8SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8SEC` writer - TIM8SEC"]
pub struct TIM8SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8SEC_W<'a> {
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
#[doc = "Field `USART1SEC` reader - USART1SEC"]
pub struct USART1SEC_R(crate::FieldReader<bool, bool>);
impl USART1SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1SEC` writer - USART1SEC"]
pub struct USART1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEC_W<'a> {
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
#[doc = "Field `TIM15SEC` reader - TIM15SEC"]
pub struct TIM15SEC_R(crate::FieldReader<bool, bool>);
impl TIM15SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15SEC` writer - TIM15SEC"]
pub struct TIM15SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15SEC_W<'a> {
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
#[doc = "Field `TIM16SEC` reader - TIM16SEC"]
pub struct TIM16SEC_R(crate::FieldReader<bool, bool>);
impl TIM16SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16SEC` writer - TIM16SEC"]
pub struct TIM16SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16SEC_W<'a> {
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
#[doc = "Field `TIM17SEC` reader - TIM17SEC"]
pub struct TIM17SEC_R(crate::FieldReader<bool, bool>);
impl TIM17SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17SEC` writer - TIM17SEC"]
pub struct TIM17SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17SEC_W<'a> {
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
#[doc = "Field `SAI1SEC` reader - SAI1SEC"]
pub struct SAI1SEC_R(crate::FieldReader<bool, bool>);
impl SAI1SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI1SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1SEC` writer - SAI1SEC"]
pub struct SAI1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SEC_W<'a> {
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
#[doc = "Field `SAI2SEC` reader - SAI2SEC"]
pub struct SAI2SEC_R(crate::FieldReader<bool, bool>);
impl SAI2SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI2SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2SEC` writer - SAI2SEC"]
pub struct SAI2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2SEC_W<'a> {
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
#[doc = "Field `DFSDM1SEC` reader - DFSDM1SEC"]
pub struct DFSDM1SEC_R(crate::FieldReader<bool, bool>);
impl DFSDM1SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDM1SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDM1SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDM1SEC` writer - DFSDM1SEC"]
pub struct DFSDM1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1SEC_W<'a> {
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
#[doc = "Field `CRCSEC` reader - CRCSEC"]
pub struct CRCSEC_R(crate::FieldReader<bool, bool>);
impl CRCSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCSEC` writer - CRCSEC"]
pub struct CRCSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSEC_W<'a> {
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
#[doc = "Field `TSCSEC` reader - TSCSEC"]
pub struct TSCSEC_R(crate::FieldReader<bool, bool>);
impl TSCSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCSEC` writer - TSCSEC"]
pub struct TSCSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCSEC_W<'a> {
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
#[doc = "Field `ICACHESEC` reader - ICACHESEC"]
pub struct ICACHESEC_R(crate::FieldReader<bool, bool>);
impl ICACHESEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHESEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHESEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHESEC` writer - ICACHESEC"]
pub struct ICACHESEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHESEC_W<'a> {
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
#[doc = "Field `ADCSEC` reader - ADCSEC"]
pub struct ADCSEC_R(crate::FieldReader<bool, bool>);
impl ADCSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCSEC` writer - ADCSEC"]
pub struct ADCSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSEC_W<'a> {
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
#[doc = "Field `AESSEC` reader - AESSEC"]
pub struct AESSEC_R(crate::FieldReader<bool, bool>);
impl AESSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESSEC` writer - AESSEC"]
pub struct AESSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> AESSEC_W<'a> {
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
#[doc = "Field `HASHSEC` reader - HASHSEC"]
pub struct HASHSEC_R(crate::FieldReader<bool, bool>);
impl HASHSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASHSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASHSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASHSEC` writer - HASHSEC"]
pub struct HASHSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHSEC_W<'a> {
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
#[doc = "Field `RNGSEC` reader - RNGSEC"]
pub struct RNGSEC_R(crate::FieldReader<bool, bool>);
impl RNGSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGSEC` writer - RNGSEC"]
pub struct RNGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSEC_W<'a> {
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
#[doc = "Field `PKASEC` reader - PKASEC"]
pub struct PKASEC_R(crate::FieldReader<bool, bool>);
impl PKASEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKASEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKASEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKASEC` writer - PKASEC"]
pub struct PKASEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PKASEC_W<'a> {
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
#[doc = "Field `SDMMC1SEC` reader - SDMMC1SEC"]
pub struct SDMMC1SEC_R(crate::FieldReader<bool, bool>);
impl SDMMC1SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC1SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC1SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC1SEC` writer - SDMMC1SEC"]
pub struct SDMMC1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1SEC_W<'a> {
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
#[doc = "Field `FSMC_REGSEC` reader - FSMC_REGSEC"]
pub struct FSMC_REGSEC_R(crate::FieldReader<bool, bool>);
impl FSMC_REGSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSMC_REGSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSMC_REGSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSMC_REGSEC` writer - FSMC_REGSEC"]
pub struct FSMC_REGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMC_REGSEC_W<'a> {
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
#[doc = "Field `OCTOSPI1_REGSEC` reader - OCTOSPI1_REGSEC"]
pub struct OCTOSPI1_REGSEC_R(crate::FieldReader<bool, bool>);
impl OCTOSPI1_REGSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCTOSPI1_REGSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCTOSPI1_REGSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCTOSPI1_REGSEC` writer - OCTOSPI1_REGSEC"]
pub struct OCTOSPI1_REGSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1_REGSEC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TIM8SEC"]
    #[inline(always)]
    pub fn tim8sec(&self) -> TIM8SEC_R {
        TIM8SEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USART1SEC"]
    #[inline(always)]
    pub fn usart1sec(&self) -> USART1SEC_R {
        USART1SEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15SEC"]
    #[inline(always)]
    pub fn tim15sec(&self) -> TIM15SEC_R {
        TIM15SEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16SEC"]
    #[inline(always)]
    pub fn tim16sec(&self) -> TIM16SEC_R {
        TIM16SEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17SEC"]
    #[inline(always)]
    pub fn tim17sec(&self) -> TIM17SEC_R {
        TIM17SEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SAI1SEC"]
    #[inline(always)]
    pub fn sai1sec(&self) -> SAI1SEC_R {
        SAI1SEC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAI2SEC"]
    #[inline(always)]
    pub fn sai2sec(&self) -> SAI2SEC_R {
        SAI2SEC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFSDM1SEC"]
    #[inline(always)]
    pub fn dfsdm1sec(&self) -> DFSDM1SEC_R {
        DFSDM1SEC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CRCSEC"]
    #[inline(always)]
    pub fn crcsec(&self) -> CRCSEC_R {
        CRCSEC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSCSEC"]
    #[inline(always)]
    pub fn tscsec(&self) -> TSCSEC_R {
        TSCSEC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ICACHESEC"]
    #[inline(always)]
    pub fn icachesec(&self) -> ICACHESEC_R {
        ICACHESEC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADCSEC"]
    #[inline(always)]
    pub fn adcsec(&self) -> ADCSEC_R {
        ADCSEC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AESSEC"]
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HASHSEC"]
    #[inline(always)]
    pub fn hashsec(&self) -> HASHSEC_R {
        HASHSEC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RNGSEC"]
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PKASEC"]
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1SEC"]
    #[inline(always)]
    pub fn sdmmc1sec(&self) -> SDMMC1SEC_R {
        SDMMC1SEC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FSMC_REGSEC"]
    #[inline(always)]
    pub fn fsmc_regsec(&self) -> FSMC_REGSEC_R {
        FSMC_REGSEC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGSEC"]
    #[inline(always)]
    pub fn octospi1_regsec(&self) -> OCTOSPI1_REGSEC_R {
        OCTOSPI1_REGSEC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8SEC"]
    #[inline(always)]
    pub fn tim8sec(&mut self) -> TIM8SEC_W {
        TIM8SEC_W { w: self }
    }
    #[doc = "Bit 1 - USART1SEC"]
    #[inline(always)]
    pub fn usart1sec(&mut self) -> USART1SEC_W {
        USART1SEC_W { w: self }
    }
    #[doc = "Bit 2 - TIM15SEC"]
    #[inline(always)]
    pub fn tim15sec(&mut self) -> TIM15SEC_W {
        TIM15SEC_W { w: self }
    }
    #[doc = "Bit 3 - TIM16SEC"]
    #[inline(always)]
    pub fn tim16sec(&mut self) -> TIM16SEC_W {
        TIM16SEC_W { w: self }
    }
    #[doc = "Bit 4 - TIM17SEC"]
    #[inline(always)]
    pub fn tim17sec(&mut self) -> TIM17SEC_W {
        TIM17SEC_W { w: self }
    }
    #[doc = "Bit 5 - SAI1SEC"]
    #[inline(always)]
    pub fn sai1sec(&mut self) -> SAI1SEC_W {
        SAI1SEC_W { w: self }
    }
    #[doc = "Bit 6 - SAI2SEC"]
    #[inline(always)]
    pub fn sai2sec(&mut self) -> SAI2SEC_W {
        SAI2SEC_W { w: self }
    }
    #[doc = "Bit 7 - DFSDM1SEC"]
    #[inline(always)]
    pub fn dfsdm1sec(&mut self) -> DFSDM1SEC_W {
        DFSDM1SEC_W { w: self }
    }
    #[doc = "Bit 8 - CRCSEC"]
    #[inline(always)]
    pub fn crcsec(&mut self) -> CRCSEC_W {
        CRCSEC_W { w: self }
    }
    #[doc = "Bit 9 - TSCSEC"]
    #[inline(always)]
    pub fn tscsec(&mut self) -> TSCSEC_W {
        TSCSEC_W { w: self }
    }
    #[doc = "Bit 10 - ICACHESEC"]
    #[inline(always)]
    pub fn icachesec(&mut self) -> ICACHESEC_W {
        ICACHESEC_W { w: self }
    }
    #[doc = "Bit 11 - ADCSEC"]
    #[inline(always)]
    pub fn adcsec(&mut self) -> ADCSEC_W {
        ADCSEC_W { w: self }
    }
    #[doc = "Bit 12 - AESSEC"]
    #[inline(always)]
    pub fn aessec(&mut self) -> AESSEC_W {
        AESSEC_W { w: self }
    }
    #[doc = "Bit 13 - HASHSEC"]
    #[inline(always)]
    pub fn hashsec(&mut self) -> HASHSEC_W {
        HASHSEC_W { w: self }
    }
    #[doc = "Bit 14 - RNGSEC"]
    #[inline(always)]
    pub fn rngsec(&mut self) -> RNGSEC_W {
        RNGSEC_W { w: self }
    }
    #[doc = "Bit 15 - PKASEC"]
    #[inline(always)]
    pub fn pkasec(&mut self) -> PKASEC_W {
        PKASEC_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1SEC"]
    #[inline(always)]
    pub fn sdmmc1sec(&mut self) -> SDMMC1SEC_W {
        SDMMC1SEC_W { w: self }
    }
    #[doc = "Bit 17 - FSMC_REGSEC"]
    #[inline(always)]
    pub fn fsmc_regsec(&mut self) -> FSMC_REGSEC_W {
        FSMC_REGSEC_W { w: self }
    }
    #[doc = "Bit 18 - OCTOSPI1_REGSEC"]
    #[inline(always)]
    pub fn octospi1_regsec(&mut self) -> OCTOSPI1_REGSEC_W {
        OCTOSPI1_REGSEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZSC secure configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_seccfgr2](index.html) module"]
pub struct TZSC_SECCFGR2_SPEC;
impl crate::RegisterSpec for TZSC_SECCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzsc_seccfgr2::R](R) reader structure"]
impl crate::Readable for TZSC_SECCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzsc_seccfgr2::W](W) writer structure"]
impl crate::Writable for TZSC_SECCFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZSC_SECCFGR2 to value 0"]
impl crate::Resettable for TZSC_SECCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
