#[doc = "Register `RGCR%s` reader"]
pub type R = crate::R<RGCRrs>;
#[doc = "Register `RGCR%s` writer"]
pub type W = crate::W<RGCRrs>;
#[doc = "Signal identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIG_ID {
    #[doc = "0: Signal `EXTIx` selected as synchronization input"]
    Exti0 = 0,
    #[doc = "1: Signal `EXTIx` selected as synchronization input"]
    Exti1 = 1,
    #[doc = "2: Signal `EXTIx` selected as synchronization input"]
    Exti2 = 2,
    #[doc = "3: Signal `EXTIx` selected as synchronization input"]
    Exti3 = 3,
    #[doc = "4: Signal `EXTIx` selected as synchronization input"]
    Exti4 = 4,
    #[doc = "5: Signal `EXTIx` selected as synchronization input"]
    Exti5 = 5,
    #[doc = "6: Signal `EXTIx` selected as synchronization input"]
    Exti6 = 6,
    #[doc = "7: Signal `EXTIx` selected as synchronization input"]
    Exti7 = 7,
    #[doc = "8: Signal `EXTIx` selected as synchronization input"]
    Exti8 = 8,
    #[doc = "9: Signal `EXTIx` selected as synchronization input"]
    Exti9 = 9,
    #[doc = "10: Signal `EXTIx` selected as synchronization input"]
    Exti10 = 10,
    #[doc = "11: Signal `EXTIx` selected as synchronization input"]
    Exti11 = 11,
    #[doc = "12: Signal `EXTIx` selected as synchronization input"]
    Exti12 = 12,
    #[doc = "13: Signal `EXTIx` selected as synchronization input"]
    Exti13 = 13,
    #[doc = "14: Signal `EXTIx` selected as synchronization input"]
    Exti14 = 14,
    #[doc = "15: Signal `EXTIx` selected as synchronization input"]
    Exti15 = 15,
    #[doc = "16: Signal `dmamux1_evt0` selected as synchronization input"]
    Dmamux1Evt0 = 16,
    #[doc = "17: Signal `dmamux1_evt1` selected as synchronization input"]
    Dmamux1Evt1 = 17,
    #[doc = "18: Signal `lptim1_out` selected as synchronization input"]
    Lptim1Out = 18,
    #[doc = "19: Signal `lptim2_out` selected as synchronization input"]
    Lptim2Out = 19,
    #[doc = "20: Signal `lptim3_out` selected as synchronization input"]
    Lptim3Out = 20,
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
#[doc = "Field `SIG_ID` reader - Signal identification"]
pub type SIG_ID_R = crate::FieldReader<SIG_ID>;
impl SIG_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SIG_ID> {
        match self.bits {
            0 => Some(SIG_ID::Exti0),
            1 => Some(SIG_ID::Exti1),
            2 => Some(SIG_ID::Exti2),
            3 => Some(SIG_ID::Exti3),
            4 => Some(SIG_ID::Exti4),
            5 => Some(SIG_ID::Exti5),
            6 => Some(SIG_ID::Exti6),
            7 => Some(SIG_ID::Exti7),
            8 => Some(SIG_ID::Exti8),
            9 => Some(SIG_ID::Exti9),
            10 => Some(SIG_ID::Exti10),
            11 => Some(SIG_ID::Exti11),
            12 => Some(SIG_ID::Exti12),
            13 => Some(SIG_ID::Exti13),
            14 => Some(SIG_ID::Exti14),
            15 => Some(SIG_ID::Exti15),
            16 => Some(SIG_ID::Dmamux1Evt0),
            17 => Some(SIG_ID::Dmamux1Evt1),
            18 => Some(SIG_ID::Lptim1Out),
            19 => Some(SIG_ID::Lptim2Out),
            20 => Some(SIG_ID::Lptim3Out),
            _ => None,
        }
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti0(&self) -> bool {
        *self == SIG_ID::Exti0
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti1(&self) -> bool {
        *self == SIG_ID::Exti1
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        *self == SIG_ID::Exti2
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti3(&self) -> bool {
        *self == SIG_ID::Exti3
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti4(&self) -> bool {
        *self == SIG_ID::Exti4
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti5(&self) -> bool {
        *self == SIG_ID::Exti5
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti6(&self) -> bool {
        *self == SIG_ID::Exti6
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti7(&self) -> bool {
        *self == SIG_ID::Exti7
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti8(&self) -> bool {
        *self == SIG_ID::Exti8
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == SIG_ID::Exti9
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti10(&self) -> bool {
        *self == SIG_ID::Exti10
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == SIG_ID::Exti11
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti12(&self) -> bool {
        *self == SIG_ID::Exti12
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti13(&self) -> bool {
        *self == SIG_ID::Exti13
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti14(&self) -> bool {
        *self == SIG_ID::Exti14
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == SIG_ID::Exti15
    }
    #[doc = "Signal `dmamux1_evt0` selected as synchronization input"]
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        *self == SIG_ID::Dmamux1Evt0
    }
    #[doc = "Signal `dmamux1_evt1` selected as synchronization input"]
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        *self == SIG_ID::Dmamux1Evt1
    }
    #[doc = "Signal `lptim1_out` selected as synchronization input"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == SIG_ID::Lptim1Out
    }
    #[doc = "Signal `lptim2_out` selected as synchronization input"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SIG_ID::Lptim2Out
    }
    #[doc = "Signal `lptim3_out` selected as synchronization input"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SIG_ID::Lptim3Out
    }
}
#[doc = "Field `SIG_ID` writer - Signal identification"]
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SIG_ID>;
impl<'a, REG> SIG_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti0(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti0)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti1(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti1)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti2(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti2)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti3(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti3)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti4(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti4)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti5(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti5)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti6(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti6)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti7(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti7)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti8(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti8)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti9)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti10(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti10)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti11)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti12(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti12)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti13(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti13)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti14(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti14)
    }
    #[doc = "Signal `EXTIx` selected as synchronization input"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti15)
    }
    #[doc = "Signal `dmamux1_evt0` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux1Evt0)
    }
    #[doc = "Signal `dmamux1_evt1` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux1Evt1)
    }
    #[doc = "Signal `lptim1_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim1Out)
    }
    #[doc = "Signal `lptim2_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim2Out)
    }
    #[doc = "Signal `lptim3_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim3Out)
    }
}
#[doc = "Trigger overrun interrupt enable\n\nValue on reset: 0"]
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
#[doc = "Field `OIE` reader - Trigger overrun interrupt enable"]
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
#[doc = "Field `OIE` writer - Trigger overrun interrupt enable"]
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
#[doc = "DMA request generator channel 0 enable\n\nValue on reset: 0"]
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
#[doc = "Field `GE` reader - DMA request generator channel 0 enable"]
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
#[doc = "Field `GE` writer - DMA request generator channel 0 enable"]
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
#[doc = "DMA request generator trigger polarity\n\nValue on reset: 0"]
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
#[doc = "Field `GPOL` reader - DMA request generator trigger polarity"]
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
#[doc = "Field `GPOL` writer - DMA request generator trigger polarity"]
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
#[doc = "Field `GNBREQ` reader - Number of DMA requests to be generated minus 1"]
pub type GNBREQ_R = crate::FieldReader;
#[doc = "Field `GNBREQ` writer - Number of DMA requests to be generated minus 1"]
pub type GNBREQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Signal identification"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel 0 enable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated minus 1"]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Signal identification"]
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<RGCRrs> {
        SIG_ID_W::new(self, 0)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<RGCRrs> {
        OIE_W::new(self, 8)
    }
    #[doc = "Bit 16 - DMA request generator channel 0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<RGCRrs> {
        GE_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<RGCRrs> {
        GPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated minus 1"]
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<RGCRrs> {
        GNBREQ_W::new(self, 19)
    }
}
#[doc = "request generator channel %s configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets RGCR%s to value 0"]
impl crate::Resettable for RGCRrs {
    const RESET_VALUE: u32 = 0;
}
