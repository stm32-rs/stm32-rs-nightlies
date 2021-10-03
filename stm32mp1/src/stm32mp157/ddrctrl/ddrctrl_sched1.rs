#[doc = "Register `DDRCTRL_SCHED1` reader"]
pub struct R(crate::R<DDRCTRL_SCHED1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_SCHED1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_SCHED1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_SCHED1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_SCHED1` writer"]
pub struct W(crate::W<DDRCTRL_SCHED1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_SCHED1_SPEC>;
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
impl From<crate::W<DDRCTRL_SCHED1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_SCHED1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAGECLOSE_TIMER` reader - PAGECLOSE_TIMER"]
pub struct PAGECLOSE_TIMER_R(crate::FieldReader<u8, u8>);
impl PAGECLOSE_TIMER_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAGECLOSE_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGECLOSE_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAGECLOSE_TIMER` writer - PAGECLOSE_TIMER"]
pub struct PAGECLOSE_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGECLOSE_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PAGECLOSE_TIMER"]
    #[inline(always)]
    pub fn pageclose_timer(&self) -> PAGECLOSE_TIMER_R {
        PAGECLOSE_TIMER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAGECLOSE_TIMER"]
    #[inline(always)]
    pub fn pageclose_timer(&mut self) -> PAGECLOSE_TIMER_W {
        PAGECLOSE_TIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL scheduler control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_sched1](index.html) module"]
pub struct DDRCTRL_SCHED1_SPEC;
impl crate::RegisterSpec for DDRCTRL_SCHED1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_sched1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_SCHED1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_sched1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_SCHED1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_SCHED1 to value 0"]
impl crate::Resettable for DDRCTRL_SCHED1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
