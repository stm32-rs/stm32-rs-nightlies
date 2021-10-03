#[doc = "Register `MACL4A1R` reader"]
pub struct R(crate::R<MACL4A1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACL4A1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACL4A1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACL4A1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACL4A1R` writer"]
pub struct W(crate::W<MACL4A1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACL4A1R_SPEC>;
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
impl From<crate::W<MACL4A1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACL4A1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L4SP1` reader - Layer 4 Source Port Number Field"]
pub struct L4SP1_R(crate::FieldReader<u16, u16>);
impl L4SP1_R {
    pub(crate) fn new(bits: u16) -> Self {
        L4SP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L4SP1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L4SP1` writer - Layer 4 Source Port Number Field"]
pub struct L4SP1_W<'a> {
    w: &'a mut W,
}
impl<'a> L4SP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `L4DP1` reader - Layer 4 Destination Port Number Field"]
pub struct L4DP1_R(crate::FieldReader<u16, u16>);
impl L4DP1_R {
    pub(crate) fn new(bits: u16) -> Self {
        L4DP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L4DP1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L4DP1` writer - Layer 4 Destination Port Number Field"]
pub struct L4DP1_W<'a> {
    w: &'a mut W,
}
impl<'a> L4DP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Layer 4 Source Port Number Field"]
    #[inline(always)]
    pub fn l4sp1(&self) -> L4SP1_R {
        L4SP1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Layer 4 Destination Port Number Field"]
    #[inline(always)]
    pub fn l4dp1(&self) -> L4DP1_R {
        L4DP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Layer 4 Source Port Number Field"]
    #[inline(always)]
    pub fn l4sp1(&mut self) -> L4SP1_W {
        L4SP1_W { w: self }
    }
    #[doc = "Bits 16:31 - Layer 4 Destination Port Number Field"]
    #[inline(always)]
    pub fn l4dp1(&mut self) -> L4DP1_W {
        L4DP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 4 address filter 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl4a1r](index.html) module"]
pub struct MACL4A1R_SPEC;
impl crate::RegisterSpec for MACL4A1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macl4a1r::R](R) reader structure"]
impl crate::Readable for MACL4A1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macl4a1r::W](W) writer structure"]
impl crate::Writable for MACL4A1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACL4A1R to value 0"]
impl crate::Resettable for MACL4A1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
