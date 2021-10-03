#[doc = "Register `CM0AR8` reader"]
pub struct R(crate::R<CM0AR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0AR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0AR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0AR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0AR8` writer"]
pub struct W(crate::W<CM0AR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0AR8_SPEC>;
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
impl From<crate::W<CM0AR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0AR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA` reader - peripheral address"]
pub struct PA_R(crate::FieldReader<u32, u32>);
impl PA_R {
    pub(crate) fn new(bits: u32) -> Self {
        PA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA` writer - peripheral address"]
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - peripheral address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0ar8](index.html) module"]
pub struct CM0AR8_SPEC;
impl crate::RegisterSpec for CM0AR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0ar8::R](R) reader structure"]
impl crate::Readable for CM0AR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0ar8::W](W) writer structure"]
impl crate::Writable for CM0AR8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM0AR8 to value 0"]
impl crate::Resettable for CM0AR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
