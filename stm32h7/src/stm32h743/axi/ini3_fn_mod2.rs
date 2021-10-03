#[doc = "Register `INI3_FN_MOD2` reader"]
pub struct R(crate::R<INI3_FN_MOD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INI3_FN_MOD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INI3_FN_MOD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INI3_FN_MOD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INI3_FN_MOD2` writer"]
pub struct W(crate::W<INI3_FN_MOD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INI3_FN_MOD2_SPEC>;
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
impl From<crate::W<INI3_FN_MOD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INI3_FN_MOD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS_MERGE` reader - Disables alteration of transactions by the up-sizer unless required by the protocol"]
pub struct BYPASS_MERGE_R(crate::FieldReader<bool, bool>);
impl BYPASS_MERGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_MERGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_MERGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS_MERGE` writer - Disables alteration of transactions by the up-sizer unless required by the protocol"]
pub struct BYPASS_MERGE_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_MERGE_W<'a> {
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
    #[doc = "Bit 0 - Disables alteration of transactions by the up-sizer unless required by the protocol"]
    #[inline(always)]
    pub fn bypass_merge(&self) -> BYPASS_MERGE_R {
        BYPASS_MERGE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disables alteration of transactions by the up-sizer unless required by the protocol"]
    #[inline(always)]
    pub fn bypass_merge(&mut self) -> BYPASS_MERGE_W {
        BYPASS_MERGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI interconnect - INI x functionality modification 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ini3_fn_mod2](index.html) module"]
pub struct INI3_FN_MOD2_SPEC;
impl crate::RegisterSpec for INI3_FN_MOD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ini3_fn_mod2::R](R) reader structure"]
impl crate::Readable for INI3_FN_MOD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ini3_fn_mod2::W](W) writer structure"]
impl crate::Writable for INI3_FN_MOD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INI3_FN_MOD2 to value 0x04"]
impl crate::Resettable for INI3_FN_MOD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
