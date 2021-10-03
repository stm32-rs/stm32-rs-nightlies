#[doc = "Register `DDRPERFM_CFG` reader"]
pub struct R(crate::R<DDRPERFM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPERFM_CFG` writer"]
pub struct W(crate::W<DDRPERFM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPERFM_CFG_SPEC>;
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
impl From<crate::W<DDRPERFM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPERFM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub struct EN_R(crate::FieldReader<u8, u8>);
impl EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - EN"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `SEL` reader - SEL"]
pub struct SEL_R(crate::FieldReader<u8, u8>);
impl SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - SEL"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - SEL"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 16:17 - SEL"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPERFM configurationl register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_cfg](index.html) module"]
pub struct DDRPERFM_CFG_SPEC;
impl crate::RegisterSpec for DDRPERFM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrperfm_cfg::R](R) reader structure"]
impl crate::Readable for DDRPERFM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrperfm_cfg::W](W) writer structure"]
impl crate::Writable for DDRPERFM_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPERFM_CFG to value 0"]
impl crate::Resettable for DDRPERFM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
