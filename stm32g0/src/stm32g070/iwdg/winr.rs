#[doc = "Register `WINR` reader"]
pub struct R(crate::R<WINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINR` writer"]
pub struct W(crate::W<WINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINR_SPEC>;
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
impl From<crate::W<WINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIN` reader - Watchdog counter window value"]
pub struct WIN_R(crate::FieldReader<u16, u16>);
impl WIN_R {
    pub(crate) fn new(bits: u16) -> Self {
        WIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIN` writer - Watchdog counter window value"]
pub struct WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W {
        WIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winr](index.html) module"]
pub struct WINR_SPEC;
impl crate::RegisterSpec for WINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [winr::R](R) reader structure"]
impl crate::Readable for WINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winr::W](W) writer structure"]
impl crate::Writable for WINR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WINR to value 0x0fff"]
impl crate::Resettable for WINR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
