#[doc = "Register `IER1` reader"]
pub struct R(crate::R<IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER1` writer"]
pub struct W(crate::W<IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER1_SPEC>;
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
impl From<crate::W<IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZICIE` reader - TZICIE"]
pub struct TZICIE_R(crate::FieldReader<bool, bool>);
impl TZICIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZICIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZICIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZICIE` writer - TZICIE"]
pub struct TZICIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZICIE_W<'a> {
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
#[doc = "Field `TZSCIE` reader - TZSCIE"]
pub struct TZSCIE_R(crate::FieldReader<bool, bool>);
impl TZSCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZSCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZSCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZSCIE` writer - TZSCIE"]
pub struct TZSCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZSCIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SUBGHZSPIIE` reader - SUBGHZSPIIE"]
pub struct SUBGHZSPIIE_R(crate::FieldReader<bool, bool>);
impl SUBGHZSPIIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUBGHZSPIIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUBGHZSPIIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBGHZSPIIE` writer - SUBGHZSPIIE"]
pub struct SUBGHZSPIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHZSPIIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `FLASHIFIE` reader - FLASHIFIE"]
pub struct FLASHIFIE_R(crate::FieldReader<bool, bool>);
impl FLASHIFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHIFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHIFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHIFIE` writer - FLASHIFIE"]
pub struct FLASHIFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHIFIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SRAM1IE` reader - SRAM1IE"]
pub struct SRAM1IE_R(crate::FieldReader<bool, bool>);
impl SRAM1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM1IE` writer - SRAM1IE"]
pub struct SRAM1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1IE_W<'a> {
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
#[doc = "Field `SRAM2IE` reader - SRAM2IE"]
pub struct SRAM2IE_R(crate::FieldReader<bool, bool>);
impl SRAM2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2IE` writer - SRAM2IE"]
pub struct SRAM2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&self) -> TZICIE_R {
        TZICIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&self) -> TZSCIE_R {
        TZSCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AESIE"]
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RNGIE"]
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPIIE"]
    #[inline(always)]
    pub fn subghzspiie(&self) -> SUBGHZSPIIE_R {
        SUBGHZSPIIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWRIE"]
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLASHIFIE"]
    #[inline(always)]
    pub fn flashifie(&self) -> FLASHIFIE_R {
        FLASHIFIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA1IE"]
    #[inline(always)]
    pub fn dma1ie(&self) -> DMA1IE_R {
        DMA1IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA2IE"]
    #[inline(always)]
    pub fn dma2ie(&self) -> DMA2IE_R {
        DMA2IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMAMUX1IE"]
    #[inline(always)]
    pub fn dmamux1ie(&self) -> DMAMUX1IE_R {
        DMAMUX1IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FLASHIE"]
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SRAM1IE"]
    #[inline(always)]
    pub fn sram1ie(&self) -> SRAM1IE_R {
        SRAM1IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SRAM2IE"]
    #[inline(always)]
    pub fn sram2ie(&self) -> SRAM2IE_R {
        SRAM2IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PKAIE"]
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&mut self) -> TZICIE_W {
        TZICIE_W { w: self }
    }
    #[doc = "Bit 1 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&mut self) -> TZSCIE_W {
        TZSCIE_W { w: self }
    }
    #[doc = "Bit 2 - AESIE"]
    #[inline(always)]
    pub fn aesie(&mut self) -> AESIE_W {
        AESIE_W { w: self }
    }
    #[doc = "Bit 3 - RNGIE"]
    #[inline(always)]
    pub fn rngie(&mut self) -> RNGIE_W {
        RNGIE_W { w: self }
    }
    #[doc = "Bit 4 - SUBGHZSPIIE"]
    #[inline(always)]
    pub fn subghzspiie(&mut self) -> SUBGHZSPIIE_W {
        SUBGHZSPIIE_W { w: self }
    }
    #[doc = "Bit 5 - PWRIE"]
    #[inline(always)]
    pub fn pwrie(&mut self) -> PWRIE_W {
        PWRIE_W { w: self }
    }
    #[doc = "Bit 6 - FLASHIFIE"]
    #[inline(always)]
    pub fn flashifie(&mut self) -> FLASHIFIE_W {
        FLASHIFIE_W { w: self }
    }
    #[doc = "Bit 7 - DMA1IE"]
    #[inline(always)]
    pub fn dma1ie(&mut self) -> DMA1IE_W {
        DMA1IE_W { w: self }
    }
    #[doc = "Bit 8 - DMA2IE"]
    #[inline(always)]
    pub fn dma2ie(&mut self) -> DMA2IE_W {
        DMA2IE_W { w: self }
    }
    #[doc = "Bit 9 - DMAMUX1IE"]
    #[inline(always)]
    pub fn dmamux1ie(&mut self) -> DMAMUX1IE_W {
        DMAMUX1IE_W { w: self }
    }
    #[doc = "Bit 10 - FLASHIE"]
    #[inline(always)]
    pub fn flashie(&mut self) -> FLASHIE_W {
        FLASHIE_W { w: self }
    }
    #[doc = "Bit 11 - SRAM1IE"]
    #[inline(always)]
    pub fn sram1ie(&mut self) -> SRAM1IE_W {
        SRAM1IE_W { w: self }
    }
    #[doc = "Bit 12 - SRAM2IE"]
    #[inline(always)]
    pub fn sram2ie(&mut self) -> SRAM2IE_W {
        SRAM2IE_W { w: self }
    }
    #[doc = "Bit 13 - PKAIE"]
    #[inline(always)]
    pub fn pkaie(&mut self) -> PKAIE_W {
        PKAIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](index.html) module"]
pub struct IER1_SPEC;
impl crate::RegisterSpec for IER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier1::R](R) reader structure"]
impl crate::Readable for IER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier1::W](W) writer structure"]
impl crate::Writable for IER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER1 to value 0xffff_ffff"]
impl crate::Resettable for IER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
