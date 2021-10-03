#[doc = "Register `WDATA` reader"]
pub struct R(crate::R<WDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDATA` writer"]
pub struct W(crate::W<WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDATA_SPEC>;
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
impl From<crate::W<WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARG` reader - ARG"]
pub struct ARG_R(crate::FieldReader<u32, u32>);
impl ARG_R {
    pub(crate) fn new(bits: u32) -> Self {
        ARG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARG` writer - ARG"]
pub struct ARG_W<'a> {
    w: &'a mut W,
}
impl<'a> ARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ARG"]
    #[inline(always)]
    pub fn arg(&self) -> ARG_R {
        ARG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ARG"]
    #[inline(always)]
    pub fn arg(&mut self) -> ARG_W {
        ARG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CORDIC argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdata](index.html) module"]
pub struct WDATA_SPEC;
impl crate::RegisterSpec for WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdata::R](R) reader structure"]
impl crate::Readable for WDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdata::W](W) writer structure"]
impl crate::Writable for WDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDATA to value 0"]
impl crate::Resettable for WDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
