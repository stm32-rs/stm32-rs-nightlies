#[doc = "Register `ETH_DMAC0IER` reader"]
pub struct R(crate::R<ETH_DMAC0IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC0IER` writer"]
pub struct W(crate::W<ETH_DMAC0IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0IER_SPEC>;
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
impl From<crate::W<ETH_DMAC0IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub struct TIE_R(crate::FieldReader<bool, bool>);
impl TIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
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
#[doc = "Field `TXSE` reader - Transmit Stopped Enable"]
pub struct TXSE_R(crate::FieldReader<bool, bool>);
impl TXSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSE` writer - Transmit Stopped Enable"]
pub struct TXSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSE_W<'a> {
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
#[doc = "Field `TBUE` reader - Transmit Buffer Unavailable Enable"]
pub struct TBUE_R(crate::FieldReader<bool, bool>);
impl TBUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBUE` writer - Transmit Buffer Unavailable Enable"]
pub struct TBUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBUE_W<'a> {
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
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub struct RIE_R(crate::FieldReader<bool, bool>);
impl RIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub struct RIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIE_W<'a> {
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
#[doc = "Field `RBUE` reader - Receive Buffer Unavailable Enable"]
pub struct RBUE_R(crate::FieldReader<bool, bool>);
impl RBUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBUE` writer - Receive Buffer Unavailable Enable"]
pub struct RBUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBUE_W<'a> {
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
#[doc = "Field `RSE` reader - Receive Stopped Enable"]
pub struct RSE_R(crate::FieldReader<bool, bool>);
impl RSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSE` writer - Receive Stopped Enable"]
pub struct RSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSE_W<'a> {
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
#[doc = "Field `RWTE` reader - Receive Watchdog Timeout Enable"]
pub struct RWTE_R(crate::FieldReader<bool, bool>);
impl RWTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWTE` writer - Receive Watchdog Timeout Enable"]
pub struct RWTE_W<'a> {
    w: &'a mut W,
}
impl<'a> RWTE_W<'a> {
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
#[doc = "Field `ETIE` reader - Early Transmit Interrupt Enable"]
pub struct ETIE_R(crate::FieldReader<bool, bool>);
impl ETIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETIE` writer - Early Transmit Interrupt Enable"]
pub struct ETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETIE_W<'a> {
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
#[doc = "Field `ERIE` reader - Early Receive Interrupt Enable"]
pub struct ERIE_R(crate::FieldReader<bool, bool>);
impl ERIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERIE` writer - Early Receive Interrupt Enable"]
pub struct ERIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERIE_W<'a> {
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
#[doc = "Field `FBEE` reader - Fatal Bus Error Enable"]
pub struct FBEE_R(crate::FieldReader<bool, bool>);
impl FBEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FBEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBEE` writer - Fatal Bus Error Enable"]
pub struct FBEE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBEE_W<'a> {
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
#[doc = "Field `CDEE` reader - Context Descriptor Error Enable"]
pub struct CDEE_R(crate::FieldReader<bool, bool>);
impl CDEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDEE` writer - Context Descriptor Error Enable"]
pub struct CDEE_W<'a> {
    w: &'a mut W,
}
impl<'a> CDEE_W<'a> {
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
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable"]
pub struct AIE_R(crate::FieldReader<bool, bool>);
impl AIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable"]
pub struct AIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AIE_W<'a> {
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
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable"]
pub struct NIE_R(crate::FieldReader<bool, bool>);
impl NIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable"]
pub struct NIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn txse(&self) -> TXSE_R {
        TXSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn tbue(&self) -> TBUE_R {
        TBUE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rbue(&self) -> RBUE_R {
        RBUE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwte(&self) -> RWTE_R {
        RWTE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbee(&self) -> FBEE_R {
        FBEE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Context Descriptor Error Enable"]
    #[inline(always)]
    pub fn cdee(&self) -> CDEE_R {
        CDEE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn txse(&mut self) -> TXSE_W {
        TXSE_W { w: self }
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn tbue(&mut self) -> TBUE_W {
        TBUE_W { w: self }
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W {
        RIE_W { w: self }
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rbue(&mut self) -> RBUE_W {
        RBUE_W { w: self }
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&mut self) -> RSE_W {
        RSE_W { w: self }
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwte(&mut self) -> RWTE_W {
        RWTE_W { w: self }
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn etie(&mut self) -> ETIE_W {
        ETIE_W { w: self }
    }
    #[doc = "Bit 11 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W {
        ERIE_W { w: self }
    }
    #[doc = "Bit 12 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbee(&mut self) -> FBEE_W {
        FBEE_W { w: self }
    }
    #[doc = "Bit 13 - Context Descriptor Error Enable"]
    #[inline(always)]
    pub fn cdee(&mut self) -> CDEE_W {
        CDEE_W { w: self }
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&mut self) -> AIE_W {
        AIE_W { w: self }
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&mut self) -> NIE_W {
        NIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0ier](index.html) module"]
pub struct ETH_DMAC0IER_SPEC;
impl crate::RegisterSpec for ETH_DMAC0IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0ier::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac0ier::W](W) writer structure"]
impl crate::Writable for ETH_DMAC0IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMAC0IER to value 0x8000"]
impl crate::Resettable for ETH_DMAC0IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
