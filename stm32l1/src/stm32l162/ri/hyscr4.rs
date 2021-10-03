#[doc = "Register `HYSCR4` reader"]
pub struct R(crate::R<HYSCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HYSCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HYSCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HYSCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HYSCR4` writer"]
pub struct W(crate::W<HYSCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HYSCR4_SPEC>;
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
impl From<crate::W<HYSCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HYSCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PG` reader - Port G hysteresis control on/off"]
pub struct PG_R(crate::FieldReader<u16, u16>);
impl PG_R {
    pub(crate) fn new(bits: u16) -> Self {
        PG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG` writer - Port G hysteresis control on/off"]
pub struct PG_W<'a> {
    w: &'a mut W,
}
impl<'a> PG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port G hysteresis control on/off"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port G hysteresis control on/off"]
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hysteresis control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hyscr4](index.html) module"]
pub struct HYSCR4_SPEC;
impl crate::RegisterSpec for HYSCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hyscr4::R](R) reader structure"]
impl crate::Readable for HYSCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hyscr4::W](W) writer structure"]
impl crate::Writable for HYSCR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HYSCR4 to value 0"]
impl crate::Resettable for HYSCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
