#[doc = "Register `DOUTR8` reader"]
pub struct R(crate::R<DOUTR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTR8` writer"]
pub struct W(crate::W<DOUTR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR8_SPEC>;
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
impl From<crate::W<DOUTR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT8` reader - Output data sent to MDIO Master during read frames"]
pub struct DOUT8_R(crate::FieldReader<u16, u16>);
impl DOUT8_R {
    pub(crate) fn new(bits: u16) -> Self {
        DOUT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT8_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT8` writer - Output data sent to MDIO Master during read frames"]
pub struct DOUT8_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout8(&self) -> DOUT8_R {
        DOUT8_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout8(&mut self) -> DOUT8_W {
        DOUT8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS output data register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr8](index.html) module"]
pub struct DOUTR8_SPEC;
impl crate::RegisterSpec for DOUTR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutr8::R](R) reader structure"]
impl crate::Readable for DOUTR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutr8::W](W) writer structure"]
impl crate::Writable for DOUTR8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOUTR8 to value 0"]
impl crate::Resettable for DOUTR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
