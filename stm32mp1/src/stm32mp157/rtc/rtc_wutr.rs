#[doc = "Register `RTC_WUTR` reader"]
pub struct R(crate::R<RTC_WUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_WUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_WUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_WUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_WUTR` writer"]
pub struct W(crate::W<RTC_WUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_WUTR_SPEC>;
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
impl From<crate::W<RTC_WUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_WUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUT` reader - WUT"]
pub struct WUT_R(crate::FieldReader<u16, u16>);
impl WUT_R {
    pub(crate) fn new(bits: u16) -> Self {
        WUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUT` writer - WUT"]
pub struct WUT_W<'a> {
    w: &'a mut W,
}
impl<'a> WUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - WUT"]
    #[inline(always)]
    pub fn wut(&self) -> WUT_R {
        WUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WUT"]
    #[inline(always)]
    pub fn wut(&mut self) -> WUT_W {
        WUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_wutr](index.html) module"]
pub struct RTC_WUTR_SPEC;
impl crate::RegisterSpec for RTC_WUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_wutr::R](R) reader structure"]
impl crate::Readable for RTC_WUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_wutr::W](W) writer structure"]
impl crate::Writable for RTC_WUTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_WUTR to value 0xffff"]
impl crate::Resettable for RTC_WUTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
