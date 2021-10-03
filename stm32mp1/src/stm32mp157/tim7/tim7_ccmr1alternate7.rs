#[doc = "Register `TIM7_CCMR1ALTERNATE7` reader"]
pub struct R(crate::R<TIM7_CCMR1ALTERNATE7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM7_CCMR1ALTERNATE7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM7_CCMR1ALTERNATE7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM7_CCMR1ALTERNATE7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM7_CCMR1ALTERNATE7` writer"]
pub struct W(crate::W<TIM7_CCMR1ALTERNATE7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM7_CCMR1ALTERNATE7_SPEC>;
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
impl From<crate::W<TIM7_CCMR1ALTERNATE7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM7_CCMR1ALTERNATE7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1S` reader - CC1S"]
pub struct CC1S_R(crate::FieldReader<u8, u8>);
impl CC1S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC1S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1S` writer - CC1S"]
pub struct CC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `IC1PSC` reader - IC1PSC"]
pub struct IC1PSC_R(crate::FieldReader<u8, u8>);
impl IC1PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC1PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1PSC` writer - IC1PSC"]
pub struct IC1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `IC1F` reader - IC1F"]
pub struct IC1F_R(crate::FieldReader<u8, u8>);
impl IC1F_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1F` writer - IC1F"]
pub struct IC1F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CC2S` reader - CC2S"]
pub struct CC2S_R(crate::FieldReader<u8, u8>);
impl CC2S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2S` writer - CC2S"]
pub struct CC2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `IC2PSC` reader - IC2PSC"]
pub struct IC2PSC_R(crate::FieldReader<u8, u8>);
impl IC2PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC2PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2PSC` writer - IC2PSC"]
pub struct IC2PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `IC2F` reader - IC2F"]
pub struct IC2F_R(crate::FieldReader<u8, u8>);
impl IC2F_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2F` writer - IC2F"]
pub struct IC2F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - IC1PSC"]
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - IC1F"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - CC2S"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - IC2PSC"]
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - IC2F"]
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
    #[doc = "Bits 2:3 - IC1PSC"]
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W {
        IC1PSC_W { w: self }
    }
    #[doc = "Bits 4:7 - IC1F"]
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W {
        IC1F_W { w: self }
    }
    #[doc = "Bits 8:9 - CC2S"]
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W {
        CC2S_W { w: self }
    }
    #[doc = "Bits 10:11 - IC2PSC"]
    #[inline(always)]
    pub fn ic2psc(&mut self) -> IC2PSC_W {
        IC2PSC_W { w: self }
    }
    #[doc = "Bits 12:15 - IC2F"]
    #[inline(always)]
    pub fn ic2f(&mut self) -> IC2F_W {
        IC2F_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_ccmr1alternate7](index.html) module"]
pub struct TIM7_CCMR1ALTERNATE7_SPEC;
impl crate::RegisterSpec for TIM7_CCMR1ALTERNATE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim7_ccmr1alternate7::R](R) reader structure"]
impl crate::Readable for TIM7_CCMR1ALTERNATE7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim7_ccmr1alternate7::W](W) writer structure"]
impl crate::Writable for TIM7_CCMR1ALTERNATE7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM7_CCMR1ALTERNATE7 to value 0"]
impl crate::Resettable for TIM7_CCMR1ALTERNATE7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
