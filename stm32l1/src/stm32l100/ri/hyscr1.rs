#[doc = "Register `HYSCR1` reader"]
pub struct R(crate::R<HYSCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HYSCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HYSCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HYSCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HYSCR1` writer"]
pub struct W(crate::W<HYSCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HYSCR1_SPEC>;
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
impl From<crate::W<HYSCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HYSCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PB` reader - Port B hysteresis control on/off"]
pub struct PB_R(crate::FieldReader<u16, u16>);
impl PB_R {
    pub(crate) fn new(bits: u16) -> Self {
        PB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB` writer - Port B hysteresis control on/off"]
pub struct PB_W<'a> {
    w: &'a mut W,
}
impl<'a> PB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `PA` reader - Port A hysteresis control on/off"]
pub struct PA_R(crate::FieldReader<u16, u16>);
impl PA_R {
    pub(crate) fn new(bits: u16) -> Self {
        PA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA` writer - Port A hysteresis control on/off"]
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Port B hysteresis control on/off"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Port A hysteresis control on/off"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Port B hysteresis control on/off"]
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W {
        PB_W { w: self }
    }
    #[doc = "Bits 0:15 - Port A hysteresis control on/off"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RI hysteresis control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hyscr1](index.html) module"]
pub struct HYSCR1_SPEC;
impl crate::RegisterSpec for HYSCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hyscr1::R](R) reader structure"]
impl crate::Readable for HYSCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hyscr1::W](W) writer structure"]
impl crate::Writable for HYSCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HYSCR1 to value 0"]
impl crate::Resettable for HYSCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
