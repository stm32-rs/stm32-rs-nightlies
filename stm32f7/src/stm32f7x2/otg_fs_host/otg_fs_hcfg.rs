#[doc = "Register `OTG_FS_HCFG` reader"]
pub struct R(crate::R<OTG_FS_HCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_HCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_HCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_HCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_HCFG` writer"]
pub struct W(crate::W<OTG_FS_HCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_HCFG_SPEC>;
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
impl From<crate::W<OTG_FS_HCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_HCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSLSPCS` reader - FS/LS PHY clock select"]
pub struct FSLSPCS_R(crate::FieldReader<u8, u8>);
impl FSLSPCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSLSPCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSLSPCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSLSPCS` writer - FS/LS PHY clock select"]
pub struct FSLSPCS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLSPCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `FSLSS` reader - FS- and LS-only support"]
pub struct FSLSS_R(crate::FieldReader<bool, bool>);
impl FSLSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSLSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSLSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-only support"]
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(&mut self) -> FSLSPCS_W {
        FSLSPCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcfg](index.html) module"]
pub struct OTG_FS_HCFG_SPEC;
impl crate::RegisterSpec for OTG_FS_HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_hcfg::R](R) reader structure"]
impl crate::Readable for OTG_FS_HCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcfg::W](W) writer structure"]
impl crate::Writable for OTG_FS_HCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_HCFG to value 0"]
impl crate::Resettable for OTG_FS_HCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
