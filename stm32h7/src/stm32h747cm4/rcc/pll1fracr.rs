#[doc = "Register `PLL1FRACR` reader"]
pub struct R(crate::R<PLL1FRACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1FRACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1FRACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1FRACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1FRACR` writer"]
pub struct W(crate::W<PLL1FRACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1FRACR_SPEC>;
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
impl From<crate::W<PLL1FRACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1FRACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRACN1` reader - Fractional part of the multiplication factor for PLL1 VCO"]
pub struct FRACN1_R(crate::FieldReader<u16, u16>);
impl FRACN1_R {
    pub(crate) fn new(bits: u16) -> Self {
        FRACN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRACN1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRACN1` writer - Fractional part of the multiplication factor for PLL1 VCO"]
pub struct FRACN1_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 3)) | ((value as u32 & 0x1fff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn fracn1(&self) -> FRACN1_R {
        FRACN1_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn fracn1(&mut self) -> FRACN1_W {
        FRACN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL1 Fractional Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1fracr](index.html) module"]
pub struct PLL1FRACR_SPEC;
impl crate::RegisterSpec for PLL1FRACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1fracr::R](R) reader structure"]
impl crate::Readable for PLL1FRACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1fracr::W](W) writer structure"]
impl crate::Writable for PLL1FRACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL1FRACR to value 0"]
impl crate::Resettable for PLL1FRACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
