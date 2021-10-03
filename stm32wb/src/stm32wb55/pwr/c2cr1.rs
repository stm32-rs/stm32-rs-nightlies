#[doc = "Register `C2CR1` reader"]
pub struct R(crate::R<C2CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2CR1` writer"]
pub struct W(crate::W<C2CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CR1_SPEC>;
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
impl From<crate::W<C2CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `_802EWKUP` reader - 802.15.4 external wakeup signal"]
pub struct _802EWKUP_R(crate::FieldReader<bool, bool>);
impl _802EWKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        _802EWKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for _802EWKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `_802EWKUP` writer - 802.15.4 external wakeup signal"]
pub struct _802EWKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> _802EWKUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `BLEEWKUP` reader - BLE external wakeup signal"]
pub struct BLEEWKUP_R(crate::FieldReader<bool, bool>);
impl BLEEWKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEEWKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEEWKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEEWKUP` writer - BLE external wakeup signal"]
pub struct BLEEWKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEEWKUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `FPDS` reader - Flash power down mode during LPSleep for CPU2"]
pub struct FPDS_R(crate::FieldReader<bool, bool>);
impl FPDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPDS` writer - Flash power down mode during LPSleep for CPU2"]
pub struct FPDS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDS_W<'a> {
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
#[doc = "Field `FPDR` reader - Flash power down mode during LPRun for CPU2"]
pub struct FPDR_R(crate::FieldReader<bool, bool>);
impl FPDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPDR` writer - Flash power down mode during LPRun for CPU2"]
pub struct FPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDR_W<'a> {
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
#[doc = "Field `LPMS` reader - Low-power mode selection for CPU2"]
pub struct LPMS_R(crate::FieldReader<u8, u8>);
impl LPMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMS` writer - Low-power mode selection for CPU2"]
pub struct LPMS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - 802.15.4 external wakeup signal"]
    #[inline(always)]
    pub fn _802ewkup(&self) -> _802EWKUP_R {
        _802EWKUP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - BLE external wakeup signal"]
    #[inline(always)]
    pub fn bleewkup(&self) -> BLEEWKUP_R {
        BLEEWKUP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flash power down mode during LPSleep for CPU2"]
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash power down mode during LPRun for CPU2"]
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - 802.15.4 external wakeup signal"]
    #[inline(always)]
    pub fn _802ewkup(&mut self) -> _802EWKUP_W {
        _802EWKUP_W { w: self }
    }
    #[doc = "Bit 14 - BLE external wakeup signal"]
    #[inline(always)]
    pub fn bleewkup(&mut self) -> BLEEWKUP_W {
        BLEEWKUP_W { w: self }
    }
    #[doc = "Bit 5 - Flash power down mode during LPSleep for CPU2"]
    #[inline(always)]
    pub fn fpds(&mut self) -> FPDS_W {
        FPDS_W { w: self }
    }
    #[doc = "Bit 4 - Flash power down mode during LPRun for CPU2"]
    #[inline(always)]
    pub fn fpdr(&mut self) -> FPDR_W {
        FPDR_W { w: self }
    }
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W {
        LPMS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 Power control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr1](index.html) module"]
pub struct C2CR1_SPEC;
impl crate::RegisterSpec for C2CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2cr1::R](R) reader structure"]
impl crate::Readable for C2CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2cr1::W](W) writer structure"]
impl crate::Writable for C2CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2CR1 to value 0"]
impl crate::Resettable for C2CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
