#[doc = "Register `EP7R` reader"]
pub struct R(crate::R<EP7R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP7R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP7R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP7R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP7R` writer"]
pub struct W(crate::W<EP7R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP7R_SPEC>;
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
impl From<crate::W<EP7R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP7R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EA` reader - EA"]
pub struct EA_R(crate::FieldReader<u8, u8>);
impl EA_R {
    pub(crate) fn new(bits: u8) -> Self {
        EA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EA` writer - EA"]
pub struct EA_W<'a> {
    w: &'a mut W,
}
impl<'a> EA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `STAT_TX` reader - STAT_TX"]
pub struct STAT_TX_R(crate::FieldReader<u8, u8>);
impl STAT_TX_R {
    pub(crate) fn new(bits: u8) -> Self {
        STAT_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAT_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAT_TX` writer - STAT_TX"]
pub struct STAT_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `DTOG_TX` reader - DTOG_TX"]
pub struct DTOG_TX_R(crate::FieldReader<bool, bool>);
impl DTOG_TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOG_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOG_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOG_TX` writer - DTOG_TX"]
pub struct DTOG_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOG_TX_W<'a> {
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
#[doc = "Field `CTR_TX` reader - CTR_TX"]
pub struct CTR_TX_R(crate::FieldReader<bool, bool>);
impl CTR_TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR_TX` writer - CTR_TX"]
pub struct CTR_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_TX_W<'a> {
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
#[doc = "Field `EP_KIND` reader - EP_KIND"]
pub struct EP_KIND_R(crate::FieldReader<bool, bool>);
impl EP_KIND_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_KIND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_KIND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_KIND` writer - EP_KIND"]
pub struct EP_KIND_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_KIND_W<'a> {
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
#[doc = "Field `EP_TYPE` reader - EP_TYPE"]
pub struct EP_TYPE_R(crate::FieldReader<u8, u8>);
impl EP_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        EP_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_TYPE` writer - EP_TYPE"]
pub struct EP_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `SETUP` reader - SETUP"]
pub struct SETUP_R(crate::FieldReader<bool, bool>);
impl SETUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUP` writer - SETUP"]
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
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
#[doc = "Field `STAT_RX` reader - STAT_RX"]
pub struct STAT_RX_R(crate::FieldReader<u8, u8>);
impl STAT_RX_R {
    pub(crate) fn new(bits: u8) -> Self {
        STAT_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAT_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAT_RX` writer - STAT_RX"]
pub struct STAT_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `DTOG_RX` reader - DTOG_RX"]
pub struct DTOG_RX_R(crate::FieldReader<bool, bool>);
impl DTOG_RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOG_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOG_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOG_RX` writer - DTOG_RX"]
pub struct DTOG_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOG_RX_W<'a> {
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
#[doc = "Field `CTR_RX` reader - CTR_RX"]
pub struct CTR_RX_R(crate::FieldReader<bool, bool>);
impl CTR_RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTR_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR_RX` writer - CTR_RX"]
pub struct CTR_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_RX_W<'a> {
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
    #[doc = "Bits 0:3 - EA"]
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - STAT_TX"]
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - DTOG_TX"]
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CTR_TX"]
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EP_KIND"]
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - EP_TYPE"]
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - SETUP"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - STAT_RX"]
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - DTOG_RX"]
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CTR_RX"]
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - EA"]
    #[inline(always)]
    pub fn ea(&mut self) -> EA_W {
        EA_W { w: self }
    }
    #[doc = "Bits 4:5 - STAT_TX"]
    #[inline(always)]
    pub fn stat_tx(&mut self) -> STAT_TX_W {
        STAT_TX_W { w: self }
    }
    #[doc = "Bit 6 - DTOG_TX"]
    #[inline(always)]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W {
        DTOG_TX_W { w: self }
    }
    #[doc = "Bit 7 - CTR_TX"]
    #[inline(always)]
    pub fn ctr_tx(&mut self) -> CTR_TX_W {
        CTR_TX_W { w: self }
    }
    #[doc = "Bit 8 - EP_KIND"]
    #[inline(always)]
    pub fn ep_kind(&mut self) -> EP_KIND_W {
        EP_KIND_W { w: self }
    }
    #[doc = "Bits 9:10 - EP_TYPE"]
    #[inline(always)]
    pub fn ep_type(&mut self) -> EP_TYPE_W {
        EP_TYPE_W { w: self }
    }
    #[doc = "Bit 11 - SETUP"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
    #[doc = "Bits 12:13 - STAT_RX"]
    #[inline(always)]
    pub fn stat_rx(&mut self) -> STAT_RX_W {
        STAT_RX_W { w: self }
    }
    #[doc = "Bit 14 - DTOG_RX"]
    #[inline(always)]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W {
        DTOG_RX_W { w: self }
    }
    #[doc = "Bit 15 - CTR_RX"]
    #[inline(always)]
    pub fn ctr_rx(&mut self) -> CTR_RX_W {
        CTR_RX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB endpoint n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7r](index.html) module"]
pub struct EP7R_SPEC;
impl crate::RegisterSpec for EP7R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep7r::R](R) reader structure"]
impl crate::Readable for EP7R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep7r::W](W) writer structure"]
impl crate::Writable for EP7R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP7R to value 0"]
impl crate::Resettable for EP7R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
