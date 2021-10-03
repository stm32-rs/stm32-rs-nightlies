#[doc = "Register `REPAR` reader"]
pub struct R(crate::R<REPAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REPAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REPAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REPAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REPAR` writer"]
pub struct W(crate::W<REPAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REPAR_SPEC>;
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
impl From<crate::W<REPAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REPAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REPx` reader - Timerx Repetition counter value"]
pub struct REPX_R(crate::FieldReader<u8, u8>);
impl REPX_R {
    pub(crate) fn new(bits: u8) -> Self {
        REPX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REPX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPx` writer - Timerx Repetition counter value"]
pub struct REPX_W<'a> {
    w: &'a mut W,
}
impl<'a> REPX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Timerx Repetition counter value"]
    #[inline(always)]
    pub fn repx(&self) -> REPX_R {
        REPX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timerx Repetition counter value"]
    #[inline(always)]
    pub fn repx(&mut self) -> REPX_W {
        REPX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Repetition Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [repar](index.html) module"]
pub struct REPAR_SPEC;
impl crate::RegisterSpec for REPAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [repar::R](R) reader structure"]
impl crate::Readable for REPAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [repar::W](W) writer structure"]
impl crate::Writable for REPAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REPAR to value 0"]
impl crate::Resettable for REPAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
