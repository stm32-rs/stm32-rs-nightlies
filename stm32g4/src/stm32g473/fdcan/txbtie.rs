#[doc = "Register `TXBTIE` reader"]
pub struct R(crate::R<TXBTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBTIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBTIE` writer"]
pub struct W(crate::W<TXBTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBTIE_SPEC>;
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
impl From<crate::W<TXBTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBTIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE` reader - TIE"]
pub struct TIE_R(crate::FieldReader<u8, u8>);
impl TIE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIE` writer - TIE"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - TIE"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbtie](index.html) module"]
pub struct TXBTIE_SPEC;
impl crate::RegisterSpec for TXBTIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbtie::R](R) reader structure"]
impl crate::Readable for TXBTIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbtie::W](W) writer structure"]
impl crate::Writable for TXBTIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBTIE to value 0"]
impl crate::Resettable for TXBTIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
