#[doc = "Register `DOUTR26` reader"]
pub struct R(crate::R<DOUTR26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR26_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTR26` writer"]
pub struct W(crate::W<DOUTR26_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR26_SPEC>;
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
impl From<crate::W<DOUTR26_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR26_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT26` reader - Output data sent to MDIO Master during read frames"]
pub struct DOUT26_R(crate::FieldReader<u16, u16>);
impl DOUT26_R {
    pub(crate) fn new(bits: u16) -> Self {
        DOUT26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT26_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT26` writer - Output data sent to MDIO Master during read frames"]
pub struct DOUT26_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT26_W<'a> {
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
    pub fn dout26(&self) -> DOUT26_R {
        DOUT26_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout26(&mut self) -> DOUT26_W {
        DOUT26_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS output data register 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr26](index.html) module"]
pub struct DOUTR26_SPEC;
impl crate::RegisterSpec for DOUTR26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutr26::R](R) reader structure"]
impl crate::Readable for DOUTR26_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutr26::W](W) writer structure"]
impl crate::Writable for DOUTR26_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOUTR26 to value 0"]
impl crate::Resettable for DOUTR26_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
