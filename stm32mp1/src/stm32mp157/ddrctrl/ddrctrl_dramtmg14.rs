#[doc = "Register `DDRCTRL_DRAMTMG14` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG14` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG14_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_XSR` reader - T_XSR"]
pub struct T_XSR_R(crate::FieldReader<u16, u16>);
impl T_XSR_R {
    pub(crate) fn new(bits: u16) -> Self {
        T_XSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_XSR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_XSR` writer - T_XSR"]
pub struct T_XSR_W<'a> {
    w: &'a mut W,
}
impl<'a> T_XSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - T_XSR"]
    #[inline(always)]
    pub fn t_xsr(&self) -> T_XSR_R {
        T_XSR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - T_XSR"]
    #[inline(always)]
    pub fn t_xsr(&mut self) -> T_XSR_W {
        T_XSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg14](index.html) module"]
pub struct DDRCTRL_DRAMTMG14_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg14::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg14::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG14 to value 0xa0"]
impl crate::Resettable for DDRCTRL_DRAMTMG14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa0
    }
}
