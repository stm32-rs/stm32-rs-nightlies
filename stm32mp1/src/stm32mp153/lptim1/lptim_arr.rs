#[doc = "Register `LPTIM_ARR` reader"]
pub struct R(crate::R<LPTIM_ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_ARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_ARR` writer"]
pub struct W(crate::W<LPTIM_ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_ARR_SPEC>;
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
impl From<crate::W<LPTIM_ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_ARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARR` reader - ARR"]
pub struct ARR_R(crate::FieldReader<u16, u16>);
impl ARR_R {
    pub(crate) fn new(bits: u16) -> Self {
        ARR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARR` writer - ARR"]
pub struct ARR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W {
        ARR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM autoreload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_arr](index.html) module"]
pub struct LPTIM_ARR_SPEC;
impl crate::RegisterSpec for LPTIM_ARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_arr::R](R) reader structure"]
impl crate::Readable for LPTIM_ARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_arr::W](W) writer structure"]
impl crate::Writable for LPTIM_ARR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPTIM_ARR to value 0x01"]
impl crate::Resettable for LPTIM_ARR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
