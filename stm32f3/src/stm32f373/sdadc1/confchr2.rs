#[doc = "Register `CONFCHR2` reader"]
pub struct R(crate::R<CONFCHR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFCHR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFCHR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFCHR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFCHR2` writer"]
pub struct W(crate::W<CONFCHR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFCHR2_SPEC>;
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
impl From<crate::W<CONFCHR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFCHR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONFCH8` reader - Channel 8 configuration"]
pub struct CONFCH8_R(crate::FieldReader<u8, u8>);
impl CONFCH8_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFCH8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFCH8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFCH8` writer - Channel 8 configuration"]
pub struct CONFCH8_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFCH8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 8 configuration"]
    #[inline(always)]
    pub fn confch8(&self) -> CONFCH8_R {
        CONFCH8_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 8 configuration"]
    #[inline(always)]
    pub fn confch8(&mut self) -> CONFCH8_W {
        CONFCH8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confchr2](index.html) module"]
pub struct CONFCHR2_SPEC;
impl crate::RegisterSpec for CONFCHR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confchr2::R](R) reader structure"]
impl crate::Readable for CONFCHR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confchr2::W](W) writer structure"]
impl crate::Writable for CONFCHR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFCHR2 to value 0"]
impl crate::Resettable for CONFCHR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
