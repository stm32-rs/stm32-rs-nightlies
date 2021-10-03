#[doc = "Register `COUNT2_TX` reader"]
pub struct R(crate::R<COUNT2_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT2_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT2_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT2_TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT2_TX` writer"]
pub struct W(crate::W<COUNT2_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT2_TX_SPEC>;
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
impl From<crate::W<COUNT2_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT2_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT2_TX` reader - Transmission byte count"]
pub struct COUNT2_TX_R(crate::FieldReader<u16, u16>);
impl COUNT2_TX_R {
    pub(crate) fn new(bits: u16) -> Self {
        COUNT2_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT2_TX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT2_TX` writer - Transmission byte count"]
pub struct COUNT2_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT2_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Transmission byte count"]
    #[inline(always)]
    pub fn count2_tx(&self) -> COUNT2_TX_R {
        COUNT2_TX_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmission byte count"]
    #[inline(always)]
    pub fn count2_tx(&mut self) -> COUNT2_TX_W {
        COUNT2_TX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmission byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count2_tx](index.html) module"]
pub struct COUNT2_TX_SPEC;
impl crate::RegisterSpec for COUNT2_TX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [count2_tx::R](R) reader structure"]
impl crate::Readable for COUNT2_TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count2_tx::W](W) writer structure"]
impl crate::Writable for COUNT2_TX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNT2_TX to value 0"]
impl crate::Resettable for COUNT2_TX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
