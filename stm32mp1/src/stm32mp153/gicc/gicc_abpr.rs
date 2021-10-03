#[doc = "Register `GICC_ABPR` reader"]
pub struct R(crate::R<GICC_ABPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_ABPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_ABPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_ABPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICC_ABPR` writer"]
pub struct W(crate::W<GICC_ABPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_ABPR_SPEC>;
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
impl From<crate::W<GICC_ABPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_ABPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BINARY_POINT` reader - BINARY_POINT"]
pub struct BINARY_POINT_R(crate::FieldReader<u8, u8>);
impl BINARY_POINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        BINARY_POINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BINARY_POINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BINARY_POINT` writer - BINARY_POINT"]
pub struct BINARY_POINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BINARY_POINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - BINARY_POINT"]
    #[inline(always)]
    pub fn binary_point(&self) -> BINARY_POINT_R {
        BINARY_POINT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BINARY_POINT"]
    #[inline(always)]
    pub fn binary_point(&mut self) -> BINARY_POINT_W {
        BINARY_POINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_abpr](index.html) module"]
pub struct GICC_ABPR_SPEC;
impl crate::RegisterSpec for GICC_ABPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_abpr::R](R) reader structure"]
impl crate::Readable for GICC_ABPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicc_abpr::W](W) writer structure"]
impl crate::Writable for GICC_ABPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICC_ABPR to value 0x03"]
impl crate::Resettable for GICC_ABPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
