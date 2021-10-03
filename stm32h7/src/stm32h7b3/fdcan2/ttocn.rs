#[doc = "Register `TTOCN` reader"]
pub struct R(crate::R<TTOCN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTOCN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTOCN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTOCN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTOCN` writer"]
pub struct W(crate::W<TTOCN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTOCN_SPEC>;
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
impl From<crate::W<TTOCN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTOCN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGT` reader - Set Global time"]
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
#[doc = "Field `SGT` writer - Set Global time"]
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
#[doc = "Field `ECS` reader - External Clock Synchronization"]
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
#[doc = "Field `ECS` writer - External Clock Synchronization"]
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
#[doc = "Field `SWP` reader - Stop Watch Polarity"]
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
#[doc = "Field `SWP` writer - Stop Watch Polarity"]
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
#[doc = "Field `SWS` reader - Stop Watch Source."]
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
#[doc = "Field `SWS` writer - Stop Watch Source."]
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
#[doc = "Field `RTIE` reader - Register Time Mark Interrupt Pulse Enable"]
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
#[doc = "Field `RTIE` writer - Register Time Mark Interrupt Pulse Enable"]
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
#[doc = "Field `TMC` reader - Register Time Mark Compare"]
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
#[doc = "Field `TMC` writer - Register Time Mark Compare"]
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
#[doc = "Field `TTIE` reader - Trigger Time Mark Interrupt Pulse Enable"]
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
#[doc = "Field `TTIE` writer - Trigger Time Mark Interrupt Pulse Enable"]
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
#[doc = "Field `GCS` reader - Gap Control Select"]
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
#[doc = "Field `GCS` writer - Gap Control Select"]
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
#[doc = "Field `FGP` reader - Finish Gap."]
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
#[doc = "Field `FGP` writer - Finish Gap."]
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
#[doc = "Field `TMG` reader - Time Mark Gap"]
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
#[doc = "Field `TMG` writer - Time Mark Gap"]
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
#[doc = "Field `NIG` reader - Next is Gap"]
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
#[doc = "Field `NIG` writer - Next is Gap"]
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
#[doc = "Field `ESCN` reader - External Synchronization Control"]
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
#[doc = "Field `ESCN` writer - External Synchronization Control"]
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
#[doc = "Field `LCKC` reader - TT Operation Control Register Locked"]
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
#[doc = "Field `LCKC` writer - TT Operation Control Register Locked"]
pub struct LCKC_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Set Global time"]
    #[inline(always)]
    pub fn sgt(&self) -> SGT_R {
        SGT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Clock Synchronization"]
    #[inline(always)]
    pub fn ecs(&self) -> ECS_R {
        ECS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop Watch Polarity"]
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Stop Watch Source."]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Register Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Register Time Mark Compare"]
    #[inline(always)]
    pub fn tmc(&self) -> TMC_R {
        TMC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Trigger Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    pub fn ttie(&self) -> TTIE_R {
        TTIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Gap Control Select"]
    #[inline(always)]
    pub fn gcs(&self) -> GCS_R {
        GCS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Finish Gap."]
    #[inline(always)]
    pub fn fgp(&self) -> FGP_R {
        FGP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Time Mark Gap"]
    #[inline(always)]
    pub fn tmg(&self) -> TMG_R {
        TMG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Next is Gap"]
    #[inline(always)]
    pub fn nig(&self) -> NIG_R {
        NIG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Synchronization Control"]
    #[inline(always)]
    pub fn escn(&self) -> ESCN_R {
        ESCN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TT Operation Control Register Locked"]
    #[inline(always)]
    pub fn lckc(&self) -> LCKC_R {
        LCKC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set Global time"]
    #[inline(always)]
    pub fn sgt(&mut self) -> SGT_W {
        SGT_W { w: self }
    }
    #[doc = "Bit 1 - External Clock Synchronization"]
    #[inline(always)]
    pub fn ecs(&mut self) -> ECS_W {
        ECS_W { w: self }
    }
    #[doc = "Bit 2 - Stop Watch Polarity"]
    #[inline(always)]
    pub fn swp(&mut self) -> SWP_W {
        SWP_W { w: self }
    }
    #[doc = "Bits 3:4 - Stop Watch Source."]
    #[inline(always)]
    pub fn sws(&mut self) -> SWS_W {
        SWS_W { w: self }
    }
    #[doc = "Bit 5 - Register Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    pub fn rtie(&mut self) -> RTIE_W {
        RTIE_W { w: self }
    }
    #[doc = "Bits 6:7 - Register Time Mark Compare"]
    #[inline(always)]
    pub fn tmc(&mut self) -> TMC_W {
        TMC_W { w: self }
    }
    #[doc = "Bit 8 - Trigger Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    pub fn ttie(&mut self) -> TTIE_W {
        TTIE_W { w: self }
    }
    #[doc = "Bit 9 - Gap Control Select"]
    #[inline(always)]
    pub fn gcs(&mut self) -> GCS_W {
        GCS_W { w: self }
    }
    #[doc = "Bit 10 - Finish Gap."]
    #[inline(always)]
    pub fn fgp(&mut self) -> FGP_W {
        FGP_W { w: self }
    }
    #[doc = "Bit 11 - Time Mark Gap"]
    #[inline(always)]
    pub fn tmg(&mut self) -> TMG_W {
        TMG_W { w: self }
    }
    #[doc = "Bit 12 - Next is Gap"]
    #[inline(always)]
    pub fn nig(&mut self) -> NIG_W {
        NIG_W { w: self }
    }
    #[doc = "Bit 13 - External Synchronization Control"]
    #[inline(always)]
    pub fn escn(&mut self) -> ESCN_W {
        ESCN_W { w: self }
    }
    #[doc = "Bit 15 - TT Operation Control Register Locked"]
    #[inline(always)]
    pub fn lckc(&mut self) -> LCKC_W {
        LCKC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttocn](index.html) module"]
pub struct TTOCN_SPEC;
impl crate::RegisterSpec for TTOCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttocn::R](R) reader structure"]
impl crate::Readable for TTOCN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttocn::W](W) writer structure"]
impl crate::Writable for TTOCN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TTOCN to value 0"]
impl crate::Resettable for TTOCN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
