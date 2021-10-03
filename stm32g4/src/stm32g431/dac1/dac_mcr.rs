#[doc = "Register `DAC_MCR` reader"]
pub struct R(crate::R<DAC_MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_MCR` writer"]
pub struct W(crate::W<DAC_MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_MCR_SPEC>;
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
impl From<crate::W<DAC_MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE1` reader - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
pub struct MODE1_R(crate::FieldReader<u8, u8>);
impl MODE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE1` writer - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `DMADOUBLE1` reader - DAC Channel1 DMA double data mode"]
pub struct DMADOUBLE1_R(crate::FieldReader<bool, bool>);
impl DMADOUBLE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMADOUBLE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMADOUBLE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMADOUBLE1` writer - DAC Channel1 DMA double data mode"]
pub struct DMADOUBLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADOUBLE1_W<'a> {
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
#[doc = "Field `SINFORMAT1` reader - Enable signed format for DAC channel1"]
pub struct SINFORMAT1_R(crate::FieldReader<bool, bool>);
impl SINFORMAT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINFORMAT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINFORMAT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINFORMAT1` writer - Enable signed format for DAC channel1"]
pub struct SINFORMAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SINFORMAT1_W<'a> {
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
#[doc = "Field `HFSEL` reader - High frequency interface mode selection"]
pub struct HFSEL_R(crate::FieldReader<u8, u8>);
impl HFSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFSEL` writer - High frequency interface mode selection"]
pub struct HFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `MODE2` reader - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
pub struct MODE2_R(crate::FieldReader<u8, u8>);
impl MODE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE2` writer - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
pub struct MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `DMADOUBLE2` reader - DAC Channel2 DMA double data mode"]
pub struct DMADOUBLE2_R(crate::FieldReader<bool, bool>);
impl DMADOUBLE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMADOUBLE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMADOUBLE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMADOUBLE2` writer - DAC Channel2 DMA double data mode"]
pub struct DMADOUBLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADOUBLE2_W<'a> {
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
#[doc = "Field `SINFORMAT2` reader - Enable signed format for DAC channel2"]
pub struct SINFORMAT2_R(crate::FieldReader<bool, bool>);
impl SINFORMAT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINFORMAT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINFORMAT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINFORMAT2` writer - Enable signed format for DAC channel2"]
pub struct SINFORMAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SINFORMAT2_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - DAC Channel1 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble1(&self) -> DMADOUBLE1_R {
        DMADOUBLE1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1"]
    #[inline(always)]
    pub fn sinformat1(&self) -> SINFORMAT1_R {
        SINFORMAT1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - High frequency interface mode selection"]
    #[inline(always)]
    pub fn hfsel(&self) -> HFSEL_R {
        HFSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 24 - DAC Channel2 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble2(&self) -> DMADOUBLE2_R {
        DMADOUBLE2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2"]
    #[inline(always)]
    pub fn sinformat2(&self) -> SINFORMAT2_R {
        SINFORMAT2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC Channel 1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 1 mode: DAC Channel 1 in normal Mode DAC Channel 1 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    #[doc = "Bit 8 - DAC Channel1 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble1(&mut self) -> DMADOUBLE1_W {
        DMADOUBLE1_W { w: self }
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1"]
    #[inline(always)]
    pub fn sinformat1(&mut self) -> SINFORMAT1_W {
        SINFORMAT1_W { w: self }
    }
    #[doc = "Bits 14:15 - High frequency interface mode selection"]
    #[inline(always)]
    pub fn hfsel(&mut self) -> HFSEL_W {
        HFSEL_W { w: self }
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC Channel 2 mode: DAC Channel 2 in normal Mode DAC Channel 2 in sample &amp; hold mode"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
    #[doc = "Bit 24 - DAC Channel2 DMA double data mode"]
    #[inline(always)]
    pub fn dmadouble2(&mut self) -> DMADOUBLE2_W {
        DMADOUBLE2_W { w: self }
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2"]
    #[inline(always)]
    pub fn sinformat2(&mut self) -> SINFORMAT2_W {
        SINFORMAT2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_mcr](index.html) module"]
pub struct DAC_MCR_SPEC;
impl crate::RegisterSpec for DAC_MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_mcr::R](R) reader structure"]
impl crate::Readable for DAC_MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_mcr::W](W) writer structure"]
impl crate::Writable for DAC_MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_MCR to value 0"]
impl crate::Resettable for DAC_MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
