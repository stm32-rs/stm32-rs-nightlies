#[doc = "Register `M2FDRH` reader"]
pub struct R(crate::R<M2FDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M2FDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M2FDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M2FDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M2FDRH` writer"]
pub struct W(crate::W<M2FDRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M2FDRH_SPEC>;
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
impl From<crate::W<M2FDRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M2FDRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub struct FDATAH_R(crate::FieldReader<u32, u32>);
impl FDATAH_R {
    pub(crate) fn new(bits: u32) -> Self {
        FDATAH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDATAH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDATAH` writer - Failing data high (64-bit memory)"]
pub struct FDATAH_W<'a> {
    w: &'a mut W,
}
impl<'a> FDATAH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&mut self) -> FDATAH_W {
        FDATAH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2fdrh](index.html) module"]
pub struct M2FDRH_SPEC;
impl crate::RegisterSpec for M2FDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m2fdrh::R](R) reader structure"]
impl crate::Readable for M2FDRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m2fdrh::W](W) writer structure"]
impl crate::Writable for M2FDRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M2FDRH to value 0"]
impl crate::Resettable for M2FDRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
