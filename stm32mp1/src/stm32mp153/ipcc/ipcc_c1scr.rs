#[doc = "Register `IPCC_C1SCR` reader"]
pub struct R(crate::R<IPCC_C1SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCC_C1SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCC_C1SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCC_C1SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCC_C1SCR` writer"]
pub struct W(crate::W<IPCC_C1SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCC_C1SCR_SPEC>;
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
impl From<crate::W<IPCC_C1SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCC_C1SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHxC` reader - CHxC"]
pub struct CHXC_R(crate::FieldReader<u8, u8>);
impl CHXC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHXC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHXC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHxC` writer - CHxC"]
pub struct CHXC_W<'a> {
    w: &'a mut W,
}
impl<'a> CHXC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `CHxS` reader - CHxS"]
pub struct CHXS_R(crate::FieldReader<u8, u8>);
impl CHXS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHXS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHXS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHxS` writer - CHxS"]
pub struct CHXS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHXS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - CHxC"]
    #[inline(always)]
    pub fn chx_c(&self) -> CHXC_R {
        CHXC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - CHxS"]
    #[inline(always)]
    pub fn chx_s(&self) -> CHXS_R {
        CHXS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CHxC"]
    #[inline(always)]
    pub fn chx_c(&mut self) -> CHXC_W {
        CHXC_W { w: self }
    }
    #[doc = "Bits 16:21 - CHxS"]
    #[inline(always)]
    pub fn chx_s(&mut self) -> CHXS_W {
        CHXS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reading this register will always return 0x0000 0000.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c1scr](index.html) module"]
pub struct IPCC_C1SCR_SPEC;
impl crate::RegisterSpec for IPCC_C1SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcc_c1scr::R](R) reader structure"]
impl crate::Readable for IPCC_C1SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcc_c1scr::W](W) writer structure"]
impl crate::Writable for IPCC_C1SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPCC_C1SCR to value 0"]
impl crate::Resettable for IPCC_C1SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
