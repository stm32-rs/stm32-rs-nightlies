#[doc = "Register `TAMP_CFGR` reader"]
pub struct R(crate::R<TAMP_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_CFGR` writer"]
pub struct W(crate::W<TAMP_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_CFGR_SPEC>;
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
impl From<crate::W<TAMP_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT3_RMP` reader - OUT3_RMP"]
pub struct OUT3_RMP_R(crate::FieldReader<bool, bool>);
impl OUT3_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT3_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT3_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT3_RMP` writer - OUT3_RMP"]
pub struct OUT3_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT3_RMP_W<'a> {
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
    #[doc = "Bit 0 - OUT3_RMP"]
    #[inline(always)]
    pub fn out3_rmp(&self) -> OUT3_RMP_R {
        OUT3_RMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OUT3_RMP"]
    #[inline(always)]
    pub fn out3_rmp(&mut self) -> OUT3_RMP_W {
        OUT3_RMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_cfgr](index.html) module"]
pub struct TAMP_CFGR_SPEC;
impl crate::RegisterSpec for TAMP_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_cfgr::R](R) reader structure"]
impl crate::Readable for TAMP_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_cfgr::W](W) writer structure"]
impl crate::Writable for TAMP_CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMP_CFGR to value 0"]
impl crate::Resettable for TAMP_CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
