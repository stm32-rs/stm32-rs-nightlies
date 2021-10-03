#[doc = "Register `MACSPI2R` reader"]
pub struct R(crate::R<MACSPI2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSPI2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSPI2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSPI2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACSPI2R` writer"]
pub struct W(crate::W<MACSPI2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACSPI2R_SPEC>;
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
impl From<crate::W<MACSPI2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACSPI2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI2` reader - Source Port Identity 2"]
pub struct SPI2_R(crate::FieldReader<u16, u16>);
impl SPI2_R {
    pub(crate) fn new(bits: u16) -> Self {
        SPI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2` writer - Source Port Identity 2"]
pub struct SPI2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Source Port Identity 2"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source Port Identity 2"]
    #[inline(always)]
    pub fn spi2(&mut self) -> SPI2_W {
        SPI2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PTP Source port identity 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macspi2r](index.html) module"]
pub struct MACSPI2R_SPEC;
impl crate::RegisterSpec for MACSPI2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macspi2r::R](R) reader structure"]
impl crate::Readable for MACSPI2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macspi2r::W](W) writer structure"]
impl crate::Writable for MACSPI2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACSPI2R to value 0"]
impl crate::Resettable for MACSPI2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
