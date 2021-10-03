#[doc = "Register `FDCAN_TXBAR` reader"]
pub struct R(crate::R<FDCAN_TXBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TXBAR` writer"]
pub struct W(crate::W<FDCAN_TXBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXBAR_SPEC>;
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
impl From<crate::W<FDCAN_TXBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AR` reader - Add Request"]
pub struct AR_R(crate::FieldReader<u8, u8>);
impl AR_R {
    pub(crate) fn new(bits: u8) -> Self {
        AR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR` writer - Add Request"]
pub struct AR_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Add Request"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Add Request"]
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W {
        AR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx Buffer Add Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbar](index.html) module"]
pub struct FDCAN_TXBAR_SPEC;
impl crate::RegisterSpec for FDCAN_TXBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbar::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_txbar::W](W) writer structure"]
impl crate::Writable for FDCAN_TXBAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TXBAR to value 0"]
impl crate::Resettable for FDCAN_TXBAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
