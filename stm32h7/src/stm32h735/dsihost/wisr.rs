#[doc = "Register `WISR` reader"]
pub struct R(crate::R<WISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WISR` writer"]
pub struct W(crate::W<WISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WISR_SPEC>;
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
impl From<crate::W<WISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRIF` reader - Regulator ready interrupt flag"]
pub struct RRIF_R(crate::FieldReader<bool, bool>);
impl RRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRIF` writer - Regulator ready interrupt flag"]
pub struct RRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RRIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `RRS` reader - Regulator ready status"]
pub struct RRS_R(crate::FieldReader<bool, bool>);
impl RRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRS` writer - Regulator ready status"]
pub struct RRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRS_W<'a> {
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
#[doc = "Field `PLLUIF` reader - PLL unlock interrupt flag"]
pub struct PLLUIF_R(crate::FieldReader<bool, bool>);
impl PLLUIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLUIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLUIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLUIF` writer - PLL unlock interrupt flag"]
pub struct PLLUIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLUIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PLLLIF` reader - PLL lock interrupt flag"]
pub struct PLLLIF_R(crate::FieldReader<bool, bool>);
impl PLLLIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLLIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLLIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLLIF` writer - PLL lock interrupt flag"]
pub struct PLLLIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLLIF_W<'a> {
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
#[doc = "Field `PLLLS` reader - PLL lock status"]
pub struct PLLLS_R(crate::FieldReader<bool, bool>);
impl PLLLS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLLS` writer - PLL lock status"]
pub struct PLLLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLLS_W<'a> {
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
#[doc = "Field `BUSY` reader - Busy flag"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` writer - Busy flag"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
#[doc = "Field `ERIF` reader - End of refresh interrupt flag"]
pub struct ERIF_R(crate::FieldReader<bool, bool>);
impl ERIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERIF` writer - End of refresh interrupt flag"]
pub struct ERIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ERIF_W<'a> {
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
#[doc = "Field `TEIF` reader - Tearing effect interrupt flag"]
pub struct TEIF_R(crate::FieldReader<bool, bool>);
impl TEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF` writer - Tearing effect interrupt flag"]
pub struct TEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIF_W<'a> {
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
impl R {
    #[doc = "Bit 13 - Regulator ready interrupt flag"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Regulator ready status"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PLL unlock interrupt flag"]
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLL lock interrupt flag"]
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL lock status"]
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Busy flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of refresh interrupt flag"]
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Tearing effect interrupt flag"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Regulator ready interrupt flag"]
    #[inline(always)]
    pub fn rrif(&mut self) -> RRIF_W {
        RRIF_W { w: self }
    }
    #[doc = "Bit 12 - Regulator ready status"]
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W {
        RRS_W { w: self }
    }
    #[doc = "Bit 10 - PLL unlock interrupt flag"]
    #[inline(always)]
    pub fn plluif(&mut self) -> PLLUIF_W {
        PLLUIF_W { w: self }
    }
    #[doc = "Bit 9 - PLL lock interrupt flag"]
    #[inline(always)]
    pub fn plllif(&mut self) -> PLLLIF_W {
        PLLLIF_W { w: self }
    }
    #[doc = "Bit 8 - PLL lock status"]
    #[inline(always)]
    pub fn pllls(&mut self) -> PLLLS_W {
        PLLLS_W { w: self }
    }
    #[doc = "Bit 2 - Busy flag"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 1 - End of refresh interrupt flag"]
    #[inline(always)]
    pub fn erif(&mut self) -> ERIF_W {
        ERIF_W { w: self }
    }
    #[doc = "Bit 0 - Tearing effect interrupt flag"]
    #[inline(always)]
    pub fn teif(&mut self) -> TEIF_W {
        TEIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI wrapper interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wisr](index.html) module"]
pub struct WISR_SPEC;
impl crate::RegisterSpec for WISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wisr::R](R) reader structure"]
impl crate::Readable for WISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wisr::W](W) writer structure"]
impl crate::Writable for WISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WISR to value 0"]
impl crate::Resettable for WISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
