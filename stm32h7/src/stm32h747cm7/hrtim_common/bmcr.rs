#[doc = "Register `BMCR` reader"]
pub struct R(crate::R<BMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMCR` writer"]
pub struct W(crate::W<BMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMCR_SPEC>;
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
impl From<crate::W<BMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMSTAT` reader - Burst Mode Status"]
pub struct BMSTAT_R(crate::FieldReader<bool, bool>);
impl BMSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMSTAT` writer - Burst Mode Status"]
pub struct BMSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BMSTAT_W<'a> {
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
#[doc = "Field `TEBM` reader - Timer E Burst Mode"]
pub struct TEBM_R(crate::FieldReader<bool, bool>);
impl TEBM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEBM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEBM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEBM` writer - Timer E Burst Mode"]
pub struct TEBM_W<'a> {
    w: &'a mut W,
}
impl<'a> TEBM_W<'a> {
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
#[doc = "Field `TDBM` reader - Timer D Burst Mode"]
pub struct TDBM_R(crate::FieldReader<bool, bool>);
impl TDBM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDBM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDBM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDBM` writer - Timer D Burst Mode"]
pub struct TDBM_W<'a> {
    w: &'a mut W,
}
impl<'a> TDBM_W<'a> {
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
#[doc = "Field `TCBM` reader - Timer C Burst Mode"]
pub struct TCBM_R(crate::FieldReader<bool, bool>);
impl TCBM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCBM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCBM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCBM` writer - Timer C Burst Mode"]
pub struct TCBM_W<'a> {
    w: &'a mut W,
}
impl<'a> TCBM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `TBBM` reader - Timer B Burst Mode"]
pub struct TBBM_R(crate::FieldReader<bool, bool>);
impl TBBM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBBM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBBM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBBM` writer - Timer B Burst Mode"]
pub struct TBBM_W<'a> {
    w: &'a mut W,
}
impl<'a> TBBM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `TABM` reader - Timer A Burst Mode"]
pub struct TABM_R(crate::FieldReader<bool, bool>);
impl TABM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TABM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TABM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TABM` writer - Timer A Burst Mode"]
pub struct TABM_W<'a> {
    w: &'a mut W,
}
impl<'a> TABM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `MTBM` reader - Master Timer Burst Mode"]
pub struct MTBM_R(crate::FieldReader<bool, bool>);
impl MTBM_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTBM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTBM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTBM` writer - Master Timer Burst Mode"]
pub struct MTBM_W<'a> {
    w: &'a mut W,
}
impl<'a> MTBM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `BMPREN` reader - Burst Mode Preload Enable"]
pub struct BMPREN_R(crate::FieldReader<bool, bool>);
impl BMPREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMPREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMPREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMPREN` writer - Burst Mode Preload Enable"]
pub struct BMPREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BMPREN_W<'a> {
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
#[doc = "Field `BMPRSC` reader - Burst Mode Prescaler"]
pub struct BMPRSC_R(crate::FieldReader<u8, u8>);
impl BMPRSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        BMPRSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMPRSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMPRSC` writer - Burst Mode Prescaler"]
pub struct BMPRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> BMPRSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | ((value as u32 & 0x0f) << 6);
        self.w
    }
}
#[doc = "Field `BMCLK` reader - Burst Mode Clock source"]
pub struct BMCLK_R(crate::FieldReader<u8, u8>);
impl BMCLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        BMCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMCLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMCLK` writer - Burst Mode Clock source"]
pub struct BMCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> BMCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `BMOM` reader - Burst Mode operating mode"]
pub struct BMOM_R(crate::FieldReader<bool, bool>);
impl BMOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMOM` writer - Burst Mode operating mode"]
pub struct BMOM_W<'a> {
    w: &'a mut W,
}
impl<'a> BMOM_W<'a> {
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
#[doc = "Field `BME` reader - Burst Mode enable"]
pub struct BME_R(crate::FieldReader<bool, bool>);
impl BME_R {
    pub(crate) fn new(bits: bool) -> Self {
        BME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BME` writer - Burst Mode enable"]
pub struct BME_W<'a> {
    w: &'a mut W,
}
impl<'a> BME_W<'a> {
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
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    pub fn bmstat(&self) -> BMSTAT_R {
        BMSTAT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    pub fn tebm(&self) -> TEBM_R {
        TEBM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    pub fn tdbm(&self) -> TDBM_R {
        TDBM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    pub fn tcbm(&self) -> TCBM_R {
        TCBM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    pub fn tbbm(&self) -> TBBM_R {
        TBBM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    pub fn tabm(&self) -> TABM_R {
        TABM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    pub fn mtbm(&self) -> MTBM_R {
        MTBM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    pub fn bmpren(&self) -> BMPREN_R {
        BMPREN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    pub fn bmprsc(&self) -> BMPRSC_R {
        BMPRSC_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    pub fn bmclk(&self) -> BMCLK_R {
        BMCLK_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    pub fn bmom(&self) -> BMOM_R {
        BMOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    pub fn bme(&self) -> BME_R {
        BME_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    pub fn bmstat(&mut self) -> BMSTAT_W {
        BMSTAT_W { w: self }
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    pub fn tebm(&mut self) -> TEBM_W {
        TEBM_W { w: self }
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    pub fn tdbm(&mut self) -> TDBM_W {
        TDBM_W { w: self }
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    pub fn tcbm(&mut self) -> TCBM_W {
        TCBM_W { w: self }
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    pub fn tbbm(&mut self) -> TBBM_W {
        TBBM_W { w: self }
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    pub fn tabm(&mut self) -> TABM_W {
        TABM_W { w: self }
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    pub fn mtbm(&mut self) -> MTBM_W {
        MTBM_W { w: self }
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    pub fn bmpren(&mut self) -> BMPREN_W {
        BMPREN_W { w: self }
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    pub fn bmprsc(&mut self) -> BMPRSC_W {
        BMPRSC_W { w: self }
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    pub fn bmclk(&mut self) -> BMCLK_W {
        BMCLK_W { w: self }
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    pub fn bmom(&mut self) -> BMOM_W {
        BMOM_W { w: self }
    }
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    pub fn bme(&mut self) -> BME_W {
        BME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmcr](index.html) module"]
pub struct BMCR_SPEC;
impl crate::RegisterSpec for BMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmcr::R](R) reader structure"]
impl crate::Readable for BMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmcr::W](W) writer structure"]
impl crate::Writable for BMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMCR to value 0"]
impl crate::Resettable for BMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
