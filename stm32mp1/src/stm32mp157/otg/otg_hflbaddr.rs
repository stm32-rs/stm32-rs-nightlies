#[doc = "Register `OTG_HFLBADDR` reader"]
pub struct R(crate::R<OTG_HFLBADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HFLBADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HFLBADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HFLBADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HFLBADDR` writer"]
pub struct W(crate::W<OTG_HFLBADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HFLBADDR_SPEC>;
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
impl From<crate::W<OTG_HFLBADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HFLBADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFLBADDR` reader - HFLBADDR"]
pub struct HFLBADDR_R(crate::FieldReader<u32, u32>);
impl HFLBADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        HFLBADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFLBADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFLBADDR` writer - HFLBADDR"]
pub struct HFLBADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HFLBADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HFLBADDR"]
    #[inline(always)]
    pub fn hflbaddr(&self) -> HFLBADDR_R {
        HFLBADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HFLBADDR"]
    #[inline(always)]
    pub fn hflbaddr(&mut self) -> HFLBADDR_W {
        HFLBADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register holds the starting address of the frame list information (scatter/gather mode).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hflbaddr](index.html) module"]
pub struct OTG_HFLBADDR_SPEC;
impl crate::RegisterSpec for OTG_HFLBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hflbaddr::R](R) reader structure"]
impl crate::Readable for OTG_HFLBADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hflbaddr::W](W) writer structure"]
impl crate::Writable for OTG_HFLBADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HFLBADDR to value 0"]
impl crate::Resettable for OTG_HFLBADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
