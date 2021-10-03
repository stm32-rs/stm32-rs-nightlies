#[doc = "Register `TZSC_PRIVCFGR2` reader"]
pub struct R(crate::R<TZSC_PRIVCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_PRIVCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_PRIVCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_PRIVCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZSC_PRIVCFGR2` writer"]
pub struct W(crate::W<TZSC_PRIVCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_PRIVCFGR2_SPEC>;
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
impl From<crate::W<TZSC_PRIVCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_PRIVCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM8PRIV` reader - TIM8PRIV"]
pub struct TIM8PRIV_R(crate::FieldReader<bool, bool>);
impl TIM8PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM8PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8PRIV` writer - TIM8PRIV"]
pub struct TIM8PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8PRIV_W<'a> {
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
#[doc = "Field `USART1PRIV` reader - USART1PRIV"]
pub struct USART1PRIV_R(crate::FieldReader<bool, bool>);
impl USART1PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1PRIV` writer - USART1PRIV"]
pub struct USART1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1PRIV_W<'a> {
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
#[doc = "Field `TIM15PRIV` reader - TIM15PRIV"]
pub struct TIM15PRIV_R(crate::FieldReader<bool, bool>);
impl TIM15PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15PRIV` writer - TIM15PRIV"]
pub struct TIM15PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15PRIV_W<'a> {
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
#[doc = "Field `TIM16PRIV` reader - TIM16PRIV"]
pub struct TIM16PRIV_R(crate::FieldReader<bool, bool>);
impl TIM16PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16PRIV` writer - TIM16PRIV"]
pub struct TIM16PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16PRIV_W<'a> {
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
#[doc = "Field `TIM17PRIV` reader - TIM17PRIV"]
pub struct TIM17PRIV_R(crate::FieldReader<bool, bool>);
impl TIM17PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17PRIV` writer - TIM17PRIV"]
pub struct TIM17PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17PRIV_W<'a> {
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
#[doc = "Field `SAI1PRIV` reader - SAI1PRIV"]
pub struct SAI1PRIV_R(crate::FieldReader<bool, bool>);
impl SAI1PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI1PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1PRIV` writer - SAI1PRIV"]
pub struct SAI1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1PRIV_W<'a> {
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
#[doc = "Field `SAI2PRIV` reader - SAI2PRIV"]
pub struct SAI2PRIV_R(crate::FieldReader<bool, bool>);
impl SAI2PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI2PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2PRIV` writer - SAI2PRIV"]
pub struct SAI2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2PRIV_W<'a> {
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
#[doc = "Field `DFSDM1PRIV` reader - DFSDM1PRIV"]
pub struct DFSDM1PRIV_R(crate::FieldReader<bool, bool>);
impl DFSDM1PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDM1PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDM1PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDM1PRIV` writer - DFSDM1PRIV"]
pub struct DFSDM1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1PRIV_W<'a> {
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
#[doc = "Field `CRCPRIV` reader - CRCPRIV"]
pub struct CRCPRIV_R(crate::FieldReader<bool, bool>);
impl CRCPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCPRIV` writer - CRCPRIV"]
pub struct CRCPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCPRIV_W<'a> {
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
#[doc = "Field `TSCPRIV` reader - TSCPRIV"]
pub struct TSCPRIV_R(crate::FieldReader<bool, bool>);
impl TSCPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCPRIV` writer - TSCPRIV"]
pub struct TSCPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCPRIV_W<'a> {
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
#[doc = "Field `ICACHEPRIV` reader - ICACHEPRIV"]
pub struct ICACHEPRIV_R(crate::FieldReader<bool, bool>);
impl ICACHEPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHEPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHEPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHEPRIV` writer - ICACHEPRIV"]
pub struct ICACHEPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHEPRIV_W<'a> {
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
#[doc = "Field `ADCPRIV` reader - ADCPRIV"]
pub struct ADCPRIV_R(crate::FieldReader<bool, bool>);
impl ADCPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCPRIV` writer - ADCPRIV"]
pub struct ADCPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPRIV_W<'a> {
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
#[doc = "Field `AESPRIV` reader - AESPRIV"]
pub struct AESPRIV_R(crate::FieldReader<bool, bool>);
impl AESPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESPRIV` writer - AESPRIV"]
pub struct AESPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> AESPRIV_W<'a> {
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
#[doc = "Field `HASHPRIV` reader - HASHPRIV"]
pub struct HASHPRIV_R(crate::FieldReader<bool, bool>);
impl HASHPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASHPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASHPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASHPRIV` writer - HASHPRIV"]
pub struct HASHPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHPRIV_W<'a> {
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
#[doc = "Field `RNGPRIV` reader - RNGPRIV"]
pub struct RNGPRIV_R(crate::FieldReader<bool, bool>);
impl RNGPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGPRIV` writer - RNGPRIV"]
pub struct RNGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGPRIV_W<'a> {
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
#[doc = "Field `PKAPRIV` reader - PKAPRIV"]
pub struct PKAPRIV_R(crate::FieldReader<bool, bool>);
impl PKAPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKAPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKAPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKAPRIV` writer - PKAPRIV"]
pub struct PKAPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAPRIV_W<'a> {
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
#[doc = "Field `SDMMC1PRIV` reader - SDMMC1PRIV"]
pub struct SDMMC1PRIV_R(crate::FieldReader<bool, bool>);
impl SDMMC1PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC1PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC1PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC1PRIV` writer - SDMMC1PRIV"]
pub struct SDMMC1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1PRIV_W<'a> {
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
#[doc = "Field `FSMC_REGPRIV` reader - FSMC_REGPRIV"]
pub struct FSMC_REGPRIV_R(crate::FieldReader<bool, bool>);
impl FSMC_REGPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSMC_REGPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSMC_REGPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSMC_REGPRIV` writer - FSMC_REGPRIV"]
pub struct FSMC_REGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMC_REGPRIV_W<'a> {
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
#[doc = "Field `OCTOSPI1_REGPRIV` reader - OCTOSPI1_REGRIV"]
pub struct OCTOSPI1_REGPRIV_R(crate::FieldReader<bool, bool>);
impl OCTOSPI1_REGPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCTOSPI1_REGPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCTOSPI1_REGPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCTOSPI1_REGPRIV` writer - OCTOSPI1_REGRIV"]
pub struct OCTOSPI1_REGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPI1_REGPRIV_W<'a> {
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
    #[doc = "Bit 0 - TIM8PRIV"]
    #[inline(always)]
    pub fn tim8priv(&self) -> TIM8PRIV_R {
        TIM8PRIV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USART1PRIV"]
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15PRIV"]
    #[inline(always)]
    pub fn tim15priv(&self) -> TIM15PRIV_R {
        TIM15PRIV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16PRIV"]
    #[inline(always)]
    pub fn tim16priv(&self) -> TIM16PRIV_R {
        TIM16PRIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17PRIV"]
    #[inline(always)]
    pub fn tim17priv(&self) -> TIM17PRIV_R {
        TIM17PRIV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SAI1PRIV"]
    #[inline(always)]
    pub fn sai1priv(&self) -> SAI1PRIV_R {
        SAI1PRIV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAI2PRIV"]
    #[inline(always)]
    pub fn sai2priv(&self) -> SAI2PRIV_R {
        SAI2PRIV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DFSDM1PRIV"]
    #[inline(always)]
    pub fn dfsdm1priv(&self) -> DFSDM1PRIV_R {
        DFSDM1PRIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CRCPRIV"]
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSCPRIV"]
    #[inline(always)]
    pub fn tscpriv(&self) -> TSCPRIV_R {
        TSCPRIV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ICACHEPRIV"]
    #[inline(always)]
    pub fn icachepriv(&self) -> ICACHEPRIV_R {
        ICACHEPRIV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADCPRIV"]
    #[inline(always)]
    pub fn adcpriv(&self) -> ADCPRIV_R {
        ADCPRIV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AESPRIV"]
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HASHPRIV"]
    #[inline(always)]
    pub fn hashpriv(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RNGPRIV"]
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PKAPRIV"]
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1PRIV"]
    #[inline(always)]
    pub fn sdmmc1priv(&self) -> SDMMC1PRIV_R {
        SDMMC1PRIV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FSMC_REGPRIV"]
    #[inline(always)]
    pub fn fsmc_regpriv(&self) -> FSMC_REGPRIV_R {
        FSMC_REGPRIV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGRIV"]
    #[inline(always)]
    pub fn octospi1_regpriv(&self) -> OCTOSPI1_REGPRIV_R {
        OCTOSPI1_REGPRIV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8PRIV"]
    #[inline(always)]
    pub fn tim8priv(&mut self) -> TIM8PRIV_W {
        TIM8PRIV_W { w: self }
    }
    #[doc = "Bit 1 - USART1PRIV"]
    #[inline(always)]
    pub fn usart1priv(&mut self) -> USART1PRIV_W {
        USART1PRIV_W { w: self }
    }
    #[doc = "Bit 2 - TIM15PRIV"]
    #[inline(always)]
    pub fn tim15priv(&mut self) -> TIM15PRIV_W {
        TIM15PRIV_W { w: self }
    }
    #[doc = "Bit 3 - TIM16PRIV"]
    #[inline(always)]
    pub fn tim16priv(&mut self) -> TIM16PRIV_W {
        TIM16PRIV_W { w: self }
    }
    #[doc = "Bit 4 - TIM17PRIV"]
    #[inline(always)]
    pub fn tim17priv(&mut self) -> TIM17PRIV_W {
        TIM17PRIV_W { w: self }
    }
    #[doc = "Bit 5 - SAI1PRIV"]
    #[inline(always)]
    pub fn sai1priv(&mut self) -> SAI1PRIV_W {
        SAI1PRIV_W { w: self }
    }
    #[doc = "Bit 6 - SAI2PRIV"]
    #[inline(always)]
    pub fn sai2priv(&mut self) -> SAI2PRIV_W {
        SAI2PRIV_W { w: self }
    }
    #[doc = "Bit 7 - DFSDM1PRIV"]
    #[inline(always)]
    pub fn dfsdm1priv(&mut self) -> DFSDM1PRIV_W {
        DFSDM1PRIV_W { w: self }
    }
    #[doc = "Bit 8 - CRCPRIV"]
    #[inline(always)]
    pub fn crcpriv(&mut self) -> CRCPRIV_W {
        CRCPRIV_W { w: self }
    }
    #[doc = "Bit 9 - TSCPRIV"]
    #[inline(always)]
    pub fn tscpriv(&mut self) -> TSCPRIV_W {
        TSCPRIV_W { w: self }
    }
    #[doc = "Bit 10 - ICACHEPRIV"]
    #[inline(always)]
    pub fn icachepriv(&mut self) -> ICACHEPRIV_W {
        ICACHEPRIV_W { w: self }
    }
    #[doc = "Bit 11 - ADCPRIV"]
    #[inline(always)]
    pub fn adcpriv(&mut self) -> ADCPRIV_W {
        ADCPRIV_W { w: self }
    }
    #[doc = "Bit 12 - AESPRIV"]
    #[inline(always)]
    pub fn aespriv(&mut self) -> AESPRIV_W {
        AESPRIV_W { w: self }
    }
    #[doc = "Bit 13 - HASHPRIV"]
    #[inline(always)]
    pub fn hashpriv(&mut self) -> HASHPRIV_W {
        HASHPRIV_W { w: self }
    }
    #[doc = "Bit 14 - RNGPRIV"]
    #[inline(always)]
    pub fn rngpriv(&mut self) -> RNGPRIV_W {
        RNGPRIV_W { w: self }
    }
    #[doc = "Bit 15 - PKAPRIV"]
    #[inline(always)]
    pub fn pkapriv(&mut self) -> PKAPRIV_W {
        PKAPRIV_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1PRIV"]
    #[inline(always)]
    pub fn sdmmc1priv(&mut self) -> SDMMC1PRIV_W {
        SDMMC1PRIV_W { w: self }
    }
    #[doc = "Bit 17 - FSMC_REGPRIV"]
    #[inline(always)]
    pub fn fsmc_regpriv(&mut self) -> FSMC_REGPRIV_W {
        FSMC_REGPRIV_W { w: self }
    }
    #[doc = "Bit 18 - OCTOSPI1_REGRIV"]
    #[inline(always)]
    pub fn octospi1_regpriv(&mut self) -> OCTOSPI1_REGPRIV_W {
        OCTOSPI1_REGPRIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZSC privilege configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_privcfgr2](index.html) module"]
pub struct TZSC_PRIVCFGR2_SPEC;
impl crate::RegisterSpec for TZSC_PRIVCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzsc_privcfgr2::R](R) reader structure"]
impl crate::Readable for TZSC_PRIVCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzsc_privcfgr2::W](W) writer structure"]
impl crate::Writable for TZSC_PRIVCFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZSC_PRIVCFGR2 to value 0"]
impl crate::Resettable for TZSC_PRIVCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
