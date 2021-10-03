#[doc = "Register `HWCFGR5` reader"]
pub struct R(crate::R<HWCFGR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR5` writer"]
pub struct W(crate::W<HWCFGR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR5_SPEC>;
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
impl From<crate::W<HWCFGR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPUEVENT` reader - HW configuration CPU event generation"]
pub struct CPUEVENT_R(crate::FieldReader<u32, u32>);
impl CPUEVENT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CPUEVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPUEVENT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPUEVENT` writer - HW configuration CPU event generation"]
pub struct CPUEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUEVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW configuration CPU event generation"]
    #[inline(always)]
    pub fn cpuevent(&mut self) -> CPUEVENT_W {
        CPUEVENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr5](index.html) module"]
pub struct HWCFGR5_SPEC;
impl crate::RegisterSpec for HWCFGR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr5::R](R) reader structure"]
impl crate::Readable for HWCFGR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr5::W](W) writer structure"]
impl crate::Writable for HWCFGR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWCFGR5 to value 0xfeaf_ffff"]
impl crate::Resettable for HWCFGR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfeaf_ffff
    }
}
