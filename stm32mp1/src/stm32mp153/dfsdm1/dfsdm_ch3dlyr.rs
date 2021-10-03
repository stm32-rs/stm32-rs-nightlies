#[doc = "Register `DFSDM_CH3DLYR` reader"]
pub struct R(crate::R<DFSDM_CH3DLYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_CH3DLYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_CH3DLYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_CH3DLYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_CH3DLYR` writer"]
pub struct W(crate::W<DFSDM_CH3DLYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_CH3DLYR_SPEC>;
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
impl From<crate::W<DFSDM_CH3DLYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_CH3DLYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLSSKP` reader - PLSSKP"]
pub struct PLSSKP_R(crate::FieldReader<u8, u8>);
impl PLSSKP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLSSKP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLSSKP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLSSKP` writer - PLSSKP"]
pub struct PLSSKP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLSSKP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    pub fn plsskp(&mut self) -> PLSSKP_W {
        PLSSKP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM channel 3 delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch3dlyr](index.html) module"]
pub struct DFSDM_CH3DLYR_SPEC;
impl crate::RegisterSpec for DFSDM_CH3DLYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_ch3dlyr::R](R) reader structure"]
impl crate::Readable for DFSDM_CH3DLYR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch3dlyr::W](W) writer structure"]
impl crate::Writable for DFSDM_CH3DLYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFSDM_CH3DLYR to value 0"]
impl crate::Resettable for DFSDM_CH3DLYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
