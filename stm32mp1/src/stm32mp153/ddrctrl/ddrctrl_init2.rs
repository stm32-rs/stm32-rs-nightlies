#[doc = "Register `DDRCTRL_INIT2` reader"]
pub struct R(crate::R<DDRCTRL_INIT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_INIT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_INIT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_INIT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_INIT2` writer"]
pub struct W(crate::W<DDRCTRL_INIT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_INIT2_SPEC>;
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
impl From<crate::W<DDRCTRL_INIT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_INIT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIN_STABLE_CLOCK_X1` reader - MIN_STABLE_CLOCK_X1"]
pub struct MIN_STABLE_CLOCK_X1_R(crate::FieldReader<u8, u8>);
impl MIN_STABLE_CLOCK_X1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MIN_STABLE_CLOCK_X1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_STABLE_CLOCK_X1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN_STABLE_CLOCK_X1` writer - MIN_STABLE_CLOCK_X1"]
pub struct MIN_STABLE_CLOCK_X1_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_STABLE_CLOCK_X1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `IDLE_AFTER_RESET_X32` reader - IDLE_AFTER_RESET_X32"]
pub struct IDLE_AFTER_RESET_X32_R(crate::FieldReader<u8, u8>);
impl IDLE_AFTER_RESET_X32_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDLE_AFTER_RESET_X32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_AFTER_RESET_X32_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_AFTER_RESET_X32` writer - IDLE_AFTER_RESET_X32"]
pub struct IDLE_AFTER_RESET_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_AFTER_RESET_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - MIN_STABLE_CLOCK_X1"]
    #[inline(always)]
    pub fn min_stable_clock_x1(&self) -> MIN_STABLE_CLOCK_X1_R {
        MIN_STABLE_CLOCK_X1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - IDLE_AFTER_RESET_X32"]
    #[inline(always)]
    pub fn idle_after_reset_x32(&self) -> IDLE_AFTER_RESET_X32_R {
        IDLE_AFTER_RESET_X32_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - MIN_STABLE_CLOCK_X1"]
    #[inline(always)]
    pub fn min_stable_clock_x1(&mut self) -> MIN_STABLE_CLOCK_X1_W {
        MIN_STABLE_CLOCK_X1_W { w: self }
    }
    #[doc = "Bits 8:15 - IDLE_AFTER_RESET_X32"]
    #[inline(always)]
    pub fn idle_after_reset_x32(&mut self) -> IDLE_AFTER_RESET_X32_W {
        IDLE_AFTER_RESET_X32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM initialization register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init2](index.html) module"]
pub struct DDRCTRL_INIT2_SPEC;
impl crate::RegisterSpec for DDRCTRL_INIT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_init2::R](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init2::W](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_INIT2 to value 0x0d05"]
impl crate::Resettable for DDRCTRL_INIT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d05
    }
}
