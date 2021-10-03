#[doc = "Register `RGCR%s` reader"]
pub struct R(crate::R<RGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGCR%s` writer"]
pub struct W(crate::W<RGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGCR_SPEC>;
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
impl From<crate::W<RGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA request trigger input selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIG_ID_A {
    #[doc = "0: Signal `dmamux1_evt0` selected as trigger input"]
    DMAMUX1_EVT0 = 0,
    #[doc = "1: Signal `dmamux1_evt1` selected as trigger input"]
    DMAMUX1_EVT1 = 1,
    #[doc = "2: Signal `dmamux1_evt2` selected as trigger input"]
    DMAMUX1_EVT2 = 2,
    #[doc = "3: Signal `lptim1_out` selected as trigger input"]
    LPTIM1_OUT = 3,
    #[doc = "4: Signal `lptim2_out` selected as trigger input"]
    LPTIM2_OUT = 4,
    #[doc = "5: Signal `lptim3_out` selected as trigger input"]
    LPTIM3_OUT = 5,
    #[doc = "6: Signal `extit0` selected as trigger input"]
    EXTIT0 = 6,
    #[doc = "7: Signal `tim12_trgo` selected as trigger input"]
    TIM12_TRGO = 7,
}
impl From<SIG_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: SIG_ID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SIG_ID` reader - DMA request trigger input selected"]
pub struct SIG_ID_R(crate::FieldReader<u8, SIG_ID_A>);
impl SIG_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIG_ID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIG_ID_A> {
        match self.bits {
            0 => Some(SIG_ID_A::DMAMUX1_EVT0),
            1 => Some(SIG_ID_A::DMAMUX1_EVT1),
            2 => Some(SIG_ID_A::DMAMUX1_EVT2),
            3 => Some(SIG_ID_A::LPTIM1_OUT),
            4 => Some(SIG_ID_A::LPTIM2_OUT),
            5 => Some(SIG_ID_A::LPTIM3_OUT),
            6 => Some(SIG_ID_A::EXTIT0),
            7 => Some(SIG_ID_A::TIM12_TRGO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_EVT0`"]
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        **self == SIG_ID_A::DMAMUX1_EVT0
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_EVT1`"]
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        **self == SIG_ID_A::DMAMUX1_EVT1
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_EVT2`"]
    #[inline(always)]
    pub fn is_dmamux1_evt2(&self) -> bool {
        **self == SIG_ID_A::DMAMUX1_EVT2
    }
    #[doc = "Checks if the value of the field is `LPTIM1_OUT`"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        **self == SIG_ID_A::LPTIM1_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM2_OUT`"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        **self == SIG_ID_A::LPTIM2_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM3_OUT`"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        **self == SIG_ID_A::LPTIM3_OUT
    }
    #[doc = "Checks if the value of the field is `EXTIT0`"]
    #[inline(always)]
    pub fn is_extit0(&self) -> bool {
        **self == SIG_ID_A::EXTIT0
    }
    #[doc = "Checks if the value of the field is `TIM12_TRGO`"]
    #[inline(always)]
    pub fn is_tim12_trgo(&self) -> bool {
        **self == SIG_ID_A::TIM12_TRGO
    }
}
impl core::ops::Deref for SIG_ID_R {
    type Target = crate::FieldReader<u8, SIG_ID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIG_ID` writer - DMA request trigger input selected"]
pub struct SIG_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIG_ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal `dmamux1_evt0` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX1_EVT0)
    }
    #[doc = "Signal `dmamux1_evt1` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX1_EVT1)
    }
    #[doc = "Signal `dmamux1_evt2` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux1_evt2(self) -> &'a mut W {
        self.variant(SIG_ID_A::DMAMUX1_EVT2)
    }
    #[doc = "Signal `lptim1_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM1_OUT)
    }
    #[doc = "Signal `lptim2_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM2_OUT)
    }
    #[doc = "Signal `lptim3_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::LPTIM3_OUT)
    }
    #[doc = "Signal `extit0` selected as trigger input"]
    #[inline(always)]
    pub fn extit0(self) -> &'a mut W {
        self.variant(SIG_ID_A::EXTIT0)
    }
    #[doc = "Signal `tim12_trgo` selected as trigger input"]
    #[inline(always)]
    pub fn tim12_trgo(self) -> &'a mut W {
        self.variant(SIG_ID_A::TIM12_TRGO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Interrupt enable at trigger event overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIE_A {
    #[doc = "0: Trigger overrun interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger overrun interrupt enabled"]
    ENABLED = 1,
}
impl From<OIE_A> for bool {
    #[inline(always)]
    fn from(variant: OIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIE` reader - Interrupt enable at trigger event overrun"]
pub struct OIE_R(crate::FieldReader<bool, OIE_A>);
impl OIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OIE_A {
        match self.bits {
            false => OIE_A::DISABLED,
            true => OIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OIE_A::ENABLED
    }
}
impl core::ops::Deref for OIE_R {
    type Target = crate::FieldReader<bool, OIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIE` writer - Interrupt enable at trigger event overrun"]
pub struct OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OIE_A::DISABLED)
    }
    #[doc = "Trigger overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OIE_A::ENABLED)
    }
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
#[doc = "DMA request generator channel enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GE_A {
    #[doc = "0: DMA request generation disabled"]
    DISABLED = 0,
    #[doc = "1: DMA request enabled"]
    ENABLED = 1,
}
impl From<GE_A> for bool {
    #[inline(always)]
    fn from(variant: GE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GE` reader - DMA request generator channel enable/disable"]
pub struct GE_R(crate::FieldReader<bool, GE_A>);
impl GE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GE_A {
        match self.bits {
            false => GE_A::DISABLED,
            true => GE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == GE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GE_A::ENABLED
    }
}
impl core::ops::Deref for GE_R {
    type Target = crate::FieldReader<bool, GE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GE` writer - DMA request generator channel enable/disable"]
pub struct GE_W<'a> {
    w: &'a mut W,
}
impl<'a> GE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA request generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GE_A::DISABLED)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GE_A::ENABLED)
    }
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
#[doc = "DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPOL_A {
    #[doc = "0: No event, i.e. no detection nor generation"]
    NOEDGE = 0,
    #[doc = "1: Rising edge"]
    RISINGEDGE = 1,
    #[doc = "2: Falling edge"]
    FALLINGEDGE = 2,
    #[doc = "3: Rising and falling edges"]
    BOTHEDGES = 3,
}
impl From<GPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPOL` reader - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub struct GPOL_R(crate::FieldReader<u8, GPOL_A>);
impl GPOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPOL_A {
        match self.bits {
            0 => GPOL_A::NOEDGE,
            1 => GPOL_A::RISINGEDGE,
            2 => GPOL_A::FALLINGEDGE,
            3 => GPOL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOEDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        **self == GPOL_A::NOEDGE
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == GPOL_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == GPOL_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        **self == GPOL_A::BOTHEDGES
    }
}
impl core::ops::Deref for GPOL_R {
    type Target = crate::FieldReader<u8, GPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPOL` writer - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub struct GPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPOL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No event, i.e. no detection nor generation"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(GPOL_A::NOEDGE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(GPOL_A::RISINGEDGE)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(GPOL_A::FALLINGEDGE)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(GPOL_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `GNBREQ` reader - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub struct GNBREQ_R(crate::FieldReader<u8, u8>);
impl GNBREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        GNBREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GNBREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GNBREQ` writer - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub struct GNBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> GNBREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W {
        SIG_ID_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W {
        OIE_W { w: self }
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W {
        GE_W { w: self }
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W {
        GPOL_W { w: self }
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W {
        GNBREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAMux - DMA request generator channel x control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgcr](index.html) module"]
pub struct RGCR_SPEC;
impl crate::RegisterSpec for RGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgcr::R](R) reader structure"]
impl crate::Readable for RGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgcr::W](W) writer structure"]
impl crate::Writable for RGCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RGCR%s to value 0"]
impl crate::Resettable for RGCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
