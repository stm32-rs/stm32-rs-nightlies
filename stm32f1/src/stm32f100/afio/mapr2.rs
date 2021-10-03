#[doc = "Register `MAPR2` reader"]
pub struct R(crate::R<MAPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPR2` writer"]
pub struct W(crate::W<MAPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPR2_SPEC>;
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
impl From<crate::W<MAPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM15_REMAP` reader - TIM15 remapping"]
pub struct TIM15_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM15_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15_REMAP` writer - TIM15 remapping"]
pub struct TIM15_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15_REMAP_W<'a> {
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
#[doc = "Field `TIM16_REMAP` reader - TIM16 remapping"]
pub struct TIM16_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM16_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16_REMAP` writer - TIM16 remapping"]
pub struct TIM16_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16_REMAP_W<'a> {
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
#[doc = "Field `TIM17_REMAP` reader - TIM17 remapping"]
pub struct TIM17_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM17_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17_REMAP` writer - TIM17 remapping"]
pub struct TIM17_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17_REMAP_W<'a> {
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
#[doc = "Field `TIM13_REMAP` reader - TIM13 remapping"]
pub struct TIM13_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM13_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM13_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM13_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM13_REMAP` writer - TIM13 remapping"]
pub struct TIM13_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13_REMAP_W<'a> {
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
#[doc = "Field `TIM14_REMAP` reader - TIM14 remapping"]
pub struct TIM14_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM14_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM14_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM14_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM14_REMAP` writer - TIM14 remapping"]
pub struct TIM14_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14_REMAP_W<'a> {
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
#[doc = "Field `FSMC_NADV` reader - NADV connect/disconnect"]
pub struct FSMC_NADV_R(crate::FieldReader<bool, bool>);
impl FSMC_NADV_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSMC_NADV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSMC_NADV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSMC_NADV` writer - NADV connect/disconnect"]
pub struct FSMC_NADV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMC_NADV_W<'a> {
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
#[doc = "Field `CEC_REMAP` reader - CEC remapping"]
pub struct CEC_REMAP_R(crate::FieldReader<bool, bool>);
impl CEC_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEC_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEC_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEC_REMAP` writer - CEC remapping"]
pub struct CEC_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CEC_REMAP_W<'a> {
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
#[doc = "Field `TIM1_DMA_REMAP` reader - TIM1 DMA remapping"]
pub struct TIM1_DMA_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM1_DMA_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1_DMA_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1_DMA_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1_DMA_REMAP` writer - TIM1 DMA remapping"]
pub struct TIM1_DMA_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_DMA_REMAP_W<'a> {
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
#[doc = "Field `TIM67_DAC_DMA_REMAP` reader - TIM67_DAC DMA remapping"]
pub struct TIM67_DAC_DMA_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM67_DAC_DMA_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM67_DAC_DMA_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM67_DAC_DMA_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM67_DAC_DMA_REMAP` writer - TIM67_DAC DMA remapping"]
pub struct TIM67_DAC_DMA_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM67_DAC_DMA_REMAP_W<'a> {
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
#[doc = "Field `TIM12_REMAP` reader - TIM12 remapping"]
pub struct TIM12_REMAP_R(crate::FieldReader<bool, bool>);
impl TIM12_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM12_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM12_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM12_REMAP` writer - TIM12 remapping"]
pub struct TIM12_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM12_REMAP_W<'a> {
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
#[doc = "Field `MISC_REMAP` reader - Miscellaneous features remapping"]
pub struct MISC_REMAP_R(crate::FieldReader<bool, bool>);
impl MISC_REMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MISC_REMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISC_REMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISC_REMAP` writer - Miscellaneous features remapping"]
pub struct MISC_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MISC_REMAP_W<'a> {
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
    #[doc = "Bit 0 - TIM15 remapping"]
    #[inline(always)]
    pub fn tim15_remap(&self) -> TIM15_REMAP_R {
        TIM15_REMAP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM16 remapping"]
    #[inline(always)]
    pub fn tim16_remap(&self) -> TIM16_REMAP_R {
        TIM16_REMAP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM17 remapping"]
    #[inline(always)]
    pub fn tim17_remap(&self) -> TIM17_REMAP_R {
        TIM17_REMAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    pub fn tim13_remap(&self) -> TIM13_REMAP_R {
        TIM13_REMAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    pub fn tim14_remap(&self) -> TIM14_REMAP_R {
        TIM14_REMAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FSMC_NADV_R {
        FSMC_NADV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CEC remapping"]
    #[inline(always)]
    pub fn cec_remap(&self) -> CEC_REMAP_R {
        CEC_REMAP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM1 DMA remapping"]
    #[inline(always)]
    pub fn tim1_dma_remap(&self) -> TIM1_DMA_REMAP_R {
        TIM1_DMA_REMAP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIM67_DAC DMA remapping"]
    #[inline(always)]
    pub fn tim67_dac_dma_remap(&self) -> TIM67_DAC_DMA_REMAP_R {
        TIM67_DAC_DMA_REMAP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TIM12 remapping"]
    #[inline(always)]
    pub fn tim12_remap(&self) -> TIM12_REMAP_R {
        TIM12_REMAP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Miscellaneous features remapping"]
    #[inline(always)]
    pub fn misc_remap(&self) -> MISC_REMAP_R {
        MISC_REMAP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM15 remapping"]
    #[inline(always)]
    pub fn tim15_remap(&mut self) -> TIM15_REMAP_W {
        TIM15_REMAP_W { w: self }
    }
    #[doc = "Bit 1 - TIM16 remapping"]
    #[inline(always)]
    pub fn tim16_remap(&mut self) -> TIM16_REMAP_W {
        TIM16_REMAP_W { w: self }
    }
    #[doc = "Bit 2 - TIM17 remapping"]
    #[inline(always)]
    pub fn tim17_remap(&mut self) -> TIM17_REMAP_W {
        TIM17_REMAP_W { w: self }
    }
    #[doc = "Bit 8 - TIM13 remapping"]
    #[inline(always)]
    pub fn tim13_remap(&mut self) -> TIM13_REMAP_W {
        TIM13_REMAP_W { w: self }
    }
    #[doc = "Bit 9 - TIM14 remapping"]
    #[inline(always)]
    pub fn tim14_remap(&mut self) -> TIM14_REMAP_W {
        TIM14_REMAP_W { w: self }
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn fsmc_nadv(&mut self) -> FSMC_NADV_W {
        FSMC_NADV_W { w: self }
    }
    #[doc = "Bit 3 - CEC remapping"]
    #[inline(always)]
    pub fn cec_remap(&mut self) -> CEC_REMAP_W {
        CEC_REMAP_W { w: self }
    }
    #[doc = "Bit 4 - TIM1 DMA remapping"]
    #[inline(always)]
    pub fn tim1_dma_remap(&mut self) -> TIM1_DMA_REMAP_W {
        TIM1_DMA_REMAP_W { w: self }
    }
    #[doc = "Bit 11 - TIM67_DAC DMA remapping"]
    #[inline(always)]
    pub fn tim67_dac_dma_remap(&mut self) -> TIM67_DAC_DMA_REMAP_W {
        TIM67_DAC_DMA_REMAP_W { w: self }
    }
    #[doc = "Bit 12 - TIM12 remapping"]
    #[inline(always)]
    pub fn tim12_remap(&mut self) -> TIM12_REMAP_W {
        TIM12_REMAP_W { w: self }
    }
    #[doc = "Bit 13 - Miscellaneous features remapping"]
    #[inline(always)]
    pub fn misc_remap(&mut self) -> MISC_REMAP_W {
        MISC_REMAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AF remap and debug I/O configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapr2](index.html) module"]
pub struct MAPR2_SPEC;
impl crate::RegisterSpec for MAPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapr2::R](R) reader structure"]
impl crate::Readable for MAPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapr2::W](W) writer structure"]
impl crate::Writable for MAPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAPR2 to value 0"]
impl crate::Resettable for MAPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
