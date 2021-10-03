#[doc = "Register `RTC_DR` reader"]
pub struct R(crate::R<RTC_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_DR` writer"]
pub struct W(crate::W<RTC_DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_DR_SPEC>;
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
impl From<crate::W<RTC_DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DU` reader - DU"]
pub struct DU_R(crate::FieldReader<u8, u8>);
impl DU_R {
    pub(crate) fn new(bits: u8) -> Self {
        DU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DU` writer - DU"]
pub struct DU_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `DT` reader - DT"]
pub struct DT_R(crate::FieldReader<u8, u8>);
impl DT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT` writer - DT"]
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `MU` reader - MU"]
pub struct MU_R(crate::FieldReader<u8, u8>);
impl MU_R {
    pub(crate) fn new(bits: u8) -> Self {
        MU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MU` writer - MU"]
pub struct MU_W<'a> {
    w: &'a mut W,
}
impl<'a> MU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `MT` reader - MT"]
pub struct MT_R(crate::FieldReader<bool, bool>);
impl MT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MT` writer - MT"]
pub struct MT_W<'a> {
    w: &'a mut W,
}
impl<'a> MT_W<'a> {
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
#[doc = "Field `WDU` reader - WDU"]
pub struct WDU_R(crate::FieldReader<u8, u8>);
impl WDU_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDU` writer - WDU"]
pub struct WDU_W<'a> {
    w: &'a mut W,
}
impl<'a> WDU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `YU` reader - YU"]
pub struct YU_R(crate::FieldReader<u8, u8>);
impl YU_R {
    pub(crate) fn new(bits: u8) -> Self {
        YU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YU` writer - YU"]
pub struct YU_W<'a> {
    w: &'a mut W,
}
impl<'a> YU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `YT` reader - YT"]
pub struct YT_R(crate::FieldReader<u8, u8>);
impl YT_R {
    pub(crate) fn new(bits: u8) -> Self {
        YT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YT` writer - YT"]
pub struct YT_W<'a> {
    w: &'a mut W,
}
impl<'a> YT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DU"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - DT"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - MU"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MT"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - WDU"]
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - YU"]
    #[inline(always)]
    pub fn yu(&self) -> YU_R {
        YU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - YT"]
    #[inline(always)]
    pub fn yt(&self) -> YT_R {
        YT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DU"]
    #[inline(always)]
    pub fn du(&mut self) -> DU_W {
        DU_W { w: self }
    }
    #[doc = "Bits 4:5 - DT"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    #[doc = "Bits 8:11 - MU"]
    #[inline(always)]
    pub fn mu(&mut self) -> MU_W {
        MU_W { w: self }
    }
    #[doc = "Bit 12 - MT"]
    #[inline(always)]
    pub fn mt(&mut self) -> MT_W {
        MT_W { w: self }
    }
    #[doc = "Bits 13:15 - WDU"]
    #[inline(always)]
    pub fn wdu(&mut self) -> WDU_W {
        WDU_W { w: self }
    }
    #[doc = "Bits 16:19 - YU"]
    #[inline(always)]
    pub fn yu(&mut self) -> YU_W {
        YU_W { w: self }
    }
    #[doc = "Bits 20:23 - YT"]
    #[inline(always)]
    pub fn yt(&mut self) -> YT_W {
        YT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_dr](index.html) module"]
pub struct RTC_DR_SPEC;
impl crate::RegisterSpec for RTC_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_dr::R](R) reader structure"]
impl crate::Readable for RTC_DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_dr::W](W) writer structure"]
impl crate::Writable for RTC_DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_DR to value 0x2101"]
impl crate::Resettable for RTC_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2101
    }
}
