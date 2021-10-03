#[doc = "Register `CHSELR` reader"]
pub struct R(crate::R<CHSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHSELR` writer"]
pub struct W(crate::W<CHSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSELR_SPEC>;
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
impl From<crate::W<CHSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL` reader - Channel-x selection"]
pub struct CHSEL_R(crate::FieldReader<u32, u32>);
impl CHSEL_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL` writer - Channel-x selection"]
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | (value as u32 & 0x0007_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18 - Channel-x selection"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chselr](index.html) module"]
pub struct CHSELR_SPEC;
impl crate::RegisterSpec for CHSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chselr::R](R) reader structure"]
impl crate::Readable for CHSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chselr::W](W) writer structure"]
impl crate::Writable for CHSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHSELR to value 0x0fff_0000"]
impl crate::Resettable for CHSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}
