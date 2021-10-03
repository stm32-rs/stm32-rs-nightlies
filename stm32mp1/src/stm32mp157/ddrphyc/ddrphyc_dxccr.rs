#[doc = "Register `DDRPHYC_DXCCR` reader"]
pub struct R(crate::R<DDRPHYC_DXCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DXCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DXCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DXCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DXCCR` writer"]
pub struct W(crate::W<DDRPHYC_DXCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DXCCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DXCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DXCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DXODT` reader - DXODT"]
pub struct DXODT_R(crate::FieldReader<bool, bool>);
impl DXODT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DXODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DXODT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DXODT` writer - DXODT"]
pub struct DXODT_W<'a> {
    w: &'a mut W,
}
impl<'a> DXODT_W<'a> {
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
#[doc = "Field `DXIOM` reader - DXIOM"]
pub struct DXIOM_R(crate::FieldReader<bool, bool>);
impl DXIOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DXIOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DXIOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DXIOM` writer - DXIOM"]
pub struct DXIOM_W<'a> {
    w: &'a mut W,
}
impl<'a> DXIOM_W<'a> {
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
#[doc = "Field `DXPDD` reader - DXPDD"]
pub struct DXPDD_R(crate::FieldReader<bool, bool>);
impl DXPDD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DXPDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DXPDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DXPDD` writer - DXPDD"]
pub struct DXPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> DXPDD_W<'a> {
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
#[doc = "Field `DXPDR` reader - DXPDR"]
pub struct DXPDR_R(crate::FieldReader<bool, bool>);
impl DXPDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DXPDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DXPDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DXPDR` writer - DXPDR"]
pub struct DXPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DXPDR_W<'a> {
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
#[doc = "Field `DQSRES` reader - DQSRES"]
pub struct DQSRES_R(crate::FieldReader<u8, u8>);
impl DQSRES_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQSRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSRES` writer - DQSRES"]
pub struct DQSRES_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `DQSNRES` reader - DQSNRES"]
pub struct DQSNRES_R(crate::FieldReader<u8, u8>);
impl DQSNRES_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQSNRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSNRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSNRES` writer - DQSNRES"]
pub struct DQSNRES_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSNRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `DQSNRST` reader - DQSNRST"]
pub struct DQSNRST_R(crate::FieldReader<bool, bool>);
impl DQSNRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQSNRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSNRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSNRST` writer - DQSNRST"]
pub struct DQSNRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSNRST_W<'a> {
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
#[doc = "Field `RVSEL` reader - RVSEL"]
pub struct RVSEL_R(crate::FieldReader<bool, bool>);
impl RVSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RVSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RVSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RVSEL` writer - RVSEL"]
pub struct RVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RVSEL_W<'a> {
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
#[doc = "Field `AWDT` reader - AWDT"]
pub struct AWDT_R(crate::FieldReader<bool, bool>);
impl AWDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWDT` writer - AWDT"]
pub struct AWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DXODT"]
    #[inline(always)]
    pub fn dxodt(&self) -> DXODT_R {
        DXODT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DXIOM"]
    #[inline(always)]
    pub fn dxiom(&self) -> DXIOM_R {
        DXIOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DXPDD"]
    #[inline(always)]
    pub fn dxpdd(&self) -> DXPDD_R {
        DXPDD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DXPDR"]
    #[inline(always)]
    pub fn dxpdr(&self) -> DXPDR_R {
        DXPDR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - DQSRES"]
    #[inline(always)]
    pub fn dqsres(&self) -> DQSRES_R {
        DQSRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DQSNRES"]
    #[inline(always)]
    pub fn dqsnres(&self) -> DQSNRES_R {
        DQSNRES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - DQSNRST"]
    #[inline(always)]
    pub fn dqsnrst(&self) -> DQSNRST_R {
        DQSNRST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RVSEL"]
    #[inline(always)]
    pub fn rvsel(&self) -> RVSEL_R {
        RVSEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AWDT"]
    #[inline(always)]
    pub fn awdt(&self) -> AWDT_R {
        AWDT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DXODT"]
    #[inline(always)]
    pub fn dxodt(&mut self) -> DXODT_W {
        DXODT_W { w: self }
    }
    #[doc = "Bit 1 - DXIOM"]
    #[inline(always)]
    pub fn dxiom(&mut self) -> DXIOM_W {
        DXIOM_W { w: self }
    }
    #[doc = "Bit 2 - DXPDD"]
    #[inline(always)]
    pub fn dxpdd(&mut self) -> DXPDD_W {
        DXPDD_W { w: self }
    }
    #[doc = "Bit 3 - DXPDR"]
    #[inline(always)]
    pub fn dxpdr(&mut self) -> DXPDR_W {
        DXPDR_W { w: self }
    }
    #[doc = "Bits 4:7 - DQSRES"]
    #[inline(always)]
    pub fn dqsres(&mut self) -> DQSRES_W {
        DQSRES_W { w: self }
    }
    #[doc = "Bits 8:11 - DQSNRES"]
    #[inline(always)]
    pub fn dqsnres(&mut self) -> DQSNRES_W {
        DQSNRES_W { w: self }
    }
    #[doc = "Bit 14 - DQSNRST"]
    #[inline(always)]
    pub fn dqsnrst(&mut self) -> DQSNRST_W {
        DQSNRST_W { w: self }
    }
    #[doc = "Bit 15 - RVSEL"]
    #[inline(always)]
    pub fn rvsel(&mut self) -> RVSEL_W {
        RVSEL_W { w: self }
    }
    #[doc = "Bit 16 - AWDT"]
    #[inline(always)]
    pub fn awdt(&mut self) -> AWDT_W {
        AWDT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DXCC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dxccr](index.html) module"]
pub struct DDRPHYC_DXCCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DXCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dxccr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DXCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dxccr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DXCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DXCCR to value 0x0800"]
impl crate::Resettable for DDRPHYC_DXCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
