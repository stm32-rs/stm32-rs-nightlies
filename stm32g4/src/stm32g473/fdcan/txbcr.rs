#[doc = "Register `TXBCR` reader"]
pub struct R(crate::R<TXBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBCR` writer"]
pub struct W(crate::W<TXBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBCR_SPEC>;
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
impl From<crate::W<TXBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR` reader - CR"]
pub struct CR_R(crate::FieldReader<u8, u8>);
impl CR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR` writer - CR"]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CR"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CR"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcr](index.html) module"]
pub struct TXBCR_SPEC;
impl crate::RegisterSpec for TXBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbcr::R](R) reader structure"]
impl crate::Readable for TXBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbcr::W](W) writer structure"]
impl crate::Writable for TXBCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBCR to value 0"]
impl crate::Resettable for TXBCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
