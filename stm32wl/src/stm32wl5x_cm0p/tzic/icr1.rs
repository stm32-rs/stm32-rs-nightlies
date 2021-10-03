#[doc = "Register `ICR1` reader"]
pub struct R(crate::R<ICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR1` writer"]
pub struct W(crate::W<ICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR1_SPEC>;
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
impl From<crate::W<ICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZICCF` reader - TZICCF"]
pub struct TZICCF_R(crate::FieldReader<bool, bool>);
impl TZICCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZICCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZICCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZICCF` writer - TZICCF"]
pub struct TZICCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TZICCF_W<'a> {
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
#[doc = "Field `TZSCCF` reader - TZSCCF"]
pub struct TZSCCF_R(crate::FieldReader<bool, bool>);
impl TZSCCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZSCCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZSCCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZSCCF` writer - TZSCCF"]
pub struct TZSCCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TZSCCF_W<'a> {
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
#[doc = "Field `AESCF` reader - AESCF"]
pub struct AESCF_R(crate::FieldReader<bool, bool>);
impl AESCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESCF` writer - AESCF"]
pub struct AESCF_W<'a> {
    w: &'a mut W,
}
impl<'a> AESCF_W<'a> {
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
#[doc = "Field `RNGCF` reader - RNGCF"]
pub struct RNGCF_R(crate::FieldReader<bool, bool>);
impl RNGCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGCF` writer - RNGCF"]
pub struct RNGCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGCF_W<'a> {
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
#[doc = "Field `SUBGHZSPICF` reader - SUBGHZSPICF"]
pub struct SUBGHZSPICF_R(crate::FieldReader<bool, bool>);
impl SUBGHZSPICF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUBGHZSPICF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUBGHZSPICF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBGHZSPICF` writer - SUBGHZSPICF"]
pub struct SUBGHZSPICF_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHZSPICF_W<'a> {
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
#[doc = "Field `PWRCF` reader - PWRCF"]
pub struct PWRCF_R(crate::FieldReader<bool, bool>);
impl PWRCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRCF` writer - PWRCF"]
pub struct PWRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRCF_W<'a> {
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
#[doc = "Field `FLASHIFCF` reader - FLASHIFCF"]
pub struct FLASHIFCF_R(crate::FieldReader<bool, bool>);
impl FLASHIFCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHIFCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHIFCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHIFCF` writer - FLASHIFCF"]
pub struct FLASHIFCF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHIFCF_W<'a> {
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
#[doc = "Field `DMA1CF` reader - DMA1CF"]
pub struct DMA1CF_R(crate::FieldReader<bool, bool>);
impl DMA1CF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1CF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1CF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1CF` writer - DMA1CF"]
pub struct DMA1CF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1CF_W<'a> {
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
#[doc = "Field `DMA2CF` reader - DMA2CF"]
pub struct DMA2CF_R(crate::FieldReader<bool, bool>);
impl DMA2CF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2CF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2CF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2CF` writer - DMA2CF"]
pub struct DMA2CF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2CF_W<'a> {
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
#[doc = "Field `DMAMUX1CF` reader - DMAMUX1CF"]
pub struct DMAMUX1CF_R(crate::FieldReader<bool, bool>);
impl DMAMUX1CF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAMUX1CF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAMUX1CF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAMUX1CF` writer - DMAMUX1CF"]
pub struct DMAMUX1CF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1CF_W<'a> {
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
#[doc = "Field `FLASHCF` reader - FLASHCF"]
pub struct FLASHCF_R(crate::FieldReader<bool, bool>);
impl FLASHCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHCF` writer - FLASHCF"]
pub struct FLASHCF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHCF_W<'a> {
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
#[doc = "Field `SRAM1CF` reader - SRAM1CF"]
pub struct SRAM1CF_R(crate::FieldReader<bool, bool>);
impl SRAM1CF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1CF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM1CF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM1CF` writer - SRAM1CF"]
pub struct SRAM1CF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1CF_W<'a> {
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
#[doc = "Field `SRAM2CF` reader - SRAM2CF"]
pub struct SRAM2CF_R(crate::FieldReader<bool, bool>);
impl SRAM2CF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2CF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2CF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2CF` writer - SRAM2CF"]
pub struct SRAM2CF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2CF_W<'a> {
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
#[doc = "Field `PKACF` reader - PKACF"]
pub struct PKACF_R(crate::FieldReader<bool, bool>);
impl PKACF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKACF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKACF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKACF` writer - PKACF"]
pub struct PKACF_W<'a> {
    w: &'a mut W,
}
impl<'a> PKACF_W<'a> {
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
    #[doc = "Bit 0 - TZICCF"]
    #[inline(always)]
    pub fn tziccf(&self) -> TZICCF_R {
        TZICCF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZSCCF"]
    #[inline(always)]
    pub fn tzsccf(&self) -> TZSCCF_R {
        TZSCCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AESCF"]
    #[inline(always)]
    pub fn aescf(&self) -> AESCF_R {
        AESCF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RNGCF"]
    #[inline(always)]
    pub fn rngcf(&self) -> RNGCF_R {
        RNGCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPICF"]
    #[inline(always)]
    pub fn subghzspicf(&self) -> SUBGHZSPICF_R {
        SUBGHZSPICF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWRCF"]
    #[inline(always)]
    pub fn pwrcf(&self) -> PWRCF_R {
        PWRCF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLASHIFCF"]
    #[inline(always)]
    pub fn flashifcf(&self) -> FLASHIFCF_R {
        FLASHIFCF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA1CF"]
    #[inline(always)]
    pub fn dma1cf(&self) -> DMA1CF_R {
        DMA1CF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA2CF"]
    #[inline(always)]
    pub fn dma2cf(&self) -> DMA2CF_R {
        DMA2CF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMAMUX1CF"]
    #[inline(always)]
    pub fn dmamux1cf(&self) -> DMAMUX1CF_R {
        DMAMUX1CF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FLASHCF"]
    #[inline(always)]
    pub fn flashcf(&self) -> FLASHCF_R {
        FLASHCF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SRAM1CF"]
    #[inline(always)]
    pub fn sram1cf(&self) -> SRAM1CF_R {
        SRAM1CF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SRAM2CF"]
    #[inline(always)]
    pub fn sram2cf(&self) -> SRAM2CF_R {
        SRAM2CF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PKACF"]
    #[inline(always)]
    pub fn pkacf(&self) -> PKACF_R {
        PKACF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZICCF"]
    #[inline(always)]
    pub fn tziccf(&mut self) -> TZICCF_W {
        TZICCF_W { w: self }
    }
    #[doc = "Bit 1 - TZSCCF"]
    #[inline(always)]
    pub fn tzsccf(&mut self) -> TZSCCF_W {
        TZSCCF_W { w: self }
    }
    #[doc = "Bit 2 - AESCF"]
    #[inline(always)]
    pub fn aescf(&mut self) -> AESCF_W {
        AESCF_W { w: self }
    }
    #[doc = "Bit 3 - RNGCF"]
    #[inline(always)]
    pub fn rngcf(&mut self) -> RNGCF_W {
        RNGCF_W { w: self }
    }
    #[doc = "Bit 4 - SUBGHZSPICF"]
    #[inline(always)]
    pub fn subghzspicf(&mut self) -> SUBGHZSPICF_W {
        SUBGHZSPICF_W { w: self }
    }
    #[doc = "Bit 5 - PWRCF"]
    #[inline(always)]
    pub fn pwrcf(&mut self) -> PWRCF_W {
        PWRCF_W { w: self }
    }
    #[doc = "Bit 6 - FLASHIFCF"]
    #[inline(always)]
    pub fn flashifcf(&mut self) -> FLASHIFCF_W {
        FLASHIFCF_W { w: self }
    }
    #[doc = "Bit 7 - DMA1CF"]
    #[inline(always)]
    pub fn dma1cf(&mut self) -> DMA1CF_W {
        DMA1CF_W { w: self }
    }
    #[doc = "Bit 8 - DMA2CF"]
    #[inline(always)]
    pub fn dma2cf(&mut self) -> DMA2CF_W {
        DMA2CF_W { w: self }
    }
    #[doc = "Bit 9 - DMAMUX1CF"]
    #[inline(always)]
    pub fn dmamux1cf(&mut self) -> DMAMUX1CF_W {
        DMAMUX1CF_W { w: self }
    }
    #[doc = "Bit 10 - FLASHCF"]
    #[inline(always)]
    pub fn flashcf(&mut self) -> FLASHCF_W {
        FLASHCF_W { w: self }
    }
    #[doc = "Bit 11 - SRAM1CF"]
    #[inline(always)]
    pub fn sram1cf(&mut self) -> SRAM1CF_W {
        SRAM1CF_W { w: self }
    }
    #[doc = "Bit 12 - SRAM2CF"]
    #[inline(always)]
    pub fn sram2cf(&mut self) -> SRAM2CF_W {
        SRAM2CF_W { w: self }
    }
    #[doc = "Bit 13 - PKACF"]
    #[inline(always)]
    pub fn pkacf(&mut self) -> PKACF_W {
        PKACF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt status clear register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr1](index.html) module"]
pub struct ICR1_SPEC;
impl crate::RegisterSpec for ICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr1::R](R) reader structure"]
impl crate::Readable for ICR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr1::W](W) writer structure"]
impl crate::Writable for ICR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR1 to value 0"]
impl crate::Resettable for ICR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
