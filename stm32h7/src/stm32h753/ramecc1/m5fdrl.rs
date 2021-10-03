#[doc = "Register `M5FDRL` reader"]
pub struct R(crate::R<M5FDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M5FDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M5FDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M5FDRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M5FDRL` writer"]
pub struct W(crate::W<M5FDRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M5FDRL_SPEC>;
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
impl From<crate::W<M5FDRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M5FDRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDATAL` reader - ECC failing data low"]
pub struct FDATAL_R(crate::FieldReader<u32, u32>);
impl FDATAL_R {
    pub(crate) fn new(bits: u32) -> Self {
        FDATAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDATAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ECC failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor 5 failing data low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5fdrl](index.html) module"]
pub struct M5FDRL_SPEC;
impl crate::RegisterSpec for M5FDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m5fdrl::R](R) reader structure"]
impl crate::Readable for M5FDRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m5fdrl::W](W) writer structure"]
impl crate::Writable for M5FDRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M5FDRL to value 0"]
impl crate::Resettable for M5FDRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
