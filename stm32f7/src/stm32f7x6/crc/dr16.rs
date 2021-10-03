#[doc = "Register `DR16` reader"]
pub struct R(crate::R<DR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR16` writer"]
pub struct W(crate::W<DR16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR16_SPEC>;
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
impl From<crate::W<DR16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DR16` reader - Data register bits"]
pub struct DR16_R(crate::FieldReader<u16, u16>);
impl DR16_R {
    pub(crate) fn new(bits: u16) -> Self {
        DR16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR16` writer - Data register bits"]
pub struct DR16_W<'a> {
    w: &'a mut W,
}
impl<'a> DR16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data register bits"]
    #[inline(always)]
    pub fn dr16(&self) -> DR16_R {
        DR16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data register bits"]
    #[inline(always)]
    pub fn dr16(&mut self) -> DR16_W {
        DR16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register - half-word sized\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr16](index.html) module"]
pub struct DR16_SPEC;
impl crate::RegisterSpec for DR16_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dr16::R](R) reader structure"]
impl crate::Readable for DR16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr16::W](W) writer structure"]
impl crate::Writable for DR16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR16 to value 0xffff"]
impl crate::Resettable for DR16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
