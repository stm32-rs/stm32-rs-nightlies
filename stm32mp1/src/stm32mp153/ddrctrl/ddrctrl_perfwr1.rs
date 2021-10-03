#[doc = "Register `DDRCTRL_PERFWR1` reader"]
pub struct R(crate::R<DDRCTRL_PERFWR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PERFWR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PERFWR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PERFWR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PERFWR1` writer"]
pub struct W(crate::W<DDRCTRL_PERFWR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PERFWR1_SPEC>;
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
impl From<crate::W<DDRCTRL_PERFWR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PERFWR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `W_MAX_STARVE` reader - W_MAX_STARVE"]
pub struct W_MAX_STARVE_R(crate::FieldReader<u16, u16>);
impl W_MAX_STARVE_R {
    pub(crate) fn new(bits: u16) -> Self {
        W_MAX_STARVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for W_MAX_STARVE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `W_MAX_STARVE` writer - W_MAX_STARVE"]
pub struct W_MAX_STARVE_W<'a> {
    w: &'a mut W,
}
impl<'a> W_MAX_STARVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `W_XACT_RUN_LENGTH` reader - W_XACT_RUN_LENGTH"]
pub struct W_XACT_RUN_LENGTH_R(crate::FieldReader<u8, u8>);
impl W_XACT_RUN_LENGTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        W_XACT_RUN_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for W_XACT_RUN_LENGTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `W_XACT_RUN_LENGTH` writer - W_XACT_RUN_LENGTH"]
pub struct W_XACT_RUN_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> W_XACT_RUN_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - W_MAX_STARVE"]
    #[inline(always)]
    pub fn w_max_starve(&self) -> W_MAX_STARVE_R {
        W_MAX_STARVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - W_XACT_RUN_LENGTH"]
    #[inline(always)]
    pub fn w_xact_run_length(&self) -> W_XACT_RUN_LENGTH_R {
        W_XACT_RUN_LENGTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - W_MAX_STARVE"]
    #[inline(always)]
    pub fn w_max_starve(&mut self) -> W_MAX_STARVE_W {
        W_MAX_STARVE_W { w: self }
    }
    #[doc = "Bits 24:31 - W_XACT_RUN_LENGTH"]
    #[inline(always)]
    pub fn w_xact_run_length(&mut self) -> W_XACT_RUN_LENGTH_W {
        W_XACT_RUN_LENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL write CAM register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_perfwr1](index.html) module"]
pub struct DDRCTRL_PERFWR1_SPEC;
impl crate::RegisterSpec for DDRCTRL_PERFWR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_perfwr1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PERFWR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_perfwr1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PERFWR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_PERFWR1 to value 0x0f00_007f"]
impl crate::Resettable for DDRCTRL_PERFWR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f00_007f
    }
}
