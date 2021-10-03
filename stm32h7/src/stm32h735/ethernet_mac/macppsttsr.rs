#[doc = "Register `MACPPSTTSR` reader"]
pub struct R(crate::R<MACPPSTTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPPSTTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPPSTTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPPSTTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACPPSTTSR` writer"]
pub struct W(crate::W<MACPPSTTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPPSTTSR_SPEC>;
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
impl From<crate::W<MACPPSTTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPPSTTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTRH0` reader - PPS Target Time Seconds Register"]
pub struct TSTRH0_R(crate::FieldReader<u32, u32>);
impl TSTRH0_R {
    pub(crate) fn new(bits: u32) -> Self {
        TSTRH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTRH0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTRH0` writer - PPS Target Time Seconds Register"]
pub struct TSTRH0_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTRH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - PPS Target Time Seconds Register"]
    #[inline(always)]
    pub fn tstrh0(&self) -> TSTRH0_R {
        TSTRH0_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - PPS Target Time Seconds Register"]
    #[inline(always)]
    pub fn tstrh0(&mut self) -> TSTRH0_W {
        TSTRH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS target time seconds register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macppsttsr](index.html) module"]
pub struct MACPPSTTSR_SPEC;
impl crate::RegisterSpec for MACPPSTTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macppsttsr::R](R) reader structure"]
impl crate::Readable for MACPPSTTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macppsttsr::W](W) writer structure"]
impl crate::Writable for MACPPSTTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACPPSTTSR to value 0"]
impl crate::Resettable for MACPPSTTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
