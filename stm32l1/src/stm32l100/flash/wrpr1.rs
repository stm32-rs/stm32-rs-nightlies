#[doc = "Register `WRPR1` reader"]
pub struct R(crate::R<WRPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRPR1` writer"]
pub struct W(crate::W<WRPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRPR1_SPEC>;
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
impl From<crate::W<WRPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRP1` reader - Write protection"]
pub struct WRP1_R(crate::FieldReader<u32, u32>);
impl WRP1_R {
    pub(crate) fn new(bits: u32) -> Self {
        WRP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRP1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRP1` writer - Write protection"]
pub struct WRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write protection"]
    #[inline(always)]
    pub fn wrp1(&self) -> WRP1_R {
        WRP1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write protection"]
    #[inline(always)]
    pub fn wrp1(&mut self) -> WRP1_W {
        WRP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrpr1](index.html) module"]
pub struct WRPR1_SPEC;
impl crate::RegisterSpec for WRPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrpr1::R](R) reader structure"]
impl crate::Readable for WRPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrpr1::W](W) writer structure"]
impl crate::Writable for WRPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRPR1 to value 0"]
impl crate::Resettable for WRPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
