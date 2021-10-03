#[doc = "Register `DDRCTRL_ZQCTL2` reader"]
pub struct R(crate::R<DDRCTRL_ZQCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ZQCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ZQCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ZQCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ZQCTL2` writer"]
pub struct W(crate::W<DDRCTRL_ZQCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ZQCTL2_SPEC>;
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
impl From<crate::W<DDRCTRL_ZQCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ZQCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZQ_RESET` reader - ZQ_RESET"]
pub struct ZQ_RESET_R(crate::FieldReader<bool, bool>);
impl ZQ_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZQ_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZQ_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZQ_RESET` writer - ZQ_RESET"]
pub struct ZQ_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQ_RESET_W<'a> {
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
    #[doc = "Bit 0 - ZQ_RESET"]
    #[inline(always)]
    pub fn zq_reset(&self) -> ZQ_RESET_R {
        ZQ_RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ZQ_RESET"]
    #[inline(always)]
    pub fn zq_reset(&mut self) -> ZQ_RESET_W {
        ZQ_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL ZQ control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_zqctl2](index.html) module"]
pub struct DDRCTRL_ZQCTL2_SPEC;
impl crate::RegisterSpec for DDRCTRL_ZQCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_zqctl2::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ZQCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_zqctl2::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ZQCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_ZQCTL2 to value 0"]
impl crate::Resettable for DDRCTRL_ZQCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
