#[doc = "Register `RG%sCR` reader"]
pub type R = crate::R<RGCRrs>;
#[doc = "Register `RG%sCR` writer"]
pub type W = crate::W<RGCRrs>;
#[doc = "DMA request trigger input selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIG_ID {
    #[doc = "0: Signal `dmamux1_evt0` selected as trigger input"]
    Dmamux1Evt0 = 0,
    #[doc = "1: Signal `dmamux1_evt1` selected as trigger input"]
    Dmamux1Evt1 = 1,
    #[doc = "2: Signal `dmamux1_evt2` selected as trigger input"]
    Dmamux1Evt2 = 2,
    #[doc = "3: Signal `lptim1_out` selected as trigger input"]
    Lptim1Out = 3,
    #[doc = "4: Signal `lptim2_out` selected as trigger input"]
    Lptim2Out = 4,
    #[doc = "5: Signal `lptim3_out` selected as trigger input"]
    Lptim3Out = 5,
    #[doc = "6: Signal `extit0` selected as trigger input"]
    Extit0 = 6,
    #[doc = "7: Signal `tim12_trgo` selected as trigger input"]
    Tim12Trgo = 7,
}
impl From<SIG_ID> for u8 {
    #[inline(always)]
    fn from(variant: SIG_ID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SIG_ID {
    type Ux = u8;
}
#[doc = "Field `SIG_ID` reader - DMA request trigger input selected"]
pub type SIG_ID_R = crate::FieldReader<SIG_ID>;
impl SIG_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SIG_ID> {
        match self.bits {
            0 => Some(SIG_ID::Dmamux1Evt0),
            1 => Some(SIG_ID::Dmamux1Evt1),
            2 => Some(SIG_ID::Dmamux1Evt2),
            3 => Some(SIG_ID::Lptim1Out),
            4 => Some(SIG_ID::Lptim2Out),
            5 => Some(SIG_ID::Lptim3Out),
            6 => Some(SIG_ID::Extit0),
            7 => Some(SIG_ID::Tim12Trgo),
            _ => None,
        }
    }
    #[doc = "Signal `dmamux1_evt0` selected as trigger input"]
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        *self == SIG_ID::Dmamux1Evt0
    }
    #[doc = "Signal `dmamux1_evt1` selected as trigger input"]
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        *self == SIG_ID::Dmamux1Evt1
    }
    #[doc = "Signal `dmamux1_evt2` selected as trigger input"]
    #[inline(always)]
    pub fn is_dmamux1_evt2(&self) -> bool {
        *self == SIG_ID::Dmamux1Evt2
    }
    #[doc = "Signal `lptim1_out` selected as trigger input"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == SIG_ID::Lptim1Out
    }
    #[doc = "Signal `lptim2_out` selected as trigger input"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SIG_ID::Lptim2Out
    }
    #[doc = "Signal `lptim3_out` selected as trigger input"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SIG_ID::Lptim3Out
    }
    #[doc = "Signal `extit0` selected as trigger input"]
    #[inline(always)]
    pub fn is_extit0(&self) -> bool {
        *self == SIG_ID::Extit0
    }
    #[doc = "Signal `tim12_trgo` selected as trigger input"]
    #[inline(always)]
    pub fn is_tim12_trgo(&self) -> bool {
        *self == SIG_ID::Tim12Trgo
    }
}
#[doc = "Field `SIG_ID` writer - DMA request trigger input selected"]
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SIG_ID>;
impl<'a, REG> SIG_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal `dmamux1_evt0` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux1Evt0)
    }
    #[doc = "Signal `dmamux1_evt1` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux1Evt1)
    }
    #[doc = "Signal `dmamux1_evt2` selected as trigger input"]
    #[inline(always)]
    pub fn dmamux1_evt2(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux1Evt2)
    }
    #[doc = "Signal `lptim1_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim1Out)
    }
    #[doc = "Signal `lptim2_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim2Out)
    }
    #[doc = "Signal `lptim3_out` selected as trigger input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim3Out)
    }
    #[doc = "Signal `extit0` selected as trigger input"]
    #[inline(always)]
    pub fn extit0(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Extit0)
    }
    #[doc = "Signal `tim12_trgo` selected as trigger input"]
    #[inline(always)]
    pub fn tim12_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Tim12Trgo)
    }
}
#[doc = "Interrupt enable at trigger event overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIE {
    #[doc = "0: Trigger overrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Trigger overrun interrupt enabled"]
    Enabled = 1,
}
impl From<OIE> for bool {
    #[inline(always)]
    fn from(variant: OIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIE` reader - Interrupt enable at trigger event overrun"]
pub type OIE_R = crate::BitReader<OIE>;
impl OIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OIE {
        match self.bits {
            false => OIE::Disabled,
            true => OIE::Enabled,
        }
    }
    #[doc = "Trigger overrun interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OIE::Disabled
    }
    #[doc = "Trigger overrun interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OIE::Enabled
    }
}
#[doc = "Field `OIE` writer - Interrupt enable at trigger event overrun"]
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG, OIE>;
impl<'a, REG> OIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::Disabled)
    }
    #[doc = "Trigger overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::Enabled)
    }
}
#[doc = "DMA request generator channel enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GE {
    #[doc = "0: DMA request generation disabled"]
    Disabled = 0,
    #[doc = "1: DMA request enabled"]
    Enabled = 1,
}
impl From<GE> for bool {
    #[inline(always)]
    fn from(variant: GE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GE` reader - DMA request generator channel enable/disable"]
pub type GE_R = crate::BitReader<GE>;
impl GE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GE {
        match self.bits {
            false => GE::Disabled,
            true => GE::Enabled,
        }
    }
    #[doc = "DMA request generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GE::Disabled
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GE::Enabled
    }
}
#[doc = "Field `GE` writer - DMA request generator channel enable/disable"]
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG, GE>;
impl<'a, REG> GE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GE::Disabled)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GE::Enabled)
    }
}
#[doc = "DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOL {
    #[doc = "0: No event, i.e. no detection nor generation"]
    NoEdge = 0,
    #[doc = "1: Rising edge"]
    RisingEdge = 1,
    #[doc = "2: Falling edge"]
    FallingEdge = 2,
    #[doc = "3: Rising and falling edges"]
    BothEdges = 3,
}
impl From<GPOL> for u8 {
    #[inline(always)]
    fn from(variant: GPOL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPOL {
    type Ux = u8;
}
#[doc = "Field `GPOL` reader - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GPOL_R = crate::FieldReader<GPOL>;
impl GPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPOL {
        match self.bits {
            0 => GPOL::NoEdge,
            1 => GPOL::RisingEdge,
            2 => GPOL::FallingEdge,
            3 => GPOL::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "No event, i.e. no detection nor generation"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == GPOL::NoEdge
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == GPOL::RisingEdge
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == GPOL::FallingEdge
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == GPOL::BothEdges
    }
}
#[doc = "Field `GPOL` writer - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GPOL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, GPOL>;
impl<'a, REG> GPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, i.e. no detection nor generation"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::NoEdge)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::RisingEdge)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::FallingEdge)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::BothEdges)
    }
}
#[doc = "Field `GNBREQ` reader - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GNBREQ_R = crate::FieldReader;
#[doc = "Field `GNBREQ` writer - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GNBREQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
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
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<RGCRrs> {
        SIG_ID_W::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<RGCRrs> {
        OIE_W::new(self, 8)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<RGCRrs> {
        GE_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<RGCRrs> {
        GPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<RGCRrs> {
        GNBREQ_W::new(self, 19)
    }
}
#[doc = "DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGCRrs;
impl crate::RegisterSpec for RGCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgcr::R`](R) reader structure"]
impl crate::Readable for RGCRrs {}
#[doc = "`write(|w| ..)` method takes [`rgcr::W`](W) writer structure"]
impl crate::Writable for RGCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RG%sCR to value 0"]
impl crate::Resettable for RGCRrs {
    const RESET_VALUE: u32 = 0;
}
