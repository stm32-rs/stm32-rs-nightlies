#[doc = "Register `DAC_SHRR` reader"]
pub struct R(crate::R<DAC_SHRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_SHRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_SHRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_SHRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_SHRR` writer"]
pub struct W(crate::W<DAC_SHRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_SHRR_SPEC>;
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
impl From<crate::W<DAC_SHRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_SHRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TREFRESH1` reader - TREFRESH1"]
pub struct TREFRESH1_R(crate::FieldReader<u8, u8>);
impl TREFRESH1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TREFRESH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TREFRESH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TREFRESH1` writer - TREFRESH1"]
pub struct TREFRESH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TREFRESH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TREFRESH2` reader - TREFRESH2"]
pub struct TREFRESH2_R(crate::FieldReader<u8, u8>);
impl TREFRESH2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TREFRESH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TREFRESH2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TREFRESH2` writer - TREFRESH2"]
pub struct TREFRESH2_W<'a> {
    w: &'a mut W,
}
impl<'a> TREFRESH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TREFRESH1"]
    #[inline(always)]
    pub fn trefresh1(&self) -> TREFRESH1_R {
        TREFRESH1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TREFRESH2"]
    #[inline(always)]
    pub fn trefresh2(&self) -> TREFRESH2_R {
        TREFRESH2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TREFRESH1"]
    #[inline(always)]
    pub fn trefresh1(&mut self) -> TREFRESH1_W {
        TREFRESH1_W { w: self }
    }
    #[doc = "Bits 16:23 - TREFRESH2"]
    #[inline(always)]
    pub fn trefresh2(&mut self) -> TREFRESH2_W {
        TREFRESH2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC sample and hold refresh time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_shrr](index.html) module"]
pub struct DAC_SHRR_SPEC;
impl crate::RegisterSpec for DAC_SHRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_shrr::R](R) reader structure"]
impl crate::Readable for DAC_SHRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_shrr::W](W) writer structure"]
impl crate::Writable for DAC_SHRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_SHRR to value 0x0001_0001"]
impl crate::Resettable for DAC_SHRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0001
    }
}
