#[doc = "Register `OTG_FS_DOEPCTL0` reader"]
pub struct R(crate::R<OTG_FS_DOEPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_DOEPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_DOEPCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_DOEPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_DOEPCTL0` writer"]
pub struct W(crate::W<OTG_FS_DOEPCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_DOEPCTL0_SPEC>;
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
impl From<crate::W<OTG_FS_DOEPCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_DOEPCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPENA` writer - EPENA"]
pub struct EPENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EPENA_W<'a> {
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
#[doc = "Field `EPDIS` reader - EPDIS"]
pub struct EPDIS_R(crate::FieldReader<bool, bool>);
impl EPDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNAK` writer - SNAK"]
pub struct SNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `CNAK` writer - CNAK"]
pub struct CNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `Stall` reader - Stall"]
pub struct STALL_R(crate::FieldReader<bool, bool>);
impl STALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Stall` writer - Stall"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SNPM` reader - SNPM"]
pub struct SNPM_R(crate::FieldReader<bool, bool>);
impl SNPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SNPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNPM` writer - SNPM"]
pub struct SNPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SNPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `EPTYP` reader - EPTYP"]
pub struct EPTYP_R(crate::FieldReader<u8, u8>);
impl EPTYP_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPTYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTYP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKSTS` reader - NAKSTS"]
pub struct NAKSTS_R(crate::FieldReader<bool, bool>);
impl NAKSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBAEP` reader - USBAEP"]
pub struct USBAEP_R(crate::FieldReader<bool, bool>);
impl USBAEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBAEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBAEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPSIZ` reader - MPSIZ"]
pub struct MPSIZ_R(crate::FieldReader<u8, u8>);
impl MPSIZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        MPSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPSIZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 30 - EPDIS"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Stall"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SNPM"]
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - EPTYP"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - NAKSTS"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USBAEP"]
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - EPENA"]
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W {
        EPENA_W { w: self }
    }
    #[doc = "Bit 27 - SNAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W {
        SNAK_W { w: self }
    }
    #[doc = "Bit 26 - CNAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W {
        CNAK_W { w: self }
    }
    #[doc = "Bit 21 - Stall"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 20 - SNPM"]
    #[inline(always)]
    pub fn snpm(&mut self) -> SNPM_W {
        SNPM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device endpoint-0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_doepctl0](index.html) module"]
pub struct OTG_FS_DOEPCTL0_SPEC;
impl crate::RegisterSpec for OTG_FS_DOEPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_doepctl0::R](R) reader structure"]
impl crate::Readable for OTG_FS_DOEPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_doepctl0::W](W) writer structure"]
impl crate::Writable for OTG_FS_DOEPCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_DOEPCTL0 to value 0x8000"]
impl crate::Resettable for OTG_FS_DOEPCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
