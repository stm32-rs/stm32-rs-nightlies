#[doc = "Register `WPCR4` reader"]
pub struct R(crate::R<WPCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR4` writer"]
pub struct W(crate::W<WPCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR4_SPEC>;
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
impl From<crate::W<WPCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THSZERO` reader - tCLK-POST"]
pub struct THSZERO_R(crate::FieldReader<u8, u8>);
impl THSZERO_R {
    pub(crate) fn new(bits: u8) -> Self {
        THSZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THSZERO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THSZERO` writer - tCLK-POST"]
pub struct THSZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> THSZERO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    pub fn thszero(&mut self) -> THSZERO_W {
        THSZERO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper PHY Configuration Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr4](index.html) module"]
pub struct WPCR4_SPEC;
impl crate::RegisterSpec for WPCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr4::R](R) reader structure"]
impl crate::Readable for WPCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr4::W](W) writer structure"]
impl crate::Writable for WPCR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR4 to value 0"]
impl crate::Resettable for WPCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
