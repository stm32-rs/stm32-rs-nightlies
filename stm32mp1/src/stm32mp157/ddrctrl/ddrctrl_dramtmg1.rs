#[doc = "Register `DDRCTRL_DRAMTMG1` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG1` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG1_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_RC` reader - T_RC"]
pub struct T_RC_R(crate::FieldReader<u8, u8>);
impl T_RC_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_RC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_RC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_RC` writer - T_RC"]
pub struct T_RC_W<'a> {
    w: &'a mut W,
}
impl<'a> T_RC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `RD2PRE` reader - RD2PRE"]
pub struct RD2PRE_R(crate::FieldReader<u8, u8>);
impl RD2PRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RD2PRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD2PRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD2PRE` writer - RD2PRE"]
pub struct RD2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RD2PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `T_XP` reader - T_XP"]
pub struct T_XP_R(crate::FieldReader<u8, u8>);
impl T_XP_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_XP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_XP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_XP` writer - T_XP"]
pub struct T_XP_W<'a> {
    w: &'a mut W,
}
impl<'a> T_XP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - T_RC"]
    #[inline(always)]
    pub fn t_rc(&self) -> T_RC_R {
        T_RC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - RD2PRE"]
    #[inline(always)]
    pub fn rd2pre(&self) -> RD2PRE_R {
        RD2PRE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - T_XP"]
    #[inline(always)]
    pub fn t_xp(&self) -> T_XP_R {
        T_XP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - T_RC"]
    #[inline(always)]
    pub fn t_rc(&mut self) -> T_RC_W {
        T_RC_W { w: self }
    }
    #[doc = "Bits 8:13 - RD2PRE"]
    #[inline(always)]
    pub fn rd2pre(&mut self) -> RD2PRE_W {
        RD2PRE_W { w: self }
    }
    #[doc = "Bits 16:20 - T_XP"]
    #[inline(always)]
    pub fn t_xp(&mut self) -> T_XP_W {
        T_XP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg1](index.html) module"]
pub struct DDRCTRL_DRAMTMG1_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG1 to value 0x0008_0414"]
impl crate::Resettable for DDRCTRL_DRAMTMG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0414
    }
}
