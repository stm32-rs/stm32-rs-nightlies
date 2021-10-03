#[doc = "Register `HWCFGR3` reader"]
pub struct R(crate::R<HWCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR3` writer"]
pub struct W(crate::W<HWCFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR3_SPEC>;
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
impl From<crate::W<HWCFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENT_TRG` reader - HW configuration event trigger type"]
pub struct EVENT_TRG_R(crate::FieldReader<u32, u32>);
impl EVENT_TRG_R {
    pub(crate) fn new(bits: u32) -> Self {
        EVENT_TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_TRG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENT_TRG` writer - HW configuration event trigger type"]
pub struct EVENT_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline(always)]
    pub fn event_trg(&mut self) -> EVENT_TRG_W {
        EVENT_TRG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr3](index.html) module"]
pub struct HWCFGR3_SPEC;
impl crate::RegisterSpec for HWCFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr3::R](R) reader structure"]
impl crate::Readable for HWCFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr3::W](W) writer structure"]
impl crate::Writable for HWCFGR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWCFGR3 to value 0"]
impl crate::Resettable for HWCFGR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
