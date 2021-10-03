#[doc = "Register `SMCR` reader"]
pub struct R(crate::R<SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCR` writer"]
pub struct W(crate::W<SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCR_SPEC>;
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
impl From<crate::W<SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKPRWDPROT` reader - Backup registers read/write protection offset"]
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
#[doc = "Field `BKPRWDPROT` writer - Backup registers read/write protection offset"]
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
#[doc = "Field `BKPWDPROT` reader - Backup registers write protection offset"]
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
#[doc = "Field `BKPWDPROT` writer - Backup registers write protection offset"]
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
#[doc = "Field `TAMPDPROT` reader - Tamper protection"]
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
#[doc = "Field `TAMPDPROT` writer - Tamper protection"]
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
    #[doc = "Bits 0:7 - Backup registers read/write protection offset"]
    #[inline(always)]
    pub fn bkprwdprot(&self) -> BKPRWDPROT_R {
        BKPRWDPROT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset"]
    #[inline(always)]
    pub fn bkpwdprot(&self) -> BKPWDPROT_R {
        BKPWDPROT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Tamper protection"]
    #[inline(always)]
    pub fn tampdprot(&self) -> TAMPDPROT_R {
        TAMPDPROT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset"]
    #[inline(always)]
    pub fn bkprwdprot(&mut self) -> BKPRWDPROT_W {
        BKPRWDPROT_W { w: self }
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset"]
    #[inline(always)]
    pub fn bkpwdprot(&mut self) -> BKPWDPROT_W {
        BKPWDPROT_W { w: self }
    }
    #[doc = "Bit 31 - Tamper protection"]
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
#[doc = "TAMP secure mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcr](index.html) module"]
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smcr::R](R) reader structure"]
impl crate::Readable for SMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcr::W](W) writer structure"]
impl crate::Writable for SMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
