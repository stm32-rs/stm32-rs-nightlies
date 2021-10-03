#[doc = "Register `TARG1_FN_MOD_LB` reader"]
pub struct R(crate::R<TARG1_FN_MOD_LB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARG1_FN_MOD_LB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARG1_FN_MOD_LB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARG1_FN_MOD_LB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARG1_FN_MOD_LB` writer"]
pub struct W(crate::W<TARG1_FN_MOD_LB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARG1_FN_MOD_LB_SPEC>;
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
impl From<crate::W<TARG1_FN_MOD_LB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARG1_FN_MOD_LB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FN_MOD_LB` reader - Controls burst breaking of long bursts"]
pub struct FN_MOD_LB_R(crate::FieldReader<bool, bool>);
impl FN_MOD_LB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FN_MOD_LB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FN_MOD_LB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FN_MOD_LB` writer - Controls burst breaking of long bursts"]
pub struct FN_MOD_LB_W<'a> {
    w: &'a mut W,
}
impl<'a> FN_MOD_LB_W<'a> {
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
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    pub fn fn_mod_lb(&self) -> FN_MOD_LB_R {
        FN_MOD_LB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    pub fn fn_mod_lb(&mut self) -> FN_MOD_LB_W {
        FN_MOD_LB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ1_fn_mod_lb](index.html) module"]
pub struct TARG1_FN_MOD_LB_SPEC;
impl crate::RegisterSpec for TARG1_FN_MOD_LB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [targ1_fn_mod_lb::R](R) reader structure"]
impl crate::Readable for TARG1_FN_MOD_LB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [targ1_fn_mod_lb::W](W) writer structure"]
impl crate::Writable for TARG1_FN_MOD_LB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TARG1_FN_MOD_LB to value 0x04"]
impl crate::Resettable for TARG1_FN_MOD_LB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
