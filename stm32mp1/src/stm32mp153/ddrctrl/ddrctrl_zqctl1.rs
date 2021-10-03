#[doc = "Register `DDRCTRL_ZQCTL1` reader"]
pub struct R(crate::R<DDRCTRL_ZQCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ZQCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ZQCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ZQCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ZQCTL1` writer"]
pub struct W(crate::W<DDRCTRL_ZQCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ZQCTL1_SPEC>;
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
impl From<crate::W<DDRCTRL_ZQCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ZQCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_ZQ_SHORT_INTERVAL_X1024` reader - T_ZQ_SHORT_INTERVAL_X1024"]
pub struct T_ZQ_SHORT_INTERVAL_X1024_R(crate::FieldReader<u32, u32>);
impl T_ZQ_SHORT_INTERVAL_X1024_R {
    pub(crate) fn new(bits: u32) -> Self {
        T_ZQ_SHORT_INTERVAL_X1024_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_ZQ_SHORT_INTERVAL_X1024_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_ZQ_SHORT_INTERVAL_X1024` writer - T_ZQ_SHORT_INTERVAL_X1024"]
pub struct T_ZQ_SHORT_INTERVAL_X1024_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ZQ_SHORT_INTERVAL_X1024_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
#[doc = "Field `T_ZQ_RESET_NOP` reader - T_ZQ_RESET_NOP"]
pub struct T_ZQ_RESET_NOP_R(crate::FieldReader<u16, u16>);
impl T_ZQ_RESET_NOP_R {
    pub(crate) fn new(bits: u16) -> Self {
        T_ZQ_RESET_NOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_ZQ_RESET_NOP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_ZQ_RESET_NOP` writer - T_ZQ_RESET_NOP"]
pub struct T_ZQ_RESET_NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ZQ_RESET_NOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - T_ZQ_SHORT_INTERVAL_X1024"]
    #[inline(always)]
    pub fn t_zq_short_interval_x1024(&self) -> T_ZQ_SHORT_INTERVAL_X1024_R {
        T_ZQ_SHORT_INTERVAL_X1024_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 20:29 - T_ZQ_RESET_NOP"]
    #[inline(always)]
    pub fn t_zq_reset_nop(&self) -> T_ZQ_RESET_NOP_R {
        T_ZQ_RESET_NOP_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - T_ZQ_SHORT_INTERVAL_X1024"]
    #[inline(always)]
    pub fn t_zq_short_interval_x1024(&mut self) -> T_ZQ_SHORT_INTERVAL_X1024_W {
        T_ZQ_SHORT_INTERVAL_X1024_W { w: self }
    }
    #[doc = "Bits 20:29 - T_ZQ_RESET_NOP"]
    #[inline(always)]
    pub fn t_zq_reset_nop(&mut self) -> T_ZQ_RESET_NOP_W {
        T_ZQ_RESET_NOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL ZQ control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_zqctl1](index.html) module"]
pub struct DDRCTRL_ZQCTL1_SPEC;
impl crate::RegisterSpec for DDRCTRL_ZQCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_zqctl1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ZQCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_zqctl1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ZQCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_ZQCTL1 to value 0x0200_0100"]
impl crate::Resettable for DDRCTRL_ZQCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0100
    }
}
