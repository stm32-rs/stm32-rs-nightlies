#[doc = "Register `ETH_MACQ0TxFCR` reader"]
pub struct R(crate::R<ETH_MACQ0TXFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACQ0TXFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACQ0TXFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACQ0TXFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACQ0TxFCR` writer"]
pub struct W(crate::W<ETH_MACQ0TXFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACQ0TXFCR_SPEC>;
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
impl From<crate::W<ETH_MACQ0TXFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACQ0TXFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCB_BPA` reader - FCB_BPA"]
pub struct FCB_BPA_R(crate::FieldReader<bool, bool>);
impl FCB_BPA_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCB_BPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCB_BPA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCB_BPA` writer - FCB_BPA"]
pub struct FCB_BPA_W<'a> {
    w: &'a mut W,
}
impl<'a> FCB_BPA_W<'a> {
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
#[doc = "Field `TFE` reader - TFE"]
pub struct TFE_R(crate::FieldReader<bool, bool>);
impl TFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFE` writer - TFE"]
pub struct TFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFE_W<'a> {
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
#[doc = "Field `PLT` reader - PLT"]
pub struct PLT_R(crate::FieldReader<u8, u8>);
impl PLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLT` writer - PLT"]
pub struct PLT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `DZPQ` reader - DZPQ"]
pub struct DZPQ_R(crate::FieldReader<bool, bool>);
impl DZPQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        DZPQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DZPQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DZPQ` writer - DZPQ"]
pub struct DZPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DZPQ_W<'a> {
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
#[doc = "Field `PT` reader - PT"]
pub struct PT_R(crate::FieldReader<u16, u16>);
impl PT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PT` writer - PT"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FCB_BPA"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TFE"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - PLT"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - DZPQ"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FCB_BPA"]
    #[inline(always)]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W {
        FCB_BPA_W { w: self }
    }
    #[doc = "Bit 1 - TFE"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W {
        TFE_W { w: self }
    }
    #[doc = "Bits 4:6 - PLT"]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W {
        PLT_W { w: self }
    }
    #[doc = "Bit 7 - DZPQ"]
    #[inline(always)]
    pub fn dzpq(&mut self) -> DZPQ_W {
        DZPQ_W { w: self }
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Flow Control register controls the generation and reception of the Control (Pause Command) packets by the Flow control module of the MAC. A Write to a register with the Busy bit set to 1 triggers the Flow Control block to generate a Pause packet. The fields of the control packet are selected as specified in the 802.3x specification, and the Pause Time value from this register is used in the Pause Time field of the control packet. The Busy bit remains set until the control packet is transferred onto the cable. The application must make sure that the Busy bit is cleared before writing to the register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macq0tx_fcr](index.html) module"]
pub struct ETH_MACQ0TXFCR_SPEC;
impl crate::RegisterSpec for ETH_MACQ0TXFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macq0tx_fcr::R](R) reader structure"]
impl crate::Readable for ETH_MACQ0TXFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macq0tx_fcr::W](W) writer structure"]
impl crate::Writable for ETH_MACQ0TXFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACQ0TxFCR to value 0"]
impl crate::Resettable for ETH_MACQ0TXFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
