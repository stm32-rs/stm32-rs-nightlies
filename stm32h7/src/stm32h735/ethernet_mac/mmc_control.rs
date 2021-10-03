#[doc = "Register `MMC_CONTROL` reader"]
pub struct R(crate::R<MMC_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_CONTROL` writer"]
pub struct W(crate::W<MMC_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_CONTROL_SPEC>;
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
impl From<crate::W<MMC_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTRST` reader - Counters Reset"]
pub struct CNTRST_R(crate::FieldReader<bool, bool>);
impl CNTRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTRST` writer - Counters Reset"]
pub struct CNTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTRST_W<'a> {
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
#[doc = "Field `CNTSTOPRO` reader - Counter Stop Rollover"]
pub struct CNTSTOPRO_R(crate::FieldReader<bool, bool>);
impl CNTSTOPRO_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTSTOPRO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTSTOPRO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTSTOPRO` writer - Counter Stop Rollover"]
pub struct CNTSTOPRO_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSTOPRO_W<'a> {
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
#[doc = "Field `RSTONRD` reader - Reset on Read"]
pub struct RSTONRD_R(crate::FieldReader<bool, bool>);
impl RSTONRD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTONRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTONRD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTONRD` writer - Reset on Read"]
pub struct RSTONRD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTONRD_W<'a> {
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
#[doc = "Field `CNTFREEZ` reader - MMC Counter Freeze"]
pub struct CNTFREEZ_R(crate::FieldReader<bool, bool>);
impl CNTFREEZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTFREEZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTFREEZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTFREEZ` writer - MMC Counter Freeze"]
pub struct CNTFREEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTFREEZ_W<'a> {
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
#[doc = "Field `CNTPRST` reader - Counters Preset"]
pub struct CNTPRST_R(crate::FieldReader<bool, bool>);
impl CNTPRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTPRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTPRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTPRST` writer - Counters Preset"]
pub struct CNTPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTPRST_W<'a> {
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
#[doc = "Field `CNTPRSTLVL` reader - Full-Half Preset"]
pub struct CNTPRSTLVL_R(crate::FieldReader<bool, bool>);
impl CNTPRSTLVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTPRSTLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTPRSTLVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTPRSTLVL` writer - Full-Half Preset"]
pub struct CNTPRSTLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTPRSTLVL_W<'a> {
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
#[doc = "Field `UCDBC` reader - Update MMC Counters for Dropped Broadcast Packets"]
pub struct UCDBC_R(crate::FieldReader<bool, bool>);
impl UCDBC_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCDBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCDBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCDBC` writer - Update MMC Counters for Dropped Broadcast Packets"]
pub struct UCDBC_W<'a> {
    w: &'a mut W,
}
impl<'a> UCDBC_W<'a> {
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
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn cntrst(&self) -> CNTRST_R {
        CNTRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter Stop Rollover"]
    #[inline(always)]
    pub fn cntstopro(&self) -> CNTSTOPRO_R {
        CNTSTOPRO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn rstonrd(&self) -> RSTONRD_R {
        RSTONRD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn cntfreez(&self) -> CNTFREEZ_R {
        CNTFREEZ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Full-Half Preset"]
    #[inline(always)]
    pub fn cntprstlvl(&self) -> CNTPRSTLVL_R {
        CNTPRSTLVL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Packets"]
    #[inline(always)]
    pub fn ucdbc(&self) -> UCDBC_R {
        UCDBC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn cntrst(&mut self) -> CNTRST_W {
        CNTRST_W { w: self }
    }
    #[doc = "Bit 1 - Counter Stop Rollover"]
    #[inline(always)]
    pub fn cntstopro(&mut self) -> CNTSTOPRO_W {
        CNTSTOPRO_W { w: self }
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn rstonrd(&mut self) -> RSTONRD_W {
        RSTONRD_W { w: self }
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn cntfreez(&mut self) -> CNTFREEZ_W {
        CNTFREEZ_W { w: self }
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&mut self) -> CNTPRST_W {
        CNTPRST_W { w: self }
    }
    #[doc = "Bit 5 - Full-Half Preset"]
    #[inline(always)]
    pub fn cntprstlvl(&mut self) -> CNTPRSTLVL_W {
        CNTPRSTLVL_W { w: self }
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Packets"]
    #[inline(always)]
    pub fn ucdbc(&mut self) -> UCDBC_W {
        UCDBC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_control](index.html) module"]
pub struct MMC_CONTROL_SPEC;
impl crate::RegisterSpec for MMC_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_control::R](R) reader structure"]
impl crate::Readable for MMC_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_control::W](W) writer structure"]
impl crate::Writable for MMC_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_CONTROL to value 0"]
impl crate::Resettable for MMC_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
