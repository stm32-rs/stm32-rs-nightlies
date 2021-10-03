#[doc = "Register `DMACCARxBR` reader"]
pub struct R(crate::R<DMACCARXBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCARXBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCARXBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCARXBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACCARxBR` writer"]
pub struct W(crate::W<DMACCARXBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACCARXBR_SPEC>;
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
impl From<crate::W<DMACCARXBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACCARXBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer"]
pub struct CURRBUFAPTR_R(crate::FieldReader<u32, u32>);
impl CURRBUFAPTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURRBUFAPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURRBUFAPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURRBUFAPTR` writer - Application Receive Buffer Address Pointer"]
pub struct CURRBUFAPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRBUFAPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&mut self) -> CURRBUFAPTR_W {
        CURRBUFAPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel current application receive buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccarx_br](index.html) module"]
pub struct DMACCARXBR_SPEC;
impl crate::RegisterSpec for DMACCARXBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccarx_br::R](R) reader structure"]
impl crate::Readable for DMACCARXBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaccarx_br::W](W) writer structure"]
impl crate::Writable for DMACCARXBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACCARxBR to value 0"]
impl crate::Resettable for DMACCARXBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
