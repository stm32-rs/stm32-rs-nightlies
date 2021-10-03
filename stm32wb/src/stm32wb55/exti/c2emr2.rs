#[doc = "Register `C2EMR2` reader"]
pub struct R(crate::R<C2EMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2EMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2EMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2EMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2EMR2` writer"]
pub struct W(crate::W<C2EMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2EMR2_SPEC>;
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
impl From<crate::W<C2EMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2EMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM` reader - CPU(m) Wakeup with event generation Mask on Event input"]
pub struct EM_R(crate::FieldReader<u8, u8>);
impl EM_R {
    pub(crate) fn new(bits: u8) -> Self {
        EM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM` writer - CPU(m) Wakeup with event generation Mask on Event input"]
pub struct EM_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em(&mut self) -> EM_W {
        EM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr2](index.html) module"]
pub struct C2EMR2_SPEC;
impl crate::RegisterSpec for C2EMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2emr2::R](R) reader structure"]
impl crate::Readable for C2EMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2emr2::W](W) writer structure"]
impl crate::Writable for C2EMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2EMR2 to value 0"]
impl crate::Resettable for C2EMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
