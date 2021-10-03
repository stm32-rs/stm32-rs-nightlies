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
#[doc = "Field `TCLKPOST` reader - tCLK-POST"]
pub struct TCLKPOST_R(crate::FieldReader<u8, u8>);
impl TCLKPOST_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCLKPOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLKPOST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCLKPOST` writer - tCLK-POST"]
pub struct TCLKPOST_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKPOST_W<'a> {
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
    pub fn tclkpost(&self) -> TCLKPOST_R {
        TCLKPOST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    pub fn tclkpost(&mut self) -> TCLKPOST_W {
        TCLKPOST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI wrapper PHY configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr4](index.html) module"]
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
