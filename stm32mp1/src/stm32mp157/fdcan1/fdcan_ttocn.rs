#[doc = "Register `FDCAN_TTOCN` reader"]
pub struct R(crate::R<FDCAN_TTOCN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTOCN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTOCN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTOCN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTOCN` writer"]
pub struct W(crate::W<FDCAN_TTOCN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTOCN_SPEC>;
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
impl From<crate::W<FDCAN_TTOCN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTOCN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGT` reader - SGT"]
pub struct SGT_R(crate::FieldReader<bool, bool>);
impl SGT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SGT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SGT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SGT` writer - SGT"]
pub struct SGT_W<'a> {
    w: &'a mut W,
}
impl<'a> SGT_W<'a> {
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
#[doc = "Field `ECS` reader - ECS"]
pub struct ECS_R(crate::FieldReader<bool, bool>);
impl ECS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECS` writer - ECS"]
pub struct ECS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECS_W<'a> {
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
#[doc = "Field `SWP` reader - SWP"]
pub struct SWP_R(crate::FieldReader<bool, bool>);
impl SWP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWP` writer - SWP"]
pub struct SWP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWP_W<'a> {
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
#[doc = "Field `SWS` reader - SWS"]
pub struct SWS_R(crate::FieldReader<u8, u8>);
impl SWS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWS` writer - SWS"]
pub struct SWS_W<'a> {
    w: &'a mut W,
}
impl<'a> SWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `RTIE` reader - RTIE"]
pub struct RTIE_R(crate::FieldReader<bool, bool>);
impl RTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTIE` writer - RTIE"]
pub struct RTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIE_W<'a> {
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
#[doc = "Field `TMC` reader - TMC"]
pub struct TMC_R(crate::FieldReader<u8, u8>);
impl TMC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMC` writer - TMC"]
pub struct TMC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `TTIE` reader - TTIE"]
pub struct TTIE_R(crate::FieldReader<bool, bool>);
impl TTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTIE` writer - TTIE"]
pub struct TTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TTIE_W<'a> {
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
#[doc = "Field `GCS` reader - GCS"]
pub struct GCS_R(crate::FieldReader<bool, bool>);
impl GCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCS` writer - GCS"]
pub struct GCS_W<'a> {
    w: &'a mut W,
}
impl<'a> GCS_W<'a> {
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
#[doc = "Field `FGP` reader - FGP"]
pub struct FGP_R(crate::FieldReader<bool, bool>);
impl FGP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FGP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FGP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FGP` writer - FGP"]
pub struct FGP_W<'a> {
    w: &'a mut W,
}
impl<'a> FGP_W<'a> {
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
#[doc = "Field `TMG` reader - TMG"]
pub struct TMG_R(crate::FieldReader<bool, bool>);
impl TMG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMG` writer - TMG"]
pub struct TMG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `NIG` reader - NIG"]
pub struct NIG_R(crate::FieldReader<bool, bool>);
impl NIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        NIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIG` writer - NIG"]
pub struct NIG_W<'a> {
    w: &'a mut W,
}
impl<'a> NIG_W<'a> {
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
#[doc = "Field `ESCN` reader - ESCN"]
pub struct ESCN_R(crate::FieldReader<bool, bool>);
impl ESCN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESCN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESCN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESCN` writer - ESCN"]
pub struct ESCN_W<'a> {
    w: &'a mut W,
}
impl<'a> ESCN_W<'a> {
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
#[doc = "Field `LCKC` reader - LCKC"]
pub struct LCKC_R(crate::FieldReader<bool, bool>);
impl LCKC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SGT"]
    #[inline(always)]
    pub fn sgt(&self) -> SGT_R {
        SGT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ECS"]
    #[inline(always)]
    pub fn ecs(&self) -> ECS_R {
        ECS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWP"]
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - SWS"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - RTIE"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - TMC"]
    #[inline(always)]
    pub fn tmc(&self) -> TMC_R {
        TMC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - TTIE"]
    #[inline(always)]
    pub fn ttie(&self) -> TTIE_R {
        TTIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GCS"]
    #[inline(always)]
    pub fn gcs(&self) -> GCS_R {
        GCS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FGP"]
    #[inline(always)]
    pub fn fgp(&self) -> FGP_R {
        FGP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TMG"]
    #[inline(always)]
    pub fn tmg(&self) -> TMG_R {
        TMG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - NIG"]
    #[inline(always)]
    pub fn nig(&self) -> NIG_R {
        NIG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ESCN"]
    #[inline(always)]
    pub fn escn(&self) -> ESCN_R {
        ESCN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCKC"]
    #[inline(always)]
    pub fn lckc(&self) -> LCKC_R {
        LCKC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SGT"]
    #[inline(always)]
    pub fn sgt(&mut self) -> SGT_W {
        SGT_W { w: self }
    }
    #[doc = "Bit 1 - ECS"]
    #[inline(always)]
    pub fn ecs(&mut self) -> ECS_W {
        ECS_W { w: self }
    }
    #[doc = "Bit 2 - SWP"]
    #[inline(always)]
    pub fn swp(&mut self) -> SWP_W {
        SWP_W { w: self }
    }
    #[doc = "Bits 3:4 - SWS"]
    #[inline(always)]
    pub fn sws(&mut self) -> SWS_W {
        SWS_W { w: self }
    }
    #[doc = "Bit 5 - RTIE"]
    #[inline(always)]
    pub fn rtie(&mut self) -> RTIE_W {
        RTIE_W { w: self }
    }
    #[doc = "Bits 6:7 - TMC"]
    #[inline(always)]
    pub fn tmc(&mut self) -> TMC_W {
        TMC_W { w: self }
    }
    #[doc = "Bit 8 - TTIE"]
    #[inline(always)]
    pub fn ttie(&mut self) -> TTIE_W {
        TTIE_W { w: self }
    }
    #[doc = "Bit 9 - GCS"]
    #[inline(always)]
    pub fn gcs(&mut self) -> GCS_W {
        GCS_W { w: self }
    }
    #[doc = "Bit 10 - FGP"]
    #[inline(always)]
    pub fn fgp(&mut self) -> FGP_W {
        FGP_W { w: self }
    }
    #[doc = "Bit 11 - TMG"]
    #[inline(always)]
    pub fn tmg(&mut self) -> TMG_W {
        TMG_W { w: self }
    }
    #[doc = "Bit 12 - NIG"]
    #[inline(always)]
    pub fn nig(&mut self) -> NIG_W {
        NIG_W { w: self }
    }
    #[doc = "Bit 13 - ESCN"]
    #[inline(always)]
    pub fn escn(&mut self) -> ESCN_W {
        ESCN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT operation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttocn](index.html) module"]
pub struct FDCAN_TTOCN_SPEC;
impl crate::RegisterSpec for FDCAN_TTOCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttocn::R](R) reader structure"]
impl crate::Readable for FDCAN_TTOCN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttocn::W](W) writer structure"]
impl crate::Writable for FDCAN_TTOCN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TTOCN to value 0"]
impl crate::Resettable for FDCAN_TTOCN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
