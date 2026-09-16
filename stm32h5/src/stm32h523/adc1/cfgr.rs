///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**Direct memory access enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    ///0: DMA disabled
    Disabled = 0,
    ///1: DMA enabled
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - Direct memory access enable
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    ///DMA disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    ///DMA enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
///Field `DMAEN` writer - Direct memory access enable
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    ///DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
/**Direct memory access configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG {
    ///0: DMA One Shot mode selected
    OneShot = 0,
    ///1: DMA Circular mode selected
    Circular = 1,
}
impl From<DMACFG> for bool {
    #[inline(always)]
    fn from(variant: DMACFG) -> Self {
        variant as u8 != 0
    }
}
///Field `DMACFG` reader - Direct memory access configuration
pub type DMACFG_R = crate::BitReader<DMACFG>;
impl DMACFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMACFG {
        match self.bits {
            false => DMACFG::OneShot,
            true => DMACFG::Circular,
        }
    }
    ///DMA One Shot mode selected
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == DMACFG::OneShot
    }
    ///DMA Circular mode selected
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        *self == DMACFG::Circular
    }
}
///Field `DMACFG` writer - Direct memory access configuration
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG, DMACFG>;
impl<'a, REG> DMACFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA One Shot mode selected
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::OneShot)
    }
    ///DMA Circular mode selected
    #[inline(always)]
    pub fn circular(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::Circular)
    }
}
/**Data resolution

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    ///0: 12-bit
    Bits12 = 0,
    ///1: 10-bit
    Bits10 = 1,
    ///2: 8-bit
    Bits8 = 2,
    ///3: 6-bit
    Bits6 = 3,
}
impl From<RES> for u8 {
    #[inline(always)]
    fn from(variant: RES) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES {
    type Ux = u8;
}
impl crate::IsEnum for RES {}
///Field `RES` reader - Data resolution
pub type RES_R = crate::FieldReader<RES>;
impl RES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RES {
        match self.bits {
            0 => RES::Bits12,
            1 => RES::Bits10,
            2 => RES::Bits8,
            3 => RES::Bits6,
            _ => unreachable!(),
        }
    }
    ///12-bit
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == RES::Bits12
    }
    ///10-bit
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == RES::Bits10
    }
    ///8-bit
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == RES::Bits8
    }
    ///6-bit
    #[inline(always)]
    pub fn is_bits6(&self) -> bool {
        *self == RES::Bits6
    }
}
///Field `RES` writer - Data resolution
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RES, crate::Safe>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///12-bit
    #[inline(always)]
    pub fn bits12(self) -> &'a mut crate::W<REG> {
        self.variant(RES::Bits12)
    }
    ///10-bit
    #[inline(always)]
    pub fn bits10(self) -> &'a mut crate::W<REG> {
        self.variant(RES::Bits10)
    }
    ///8-bit
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(RES::Bits8)
    }
    ///6-bit
    #[inline(always)]
    pub fn bits6(self) -> &'a mut crate::W<REG> {
        self.variant(RES::Bits6)
    }
}
/**External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    ///0: Timer 1 CC1 event
    Tim1Cc1 = 0,
    ///1: Timer 1 CC2 event
    Tim1Cc2 = 1,
    ///2: Timer 1 CC3 event
    Tim1Cc3 = 2,
    ///3: Timer 2 CC2 event
    Tim2Cc2 = 3,
    ///4: Timer 3 TRGO event
    Tim3Trgo = 4,
    ///6: EXTI line 11
    Exti11 = 6,
    ///9: Timer 1 TRGO event
    Tim1Trgo = 9,
    ///10: Timer 1 TRGO2 event
    Tim1Trgo2 = 10,
    ///11: Timer 2 TRGO event
    Tim2Trgo = 11,
    ///13: Timer 6 TRGO event
    Tim6Trgo = 13,
    ///14: Timer 15 TRGO event
    Tim15Trgo = 14,
    ///15: Timer 3 CC4 event
    Tim3Cc4 = 15,
}
impl From<EXTSEL> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTSEL {
    type Ux = u8;
}
impl crate::IsEnum for EXTSEL {}
///Field `EXTSEL` reader - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EXTSEL_R = crate::FieldReader<EXTSEL>;
impl EXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTSEL> {
        match self.bits {
            0 => Some(EXTSEL::Tim1Cc1),
            1 => Some(EXTSEL::Tim1Cc2),
            2 => Some(EXTSEL::Tim1Cc3),
            3 => Some(EXTSEL::Tim2Cc2),
            4 => Some(EXTSEL::Tim3Trgo),
            6 => Some(EXTSEL::Exti11),
            9 => Some(EXTSEL::Tim1Trgo),
            10 => Some(EXTSEL::Tim1Trgo2),
            11 => Some(EXTSEL::Tim2Trgo),
            13 => Some(EXTSEL::Tim6Trgo),
            14 => Some(EXTSEL::Tim15Trgo),
            15 => Some(EXTSEL::Tim3Cc4),
            _ => None,
        }
    }
    ///Timer 1 CC1 event
    #[inline(always)]
    pub fn is_tim1_cc1(&self) -> bool {
        *self == EXTSEL::Tim1Cc1
    }
    ///Timer 1 CC2 event
    #[inline(always)]
    pub fn is_tim1_cc2(&self) -> bool {
        *self == EXTSEL::Tim1Cc2
    }
    ///Timer 1 CC3 event
    #[inline(always)]
    pub fn is_tim1_cc3(&self) -> bool {
        *self == EXTSEL::Tim1Cc3
    }
    ///Timer 2 CC2 event
    #[inline(always)]
    pub fn is_tim2_cc2(&self) -> bool {
        *self == EXTSEL::Tim2Cc2
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == EXTSEL::Tim3Trgo
    }
    ///EXTI line 11
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL::Exti11
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == EXTSEL::Tim1Trgo
    }
    ///Timer 1 TRGO2 event
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == EXTSEL::Tim1Trgo2
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == EXTSEL::Tim2Trgo
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == EXTSEL::Tim6Trgo
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == EXTSEL::Tim15Trgo
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn is_tim3_cc4(&self) -> bool {
        *self == EXTSEL::Tim3Cc4
    }
}
///Field `EXTSEL` writer - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, EXTSEL>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 1 CC1 event
    #[inline(always)]
    pub fn tim1_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Cc1)
    }
    ///Timer 1 CC2 event
    #[inline(always)]
    pub fn tim1_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Cc2)
    }
    ///Timer 1 CC3 event
    #[inline(always)]
    pub fn tim1_cc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Cc3)
    }
    ///Timer 2 CC2 event
    #[inline(always)]
    pub fn tim2_cc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Cc2)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3Trgo)
    }
    ///EXTI line 11
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Exti11)
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Trgo)
    }
    ///Timer 1 TRGO2 event
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Trgo2)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Trgo)
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim6Trgo)
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim15Trgo)
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn tim3_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3Cc4)
    }
}
/**External trigger enable and polarity selection for regular channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN {
    ///0: Trigger detection disabled
    Disabled = 0,
    ///1: Trigger detection on the rising edge
    RisingEdge = 1,
    ///2: Trigger detection on the falling edge
    FallingEdge = 2,
    ///3: Trigger detection on both the rising and falling edges
    BothEdges = 3,
}
impl From<EXTEN> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTEN {
    type Ux = u8;
}
impl crate::IsEnum for EXTEN {}
///Field `EXTEN` reader - External trigger enable and polarity selection for regular channels
pub type EXTEN_R = crate::FieldReader<EXTEN>;
impl EXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTEN {
        match self.bits {
            0 => EXTEN::Disabled,
            1 => EXTEN::RisingEdge,
            2 => EXTEN::FallingEdge,
            3 => EXTEN::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Trigger detection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN::Disabled
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN::RisingEdge
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN::FallingEdge
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN::BothEdges
    }
}
///Field `EXTEN` writer - External trigger enable and polarity selection for regular channels
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTEN, crate::Safe>;
impl<'a, REG> EXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::Disabled)
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::RisingEdge)
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::FallingEdge)
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::BothEdges)
    }
}
/**Overrun mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD {
    ///0: Preserve DR register when an overrun is detected
    Preserve = 0,
    ///1: Overwrite DR register when an overrun is detected
    Overwrite = 1,
}
impl From<OVRMOD> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRMOD` reader - Overrun mode
pub type OVRMOD_R = crate::BitReader<OVRMOD>;
impl OVRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRMOD {
        match self.bits {
            false => OVRMOD::Preserve,
            true => OVRMOD::Overwrite,
        }
    }
    ///Preserve DR register when an overrun is detected
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        *self == OVRMOD::Preserve
    }
    ///Overwrite DR register when an overrun is detected
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == OVRMOD::Overwrite
    }
}
///Field `OVRMOD` writer - Overrun mode
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG, OVRMOD>;
impl<'a, REG> OVRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Preserve DR register when an overrun is detected
    #[inline(always)]
    pub fn preserve(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Preserve)
    }
    ///Overwrite DR register when an overrun is detected
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Overwrite)
    }
}
/**Single / continuous conversion mode for regular conversions

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT {
    ///0: Single conversion mode
    Single = 0,
    ///1: Continuous conversion mode
    Continuous = 1,
}
impl From<CONT> for bool {
    #[inline(always)]
    fn from(variant: CONT) -> Self {
        variant as u8 != 0
    }
}
///Field `CONT` reader - Single / continuous conversion mode for regular conversions
pub type CONT_R = crate::BitReader<CONT>;
impl CONT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CONT {
        match self.bits {
            false => CONT::Single,
            true => CONT::Continuous,
        }
    }
    ///Single conversion mode
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT::Single
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT::Continuous
    }
}
///Field `CONT` writer - Single / continuous conversion mode for regular conversions
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single conversion mode
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Single)
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CONT::Continuous)
    }
}
/**Delayed conversion mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTDLY {
    ///0: Auto delayed conversion mode off
    Off = 0,
    ///1: Auto delayed conversion mode on
    On = 1,
}
impl From<AUTDLY> for bool {
    #[inline(always)]
    fn from(variant: AUTDLY) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTDLY` reader - Delayed conversion mode
pub type AUTDLY_R = crate::BitReader<AUTDLY>;
impl AUTDLY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUTDLY {
        match self.bits {
            false => AUTDLY::Off,
            true => AUTDLY::On,
        }
    }
    ///Auto delayed conversion mode off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == AUTDLY::Off
    }
    ///Auto delayed conversion mode on
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == AUTDLY::On
    }
}
///Field `AUTDLY` writer - Delayed conversion mode
pub type AUTDLY_W<'a, REG> = crate::BitWriter<'a, REG, AUTDLY>;
impl<'a, REG> AUTDLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Auto delayed conversion mode off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(AUTDLY::Off)
    }
    ///Auto delayed conversion mode on
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(AUTDLY::On)
    }
}
/**Data alignment

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN {
    ///0: Right alignment
    Right = 0,
    ///1: Left alignment
    Left = 1,
}
impl From<ALIGN> for bool {
    #[inline(always)]
    fn from(variant: ALIGN) -> Self {
        variant as u8 != 0
    }
}
///Field `ALIGN` reader - Data alignment
pub type ALIGN_R = crate::BitReader<ALIGN>;
impl ALIGN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALIGN {
        match self.bits {
            false => ALIGN::Right,
            true => ALIGN::Left,
        }
    }
    ///Right alignment
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN::Right
    }
    ///Left alignment
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN::Left
    }
}
///Field `ALIGN` writer - Data alignment
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG, ALIGN>;
impl<'a, REG> ALIGN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Right alignment
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN::Right)
    }
    ///Left alignment
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN::Left)
    }
}
/**Discontinuous mode for regular channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN {
    ///0: Discontinuous mode on regular channels disabled
    Disabled = 0,
    ///1: Discontinuous mode on regular channels enabled
    Enabled = 1,
}
impl From<DISCEN> for bool {
    #[inline(always)]
    fn from(variant: DISCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DISCEN` reader - Discontinuous mode for regular channels
pub type DISCEN_R = crate::BitReader<DISCEN>;
impl DISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DISCEN {
        match self.bits {
            false => DISCEN::Disabled,
            true => DISCEN::Enabled,
        }
    }
    ///Discontinuous mode on regular channels disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN::Disabled
    }
    ///Discontinuous mode on regular channels enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN::Enabled
    }
}
///Field `DISCEN` writer - Discontinuous mode for regular channels
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG, DISCEN>;
impl<'a, REG> DISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discontinuous mode on regular channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Disabled)
    }
    ///Discontinuous mode on regular channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Enabled)
    }
}
///Field `DISCNUM` reader - Discontinuous mode channel count
pub type DISCNUM_R = crate::FieldReader;
///Field `DISCNUM` writer - Discontinuous mode channel count
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/**Discontinuous mode on injected channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDISCEN {
    ///0: Discontinuous mode on injected channels disabled
    Disabled = 0,
    ///1: Discontinuous mode on injected channels enabled
    Enabled = 1,
}
impl From<JDISCEN> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `JDISCEN` reader - Discontinuous mode on injected channels
pub type JDISCEN_R = crate::BitReader<JDISCEN>;
impl JDISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JDISCEN {
        match self.bits {
            false => JDISCEN::Disabled,
            true => JDISCEN::Enabled,
        }
    }
    ///Discontinuous mode on injected channels disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDISCEN::Disabled
    }
    ///Discontinuous mode on injected channels enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDISCEN::Enabled
    }
}
///Field `JDISCEN` writer - Discontinuous mode on injected channels
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG, JDISCEN>;
impl<'a, REG> JDISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discontinuous mode on injected channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN::Disabled)
    }
    ///Discontinuous mode on injected channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDISCEN::Enabled)
    }
}
/**JSQR queue mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQM {
    ///0: JSQR Mode 0: Queue maintains the last written configuration into JSQR
    Mode0 = 0,
    ///1: JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence
    Mode1 = 1,
}
impl From<JQM> for bool {
    #[inline(always)]
    fn from(variant: JQM) -> Self {
        variant as u8 != 0
    }
}
///Field `JQM` reader - JSQR queue mode
pub type JQM_R = crate::BitReader<JQM>;
impl JQM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JQM {
        match self.bits {
            false => JQM::Mode0,
            true => JQM::Mode1,
        }
    }
    ///JSQR Mode 0: Queue maintains the last written configuration into JSQR
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == JQM::Mode0
    }
    ///JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == JQM::Mode1
    }
}
///Field `JQM` writer - JSQR queue mode
pub type JQM_W<'a, REG> = crate::BitWriter<'a, REG, JQM>;
impl<'a, REG> JQM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///JSQR Mode 0: Queue maintains the last written configuration into JSQR
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(JQM::Mode0)
    }
    ///JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(JQM::Mode1)
    }
}
/**Enable the watchdog 1 on a single channel or on all channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL {
    ///0: Analog watchdog 1 enabled on all channels
    All = 0,
    ///1: Analog watchdog 1 enabled on single channel selected in AWD1CH
    Single = 1,
}
impl From<AWD1SGL> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1SGL` reader - Enable the watchdog 1 on a single channel or on all channels
pub type AWD1SGL_R = crate::BitReader<AWD1SGL>;
impl AWD1SGL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1SGL {
        match self.bits {
            false => AWD1SGL::All,
            true => AWD1SGL::Single,
        }
    }
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == AWD1SGL::All
    }
    ///Analog watchdog 1 enabled on single channel selected in AWD1CH
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == AWD1SGL::Single
    }
}
///Field `AWD1SGL` writer - Enable the watchdog 1 on a single channel or on all channels
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG, AWD1SGL>;
impl<'a, REG> AWD1SGL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::All)
    }
    ///Analog watchdog 1 enabled on single channel selected in AWD1CH
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::Single)
    }
}
/**Analog watchdog 1 enable on regular channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN {
    ///0: Analog watchdog 1 disabled on regular channels
    Disabled = 0,
    ///1: Analog watchdog 1 enabled on regular channels
    Enabled = 1,
}
impl From<AWD1EN> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1EN` reader - Analog watchdog 1 enable on regular channels
pub type AWD1EN_R = crate::BitReader<AWD1EN>;
impl AWD1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1EN {
        match self.bits {
            false => AWD1EN::Disabled,
            true => AWD1EN::Enabled,
        }
    }
    ///Analog watchdog 1 disabled on regular channels
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1EN::Disabled
    }
    ///Analog watchdog 1 enabled on regular channels
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1EN::Enabled
    }
}
///Field `AWD1EN` writer - Analog watchdog 1 enable on regular channels
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, AWD1EN>;
impl<'a, REG> AWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 disabled on regular channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::Disabled)
    }
    ///Analog watchdog 1 enabled on regular channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::Enabled)
    }
}
/**Analog watchdog 1 enable on injected channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAWD1EN {
    ///0: Analog watchdog 1 disabled on injected channels
    Disabled = 0,
    ///1: Analog watchdog 1 enabled on injected channels
    Enabled = 1,
}
impl From<JAWD1EN> for bool {
    #[inline(always)]
    fn from(variant: JAWD1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `JAWD1EN` reader - Analog watchdog 1 enable on injected channels
pub type JAWD1EN_R = crate::BitReader<JAWD1EN>;
impl JAWD1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JAWD1EN {
        match self.bits {
            false => JAWD1EN::Disabled,
            true => JAWD1EN::Enabled,
        }
    }
    ///Analog watchdog 1 disabled on injected channels
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAWD1EN::Disabled
    }
    ///Analog watchdog 1 enabled on injected channels
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAWD1EN::Enabled
    }
}
///Field `JAWD1EN` writer - Analog watchdog 1 enable on injected channels
pub type JAWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, JAWD1EN>;
impl<'a, REG> JAWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 disabled on injected channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAWD1EN::Disabled)
    }
    ///Analog watchdog 1 enabled on injected channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAWD1EN::Enabled)
    }
}
/**Automatic injected group conversion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAUTO {
    ///0: Automatic injected group conversion disabled
    Disabled = 0,
    ///1: Automatic injected group conversion enabled
    Enabled = 1,
}
impl From<JAUTO> for bool {
    #[inline(always)]
    fn from(variant: JAUTO) -> Self {
        variant as u8 != 0
    }
}
///Field `JAUTO` reader - Automatic injected group conversion
pub type JAUTO_R = crate::BitReader<JAUTO>;
impl JAUTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JAUTO {
        match self.bits {
            false => JAUTO::Disabled,
            true => JAUTO::Enabled,
        }
    }
    ///Automatic injected group conversion disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAUTO::Disabled
    }
    ///Automatic injected group conversion enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAUTO::Enabled
    }
}
///Field `JAUTO` writer - Automatic injected group conversion
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG, JAUTO>;
impl<'a, REG> JAUTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic injected group conversion disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO::Disabled)
    }
    ///Automatic injected group conversion enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JAUTO::Enabled)
    }
}
///Field `AWD1CH` reader - Analog watchdog 1 channel selection
pub type AWD1CH_R = crate::FieldReader;
///Field `AWD1CH` writer - Analog watchdog 1 channel selection
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `JQDIS` reader - Injected queue disable
pub type JQDIS_R = crate::BitReader;
///Field `JQDIS` writer - Injected queue disable
pub type JQDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Direct memory access enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Direct memory access configuration
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 3:4 - Data resolution
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:9 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:11 - External trigger enable and polarity selection for regular channels
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Overrun mode
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Single / continuous conversion mode for regular conversions
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Delayed conversion mode
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Data alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Discontinuous mode for regular channels
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - Discontinuous mode channel count
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - Discontinuous mode on injected channels
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - JSQR queue mode
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Enable the watchdog 1 on a single channel or on all channels
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog 1 enable on regular channels
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Analog watchdog 1 enable on injected channels
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Automatic injected group conversion
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:30 - Analog watchdog 1 channel selection
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - Injected queue disable
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("dmaen", &self.dmaen())
            .field("dmacfg", &self.dmacfg())
            .field("res", &self.res())
            .field("extsel", &self.extsel())
            .field("exten", &self.exten())
            .field("ovrmod", &self.ovrmod())
            .field("cont", &self.cont())
            .field("autdly", &self.autdly())
            .field("align", &self.align())
            .field("discen", &self.discen())
            .field("discnum", &self.discnum())
            .field("jdiscen", &self.jdiscen())
            .field("jqm", &self.jqm())
            .field("awd1sgl", &self.awd1sgl())
            .field("awd1en", &self.awd1en())
            .field("jawd1en", &self.jawd1en())
            .field("jauto", &self.jauto())
            .field("awd1ch", &self.awd1ch())
            .field("jqdis", &self.jqdis())
            .finish()
    }
}
impl W {
    ///Bit 0 - Direct memory access enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, CFGRrs> {
        DMAEN_W::new(self, 0)
    }
    ///Bit 1 - Direct memory access configuration
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W<'_, CFGRrs> {
        DMACFG_W::new(self, 1)
    }
    ///Bits 3:4 - Data resolution
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<'_, CFGRrs> {
        RES_W::new(self, 3)
    }
    ///Bits 5:9 - External trigger selection for regular group These bits select the external event used to trigger the start of conversion of a regular group: ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CFGRrs> {
        EXTSEL_W::new(self, 5)
    }
    ///Bits 10:11 - External trigger enable and polarity selection for regular channels
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, CFGRrs> {
        EXTEN_W::new(self, 10)
    }
    ///Bit 12 - Overrun mode
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<'_, CFGRrs> {
        OVRMOD_W::new(self, 12)
    }
    ///Bit 13 - Single / continuous conversion mode for regular conversions
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CFGRrs> {
        CONT_W::new(self, 13)
    }
    ///Bit 14 - Delayed conversion mode
    #[inline(always)]
    pub fn autdly(&mut self) -> AUTDLY_W<'_, CFGRrs> {
        AUTDLY_W::new(self, 14)
    }
    ///Bit 15 - Data alignment
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<'_, CFGRrs> {
        ALIGN_W::new(self, 15)
    }
    ///Bit 16 - Discontinuous mode for regular channels
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<'_, CFGRrs> {
        DISCEN_W::new(self, 16)
    }
    ///Bits 17:19 - Discontinuous mode channel count
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W<'_, CFGRrs> {
        DISCNUM_W::new(self, 17)
    }
    ///Bit 20 - Discontinuous mode on injected channels
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W<'_, CFGRrs> {
        JDISCEN_W::new(self, 20)
    }
    ///Bit 21 - JSQR queue mode
    #[inline(always)]
    pub fn jqm(&mut self) -> JQM_W<'_, CFGRrs> {
        JQM_W::new(self, 21)
    }
    ///Bit 22 - Enable the watchdog 1 on a single channel or on all channels
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<'_, CFGRrs> {
        AWD1SGL_W::new(self, 22)
    }
    ///Bit 23 - Analog watchdog 1 enable on regular channels
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W<'_, CFGRrs> {
        AWD1EN_W::new(self, 23)
    }
    ///Bit 24 - Analog watchdog 1 enable on injected channels
    #[inline(always)]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<'_, CFGRrs> {
        JAWD1EN_W::new(self, 24)
    }
    ///Bit 25 - Automatic injected group conversion
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W<'_, CFGRrs> {
        JAUTO_W::new(self, 25)
    }
    ///Bits 26:30 - Analog watchdog 1 channel selection
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W<'_, CFGRrs> {
        AWD1CH_W::new(self, 26)
    }
    ///Bit 31 - Injected queue disable
    #[inline(always)]
    pub fn jqdis(&mut self) -> JQDIS_W<'_, CFGRrs> {
        JQDIS_W::new(self, 31)
    }
}
/**ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#ADC1:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0x8000_0000
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
