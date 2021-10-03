#[doc = "Register `ETH_MACPOCR` reader"]
pub struct R(crate::R<ETH_MACPOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPOCR` writer"]
pub struct W(crate::W<ETH_MACPOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPOCR_SPEC>;
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
impl From<crate::W<ETH_MACPOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTOEN` reader - PTOEN"]
pub struct PTOEN_R(crate::FieldReader<bool, bool>);
impl PTOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTOEN` writer - PTOEN"]
pub struct PTOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTOEN_W<'a> {
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
#[doc = "Field `ASYNCEN` reader - ASYNCEN"]
pub struct ASYNCEN_R(crate::FieldReader<bool, bool>);
impl ASYNCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASYNCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASYNCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASYNCEN` writer - ASYNCEN"]
pub struct ASYNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCEN_W<'a> {
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
#[doc = "Field `APDREQEN` reader - APDREQEN"]
pub struct APDREQEN_R(crate::FieldReader<bool, bool>);
impl APDREQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        APDREQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APDREQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APDREQEN` writer - APDREQEN"]
pub struct APDREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APDREQEN_W<'a> {
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
#[doc = "Field `ASYNCTRIG` reader - ASYNCTRIG"]
pub struct ASYNCTRIG_R(crate::FieldReader<bool, bool>);
impl ASYNCTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASYNCTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASYNCTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASYNCTRIG` writer - ASYNCTRIG"]
pub struct ASYNCTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCTRIG_W<'a> {
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
#[doc = "Field `APDREQTRIG` reader - APDREQTRIG"]
pub struct APDREQTRIG_R(crate::FieldReader<bool, bool>);
impl APDREQTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        APDREQTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APDREQTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APDREQTRIG` writer - APDREQTRIG"]
pub struct APDREQTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> APDREQTRIG_W<'a> {
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
#[doc = "Field `DRRDIS` reader - DRRDIS"]
pub struct DRRDIS_R(crate::FieldReader<bool, bool>);
impl DRRDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRRDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRRDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRRDIS` writer - DRRDIS"]
pub struct DRRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRRDIS_W<'a> {
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
#[doc = "Field `DN` reader - DN"]
pub struct DN_R(crate::FieldReader<u8, u8>);
impl DN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DN` writer - DN"]
pub struct DN_W<'a> {
    w: &'a mut W,
}
impl<'a> DN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PTOEN"]
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ASYNCEN"]
    #[inline(always)]
    pub fn asyncen(&self) -> ASYNCEN_R {
        ASYNCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - APDREQEN"]
    #[inline(always)]
    pub fn apdreqen(&self) -> APDREQEN_R {
        APDREQEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ASYNCTRIG"]
    #[inline(always)]
    pub fn asynctrig(&self) -> ASYNCTRIG_R {
        ASYNCTRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - APDREQTRIG"]
    #[inline(always)]
    pub fn apdreqtrig(&self) -> APDREQTRIG_R {
        APDREQTRIG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DRRDIS"]
    #[inline(always)]
    pub fn drrdis(&self) -> DRRDIS_R {
        DRRDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - DN"]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PTOEN"]
    #[inline(always)]
    pub fn ptoen(&mut self) -> PTOEN_W {
        PTOEN_W { w: self }
    }
    #[doc = "Bit 1 - ASYNCEN"]
    #[inline(always)]
    pub fn asyncen(&mut self) -> ASYNCEN_W {
        ASYNCEN_W { w: self }
    }
    #[doc = "Bit 2 - APDREQEN"]
    #[inline(always)]
    pub fn apdreqen(&mut self) -> APDREQEN_W {
        APDREQEN_W { w: self }
    }
    #[doc = "Bit 4 - ASYNCTRIG"]
    #[inline(always)]
    pub fn asynctrig(&mut self) -> ASYNCTRIG_W {
        ASYNCTRIG_W { w: self }
    }
    #[doc = "Bit 5 - APDREQTRIG"]
    #[inline(always)]
    pub fn apdreqtrig(&mut self) -> APDREQTRIG_W {
        APDREQTRIG_W { w: self }
    }
    #[doc = "Bit 6 - DRRDIS"]
    #[inline(always)]
    pub fn drrdis(&mut self) -> DRRDIS_W {
        DRRDIS_W { w: self }
    }
    #[doc = "Bits 8:15 - DN"]
    #[inline(always)]
    pub fn dn(&mut self) -> DN_W {
        DN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the PTP Offload Engine operation. This register is available only when the Enable PTP Timestamp Offload feature is selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macpocr](index.html) module"]
pub struct ETH_MACPOCR_SPEC;
impl crate::RegisterSpec for ETH_MACPOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macpocr::R](R) reader structure"]
impl crate::Readable for ETH_MACPOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macpocr::W](W) writer structure"]
impl crate::Writable for ETH_MACPOCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACPOCR to value 0"]
impl crate::Resettable for ETH_MACPOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
