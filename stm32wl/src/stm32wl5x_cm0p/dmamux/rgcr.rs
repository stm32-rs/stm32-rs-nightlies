///Register `RGCR%s` reader
pub type R = crate::R<RGCRrs>;
///Register `RGCR%s` writer
pub type W = crate::W<RGCRrs>;
/**Signal identification

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIG_ID {
    ///0: Signal `EXTIx` selected as synchronization input
    Exti0 = 0,
    ///1: Signal `EXTIx` selected as synchronization input
    Exti1 = 1,
    ///2: Signal `EXTIx` selected as synchronization input
    Exti2 = 2,
    ///3: Signal `EXTIx` selected as synchronization input
    Exti3 = 3,
    ///4: Signal `EXTIx` selected as synchronization input
    Exti4 = 4,
    ///5: Signal `EXTIx` selected as synchronization input
    Exti5 = 5,
    ///6: Signal `EXTIx` selected as synchronization input
    Exti6 = 6,
    ///7: Signal `EXTIx` selected as synchronization input
    Exti7 = 7,
    ///8: Signal `EXTIx` selected as synchronization input
    Exti8 = 8,
    ///9: Signal `EXTIx` selected as synchronization input
    Exti9 = 9,
    ///10: Signal `EXTIx` selected as synchronization input
    Exti10 = 10,
    ///11: Signal `EXTIx` selected as synchronization input
    Exti11 = 11,
    ///12: Signal `EXTIx` selected as synchronization input
    Exti12 = 12,
    ///13: Signal `EXTIx` selected as synchronization input
    Exti13 = 13,
    ///14: Signal `EXTIx` selected as synchronization input
    Exti14 = 14,
    ///15: Signal `EXTIx` selected as synchronization input
    Exti15 = 15,
    ///16: Signal `dmamux1_evt0` selected as synchronization input
    Dmamux1Evt0 = 16,
    ///17: Signal `dmamux1_evt1` selected as synchronization input
    Dmamux1Evt1 = 17,
    ///18: Signal `lptim1_out` selected as synchronization input
    Lptim1Out = 18,
    ///19: Signal `lptim2_out` selected as synchronization input
    Lptim2Out = 19,
    ///20: Signal `lptim3_out` selected as synchronization input
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
impl crate::IsEnum for SIG_ID {}
///Field `SIG_ID` reader - Signal identification
pub type SIG_ID_R = crate::FieldReader<SIG_ID>;
impl SIG_ID_R {
    ///Get enumerated values variant
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
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti0(&self) -> bool {
        *self == SIG_ID::Exti0
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti1(&self) -> bool {
        *self == SIG_ID::Exti1
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        *self == SIG_ID::Exti2
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti3(&self) -> bool {
        *self == SIG_ID::Exti3
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti4(&self) -> bool {
        *self == SIG_ID::Exti4
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti5(&self) -> bool {
        *self == SIG_ID::Exti5
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti6(&self) -> bool {
        *self == SIG_ID::Exti6
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti7(&self) -> bool {
        *self == SIG_ID::Exti7
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti8(&self) -> bool {
        *self == SIG_ID::Exti8
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == SIG_ID::Exti9
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti10(&self) -> bool {
        *self == SIG_ID::Exti10
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == SIG_ID::Exti11
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti12(&self) -> bool {
        *self == SIG_ID::Exti12
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti13(&self) -> bool {
        *self == SIG_ID::Exti13
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti14(&self) -> bool {
        *self == SIG_ID::Exti14
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == SIG_ID::Exti15
    }
    ///Signal `dmamux1_evt0` selected as synchronization input
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        *self == SIG_ID::Dmamux1Evt0
    }
    ///Signal `dmamux1_evt1` selected as synchronization input
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        *self == SIG_ID::Dmamux1Evt1
    }
    ///Signal `lptim1_out` selected as synchronization input
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == SIG_ID::Lptim1Out
    }
    ///Signal `lptim2_out` selected as synchronization input
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SIG_ID::Lptim2Out
    }
    ///Signal `lptim3_out` selected as synchronization input
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SIG_ID::Lptim3Out
    }
}
///Field `SIG_ID` writer - Signal identification
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SIG_ID>;
impl<'a, REG> SIG_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti0(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti0)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti1(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti1)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti2(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti2)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti3(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti3)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti4(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti4)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti5(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti5)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti6(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti6)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti7(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti7)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti8(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti8)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti9(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti9)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti10(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti10)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti11)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti12(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti12)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti13(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti13)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti14(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti14)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Exti15)
    }
    ///Signal `dmamux1_evt0` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux1Evt0)
    }
    ///Signal `dmamux1_evt1` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Dmamux1Evt1)
    }
    ///Signal `lptim1_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim1Out)
    }
    ///Signal `lptim2_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim2Out)
    }
    ///Signal `lptim3_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(SIG_ID::Lptim3Out)
    }
}
/**Trigger overrun interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIE {
    ///0: Trigger overrun interrupt disabled
    Disabled = 0,
    ///1: Trigger overrun interrupt enabled
    Enabled = 1,
}
impl From<OIE> for bool {
    #[inline(always)]
    fn from(variant: OIE) -> Self {
        variant as u8 != 0
    }
}
///Field `OIE` reader - Trigger overrun interrupt enable
pub type OIE_R = crate::BitReader<OIE>;
impl OIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OIE {
        match self.bits {
            false => OIE::Disabled,
            true => OIE::Enabled,
        }
    }
    ///Trigger overrun interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OIE::Disabled
    }
    ///Trigger overrun interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OIE::Enabled
    }
}
///Field `OIE` writer - Trigger overrun interrupt enable
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG, OIE>;
impl<'a, REG> OIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::Disabled)
    }
    ///Trigger overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OIE::Enabled)
    }
}
/**DMA request generator channel x enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GE {
    ///0: DMA request generation disabled
    Disabled = 0,
    ///1: DMA request enabled
    Enabled = 1,
}
impl From<GE> for bool {
    #[inline(always)]
    fn from(variant: GE) -> Self {
        variant as u8 != 0
    }
}
///Field `GE` reader - DMA request generator channel x enable
pub type GE_R = crate::BitReader<GE>;
impl GE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GE {
        match self.bits {
            false => GE::Disabled,
            true => GE::Enabled,
        }
    }
    ///DMA request generation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GE::Disabled
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GE::Enabled
    }
}
///Field `GE` writer - DMA request generator channel x enable
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG, GE>;
impl<'a, REG> GE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA request generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GE::Disabled)
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GE::Enabled)
    }
}
/**DMA request generator trigger polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOL {
    ///0: No event, i.e. no detection nor generation
    NoEdge = 0,
    ///1: Rising edge
    RisingEdge = 1,
    ///2: Falling edge
    FallingEdge = 2,
    ///3: Rising and falling edges
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
impl crate::IsEnum for GPOL {}
///Field `GPOL` reader - DMA request generator trigger polarity
pub type GPOL_R = crate::FieldReader<GPOL>;
impl GPOL_R {
    ///Get enumerated values variant
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
    ///No event, i.e. no detection nor generation
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == GPOL::NoEdge
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == GPOL::RisingEdge
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == GPOL::FallingEdge
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == GPOL::BothEdges
    }
}
///Field `GPOL` writer - DMA request generator trigger polarity
pub type GPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, GPOL, crate::Safe>;
impl<'a, REG> GPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No event, i.e. no detection nor generation
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::NoEdge)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::RisingEdge)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::FallingEdge)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL::BothEdges)
    }
}
///Field `GNBREQ` reader - Number of DMA requests to be generated (minus 1)
pub type GNBREQ_R = crate::FieldReader;
///Field `GNBREQ` writer - Number of DMA requests to be generated (minus 1)
pub type GNBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bits 0:4 - Signal identification
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Trigger overrun interrupt enable
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - DMA request generator channel x enable
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - DMA request generator trigger polarity
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests to be generated (minus 1)
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGCR")
            .field("gnbreq", &self.gnbreq())
            .field("gpol", &self.gpol())
            .field("ge", &self.ge())
            .field("oie", &self.oie())
            .field("sig_id", &self.sig_id())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Signal identification
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W<'_, RGCRrs> {
        SIG_ID_W::new(self, 0)
    }
    ///Bit 8 - Trigger overrun interrupt enable
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W<'_, RGCRrs> {
        OIE_W::new(self, 8)
    }
    ///Bit 16 - DMA request generator channel x enable
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W<'_, RGCRrs> {
        GE_W::new(self, 16)
    }
    ///Bits 17:18 - DMA request generator trigger polarity
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W<'_, RGCRrs> {
        GPOL_W::new(self, 17)
    }
    ///Bits 19:23 - Number of DMA requests to be generated (minus 1)
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W<'_, RGCRrs> {
        GNBREQ_W::new(self, 19)
    }
}
/**request generator channel x configuration register

You can [`read`](crate::Reg::read) this register and get [`rgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#DMAMUX:RGCR[0])*/
pub struct RGCRrs;
impl crate::RegisterSpec for RGCRrs {
    type Ux = u32;
}
///`read()` method returns [`rgcr::R`](R) reader structure
impl crate::Readable for RGCRrs {}
///`write(|w| ..)` method takes [`rgcr::W`](W) writer structure
impl crate::Writable for RGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RGCR%s to value 0
impl crate::Resettable for RGCRrs {}
