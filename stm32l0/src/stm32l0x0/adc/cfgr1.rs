///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
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
/**Direct memery access configuration

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
///Field `DMACFG` reader - Direct memery access configuration
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
///Field `DMACFG` writer - Direct memery access configuration
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
    ///0: Upward scan (from CHSEL0 to CHSEL18)
    Upward = 0,
    ///1: Backward scan (from CHSEL18 to CHSEL0)
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
    ///Upward scan (from CHSEL0 to CHSEL18)
    #[inline(always)]
    pub fn is_upward(&self) -> bool {
        *self == SCANDIR::Upward
    }
    ///Backward scan (from CHSEL18 to CHSEL0)
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
    ///Upward scan (from CHSEL0 to CHSEL18)
    #[inline(always)]
    pub fn upward(self) -> &'a mut crate::W<REG> {
        self.variant(SCANDIR::Upward)
    }
    ///Backward scan (from CHSEL18 to CHSEL0)
    #[inline(always)]
    pub fn backward(self) -> &'a mut crate::W<REG> {
        self.variant(SCANDIR::Backward)
    }
}
/**Data resolution

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    ///0: 12 bits
    TwelveBit = 0,
    ///1: 10 bits
    TenBit = 1,
    ///2: 8 bits
    EightBit = 2,
    ///3: 6 bits
    SixBit = 3,
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
            0 => RES::TwelveBit,
            1 => RES::TenBit,
            2 => RES::EightBit,
            3 => RES::SixBit,
            _ => unreachable!(),
        }
    }
    ///12 bits
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == RES::TwelveBit
    }
    ///10 bits
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == RES::TenBit
    }
    ///8 bits
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == RES::EightBit
    }
    ///6 bits
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        *self == RES::SixBit
    }
}
///Field `RES` writer - Data resolution
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RES, crate::Safe>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///12 bits
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TwelveBit)
    }
    ///10 bits
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::TenBit)
    }
    ///8 bits
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::EightBit)
    }
    ///6 bits
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::SixBit)
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
/**External trigger selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL {
    ///0: Timer 6 TRGO event
    Tim6Trgo = 0,
    ///1: Timer 21 CH2 event
    Tim21Ch2 = 1,
    ///2: Timer 2 TRGO event
    Tim2Trgo = 2,
    ///3: Timer 2 CH4 event
    Tim2Ch4 = 3,
    ///4: Timer 22 TRGO, Timer 21 TRGO event
    Tim22Trgo = 4,
    ///5: Timer 2 CH3 event
    Tim2Ch3 = 5,
    ///6: Timer 3 TRGO event
    Tim3Trgo = 6,
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
///Field `EXTSEL` reader - External trigger selection
pub type EXTSEL_R = crate::FieldReader<EXTSEL>;
impl EXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXTSEL {
        match self.bits {
            0 => EXTSEL::Tim6Trgo,
            1 => EXTSEL::Tim21Ch2,
            2 => EXTSEL::Tim2Trgo,
            3 => EXTSEL::Tim2Ch4,
            4 => EXTSEL::Tim22Trgo,
            5 => EXTSEL::Tim2Ch3,
            6 => EXTSEL::Tim3Trgo,
            7 => EXTSEL::ExtiLine11,
            _ => unreachable!(),
        }
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == EXTSEL::Tim6Trgo
    }
    ///Timer 21 CH2 event
    #[inline(always)]
    pub fn is_tim21_ch2(&self) -> bool {
        *self == EXTSEL::Tim21Ch2
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
    ///Timer 22 TRGO, Timer 21 TRGO event
    #[inline(always)]
    pub fn is_tim22_trgo(&self) -> bool {
        *self == EXTSEL::Tim22Trgo
    }
    ///Timer 2 CH3 event
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == EXTSEL::Tim2Ch3
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == EXTSEL::Tim3Trgo
    }
    ///EXTI line 11 event
    #[inline(always)]
    pub fn is_exti_line11(&self) -> bool {
        *self == EXTSEL::ExtiLine11
    }
}
///Field `EXTSEL` writer - External trigger selection
pub type EXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTSEL, crate::Safe>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim6Trgo)
    }
    ///Timer 21 CH2 event
    #[inline(always)]
    pub fn tim21_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim21Ch2)
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
    ///Timer 22 TRGO, Timer 21 TRGO event
    #[inline(always)]
    pub fn tim22_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim22Trgo)
    }
    ///Timer 2 CH3 event
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim2Ch3)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::Tim3Trgo)
    }
    ///EXTI line 11 event
    #[inline(always)]
    pub fn exti_line11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL::ExtiLine11)
    }
}
/**External trigger enable and polarity selection

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
///Field `EXTEN` reader - External trigger enable and polarity selection
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
///Field `EXTEN` writer - External trigger enable and polarity selection
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
/**Overrun management mode

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
///Field `OVRMOD` reader - Overrun management mode
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
///Field `OVRMOD` writer - Overrun management mode
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
/**Single / continuous conversion mode

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
///Field `CONT` reader - Single / continuous conversion mode
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
///Field `CONT` writer - Single / continuous conversion mode
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
/**Auto-delayed conversion mode

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
///Field `WAIT` reader - Auto-delayed conversion mode
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
///Field `WAIT` writer - Auto-delayed conversion mode
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
/**Discontinuous mode

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
///Field `DISCEN` reader - Discontinuous mode
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
///Field `DISCEN` writer - Discontinuous mode
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
/**Enable the watchdog on a single channel or on all channels

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDSGL {
    ///0: Analog watchdog enabled on all channels
    AllChannels = 0,
    ///1: Analog watchdog enabled on a single channel
    SingleChannel = 1,
}
impl From<AWDSGL> for bool {
    #[inline(always)]
    fn from(variant: AWDSGL) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDSGL` reader - Enable the watchdog on a single channel or on all channels
pub type AWDSGL_R = crate::BitReader<AWDSGL>;
impl AWDSGL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWDSGL {
        match self.bits {
            false => AWDSGL::AllChannels,
            true => AWDSGL::SingleChannel,
        }
    }
    ///Analog watchdog enabled on all channels
    #[inline(always)]
    pub fn is_all_channels(&self) -> bool {
        *self == AWDSGL::AllChannels
    }
    ///Analog watchdog enabled on a single channel
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == AWDSGL::SingleChannel
    }
}
///Field `AWDSGL` writer - Enable the watchdog on a single channel or on all channels
pub type AWDSGL_W<'a, REG> = crate::BitWriter<'a, REG, AWDSGL>;
impl<'a, REG> AWDSGL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog enabled on all channels
    #[inline(always)]
    pub fn all_channels(self) -> &'a mut crate::W<REG> {
        self.variant(AWDSGL::AllChannels)
    }
    ///Analog watchdog enabled on a single channel
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut crate::W<REG> {
        self.variant(AWDSGL::SingleChannel)
    }
}
/**Analog watchdog enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDEN {
    ///0: Analog watchdog disabled
    Disabled = 0,
    ///1: Analog watchdog enabled
    Enabled = 1,
}
impl From<AWDEN> for bool {
    #[inline(always)]
    fn from(variant: AWDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDEN` reader - Analog watchdog enable
pub type AWDEN_R = crate::BitReader<AWDEN>;
impl AWDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWDEN {
        match self.bits {
            false => AWDEN::Disabled,
            true => AWDEN::Enabled,
        }
    }
    ///Analog watchdog disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDEN::Disabled
    }
    ///Analog watchdog enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDEN::Enabled
    }
}
///Field `AWDEN` writer - Analog watchdog enable
pub type AWDEN_W<'a, REG> = crate::BitWriter<'a, REG, AWDEN>;
impl<'a, REG> AWDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDEN::Disabled)
    }
    ///Analog watchdog enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWDEN::Enabled)
    }
}
///Field `AWDCH` reader - Analog watchdog channel selection
pub type AWDCH_R = crate::FieldReader;
///Field `AWDCH` writer - Analog watchdog channel selection
pub type AWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - Direct memory access enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Direct memery access configuration
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Scan sequence direction
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - Data resolution
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - Data alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:8 - External trigger selection
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 10:11 - External trigger enable and polarity selection
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Overrun management mode
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Single / continuous conversion mode
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Auto-delayed conversion mode
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Auto-off mode
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Discontinuous mode
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 22 - Enable the watchdog on a single channel or on all channels
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog enable
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 26:30 - Analog watchdog channel selection
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("awdch", &self.awdch())
            .field("awden", &self.awden())
            .field("awdsgl", &self.awdsgl())
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
    ///Bit 0 - Direct memory access enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, CFGR1rs> {
        DMAEN_W::new(self, 0)
    }
    ///Bit 1 - Direct memery access configuration
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W<'_, CFGR1rs> {
        DMACFG_W::new(self, 1)
    }
    ///Bit 2 - Scan sequence direction
    #[inline(always)]
    pub fn scandir(&mut self) -> SCANDIR_W<'_, CFGR1rs> {
        SCANDIR_W::new(self, 2)
    }
    ///Bits 3:4 - Data resolution
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<'_, CFGR1rs> {
        RES_W::new(self, 3)
    }
    ///Bit 5 - Data alignment
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<'_, CFGR1rs> {
        ALIGN_W::new(self, 5)
    }
    ///Bits 6:8 - External trigger selection
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<'_, CFGR1rs> {
        EXTSEL_W::new(self, 6)
    }
    ///Bits 10:11 - External trigger enable and polarity selection
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<'_, CFGR1rs> {
        EXTEN_W::new(self, 10)
    }
    ///Bit 12 - Overrun management mode
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<'_, CFGR1rs> {
        OVRMOD_W::new(self, 12)
    }
    ///Bit 13 - Single / continuous conversion mode
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CFGR1rs> {
        CONT_W::new(self, 13)
    }
    ///Bit 14 - Auto-delayed conversion mode
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W<'_, CFGR1rs> {
        WAIT_W::new(self, 14)
    }
    ///Bit 15 - Auto-off mode
    #[inline(always)]
    pub fn autoff(&mut self) -> AUTOFF_W<'_, CFGR1rs> {
        AUTOFF_W::new(self, 15)
    }
    ///Bit 16 - Discontinuous mode
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<'_, CFGR1rs> {
        DISCEN_W::new(self, 16)
    }
    ///Bit 22 - Enable the watchdog on a single channel or on all channels
    #[inline(always)]
    pub fn awdsgl(&mut self) -> AWDSGL_W<'_, CFGR1rs> {
        AWDSGL_W::new(self, 22)
    }
    ///Bit 23 - Analog watchdog enable
    #[inline(always)]
    pub fn awden(&mut self) -> AWDEN_W<'_, CFGR1rs> {
        AWDEN_W::new(self, 23)
    }
    ///Bits 26:30 - Analog watchdog channel selection
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W<'_, CFGR1rs> {
        AWDCH_W::new(self, 26)
    }
}
/**configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#ADC:CFGR1)*/
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
