#[doc = "Register `SFR` reader"]
pub struct R(crate::R<SFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFR` writer"]
pub struct W(crate::W<SFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFR_SPEC>;
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
impl From<crate::W<SFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFSA` reader - Secure flash start address"]
pub struct SFSA_R(crate::FieldReader<u8, u8>);
impl SFSA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SFSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFSA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFSA` writer - Secure flash start address"]
pub struct SFSA_W<'a> {
    w: &'a mut W,
}
impl<'a> SFSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DDS` reader - Disable Cortex M0 debug access"]
pub struct DDS_R(crate::FieldReader<bool, bool>);
impl DDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDS` writer - Disable Cortex M0 debug access"]
pub struct DDS_W<'a> {
    w: &'a mut W,
}
impl<'a> DDS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FSD` reader - Flash security disable"]
pub struct FSD_R(crate::FieldReader<bool, bool>);
impl FSD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSD` writer - Flash security disable"]
pub struct FSD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Secure flash start address"]
    #[inline(always)]
    pub fn sfsa(&self) -> SFSA_R {
        SFSA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Disable Cortex M0 debug access"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash security disable"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secure flash start address"]
    #[inline(always)]
    pub fn sfsa(&mut self) -> SFSA_W {
        SFSA_W { w: self }
    }
    #[doc = "Bit 12 - Disable Cortex M0 debug access"]
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W {
        DDS_W { w: self }
    }
    #[doc = "Bit 8 - Flash security disable"]
    #[inline(always)]
    pub fn fsd(&mut self) -> FSD_W {
        FSD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure flash start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](index.html) module"]
pub struct SFR_SPEC;
impl crate::RegisterSpec for SFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfr::R](R) reader structure"]
impl crate::Readable for SFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfr::W](W) writer structure"]
impl crate::Writable for SFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFR to value 0xffff_ee00"]
impl crate::Resettable for SFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ee00
    }
}
