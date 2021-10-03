#[doc = "Register `RCC_PLL3FRACR` reader"]
pub struct R(crate::R<RCC_PLL3FRACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL3FRACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL3FRACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL3FRACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_PLL3FRACR` writer"]
pub struct W(crate::W<RCC_PLL3FRACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL3FRACR_SPEC>;
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
impl From<crate::W<RCC_PLL3FRACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL3FRACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRACV` reader - FRACV"]
pub struct FRACV_R(crate::FieldReader<u16, u16>);
impl FRACV_R {
    pub(crate) fn new(bits: u16) -> Self {
        FRACV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRACV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRACV` writer - FRACV"]
pub struct FRACV_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 3)) | ((value as u32 & 0x1fff) << 3);
        self.w
    }
}
#[doc = "Field `FRACLE` reader - FRACLE"]
pub struct FRACLE_R(crate::FieldReader<bool, bool>);
impl FRACLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRACLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRACLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRACLE` writer - FRACLE"]
pub struct FRACLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:15 - FRACV"]
    #[inline(always)]
    pub fn fracv(&self) -> FRACV_R {
        FRACV_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - FRACLE"]
    #[inline(always)]
    pub fn fracle(&self) -> FRACLE_R {
        FRACLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:15 - FRACV"]
    #[inline(always)]
    pub fn fracv(&mut self) -> FRACV_W {
        FRACV_W { w: self }
    }
    #[doc = "Bit 16 - FRACLE"]
    #[inline(always)]
    pub fn fracle(&mut self) -> FRACLE_W {
        FRACLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll3fracr](index.html) module"]
pub struct RCC_PLL3FRACR_SPEC;
impl crate::RegisterSpec for RCC_PLL3FRACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_pll3fracr::R](R) reader structure"]
impl crate::Readable for RCC_PLL3FRACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_pll3fracr::W](W) writer structure"]
impl crate::Writable for RCC_PLL3FRACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_PLL3FRACR to value 0"]
impl crate::Resettable for RCC_PLL3FRACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
