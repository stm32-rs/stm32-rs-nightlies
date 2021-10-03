#[doc = "Register `MMC_RX_INTERRUPT_MASK` reader"]
pub struct R(crate::R<MMC_RX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_RX_INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_RX_INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_RX_INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_RX_INTERRUPT_MASK` writer"]
pub struct W(crate::W<MMC_RX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_RX_INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<MMC_RX_INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_RX_INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXCRCERPIM` reader - RXCRCERPIM"]
pub struct RXCRCERPIM_R(crate::FieldReader<bool, bool>);
impl RXCRCERPIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXCRCERPIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCRCERPIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCRCERPIM` writer - RXCRCERPIM"]
pub struct RXCRCERPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCRCERPIM_W<'a> {
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
#[doc = "Field `RXALGNERPIM` reader - RXALGNERPIM"]
pub struct RXALGNERPIM_R(crate::FieldReader<bool, bool>);
impl RXALGNERPIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXALGNERPIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXALGNERPIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXALGNERPIM` writer - RXALGNERPIM"]
pub struct RXALGNERPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXALGNERPIM_W<'a> {
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
#[doc = "Field `RXUCGPIM` reader - RXUCGPIM"]
pub struct RXUCGPIM_R(crate::FieldReader<bool, bool>);
impl RXUCGPIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUCGPIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUCGPIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUCGPIM` writer - RXUCGPIM"]
pub struct RXUCGPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUCGPIM_W<'a> {
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
#[doc = "Field `RXLPIUSCIM` reader - RXLPIUSCIM"]
pub struct RXLPIUSCIM_R(crate::FieldReader<bool, bool>);
impl RXLPIUSCIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXLPIUSCIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXLPIUSCIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLPIUSCIM` writer - RXLPIUSCIM"]
pub struct RXLPIUSCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLPIUSCIM_W<'a> {
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
#[doc = "Field `RXLPITRCIM` reader - RXLPITRCIM"]
pub struct RXLPITRCIM_R(crate::FieldReader<bool, bool>);
impl RXLPITRCIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXLPITRCIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXLPITRCIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 5 - RXCRCERPIM"]
    #[inline(always)]
    pub fn rxcrcerpim(&self) -> RXCRCERPIM_R {
        RXCRCERPIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RXALGNERPIM"]
    #[inline(always)]
    pub fn rxalgnerpim(&self) -> RXALGNERPIM_R {
        RXALGNERPIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RXUCGPIM"]
    #[inline(always)]
    pub fn rxucgpim(&self) -> RXUCGPIM_R {
        RXUCGPIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RXLPIUSCIM"]
    #[inline(always)]
    pub fn rxlpiuscim(&self) -> RXLPIUSCIM_R {
        RXLPIUSCIM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - RXLPITRCIM"]
    #[inline(always)]
    pub fn rxlpitrcim(&self) -> RXLPITRCIM_R {
        RXLPITRCIM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - RXCRCERPIM"]
    #[inline(always)]
    pub fn rxcrcerpim(&mut self) -> RXCRCERPIM_W {
        RXCRCERPIM_W { w: self }
    }
    #[doc = "Bit 6 - RXALGNERPIM"]
    #[inline(always)]
    pub fn rxalgnerpim(&mut self) -> RXALGNERPIM_W {
        RXALGNERPIM_W { w: self }
    }
    #[doc = "Bit 17 - RXUCGPIM"]
    #[inline(always)]
    pub fn rxucgpim(&mut self) -> RXUCGPIM_W {
        RXUCGPIM_W { w: self }
    }
    #[doc = "Bit 26 - RXLPIUSCIM"]
    #[inline(always)]
    pub fn rxlpiuscim(&mut self) -> RXLPIUSCIM_W {
        RXLPIUSCIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_rx_interrupt_mask](index.html) module"]
pub struct MMC_RX_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_RX_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_rx_interrupt_mask::R](R) reader structure"]
impl crate::Readable for MMC_RX_INTERRUPT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_rx_interrupt_mask::W](W) writer structure"]
impl crate::Writable for MMC_RX_INTERRUPT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_RX_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_RX_INTERRUPT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
