#[doc = "Register `SDMMC_IDMALAR` reader"]
pub struct R(crate::R<SDMMC_IDMALAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_IDMALAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_IDMALAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_IDMALAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_IDMALAR` writer"]
pub struct W(crate::W<SDMMC_IDMALAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_IDMALAR_SPEC>;
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
impl From<crate::W<SDMMC_IDMALAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_IDMALAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDMALA` reader - IDMALA"]
pub struct IDMALA_R(crate::FieldReader<u16, u16>);
impl IDMALA_R {
    pub(crate) fn new(bits: u16) -> Self {
        IDMALA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMALA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMALA` writer - IDMALA"]
pub struct IDMALA_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMALA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Field `ABR` reader - ABR"]
pub struct ABR_R(crate::FieldReader<bool, bool>);
impl ABR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABR` writer - ABR"]
pub struct ABR_W<'a> {
    w: &'a mut W,
}
impl<'a> ABR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `ULS` reader - ULS"]
pub struct ULS_R(crate::FieldReader<bool, bool>);
impl ULS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ULS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULS` writer - ULS"]
pub struct ULS_W<'a> {
    w: &'a mut W,
}
impl<'a> ULS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ULA` reader - ULA"]
pub struct ULA_R(crate::FieldReader<bool, bool>);
impl ULA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ULA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULA` writer - ULA"]
pub struct ULA_W<'a> {
    w: &'a mut W,
}
impl<'a> ULA_W<'a> {
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
    #[doc = "Bits 2:15 - IDMALA"]
    #[inline(always)]
    pub fn idmala(&self) -> IDMALA_R {
        IDMALA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 29 - ABR"]
    #[inline(always)]
    pub fn abr(&self) -> ABR_R {
        ABR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ULS"]
    #[inline(always)]
    pub fn uls(&self) -> ULS_R {
        ULS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ULA"]
    #[inline(always)]
    pub fn ula(&self) -> ULA_R {
        ULA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - IDMALA"]
    #[inline(always)]
    pub fn idmala(&mut self) -> IDMALA_W {
        IDMALA_W { w: self }
    }
    #[doc = "Bit 29 - ABR"]
    #[inline(always)]
    pub fn abr(&mut self) -> ABR_W {
        ABR_W { w: self }
    }
    #[doc = "Bit 30 - ULS"]
    #[inline(always)]
    pub fn uls(&mut self) -> ULS_W {
        ULS_W { w: self }
    }
    #[doc = "Bit 31 - ULA"]
    #[inline(always)]
    pub fn ula(&mut self) -> ULA_W {
        ULA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDMMC IDMA linked list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_idmalar](index.html) module"]
pub struct SDMMC_IDMALAR_SPEC;
impl crate::RegisterSpec for SDMMC_IDMALAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_idmalar::R](R) reader structure"]
impl crate::Readable for SDMMC_IDMALAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_idmalar::W](W) writer structure"]
impl crate::Writable for SDMMC_IDMALAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_IDMALAR to value 0"]
impl crate::Resettable for SDMMC_IDMALAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
