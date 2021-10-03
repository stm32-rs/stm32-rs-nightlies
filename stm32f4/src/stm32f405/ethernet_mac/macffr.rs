#[doc = "Register `MACFFR` reader"]
pub struct R(crate::R<MACFFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACFFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACFFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACFFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACFFR` writer"]
pub struct W(crate::W<MACFFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACFFR_SPEC>;
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
impl From<crate::W<MACFFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACFFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM` reader - PM"]
pub struct PM_R(crate::FieldReader<bool, bool>);
impl PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM` writer - PM"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `HU` reader - HU"]
pub struct HU_R(crate::FieldReader<bool, bool>);
impl HU_R {
    pub(crate) fn new(bits: bool) -> Self {
        HU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HU` writer - HU"]
pub struct HU_W<'a> {
    w: &'a mut W,
}
impl<'a> HU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `HM` reader - HM"]
pub struct HM_R(crate::FieldReader<bool, bool>);
impl HM_R {
    pub(crate) fn new(bits: bool) -> Self {
        HM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HM` writer - HM"]
pub struct HM_W<'a> {
    w: &'a mut W,
}
impl<'a> HM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DAIF` reader - DAIF"]
pub struct DAIF_R(crate::FieldReader<bool, bool>);
impl DAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAIF` writer - DAIF"]
pub struct DAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DAIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RAM` reader - RAM"]
pub struct RAM_R(crate::FieldReader<bool, bool>);
impl RAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM` writer - RAM"]
pub struct RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `BFD` reader - BFD"]
pub struct BFD_R(crate::FieldReader<bool, bool>);
impl BFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BFD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BFD` writer - BFD"]
pub struct BFD_W<'a> {
    w: &'a mut W,
}
impl<'a> BFD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PCF` reader - PCF"]
pub struct PCF_R(crate::FieldReader<bool, bool>);
impl PCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCF` writer - PCF"]
pub struct PCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `SAIF` reader - SAIF"]
pub struct SAIF_R(crate::FieldReader<bool, bool>);
impl SAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAIF` writer - SAIF"]
pub struct SAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SAIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SAF` reader - SAF"]
pub struct SAF_R(crate::FieldReader<bool, bool>);
impl SAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAF` writer - SAF"]
pub struct SAF_W<'a> {
    w: &'a mut W,
}
impl<'a> SAF_W<'a> {
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
#[doc = "Field `HPF` reader - HPF"]
pub struct HPF_R(crate::FieldReader<bool, bool>);
impl HPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPF` writer - HPF"]
pub struct HPF_W<'a> {
    w: &'a mut W,
}
impl<'a> HPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `RA` reader - RA"]
pub struct RA_R(crate::FieldReader<bool, bool>);
impl RA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - RA"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
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
    #[doc = "Bit 0 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HU"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HM"]
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RAM"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BFD"]
    #[inline(always)]
    pub fn bfd(&self) -> BFD_R {
        BFD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PCF"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SAIF"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SAF"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HPF"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PM"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 1 - HU"]
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W {
        HU_W { w: self }
    }
    #[doc = "Bit 2 - HM"]
    #[inline(always)]
    pub fn hm(&mut self) -> HM_W {
        HM_W { w: self }
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W {
        DAIF_W { w: self }
    }
    #[doc = "Bit 4 - RAM"]
    #[inline(always)]
    pub fn ram(&mut self) -> RAM_W {
        RAM_W { w: self }
    }
    #[doc = "Bit 5 - BFD"]
    #[inline(always)]
    pub fn bfd(&mut self) -> BFD_W {
        BFD_W { w: self }
    }
    #[doc = "Bit 6 - PCF"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W {
        PCF_W { w: self }
    }
    #[doc = "Bit 7 - SAIF"]
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W {
        SAIF_W { w: self }
    }
    #[doc = "Bit 8 - SAF"]
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W {
        SAF_W { w: self }
    }
    #[doc = "Bit 9 - HPF"]
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W {
        HPF_W { w: self }
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC frame filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macffr](index.html) module"]
pub struct MACFFR_SPEC;
impl crate::RegisterSpec for MACFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macffr::R](R) reader structure"]
impl crate::Readable for MACFFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macffr::W](W) writer structure"]
impl crate::Writable for MACFFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACFFR to value 0"]
impl crate::Resettable for MACFFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
