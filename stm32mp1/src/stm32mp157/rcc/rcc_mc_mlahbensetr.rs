#[doc = "Register `RCC_MC_MLAHBENSETR` reader"]
pub struct R(crate::R<RCC_MC_MLAHBENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_MLAHBENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_MLAHBENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_MLAHBENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_MLAHBENSETR` writer"]
pub struct W(crate::W<RCC_MC_MLAHBENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_MLAHBENSETR_SPEC>;
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
impl From<crate::W<RCC_MC_MLAHBENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_MLAHBENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETRAMEN` reader - RETRAMEN"]
pub struct RETRAMEN_R(crate::FieldReader<bool, bool>);
impl RETRAMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RETRAMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETRAMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETRAMEN` writer - RETRAMEN"]
pub struct RETRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRAMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - RETRAMEN"]
    #[inline(always)]
    pub fn retramen(&self) -> RETRAMEN_R {
        RETRAMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RETRAMEN"]
    #[inline(always)]
    pub fn retramen(&mut self) -> RETRAMEN_W {
        RETRAMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_mlahbensetr](index.html) module"]
pub struct RCC_MC_MLAHBENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_MLAHBENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_mlahbensetr::R](R) reader structure"]
impl crate::Readable for RCC_MC_MLAHBENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_mlahbensetr::W](W) writer structure"]
impl crate::Writable for RCC_MC_MLAHBENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_MLAHBENSETR to value 0x10"]
impl crate::Resettable for RCC_MC_MLAHBENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
