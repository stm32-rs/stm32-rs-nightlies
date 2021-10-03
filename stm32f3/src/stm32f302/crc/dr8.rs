#[doc = "Register `DR8` reader"]
pub struct R(crate::R<DR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR8` writer"]
pub struct W(crate::W<DR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR8_SPEC>;
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
impl From<crate::W<DR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DR8` reader - Data register bits"]
pub struct DR8_R(crate::FieldReader<u8, u8>);
impl DR8_R {
    pub(crate) fn new(bits: u8) -> Self {
        DR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR8` writer - Data register bits"]
pub struct DR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DR8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data register bits"]
    #[inline(always)]
    pub fn dr8(&self) -> DR8_R {
        DR8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data register bits"]
    #[inline(always)]
    pub fn dr8(&mut self) -> DR8_W {
        DR8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register - byte sized\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr8](index.html) module"]
pub struct DR8_SPEC;
impl crate::RegisterSpec for DR8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dr8::R](R) reader structure"]
impl crate::Readable for DR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr8::W](W) writer structure"]
impl crate::Writable for DR8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR8 to value 0xff"]
impl crate::Resettable for DR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
