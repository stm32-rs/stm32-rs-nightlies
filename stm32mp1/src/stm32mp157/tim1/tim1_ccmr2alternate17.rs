#[doc = "Register `TIM1_CCMR2ALTERNATE17` reader"]
pub struct R(crate::R<TIM1_CCMR2ALTERNATE17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_CCMR2ALTERNATE17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_CCMR2ALTERNATE17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_CCMR2ALTERNATE17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM1_CCMR2ALTERNATE17` writer"]
pub struct W(crate::W<TIM1_CCMR2ALTERNATE17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_CCMR2ALTERNATE17_SPEC>;
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
impl From<crate::W<TIM1_CCMR2ALTERNATE17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_CCMR2ALTERNATE17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC3S` reader - CC3S"]
pub struct CC3S_R(crate::FieldReader<u8, u8>);
impl CC3S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC3S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC3S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC3S` writer - CC3S"]
pub struct CC3S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `IC3PSC` reader - IC3PSC"]
pub struct IC3PSC_R(crate::FieldReader<u8, u8>);
impl IC3PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC3PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3PSC` writer - IC3PSC"]
pub struct IC3PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `IC3F` reader - IC3F"]
pub struct IC3F_R(crate::FieldReader<u8, u8>);
impl IC3F_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3F` writer - IC3F"]
pub struct IC3F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CC4S` reader - CC4S"]
pub struct CC4S_R(crate::FieldReader<u8, u8>);
impl CC4S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC4S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC4S` writer - CC4S"]
pub struct CC4S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `IC4PSC` reader - IC4PSC"]
pub struct IC4PSC_R(crate::FieldReader<u8, u8>);
impl IC4PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC4PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4PSC` writer - IC4PSC"]
pub struct IC4PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `IC4F` reader - IC4F"]
pub struct IC4F_R(crate::FieldReader<u8, u8>);
impl IC4F_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC4F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4F` writer - IC4F"]
pub struct IC4F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - IC3PSC"]
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - IC3F"]
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - IC4PSC"]
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - IC4F"]
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W {
        CC3S_W { w: self }
    }
    #[doc = "Bits 2:3 - IC3PSC"]
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W {
        IC3PSC_W { w: self }
    }
    #[doc = "Bits 4:7 - IC3F"]
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W {
        IC3F_W { w: self }
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W {
        CC4S_W { w: self }
    }
    #[doc = "Bits 10:11 - IC4PSC"]
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W {
        IC4PSC_W { w: self }
    }
    #[doc = "Bits 12:15 - IC4F"]
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W {
        IC4F_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccmr2alternate17](index.html) module"]
pub struct TIM1_CCMR2ALTERNATE17_SPEC;
impl crate::RegisterSpec for TIM1_CCMR2ALTERNATE17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim1_ccmr2alternate17::R](R) reader structure"]
impl crate::Readable for TIM1_CCMR2ALTERNATE17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim1_ccmr2alternate17::W](W) writer structure"]
impl crate::Writable for TIM1_CCMR2ALTERNATE17_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM1_CCMR2ALTERNATE17 to value 0"]
impl crate::Resettable for TIM1_CCMR2ALTERNATE17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
