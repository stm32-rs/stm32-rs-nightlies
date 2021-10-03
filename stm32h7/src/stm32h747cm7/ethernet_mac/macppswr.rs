#[doc = "Register `MACPPSWR` reader"]
pub struct R(crate::R<MACPPSWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPPSWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPPSWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPPSWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACPPSWR` writer"]
pub struct W(crate::W<MACPPSWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPPSWR_SPEC>;
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
impl From<crate::W<MACPPSWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPPSWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSWIDTH0` reader - PPS Output Signal Width"]
pub struct PPSWIDTH0_R(crate::FieldReader<u32, u32>);
impl PPSWIDTH0_R {
    pub(crate) fn new(bits: u32) -> Self {
        PPSWIDTH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPSWIDTH0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPSWIDTH0` writer - PPS Output Signal Width"]
pub struct PPSWIDTH0_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSWIDTH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PPS Output Signal Width"]
    #[inline(always)]
    pub fn ppswidth0(&self) -> PPSWIDTH0_R {
        PPSWIDTH0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS Output Signal Width"]
    #[inline(always)]
    pub fn ppswidth0(&mut self) -> PPSWIDTH0_W {
        PPSWIDTH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS width register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macppswr](index.html) module"]
pub struct MACPPSWR_SPEC;
impl crate::RegisterSpec for MACPPSWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macppswr::R](R) reader structure"]
impl crate::Readable for MACPPSWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macppswr::W](W) writer structure"]
impl crate::Writable for MACPPSWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACPPSWR to value 0"]
impl crate::Resettable for MACPPSWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
