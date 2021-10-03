#[doc = "Register `CRYP_CSGCMCCM0R` reader"]
pub struct R(crate::R<CRYP_CSGCMCCM0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_CSGCMCCM0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_CSGCMCCM0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_CSGCMCCM0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYP_CSGCMCCM0R` writer"]
pub struct W(crate::W<CRYP_CSGCMCCM0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_CSGCMCCM0R_SPEC>;
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
impl From<crate::W<CRYP_CSGCMCCM0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_CSGCMCCM0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCMCCM0` reader - CSGCMCCM0"]
pub struct CSGCMCCM0_R(crate::FieldReader<u32, u32>);
impl CSGCMCCM0_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSGCMCCM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSGCMCCM0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSGCMCCM0` writer - CSGCMCCM0"]
pub struct CSGCMCCM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM0"]
    #[inline(always)]
    pub fn csgcmccm0(&self) -> CSGCMCCM0_R {
        CSGCMCCM0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM0"]
    #[inline(always)]
    pub fn csgcmccm0(&mut self) -> CSGCMCCM0_W {
        CSGCMCCM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcmccm0r](index.html) module"]
pub struct CRYP_CSGCMCCM0R_SPEC;
impl crate::RegisterSpec for CRYP_CSGCMCCM0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_csgcmccm0r::R](R) reader structure"]
impl crate::Readable for CRYP_CSGCMCCM0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryp_csgcmccm0r::W](W) writer structure"]
impl crate::Writable for CRYP_CSGCMCCM0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYP_CSGCMCCM0R to value 0"]
impl crate::Resettable for CRYP_CSGCMCCM0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
