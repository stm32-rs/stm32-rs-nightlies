#[doc = "Register `DDRPHYC_ZQ0CR1` reader"]
pub struct R(crate::R<DDRPHYC_ZQ0CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ZQ0CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ZQ0CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ZQ0CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_ZQ0CR1` writer"]
pub struct W(crate::W<DDRPHYC_ZQ0CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_ZQ0CR1_SPEC>;
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
impl From<crate::W<DDRPHYC_ZQ0CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_ZQ0CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZPROG` reader - ZPROG"]
pub struct ZPROG_R(crate::FieldReader<u8, u8>);
impl ZPROG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZPROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZPROG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZPROG` writer - ZPROG"]
pub struct ZPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> ZPROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ZPROG"]
    #[inline(always)]
    pub fn zprog(&self) -> ZPROG_R {
        ZPROG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ZPROG"]
    #[inline(always)]
    pub fn zprog(&mut self) -> ZPROG_W {
        ZPROG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC ZQ0CR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0cr1](index.html) module"]
pub struct DDRPHYC_ZQ0CR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_ZQ0CR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ddrphyc_zq0cr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_zq0cr1::W](W) writer structure"]
impl crate::Writable for DDRPHYC_ZQ0CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_ZQ0CR1 to value 0x7b"]
impl crate::Resettable for DDRPHYC_ZQ0CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7b
    }
}
