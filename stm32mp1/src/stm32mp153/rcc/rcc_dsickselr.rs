#[doc = "Register `RCC_DSICKSELR` reader"]
pub struct R(crate::R<RCC_DSICKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_DSICKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_DSICKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_DSICKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_DSICKSELR` writer"]
pub struct W(crate::W<RCC_DSICKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_DSICKSELR_SPEC>;
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
impl From<crate::W<RCC_DSICKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_DSICKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSISRC` reader - DSISRC"]
pub struct DSISRC_R(crate::FieldReader<bool, bool>);
impl DSISRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSISRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSISRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSISRC` writer - DSISRC"]
pub struct DSISRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSISRC_W<'a> {
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
    #[doc = "Bit 0 - DSISRC"]
    #[inline(always)]
    pub fn dsisrc(&self) -> DSISRC_R {
        DSISRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSISRC"]
    #[inline(always)]
    pub fn dsisrc(&mut self) -> DSISRC_W {
        DSISRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the DSI block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_dsickselr](index.html) module"]
pub struct RCC_DSICKSELR_SPEC;
impl crate::RegisterSpec for RCC_DSICKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_dsickselr::R](R) reader structure"]
impl crate::Readable for RCC_DSICKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_dsickselr::W](W) writer structure"]
impl crate::Writable for RCC_DSICKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_DSICKSELR to value 0"]
impl crate::Resettable for RCC_DSICKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
