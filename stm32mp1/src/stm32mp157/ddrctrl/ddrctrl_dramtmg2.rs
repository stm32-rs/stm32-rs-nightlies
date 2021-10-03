#[doc = "Register `DDRCTRL_DRAMTMG2` reader"]
pub struct R(crate::R<DDRCTRL_DRAMTMG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DRAMTMG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DRAMTMG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DRAMTMG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DRAMTMG2` writer"]
pub struct W(crate::W<DDRCTRL_DRAMTMG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DRAMTMG2_SPEC>;
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
impl From<crate::W<DDRCTRL_DRAMTMG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DRAMTMG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR2RD` reader - WR2RD"]
pub struct WR2RD_R(crate::FieldReader<u8, u8>);
impl WR2RD_R {
    pub(crate) fn new(bits: u8) -> Self {
        WR2RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR2RD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR2RD` writer - WR2RD"]
pub struct WR2RD_W<'a> {
    w: &'a mut W,
}
impl<'a> WR2RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `RD2WR` reader - RD2WR"]
pub struct RD2WR_R(crate::FieldReader<u8, u8>);
impl RD2WR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RD2WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD2WR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD2WR` writer - RD2WR"]
pub struct RD2WR_W<'a> {
    w: &'a mut W,
}
impl<'a> RD2WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `READ_LATENCY` reader - READ_LATENCY"]
pub struct READ_LATENCY_R(crate::FieldReader<u8, u8>);
impl READ_LATENCY_R {
    pub(crate) fn new(bits: u8) -> Self {
        READ_LATENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_LATENCY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_LATENCY` writer - READ_LATENCY"]
pub struct READ_LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_LATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `WRITE_LATENCY` reader - WRITE_LATENCY"]
pub struct WRITE_LATENCY_R(crate::FieldReader<u8, u8>);
impl WRITE_LATENCY_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRITE_LATENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITE_LATENCY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITE_LATENCY` writer - WRITE_LATENCY"]
pub struct WRITE_LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_LATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - WR2RD"]
    #[inline(always)]
    pub fn wr2rd(&self) -> WR2RD_R {
        WR2RD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - RD2WR"]
    #[inline(always)]
    pub fn rd2wr(&self) -> RD2WR_R {
        RD2WR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - READ_LATENCY"]
    #[inline(always)]
    pub fn read_latency(&self) -> READ_LATENCY_R {
        READ_LATENCY_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - WRITE_LATENCY"]
    #[inline(always)]
    pub fn write_latency(&self) -> WRITE_LATENCY_R {
        WRITE_LATENCY_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - WR2RD"]
    #[inline(always)]
    pub fn wr2rd(&mut self) -> WR2RD_W {
        WR2RD_W { w: self }
    }
    #[doc = "Bits 8:13 - RD2WR"]
    #[inline(always)]
    pub fn rd2wr(&mut self) -> RD2WR_W {
        RD2WR_W { w: self }
    }
    #[doc = "Bits 16:21 - READ_LATENCY"]
    #[inline(always)]
    pub fn read_latency(&mut self) -> READ_LATENCY_W {
        READ_LATENCY_W { w: self }
    }
    #[doc = "Bits 24:29 - WRITE_LATENCY"]
    #[inline(always)]
    pub fn write_latency(&mut self) -> WRITE_LATENCY_W {
        WRITE_LATENCY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dramtmg2](index.html) module"]
pub struct DDRCTRL_DRAMTMG2_SPEC;
impl crate::RegisterSpec for DDRCTRL_DRAMTMG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dramtmg2::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DRAMTMG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dramtmg2::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DRAMTMG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DRAMTMG2 to value 0x0305_060d"]
impl crate::Resettable for DDRCTRL_DRAMTMG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0305_060d
    }
}
