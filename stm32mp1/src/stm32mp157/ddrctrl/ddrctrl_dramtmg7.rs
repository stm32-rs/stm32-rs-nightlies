#[doc = "Register `DDRCTRL_DRAMTMG7` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG7` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG7_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_CKPDX` reader - T_CKPDX"]
pub struct T_CKPDX_R(crate::FieldReader<u8, u8>);
impl T_CKPDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_CKPDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_CKPDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_CKPDX` writer - T_CKPDX"]
pub struct T_CKPDX_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKPDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `T_CKPDE` reader - T_CKPDE"]
pub struct T_CKPDE_R(crate::FieldReader<u8, u8>);
impl T_CKPDE_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_CKPDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_CKPDE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_CKPDE` writer - T_CKPDE"]
pub struct T_CKPDE_W<'a> {
    w: &'a mut W,
}
impl<'a> T_CKPDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - T_CKPDX"]
    #[inline(always)]
    pub fn t_ckpdx(&self) -> T_CKPDX_R {
        T_CKPDX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - T_CKPDE"]
    #[inline(always)]
    pub fn t_ckpde(&self) -> T_CKPDE_R {
        T_CKPDE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - T_CKPDX"]
    #[inline(always)]
    pub fn t_ckpdx(&mut self) -> T_CKPDX_W {
        T_CKPDX_W { w: self }
    }
    #[doc = "Bits 8:11 - T_CKPDE"]
    #[inline(always)]
    pub fn t_ckpde(&mut self) -> T_CKPDE_W {
        T_CKPDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg7](index.html) module"]
pub struct DDRCTRL_DRAMTMG7_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg7::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg7::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG7 to value 0x0202"]
impl crate::Resettable for DDRCTRL_DRAMTMG7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0202
    }
}
