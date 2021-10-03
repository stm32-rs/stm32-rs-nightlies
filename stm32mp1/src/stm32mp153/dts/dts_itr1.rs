#[doc = "Register `DTS_ITR1` reader"]
pub struct R(crate::R<DTS_ITR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_ITR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_ITR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_ITR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTS_ITR1` writer"]
pub struct W(crate::W<DTS_ITR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTS_ITR1_SPEC>;
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
impl From<crate::W<DTS_ITR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTS_ITR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS1_LITTHD` reader - TS1_LITTHD"]
pub struct TS1_LITTHD_R(crate::FieldReader<u16, u16>);
impl TS1_LITTHD_R {
    pub(crate) fn new(bits: u16) -> Self {
        TS1_LITTHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_LITTHD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_LITTHD` writer - TS1_LITTHD"]
pub struct TS1_LITTHD_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_LITTHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `TS1_HITTHD` reader - TS1_HITTHD"]
pub struct TS1_HITTHD_R(crate::FieldReader<u16, u16>);
impl TS1_HITTHD_R {
    pub(crate) fn new(bits: u16) -> Self {
        TS1_HITTHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_HITTHD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_HITTHD` writer - TS1_HITTHD"]
pub struct TS1_HITTHD_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_HITTHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TS1_LITTHD"]
    #[inline(always)]
    pub fn ts1_litthd(&self) -> TS1_LITTHD_R {
        TS1_LITTHD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TS1_HITTHD"]
    #[inline(always)]
    pub fn ts1_hitthd(&self) -> TS1_HITTHD_R {
        TS1_HITTHD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TS1_LITTHD"]
    #[inline(always)]
    pub fn ts1_litthd(&mut self) -> TS1_LITTHD_W {
        TS1_LITTHD_W { w: self }
    }
    #[doc = "Bits 16:31 - TS1_HITTHD"]
    #[inline(always)]
    pub fn ts1_hitthd(&mut self) -> TS1_HITTHD_W {
        TS1_HITTHD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTS_ITR1 contains the threshold values for sensor 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_itr1](index.html) module"]
pub struct DTS_ITR1_SPEC;
impl crate::RegisterSpec for DTS_ITR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_itr1::R](R) reader structure"]
impl crate::Readable for DTS_ITR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dts_itr1::W](W) writer structure"]
impl crate::Writable for DTS_ITR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTS_ITR1 to value 0"]
impl crate::Resettable for DTS_ITR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
