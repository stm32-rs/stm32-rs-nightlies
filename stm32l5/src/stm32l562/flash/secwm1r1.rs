#[doc = "Register `SECWM1R1` reader"]
pub struct R(crate::R<SECWM1R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECWM1R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECWM1R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECWM1R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECWM1R1` writer"]
pub struct W(crate::W<SECWM1R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECWM1R1_SPEC>;
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
impl From<crate::W<SECWM1R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECWM1R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECWM1_PSTRT` reader - SECWM1_PSTRT"]
pub struct SECWM1_PSTRT_R(crate::FieldReader<u8, u8>);
impl SECWM1_PSTRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SECWM1_PSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECWM1_PSTRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECWM1_PSTRT` writer - SECWM1_PSTRT"]
pub struct SECWM1_PSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SECWM1_PSTRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `SECWM1_PEND` reader - SECWM1_PEND"]
pub struct SECWM1_PEND_R(crate::FieldReader<u8, u8>);
impl SECWM1_PEND_R {
    pub(crate) fn new(bits: u8) -> Self {
        SECWM1_PEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECWM1_PEND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECWM1_PEND` writer - SECWM1_PEND"]
pub struct SECWM1_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SECWM1_PEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - SECWM1_PSTRT"]
    #[inline(always)]
    pub fn secwm1_pstrt(&self) -> SECWM1_PSTRT_R {
        SECWM1_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - SECWM1_PEND"]
    #[inline(always)]
    pub fn secwm1_pend(&self) -> SECWM1_PEND_R {
        SECWM1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SECWM1_PSTRT"]
    #[inline(always)]
    pub fn secwm1_pstrt(&mut self) -> SECWM1_PSTRT_W {
        SECWM1_PSTRT_W { w: self }
    }
    #[doc = "Bits 16:22 - SECWM1_PEND"]
    #[inline(always)]
    pub fn secwm1_pend(&mut self) -> SECWM1_PEND_W {
        SECWM1_PEND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash bank 1 secure watermak1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm1r1](index.html) module"]
pub struct SECWM1R1_SPEC;
impl crate::RegisterSpec for SECWM1R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secwm1r1::R](R) reader structure"]
impl crate::Readable for SECWM1R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secwm1r1::W](W) writer structure"]
impl crate::Writable for SECWM1R1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECWM1R1 to value 0xff00_ff00"]
impl crate::Resettable for SECWM1R1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00_ff00
    }
}
