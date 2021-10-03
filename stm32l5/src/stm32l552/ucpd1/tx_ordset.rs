#[doc = "Register `TX_ORDSET` reader"]
pub struct R(crate::R<TX_ORDSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_ORDSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_ORDSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_ORDSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_ORDSET` writer"]
pub struct W(crate::W<TX_ORDSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_ORDSET_SPEC>;
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
impl From<crate::W<TX_ORDSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_ORDSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXORDSET` reader - TXORDSET"]
pub struct TXORDSET_R(crate::FieldReader<u32, u32>);
impl TXORDSET_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXORDSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXORDSET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXORDSET` writer - TXORDSET"]
pub struct TXORDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TXORDSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - TXORDSET"]
    #[inline(always)]
    pub fn txordset(&self) -> TXORDSET_R {
        TXORDSET_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - TXORDSET"]
    #[inline(always)]
    pub fn txordset(&mut self) -> TXORDSET_W {
        TXORDSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD Tx Ordered Set Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ordset](index.html) module"]
pub struct TX_ORDSET_SPEC;
impl crate::RegisterSpec for TX_ORDSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ordset::R](R) reader structure"]
impl crate::Readable for TX_ORDSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ordset::W](W) writer structure"]
impl crate::Writable for TX_ORDSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_ORDSET to value 0"]
impl crate::Resettable for TX_ORDSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
