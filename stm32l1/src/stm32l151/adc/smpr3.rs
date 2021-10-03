#[doc = "Register `SMPR3` reader"]
pub struct R(crate::R<SMPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR3` writer"]
pub struct W(crate::W<SMPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR3_SPEC>;
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
impl From<crate::W<SMPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP` reader - Channel Sample time selection"]
pub struct SMP_R(crate::FieldReader<u32, u32>);
impl SMP_R {
    pub(crate) fn new(bits: u32) -> Self {
        SMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP` writer - Channel Sample time selection"]
pub struct SMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Channel Sample time selection"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Channel Sample time selection"]
    #[inline(always)]
    pub fn smp(&mut self) -> SMP_W {
        SMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sample time register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr3](index.html) module"]
pub struct SMPR3_SPEC;
impl crate::RegisterSpec for SMPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr3::R](R) reader structure"]
impl crate::Readable for SMPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr3::W](W) writer structure"]
impl crate::Writable for SMPR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR3 to value 0"]
impl crate::Resettable for SMPR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
