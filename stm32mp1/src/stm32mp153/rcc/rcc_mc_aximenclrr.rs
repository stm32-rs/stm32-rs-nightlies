#[doc = "Register `RCC_MC_AXIMENCLRR` reader"]
pub struct R(crate::R<RCC_MC_AXIMENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AXIMENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AXIMENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AXIMENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_AXIMENCLRR` writer"]
pub struct W(crate::W<RCC_MC_AXIMENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AXIMENCLRR_SPEC>;
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
impl From<crate::W<RCC_MC_AXIMENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AXIMENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSRAMEN` reader - SYSRAMEN"]
pub struct SYSRAMEN_R(crate::FieldReader<bool, bool>);
impl SYSRAMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSRAMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRAMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRAMEN` writer - SYSRAMEN"]
pub struct SYSRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SYSRAMEN"]
    #[inline(always)]
    pub fn sysramen(&self) -> SYSRAMEN_R {
        SYSRAMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSRAMEN"]
    #[inline(always)]
    pub fn sysramen(&mut self) -> SYSRAMEN_W {
        SYSRAMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to clear the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_aximenclrr](index.html) module"]
pub struct RCC_MC_AXIMENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MC_AXIMENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_aximenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MC_AXIMENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_aximenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MC_AXIMENCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_AXIMENCLRR to value 0"]
impl crate::Resettable for RCC_MC_AXIMENCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
