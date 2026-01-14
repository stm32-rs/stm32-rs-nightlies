///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
/**ADC DMA transfer enable

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
///Field `DMAEN` reader - ADC DMA transfer enable
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
///Field `DMAEN` writer - ADC DMA transfer enable
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
/**ADC DMA transfer configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG {
    ///0: DMA one shot mode selected
    OneShot = 0,
    ///1: DMA circular mode selected
    Circular = 1,
}
impl From<DMACFG> for bool {
    #[inline(always)]
    fn from(variant: DMACFG) -> Self {
        variant as u8 != 0
    }
}
///Field `DMACFG` reader - ADC DMA transfer configuration
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
    ///DMA one shot mode selected
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == DMACFG::OneShot
    }
    ///DMA circular mode selected
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        *self == DMACFG::Circular
    }
}
///Field `DMACFG` writer - ADC DMA transfer configuration
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG, DMACFG>;
impl<'a, REG> DMACFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA one shot mode selected
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::OneShot)
    }
    ///DMA circular mode selected
    #[inline(always)]
    pub fn circular(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG::Circular)
    }
}
/**Scan sequence direction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCANDIR {
    ///0: Upward scan (from CHSEL0 to CHSEL17)
    Upward = 0,
    ///1: Backward scan (from CHSEL17 to CHSEL0)
    Backward = 1,
}
impl From<SCANDIR> for bool {
    #[inline(always)]
    fn from(variant: SCANDIR) -> Self {
        variant as u8 != 0
    }
}
///Field `SCANDIR` reader - Scan sequence direction
pub type SCANDIR_R = crate::BitReader<SCANDIR>;
impl SCANDIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCANDIR {
        match self.bits {
            false => SCANDIR::Upward,
            true => SCANDIR::Backward,
        }
    }
    ///Upward scan (from CHSEL0 to CHSEL17)
    #[inline(always)]
    pub fn is_upward(&self) -> bool {
        *self == SCANDIR::Upward
    }
    ///Backward scan (from CHSEL17 to CHSEL0)
    #[inline(always)]
    pub fn is_backward(&self) -> bool {
        *self == SCANDIR::Backward
    }
}
///Field `SCANDIR` writer - Scan sequence direction
pub type SCANDIR_W<'a, REG> = crate::BitWriter<'a, REG, SCANDIR>;
impl<'a, REG> SCANDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Upward scan (from CHSEL0 to CHSEL17)
    #[inline(always)]
    pub fn upward(self) -> &'a mut crate::W<REG> {
        self.variant(SCANDIR::Upward)
    }
    ///Backward scan (from CHSEL17 to CHSEL0)
    #[inline(always)]
    pub fn backward(self) -> &'a mut crate::W<REG> {
        self.variant(SCANDIR::Backward)
    }
}
/**ADC data resolution

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    ///0: 12 bits
    Bits12 = 0,
    ///1: 10 bits
    Bits10 = 1,
    ///2: 8 bits
    Bits8 = 2,
    ///3: 6 bits
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
///Field `RES` reader - ADC data resolution
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
    ///12 bits
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == RES::Bits12
    }
    ///10 bits
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == RES::Bits10
    }
    ///8 bits
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == RES::Bits8
    }
    ///6 bits
    #[inline(always)]
    pub fn is_bits6(&self) -> bool {
        *self == RES::Bits6
    }
}
///Field `RES` writer - ADC data resolution
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RES, crate::Safe>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///12 bits
    #[inline(always)]
    pub fn bits12(self) -> &'a mut crate::W<REG> {
        self.variant(RES::Bits12)
    }
    ///10 bits
    #[inline(always)]
    pub fn bits10(self) -> &'a mut crate::W<REG> {
        self.variant(RES::Bits10)
    }
    ///8 bits
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(RES::Bits8)
    }
    ///6 bits
    #[inline(always)]
    pub fn bits6(self) -> &'a mut crate::W<REG> {
        self.variant(RES::Bits6)
    }
}
/**ADC data alignement

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
///Field `ALIGN` reader - ADC data alignement
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
///Field `ALIGN` writer - ADC data alignement
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
/**ADC group regular external trigger source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    ///0: Timer 1 TRGO event
    Tim1Trgo = 0,
    ///1: Timer 1 CC4 event
    Tim1Cc4 = 1,
    ///2: Timer 2 TRGO event
    Tim2Trgo = 2,
    ///3: Timer 2 CH4 event
    Tim2Ch4 = 3,
    ///5: Timer 2 CH3 event
    Tim2Ch3 = 5,
    ///7: EXTI line 11 event
    ExtiLine11 = 7,
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
///Field `EXTSEL` reader - ADC group regular external trigger source
pub type EXTSEL_R = crate::FieldReader<EXTSEL>;
impl EXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTSEL> {
        match self.bits {
            0 => Some(EXTSEL::Tim1Trgo),
            1 => Some(EXTSEL::Tim1Cc4),
            2 => Some(EXTSEL::Tim2Trgo),
            3 => Some(EXTSEL::Tim2Ch4),
            5 => Some(EXTSEL::Tim2Ch3),
            7 => Some(EXTSEL::ExtiLine11),
            _ => None,
        }
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == EXTSEL::Tim1Trgo
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn is_tim1_cc4(&self) -> bool {
        *self == EXTSEL::Tim1Cc4
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == EXTSEL::Tim2Trgo
    }
    ///Timer 2 CH4 event
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        *self == EXTSEL::Tim2Ch4
    }
    ///Timer 2 CH3 event
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == EXTSEL::Tim2Ch3
    }
    ///EXTI line 11 event
    #[inline(always)]
    pub fn is_exti_line11(&self) -> bool {
        *self == EXTSEL::ExtiLine11
    }
}
///Field `EXTSEL` writer - ADC group regular external trigger source
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTSEL>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Trgo)
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn tim1_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim1Cc4)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Trgo)
    }
    ///Timer 2 CH4 event
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Ch4)
    }
    ///Timer 2 CH3 event
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Ch3)
    }
    ///EXTI line 11 event
    #[inline(always)]
    pub fn exti_line11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::ExtiLine11)
    }
}
/**ADC group regular external trigger polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN {
    ///0: Hardware trigger detection disabled
    Disabled = 0,
    ///1: Hardware trigger detection on the rising edge
    RisingEdge = 1,
    ///2: Hardware trigger detection on the falling edge
    FallingEdge = 2,
    ///3: Hardware trigger detection on both the rising and falling edges
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
///Field `EXTEN` reader - ADC group regular external trigger polarity
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
    ///Hardware trigger detection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN::Disabled
    }
    ///Hardware trigger detection on the rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN::RisingEdge
    }
    ///Hardware trigger detection on the falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN::FallingEdge
    }
    ///Hardware trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN::BothEdges
    }
}
///Field `EXTEN` writer - ADC group regular external trigger polarity
pub type EXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXTEN, crate::Safe>;
impl<'a, REG> EXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Hardware trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::Disabled)
    }
    ///Hardware trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::RisingEdge)
    }
    ///Hardware trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::FallingEdge)
    }
    ///Hardware trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN::BothEdges)
    }
}
/**ADC group regular overrun configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD {
    ///0: ADC_DR register is preserved with the old data when an overrun is detected
    Preserve = 0,
    ///1: ADC_DR register is overwritten with the last conversion result when an overrun is detected
    Overwrite = 1,
}
impl From<OVRMOD> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRMOD` reader - ADC group regular overrun configuration
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
    ///ADC_DR register is preserved with the old data when an overrun is detected
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        *self == OVRMOD::Preserve
    }
    ///ADC_DR register is overwritten with the last conversion result when an overrun is detected
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == OVRMOD::Overwrite
    }
}
///Field `OVRMOD` writer - ADC group regular overrun configuration
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG, OVRMOD>;
impl<'a, REG> OVRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC_DR register is preserved with the old data when an overrun is detected
    #[inline(always)]
    pub fn preserve(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Preserve)
    }
    ///ADC_DR register is overwritten with the last conversion result when an overrun is detected
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD::Overwrite)
    }
}
/**ADC group regular continuous conversion mode

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
///Field `CONT` reader - ADC group regular continuous conversion mode
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
///Field `CONT` writer - ADC group regular continuous conversion mode
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
/**Wait conversion mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT {
    ///0: Wait conversion mode off
    Disabled = 0,
    ///1: Wait conversion mode on
    Enabled = 1,
}
impl From<WAIT> for bool {
    #[inline(always)]
    fn from(variant: WAIT) -> Self {
        variant as u8 != 0
    }
}
///Field `WAIT` reader - Wait conversion mode
pub type WAIT_R = crate::BitReader<WAIT>;
impl WAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAIT {
        match self.bits {
            false => WAIT::Disabled,
            true => WAIT::Enabled,
        }
    }
    ///Wait conversion mode off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAIT::Disabled
    }
    ///Wait conversion mode on
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAIT::Enabled
    }
}
///Field `WAIT` writer - Wait conversion mode
pub type WAIT_W<'a, REG> = crate::BitWriter<'a, REG, WAIT>;
impl<'a, REG> WAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wait conversion mode off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT::Disabled)
    }
    ///Wait conversion mode on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT::Enabled)
    }
}
/**Auto-off mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOFF {
    ///0: Auto-off mode disabled
    Disabled = 0,
    ///1: Auto-off mode enabled
    Enabled = 1,
}
impl From<AUTOFF> for bool {
    #[inline(always)]
    fn from(variant: AUTOFF) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTOFF` reader - Auto-off mode
pub type AUTOFF_R = crate::BitReader<AUTOFF>;
impl AUTOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUTOFF {
        match self.bits {
            false => AUTOFF::Disabled,
            true => AUTOFF::Enabled,
        }
    }
    ///Auto-off mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOFF::Disabled
    }
    ///Auto-off mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOFF::Enabled
    }
}
///Field `AUTOFF` writer - Auto-off mode
pub type AUTOFF_W<'a, REG> = crate::BitWriter<'a, REG, AUTOFF>;
impl<'a, REG> AUTOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Auto-off mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOFF::Disabled)
    }
    ///Auto-off mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOFF::Enabled)
    }
}
/**ADC group regular sequencer discontinuous mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN {
    ///0: Discontinuous mode disabled
    Disabled = 0,
    ///1: Discontinuous mode enabled
    Enabled = 1,
}
impl From<DISCEN> for bool {
    #[inline(always)]
    fn from(variant: DISCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DISCEN` reader - ADC group regular sequencer discontinuous mode
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
    ///Discontinuous mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN::Disabled
    }
    ///Discontinuous mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN::Enabled
    }
}
///Field `DISCEN` writer - ADC group regular sequencer discontinuous mode
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG, DISCEN>;
impl<'a, REG> DISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Discontinuous mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Disabled)
    }
    ///Discontinuous mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN::Enabled)
    }
}
/**Mode selection of the ADC_CHSELR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELRMOD {
    ///0: Each bit of the ADC_CHSELR register enables an input
    BitPerInput = 0,
    ///1: ADC_CHSELR register is able to sequence up to 8 channels
    Sequence = 1,
}
impl From<CHSELRMOD> for bool {
    #[inline(always)]
    fn from(variant: CHSELRMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSELRMOD` reader - Mode selection of the ADC_CHSELR register
pub type CHSELRMOD_R = crate::BitReader<CHSELRMOD>;
impl CHSELRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSELRMOD {
        match self.bits {
            false => CHSELRMOD::BitPerInput,
            true => CHSELRMOD::Sequence,
        }
    }
    ///Each bit of the ADC_CHSELR register enables an input
    #[inline(always)]
    pub fn is_bit_per_input(&self) -> bool {
        *self == CHSELRMOD::BitPerInput
    }
    ///ADC_CHSELR register is able to sequence up to 8 channels
    #[inline(always)]
    pub fn is_sequence(&self) -> bool {
        *self == CHSELRMOD::Sequence
    }
}
///Field `CHSELRMOD` writer - Mode selection of the ADC_CHSELR register
pub type CHSELRMOD_W<'a, REG> = crate::BitWriter<'a, REG, CHSELRMOD>;
impl<'a, REG> CHSELRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Each bit of the ADC_CHSELR register enables an input
    #[inline(always)]
    pub fn bit_per_input(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELRMOD::BitPerInput)
    }
    ///ADC_CHSELR register is able to sequence up to 8 channels
    #[inline(always)]
    pub fn sequence(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELRMOD::Sequence)
    }
}
/**ADC analog watchdog 1 monitoring a single channel or all channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL {
    ///0: Analog watchdog 1 enabled on all channels
    AllChannels = 0,
    ///1: Analog watchdog 1 enabled on a single channel
    SingleChannel = 1,
}
impl From<AWD1SGL> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1SGL` reader - ADC analog watchdog 1 monitoring a single channel or all channels
pub type AWD1SGL_R = crate::BitReader<AWD1SGL>;
impl AWD1SGL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1SGL {
        match self.bits {
            false => AWD1SGL::AllChannels,
            true => AWD1SGL::SingleChannel,
        }
    }
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn is_all_channels(&self) -> bool {
        *self == AWD1SGL::AllChannels
    }
    ///Analog watchdog 1 enabled on a single channel
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == AWD1SGL::SingleChannel
    }
}
///Field `AWD1SGL` writer - ADC analog watchdog 1 monitoring a single channel or all channels
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG, AWD1SGL>;
impl<'a, REG> AWD1SGL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn all_channels(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::AllChannels)
    }
    ///Analog watchdog 1 enabled on a single channel
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL::SingleChannel)
    }
}
/**ADC analog watchdog 1 enable on scope ADC group regular

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN {
    ///0: Analog watchdog 1 disabled
    Disabled = 0,
    ///1: Analog watchdog 1 enabled
    Enabled = 1,
}
impl From<AWD1EN> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1EN` reader - ADC analog watchdog 1 enable on scope ADC group regular
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
    ///Analog watchdog 1 disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1EN::Disabled
    }
    ///Analog watchdog 1 enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1EN::Enabled
    }
}
///Field `AWD1EN` writer - ADC analog watchdog 1 enable on scope ADC group regular
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, AWD1EN>;
impl<'a, REG> AWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog 1 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::Disabled)
    }
    ///Analog watchdog 1 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN::Enabled)
    }
}
///Field `AWD1CH` reader - ADC analog watchdog 1 monitored channel selection
pub type AWD1CH_R = crate::FieldReader;
///Field `AWD1CH` writer - ADC analog watchdog 1 monitored channel selection
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - ADC DMA transfer enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC DMA transfer configuration
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Scan sequence direction
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - ADC data resolution
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - ADC data alignement
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:8 - ADC group regular external trigger source
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 10:11 - ADC group regular external trigger polarity
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - ADC group regular overrun configuration
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC group regular continuous conversion mode
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Wait conversion mode
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Auto-off mode
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ADC group regular sequencer discontinuous mode
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - Mode selection of the ADC_CHSELR register
    #[inline(always)]
    pub fn chselrmod(&self) -> CHSELRMOD_R {
        CHSELRMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 26:30 - ADC analog watchdog 1 monitored channel selection
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("awd1ch", &self.awd1ch())
            .field("awd1en", &self.awd1en())
            .field("awd1sgl", &self.awd1sgl())
            .field("chselrmod", &self.chselrmod())
            .field("discen", &self.discen())
            .field("autoff", &self.autoff())
            .field("wait", &self.wait())
            .field("cont", &self.cont())
            .field("ovrmod", &self.ovrmod())
            .field("exten", &self.exten())
            .field("extsel", &self.extsel())
            .field("align", &self.align())
            .field("res", &self.res())
            .field("scandir", &self.scandir())
            .field("dmacfg", &self.dmacfg())
            .field("dmaen", &self.dmaen())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC DMA transfer enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, CFGR1rs> {
        DMAEN_W::new(self, 0)
    }
    ///Bit 1 - ADC DMA transfer configuration
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W<'_, CFGR1rs> {
        DMACFG_W::new(self, 1)
    }
    ///Bit 2 - Scan sequence direction
    #[inline(always)]
    pub fn scandir(&mut self) -> SCANDIR_W<'_, CFGR1rs> {
        SCANDIR_W::new(self, 2)
    }
    ///Bits 3:4 - ADC data resolution
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<'_, CFGR1rs> {
        RES_W::new(self, 3)
    }
    ///Bit 5 - ADC data alignement
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<'_, CFGR1rs> {
        ALIGN_W::new(self, 5)
    }
    ///Bits 6:8 - ADC group regular external trigger source
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CFGR1rs> {
        EXTSEL_W::new(self, 6)
    }
    ///Bits 10:11 - ADC group regular external trigger polarity
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, CFGR1rs> {
        EXTEN_W::new(self, 10)
    }
    ///Bit 12 - ADC group regular overrun configuration
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<'_, CFGR1rs> {
        OVRMOD_W::new(self, 12)
    }
    ///Bit 13 - ADC group regular continuous conversion mode
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CFGR1rs> {
        CONT_W::new(self, 13)
    }
    ///Bit 14 - Wait conversion mode
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W<'_, CFGR1rs> {
        WAIT_W::new(self, 14)
    }
    ///Bit 15 - Auto-off mode
    #[inline(always)]
    pub fn autoff(&mut self) -> AUTOFF_W<'_, CFGR1rs> {
        AUTOFF_W::new(self, 15)
    }
    ///Bit 16 - ADC group regular sequencer discontinuous mode
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<'_, CFGR1rs> {
        DISCEN_W::new(self, 16)
    }
    ///Bit 21 - Mode selection of the ADC_CHSELR register
    #[inline(always)]
    pub fn chselrmod(&mut self) -> CHSELRMOD_W<'_, CFGR1rs> {
        CHSELRMOD_W::new(self, 21)
    }
    ///Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<'_, CFGR1rs> {
        AWD1SGL_W::new(self, 22)
    }
    ///Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W<'_, CFGR1rs> {
        AWD1EN_W::new(self, 23)
    }
    ///Bits 26:30 - ADC analog watchdog 1 monitored channel selection
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W<'_, CFGR1rs> {
        AWD1CH_W::new(self, 26)
    }
}
/**ADC configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#ADC:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
