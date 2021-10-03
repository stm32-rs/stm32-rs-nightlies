#[doc = "Register `RXCRC` reader"]
pub struct R(crate::R<RXCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCRC` writer"]
pub struct W(crate::W<RXCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCRC_SPEC>;
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
impl From<crate::W<RXCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXCRC` reader - CRC register for receiver"]
pub struct RXCRC_R(crate::FieldReader<u32, u32>);
impl RXCRC_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCRC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCRC` writer - CRC register for receiver"]
pub struct RXCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC register for receiver"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC register for receiver"]
    #[inline(always)]
    pub fn rxcrc(&mut self) -> RXCRC_W {
        RXCRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver CRC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcrc](index.html) module"]
pub struct RXCRC_SPEC;
impl crate::RegisterSpec for RXCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcrc::R](R) reader structure"]
impl crate::Readable for RXCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcrc::W](W) writer structure"]
impl crate::Writable for RXCRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCRC to value 0"]
impl crate::Resettable for RXCRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
