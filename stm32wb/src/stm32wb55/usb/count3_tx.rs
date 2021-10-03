#[doc = "Register `COUNT3_TX` reader"]
pub struct R(crate::R<COUNT3_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT3_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT3_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT3_TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT3_TX` writer"]
pub struct W(crate::W<COUNT3_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT3_TX_SPEC>;
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
impl From<crate::W<COUNT3_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT3_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT3_TX` reader - Transmission byte count"]
pub struct COUNT3_TX_R(crate::FieldReader<u16, u16>);
impl COUNT3_TX_R {
    pub(crate) fn new(bits: u16) -> Self {
        COUNT3_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT3_TX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT3_TX` writer - Transmission byte count"]
pub struct COUNT3_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT3_TX_W<'a> {
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
    pub fn count3_tx(&self) -> COUNT3_TX_R {
        COUNT3_TX_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmission byte count"]
    #[inline(always)]
    pub fn count3_tx(&mut self) -> COUNT3_TX_W {
        COUNT3_TX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmission byte count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count3_tx](index.html) module"]
pub struct COUNT3_TX_SPEC;
impl crate::RegisterSpec for COUNT3_TX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [count3_tx::R](R) reader structure"]
impl crate::Readable for COUNT3_TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count3_tx::W](W) writer structure"]
impl crate::Writable for COUNT3_TX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNT3_TX to value 0"]
impl crate::Resettable for COUNT3_TX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
