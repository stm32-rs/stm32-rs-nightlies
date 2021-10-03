#[doc = "Register `TAMP_SMCR` reader"]
pub struct R(crate::R<TAMP_SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_SMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_SMCR` writer"]
pub struct W(crate::W<TAMP_SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_SMCR_SPEC>;
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
impl From<crate::W<TAMP_SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_SMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKPRWDPROT` reader - BKPRWDPROT"]
pub struct BKPRWDPROT_R(crate::FieldReader<u8, u8>);
impl BKPRWDPROT_R {
    pub(crate) fn new(bits: u8) -> Self {
        BKPRWDPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKPRWDPROT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKPRWDPROT` writer - BKPRWDPROT"]
pub struct BKPRWDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPRWDPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `BKPWDPROT` reader - BKPWDPROT"]
pub struct BKPWDPROT_R(crate::FieldReader<u8, u8>);
impl BKPWDPROT_R {
    pub(crate) fn new(bits: u8) -> Self {
        BKPWDPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKPWDPROT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKPWDPROT` writer - BKPWDPROT"]
pub struct BKPWDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPWDPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `TAMPDPROT` reader - TAMPDPROT"]
pub struct TAMPDPROT_R(crate::FieldReader<bool, bool>);
impl TAMPDPROT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPDPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPDPROT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPDPROT` writer - TAMPDPROT"]
pub struct TAMPDPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPDPROT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - BKPRWDPROT"]
    #[inline(always)]
    pub fn bkprwdprot(&self) -> BKPRWDPROT_R {
        BKPRWDPROT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - BKPWDPROT"]
    #[inline(always)]
    pub fn bkpwdprot(&self) -> BKPWDPROT_R {
        BKPWDPROT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - TAMPDPROT"]
    #[inline(always)]
    pub fn tampdprot(&self) -> TAMPDPROT_R {
        TAMPDPROT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - BKPRWDPROT"]
    #[inline(always)]
    pub fn bkprwdprot(&mut self) -> BKPRWDPROT_W {
        BKPRWDPROT_W { w: self }
    }
    #[doc = "Bits 16:23 - BKPWDPROT"]
    #[inline(always)]
    pub fn bkpwdprot(&mut self) -> BKPWDPROT_W {
        BKPWDPROT_W { w: self }
    }
    #[doc = "Bit 31 - TAMPDPROT"]
    #[inline(always)]
    pub fn tampdprot(&mut self) -> TAMPDPROT_W {
        TAMPDPROT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be written only when the APB access is secure.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_smcr](index.html) module"]
pub struct TAMP_SMCR_SPEC;
impl crate::RegisterSpec for TAMP_SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_smcr::R](R) reader structure"]
impl crate::Readable for TAMP_SMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_smcr::W](W) writer structure"]
impl crate::Writable for TAMP_SMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMP_SMCR to value 0x8000_0000"]
impl crate::Resettable for TAMP_SMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
