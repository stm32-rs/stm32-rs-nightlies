#[doc = "Register `DDRPHYC_DTPR0` reader"]
pub struct R(crate::R<DDRPHYC_DTPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DTPR0` writer"]
pub struct W(crate::W<DDRPHYC_DTPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTPR0_SPEC>;
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
impl From<crate::W<DDRPHYC_DTPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMRD` reader - TMRD"]
pub struct TMRD_R(crate::FieldReader<u8, u8>);
impl TMRD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRD` writer - TMRD"]
pub struct TMRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TRTP` reader - TRTP"]
pub struct TRTP_R(crate::FieldReader<u8, u8>);
impl TRTP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRTP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRTP` writer - TRTP"]
pub struct TRTP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `TWTR` reader - TWTR"]
pub struct TWTR_R(crate::FieldReader<u8, u8>);
impl TWTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TWTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWTR` writer - TWTR"]
pub struct TWTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TWTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `TRP` reader - TRP"]
pub struct TRP_R(crate::FieldReader<u8, u8>);
impl TRP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRP` writer - TRP"]
pub struct TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TRCD` reader - TRCD"]
pub struct TRCD_R(crate::FieldReader<u8, u8>);
impl TRCD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRCD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRCD` writer - TRCD"]
pub struct TRCD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `TRAS` reader - TRAS"]
pub struct TRAS_R(crate::FieldReader<u8, u8>);
impl TRAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRAS` writer - TRAS"]
pub struct TRAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `TRRD` reader - TRRD"]
pub struct TRRD_R(crate::FieldReader<u8, u8>);
impl TRRD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRRD` writer - TRRD"]
pub struct TRRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 21)) | ((value as u32 & 0x0f) << 21);
        self.w
    }
}
#[doc = "Field `TRC` reader - TRC"]
pub struct TRC_R(crate::FieldReader<u8, u8>);
impl TRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRC` writer - TRC"]
pub struct TRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 25)) | ((value as u32 & 0x3f) << 25);
        self.w
    }
}
#[doc = "Field `TCCD` reader - TCCD"]
pub struct TCCD_R(crate::FieldReader<bool, bool>);
impl TCCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCCD` writer - TCCD"]
pub struct TCCD_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - TMRD"]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - TRTP"]
    #[inline(always)]
    pub fn trtp(&self) -> TRTP_R {
        TRTP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - TWTR"]
    #[inline(always)]
    pub fn twtr(&self) -> TWTR_R {
        TWTR_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - TRP"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TRCD"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - TRAS"]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:24 - TRRD"]
    #[inline(always)]
    pub fn trrd(&self) -> TRRD_R {
        TRRD_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 25:30 - TRC"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - TCCD"]
    #[inline(always)]
    pub fn tccd(&self) -> TCCD_R {
        TCCD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - TMRD"]
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W {
        TMRD_W { w: self }
    }
    #[doc = "Bits 2:4 - TRTP"]
    #[inline(always)]
    pub fn trtp(&mut self) -> TRTP_W {
        TRTP_W { w: self }
    }
    #[doc = "Bits 5:7 - TWTR"]
    #[inline(always)]
    pub fn twtr(&mut self) -> TWTR_W {
        TWTR_W { w: self }
    }
    #[doc = "Bits 8:11 - TRP"]
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W {
        TRP_W { w: self }
    }
    #[doc = "Bits 12:15 - TRCD"]
    #[inline(always)]
    pub fn trcd(&mut self) -> TRCD_W {
        TRCD_W { w: self }
    }
    #[doc = "Bits 16:20 - TRAS"]
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W {
        TRAS_W { w: self }
    }
    #[doc = "Bits 21:24 - TRRD"]
    #[inline(always)]
    pub fn trrd(&mut self) -> TRRD_W {
        TRRD_W { w: self }
    }
    #[doc = "Bits 25:30 - TRC"]
    #[inline(always)]
    pub fn trc(&mut self) -> TRC_W {
        TRC_W { w: self }
    }
    #[doc = "Bit 31 - TCCD"]
    #[inline(always)]
    pub fn tccd(&mut self) -> TCCD_W {
        TCCD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DTP register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtpr0](index.html) module"]
pub struct DDRPHYC_DTPR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dtpr0::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtpr0::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DTPR0 to value 0x3012_666e"]
impl crate::Resettable for DDRPHYC_DTPR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3012_666e
    }
}
