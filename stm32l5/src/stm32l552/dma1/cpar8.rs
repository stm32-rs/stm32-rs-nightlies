#[doc = "Register `CPAR8` reader"]
pub struct R(crate::R<CPAR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPAR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPAR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPAR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPAR8` writer"]
pub struct W(crate::W<CPAR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPAR8_SPEC>;
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
impl From<crate::W<CPAR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPAR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - number of data to transfer"]
pub struct NDT_R(crate::FieldReader<u32, u32>);
impl NDT_R {
    pub(crate) fn new(bits: u32) -> Self {
        NDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDT` writer - number of data to transfer"]
pub struct NDT_W<'a> {
    w: &'a mut W,
}
impl<'a> NDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W {
        NDT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar8](index.html) module"]
pub struct CPAR8_SPEC;
impl crate::RegisterSpec for CPAR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpar8::R](R) reader structure"]
impl crate::Readable for CPAR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpar8::W](W) writer structure"]
impl crate::Writable for CPAR8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPAR8 to value 0"]
impl crate::Resettable for CPAR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}