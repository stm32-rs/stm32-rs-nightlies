#[doc = "Register `MMC_TX_INTERRUPT_MASK` reader"]
pub struct R(crate::R<MMC_TX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_TX_INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_TX_INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_TX_INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_TX_INTERRUPT_MASK` writer"]
pub struct W(crate::W<MMC_TX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_TX_INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<MMC_TX_INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_TX_INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXSCOLGPIM` reader - MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
pub struct TXSCOLGPIM_R(crate::FieldReader<bool, bool>);
impl TXSCOLGPIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSCOLGPIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSCOLGPIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSCOLGPIM` writer - MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
pub struct TXSCOLGPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSCOLGPIM_W<'a> {
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
#[doc = "Field `TXMCOLGPIM` reader - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
pub struct TXMCOLGPIM_R(crate::FieldReader<bool, bool>);
impl TXMCOLGPIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMCOLGPIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMCOLGPIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMCOLGPIM` writer - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
pub struct TXMCOLGPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMCOLGPIM_W<'a> {
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
#[doc = "Field `TXGPKTIM` reader - MMC Transmit Good Packet Counter Interrupt Mask"]
pub struct TXGPKTIM_R(crate::FieldReader<bool, bool>);
impl TXGPKTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXGPKTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXGPKTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXGPKTIM` writer - MMC Transmit Good Packet Counter Interrupt Mask"]
pub struct TXGPKTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXGPKTIM_W<'a> {
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
#[doc = "Field `TXLPIUSCIM` reader - MMC Transmit LPI microsecond counter interrupt Mask"]
pub struct TXLPIUSCIM_R(crate::FieldReader<bool, bool>);
impl TXLPIUSCIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXLPIUSCIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLPIUSCIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLPIUSCIM` writer - MMC Transmit LPI microsecond counter interrupt Mask"]
pub struct TXLPIUSCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLPIUSCIM_W<'a> {
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
#[doc = "Field `TXLPITRCIM` reader - MMC Transmit LPI transition counter interrupt Mask"]
pub struct TXLPITRCIM_R(crate::FieldReader<bool, bool>);
impl TXLPITRCIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXLPITRCIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLPITRCIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgpim(&self) -> TXSCOLGPIM_R {
        TXSCOLGPIM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgpim(&self) -> TXMCOLGPIM_R {
        TXMCOLGPIM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgpktim(&self) -> TXGPKTIM_R {
        TXGPKTIM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 26 - MMC Transmit LPI microsecond counter interrupt Mask"]
    #[inline(always)]
    pub fn txlpiuscim(&self) -> TXLPIUSCIM_R {
        TXLPIUSCIM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - MMC Transmit LPI transition counter interrupt Mask"]
    #[inline(always)]
    pub fn txlpitrcim(&self) -> TXLPITRCIM_R {
        TXLPITRCIM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgpim(&mut self) -> TXSCOLGPIM_W {
        TXSCOLGPIM_W { w: self }
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgpim(&mut self) -> TXMCOLGPIM_W {
        TXMCOLGPIM_W { w: self }
    }
    #[doc = "Bit 21 - MMC Transmit Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgpktim(&mut self) -> TXGPKTIM_W {
        TXGPKTIM_W { w: self }
    }
    #[doc = "Bit 26 - MMC Transmit LPI microsecond counter interrupt Mask"]
    #[inline(always)]
    pub fn txlpiuscim(&mut self) -> TXLPIUSCIM_W {
        TXLPIUSCIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Tx interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_tx_interrupt_mask](index.html) module"]
pub struct MMC_TX_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_TX_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_tx_interrupt_mask::R](R) reader structure"]
impl crate::Readable for MMC_TX_INTERRUPT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_tx_interrupt_mask::W](W) writer structure"]
impl crate::Writable for MMC_TX_INTERRUPT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_TX_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_TX_INTERRUPT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
