#[doc = "Register `DMACCARxDR` reader"]
pub struct R(crate::R<DMACCARXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCARXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCARXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCARXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACCARxDR` writer"]
pub struct W(crate::W<DMACCARXDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACCARXDR_SPEC>;
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
impl From<crate::W<DMACCARXDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACCARXDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer"]
pub struct CURRDESAPTR_R(crate::FieldReader<u32, u32>);
impl CURRDESAPTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURRDESAPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURRDESAPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURRDESAPTR` writer - Application Receive Descriptor Address Pointer"]
pub struct CURRDESAPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRDESAPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&mut self) -> CURRDESAPTR_W {
        CURRDESAPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel current application receive descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccarx_dr](index.html) module"]
pub struct DMACCARXDR_SPEC;
impl crate::RegisterSpec for DMACCARXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccarx_dr::R](R) reader structure"]
impl crate::Readable for DMACCARXDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaccarx_dr::W](W) writer structure"]
impl crate::Writable for DMACCARXDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACCARxDR to value 0"]
impl crate::Resettable for DMACCARXDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
