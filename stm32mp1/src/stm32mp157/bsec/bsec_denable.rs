#[doc = "Register `BSEC_DENABLE` reader"]
pub struct R(crate::R<BSEC_DENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_DENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_DENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_DENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_DENABLE` writer"]
pub struct W(crate::W<BSEC_DENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_DENABLE_SPEC>;
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
impl From<crate::W<BSEC_DENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_DENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFTEN` reader - DFTEN"]
pub struct DFTEN_R(crate::FieldReader<bool, bool>);
impl DFTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFTEN` writer - DFTEN"]
pub struct DFTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFTEN_W<'a> {
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
#[doc = "Field `DBGEN` reader - DBGEN"]
pub struct DBGEN_R(crate::FieldReader<bool, bool>);
impl DBGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGEN` writer - DBGEN"]
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
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
#[doc = "Field `NIDEN` reader - NIDEN"]
pub struct NIDEN_R(crate::FieldReader<bool, bool>);
impl NIDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        NIDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NIDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIDEN` writer - NIDEN"]
pub struct NIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NIDEN_W<'a> {
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
#[doc = "Field `DEVICEEN` reader - DEVICEEN"]
pub struct DEVICEEN_R(crate::FieldReader<bool, bool>);
impl DEVICEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEVICEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVICEEN` writer - DEVICEEN"]
pub struct DEVICEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICEEN_W<'a> {
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
#[doc = "Field `HDPEN` reader - HDPEN"]
pub struct HDPEN_R(crate::FieldReader<bool, bool>);
impl HDPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDPEN` writer - HDPEN"]
pub struct HDPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPEN_W<'a> {
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
#[doc = "Field `SPIDEN` reader - SPIDEN"]
pub struct SPIDEN_R(crate::FieldReader<bool, bool>);
impl SPIDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIDEN` writer - SPIDEN"]
pub struct SPIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIDEN_W<'a> {
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
#[doc = "Field `SPNIDEN` reader - SPNIDEN"]
pub struct SPNIDEN_R(crate::FieldReader<bool, bool>);
impl SPNIDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPNIDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPNIDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPNIDEN` writer - SPNIDEN"]
pub struct SPNIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPNIDEN_W<'a> {
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
#[doc = "Field `CP15SDISABLE` reader - CP15SDISABLE"]
pub struct CP15SDISABLE_R(crate::FieldReader<u8, u8>);
impl CP15SDISABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CP15SDISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CP15SDISABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CP15SDISABLE` writer - CP15SDISABLE"]
pub struct CP15SDISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CP15SDISABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `CFGSDISABLE` reader - CFGSDISABLE"]
pub struct CFGSDISABLE_R(crate::FieldReader<bool, bool>);
impl CFGSDISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGSDISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFGSDISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGSDISABLE` writer - CFGSDISABLE"]
pub struct CFGSDISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGSDISABLE_W<'a> {
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
#[doc = "Field `DBGSWENABLE` reader - DBGSWENABLE"]
pub struct DBGSWENABLE_R(crate::FieldReader<bool, bool>);
impl DBGSWENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSWENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSWENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSWENABLE` writer - DBGSWENABLE"]
pub struct DBGSWENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSWENABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DFTEN"]
    #[inline(always)]
    pub fn dften(&self) -> DFTEN_R {
        DFTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DBGEN"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NIDEN"]
    #[inline(always)]
    pub fn niden(&self) -> NIDEN_R {
        NIDEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DEVICEEN"]
    #[inline(always)]
    pub fn deviceen(&self) -> DEVICEEN_R {
        DEVICEEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPIDEN"]
    #[inline(always)]
    pub fn spiden(&self) -> SPIDEN_R {
        SPIDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPNIDEN"]
    #[inline(always)]
    pub fn spniden(&self) -> SPNIDEN_R {
        SPNIDEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - CP15SDISABLE"]
    #[inline(always)]
    pub fn cp15sdisable(&self) -> CP15SDISABLE_R {
        CP15SDISABLE_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - CFGSDISABLE"]
    #[inline(always)]
    pub fn cfgsdisable(&self) -> CFGSDISABLE_R {
        CFGSDISABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DBGSWENABLE"]
    #[inline(always)]
    pub fn dbgswenable(&self) -> DBGSWENABLE_R {
        DBGSWENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFTEN"]
    #[inline(always)]
    pub fn dften(&mut self) -> DFTEN_W {
        DFTEN_W { w: self }
    }
    #[doc = "Bit 1 - DBGEN"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
    #[doc = "Bit 2 - NIDEN"]
    #[inline(always)]
    pub fn niden(&mut self) -> NIDEN_W {
        NIDEN_W { w: self }
    }
    #[doc = "Bit 3 - DEVICEEN"]
    #[inline(always)]
    pub fn deviceen(&mut self) -> DEVICEEN_W {
        DEVICEEN_W { w: self }
    }
    #[doc = "Bit 4 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&mut self) -> HDPEN_W {
        HDPEN_W { w: self }
    }
    #[doc = "Bit 5 - SPIDEN"]
    #[inline(always)]
    pub fn spiden(&mut self) -> SPIDEN_W {
        SPIDEN_W { w: self }
    }
    #[doc = "Bit 6 - SPNIDEN"]
    #[inline(always)]
    pub fn spniden(&mut self) -> SPNIDEN_W {
        SPNIDEN_W { w: self }
    }
    #[doc = "Bits 7:8 - CP15SDISABLE"]
    #[inline(always)]
    pub fn cp15sdisable(&mut self) -> CP15SDISABLE_W {
        CP15SDISABLE_W { w: self }
    }
    #[doc = "Bit 9 - CFGSDISABLE"]
    #[inline(always)]
    pub fn cfgsdisable(&mut self) -> CFGSDISABLE_W {
        CFGSDISABLE_W { w: self }
    }
    #[doc = "Bit 10 - DBGSWENABLE"]
    #[inline(always)]
    pub fn dbgswenable(&mut self) -> DBGSWENABLE_W {
        DBGSWENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_denable](index.html) module"]
pub struct BSEC_DENABLE_SPEC;
impl crate::RegisterSpec for BSEC_DENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_denable::R](R) reader structure"]
impl crate::Readable for BSEC_DENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_denable::W](W) writer structure"]
impl crate::Writable for BSEC_DENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSEC_DENABLE to value 0"]
impl crate::Resettable for BSEC_DENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
