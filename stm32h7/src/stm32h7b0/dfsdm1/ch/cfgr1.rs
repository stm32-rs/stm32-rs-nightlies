///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
/**Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SITP {
    ///0: SPI with rising edge to strobe data
    SpirisingEdge = 0,
    ///1: SPI with falling edge to strobe data
    SpifallingEdge = 1,
    ///2: Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1
    Manchester = 2,
    ///3: Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0
    ManchesterInverted = 3,
}
impl From<SITP> for u8 {
    #[inline(always)]
    fn from(variant: SITP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SITP {
    type Ux = u8;
}
impl crate::IsEnum for SITP {}
///Field `SITP` reader - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type SITP_R = crate::FieldReader<SITP>;
impl SITP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SITP {
        match self.bits {
            0 => SITP::SpirisingEdge,
            1 => SITP::SpifallingEdge,
            2 => SITP::Manchester,
            3 => SITP::ManchesterInverted,
            _ => unreachable!(),
        }
    }
    ///SPI with rising edge to strobe data
    #[inline(always)]
    pub fn is_spirising_edge(&self) -> bool {
        *self == SITP::SpirisingEdge
    }
    ///SPI with falling edge to strobe data
    #[inline(always)]
    pub fn is_spifalling_edge(&self) -> bool {
        *self == SITP::SpifallingEdge
    }
    ///Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1
    #[inline(always)]
    pub fn is_manchester(&self) -> bool {
        *self == SITP::Manchester
    }
    ///Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0
    #[inline(always)]
    pub fn is_manchester_inverted(&self) -> bool {
        *self == SITP::ManchesterInverted
    }
}
///Field `SITP` writer - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type SITP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SITP, crate::Safe>;
impl<'a, REG> SITP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SPI with rising edge to strobe data
    #[inline(always)]
    pub fn spirising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SITP::SpirisingEdge)
    }
    ///SPI with falling edge to strobe data
    #[inline(always)]
    pub fn spifalling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SITP::SpifallingEdge)
    }
    ///Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1
    #[inline(always)]
    pub fn manchester(self) -> &'a mut crate::W<REG> {
        self.variant(SITP::Manchester)
    }
    ///Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0
    #[inline(always)]
    pub fn manchester_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(SITP::ManchesterInverted)
    }
}
/**SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPICKSEL {
    ///0: Clock coming from external CKINy input - sampling point according SITP\[1:0\]
    Ckin = 0,
    ///1: Clock coming from internal CKOUT output - sampling point according SITP\[1:0\]
    Ckout = 1,
    ///2: Clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge)
    CkoutsecondFalling = 2,
    ///3: Clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge)
    CkoutsecondRising = 3,
}
impl From<SPICKSEL> for u8 {
    #[inline(always)]
    fn from(variant: SPICKSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPICKSEL {
    type Ux = u8;
}
impl crate::IsEnum for SPICKSEL {}
///Field `SPICKSEL` reader - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type SPICKSEL_R = crate::FieldReader<SPICKSEL>;
impl SPICKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPICKSEL {
        match self.bits {
            0 => SPICKSEL::Ckin,
            1 => SPICKSEL::Ckout,
            2 => SPICKSEL::CkoutsecondFalling,
            3 => SPICKSEL::CkoutsecondRising,
            _ => unreachable!(),
        }
    }
    ///Clock coming from external CKINy input - sampling point according SITP\[1:0\]
    #[inline(always)]
    pub fn is_ckin(&self) -> bool {
        *self == SPICKSEL::Ckin
    }
    ///Clock coming from internal CKOUT output - sampling point according SITP\[1:0\]
    #[inline(always)]
    pub fn is_ckout(&self) -> bool {
        *self == SPICKSEL::Ckout
    }
    ///Clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge)
    #[inline(always)]
    pub fn is_ckoutsecond_falling(&self) -> bool {
        *self == SPICKSEL::CkoutsecondFalling
    }
    ///Clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge)
    #[inline(always)]
    pub fn is_ckoutsecond_rising(&self) -> bool {
        *self == SPICKSEL::CkoutsecondRising
    }
}
///Field `SPICKSEL` writer - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type SPICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPICKSEL, crate::Safe>;
impl<'a, REG> SPICKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Clock coming from external CKINy input - sampling point according SITP\[1:0\]
    #[inline(always)]
    pub fn ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SPICKSEL::Ckin)
    }
    ///Clock coming from internal CKOUT output - sampling point according SITP\[1:0\]
    #[inline(always)]
    pub fn ckout(self) -> &'a mut crate::W<REG> {
        self.variant(SPICKSEL::Ckout)
    }
    ///Clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge)
    #[inline(always)]
    pub fn ckoutsecond_falling(self) -> &'a mut crate::W<REG> {
        self.variant(SPICKSEL::CkoutsecondFalling)
    }
    ///Clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge)
    #[inline(always)]
    pub fn ckoutsecond_rising(self) -> &'a mut crate::W<REG> {
        self.variant(SPICKSEL::CkoutsecondRising)
    }
}
/**Short-circuit detector enable on channel y

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCDEN {
    ///0: Input channel y will not be guarded by the short-circuit detector
    Disabled = 0,
    ///1: Input channel y will be continuously guarded by the short-circuit detector
    Enabled = 1,
}
impl From<SCDEN> for bool {
    #[inline(always)]
    fn from(variant: SCDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SCDEN` reader - Short-circuit detector enable on channel y
pub type SCDEN_R = crate::BitReader<SCDEN>;
impl SCDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCDEN {
        match self.bits {
            false => SCDEN::Disabled,
            true => SCDEN::Enabled,
        }
    }
    ///Input channel y will not be guarded by the short-circuit detector
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCDEN::Disabled
    }
    ///Input channel y will be continuously guarded by the short-circuit detector
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCDEN::Enabled
    }
}
///Field `SCDEN` writer - Short-circuit detector enable on channel y
pub type SCDEN_W<'a, REG> = crate::BitWriter<'a, REG, SCDEN>;
impl<'a, REG> SCDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input channel y will not be guarded by the short-circuit detector
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCDEN::Disabled)
    }
    ///Input channel y will be continuously guarded by the short-circuit detector
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCDEN::Enabled)
    }
}
/**Clock absence detector enable on channel y

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKABEN {
    ///0: Clock absence detector disabled on channel y
    Disabled = 0,
    ///1: Clock absence detector enabled on channel y
    Enabled = 1,
}
impl From<CKABEN> for bool {
    #[inline(always)]
    fn from(variant: CKABEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CKABEN` reader - Clock absence detector enable on channel y
pub type CKABEN_R = crate::BitReader<CKABEN>;
impl CKABEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKABEN {
        match self.bits {
            false => CKABEN::Disabled,
            true => CKABEN::Enabled,
        }
    }
    ///Clock absence detector disabled on channel y
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CKABEN::Disabled
    }
    ///Clock absence detector enabled on channel y
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CKABEN::Enabled
    }
}
///Field `CKABEN` writer - Clock absence detector enable on channel y
pub type CKABEN_W<'a, REG> = crate::BitWriter<'a, REG, CKABEN>;
impl<'a, REG> CKABEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock absence detector disabled on channel y
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CKABEN::Disabled)
    }
    ///Clock absence detector enabled on channel y
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CKABEN::Enabled)
    }
}
/**Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHEN {
    ///0: Channel y disabled
    Disabled = 0,
    ///1: Channel y enabled
    Enabled = 1,
}
impl From<CHEN> for bool {
    #[inline(always)]
    fn from(variant: CHEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CHEN` reader - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting.
pub type CHEN_R = crate::BitReader<CHEN>;
impl CHEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHEN {
        match self.bits {
            false => CHEN::Disabled,
            true => CHEN::Enabled,
        }
    }
    ///Channel y disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHEN::Disabled
    }
    ///Channel y enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHEN::Enabled
    }
}
///Field `CHEN` writer - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting.
pub type CHEN_W<'a, REG> = crate::BitWriter<'a, REG, CHEN>;
impl<'a, REG> CHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel y disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHEN::Disabled)
    }
    ///Channel y enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHEN::Enabled)
    }
}
/**Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHINSEL {
    ///0: Channel inputs are taken from pins of the same channel y
    SameChannel = 0,
    ///1: Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)
    FollowingChannel = 1,
}
impl From<CHINSEL> for bool {
    #[inline(always)]
    fn from(variant: CHINSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `CHINSEL` reader - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type CHINSEL_R = crate::BitReader<CHINSEL>;
impl CHINSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHINSEL {
        match self.bits {
            false => CHINSEL::SameChannel,
            true => CHINSEL::FollowingChannel,
        }
    }
    ///Channel inputs are taken from pins of the same channel y
    #[inline(always)]
    pub fn is_same_channel(&self) -> bool {
        *self == CHINSEL::SameChannel
    }
    ///Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)
    #[inline(always)]
    pub fn is_following_channel(&self) -> bool {
        *self == CHINSEL::FollowingChannel
    }
}
///Field `CHINSEL` writer - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type CHINSEL_W<'a, REG> = crate::BitWriter<'a, REG, CHINSEL>;
impl<'a, REG> CHINSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel inputs are taken from pins of the same channel y
    #[inline(always)]
    pub fn same_channel(self) -> &'a mut crate::W<REG> {
        self.variant(CHINSEL::SameChannel)
    }
    ///Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)
    #[inline(always)]
    pub fn following_channel(self) -> &'a mut crate::W<REG> {
        self.variant(CHINSEL::FollowingChannel)
    }
}
/**Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\[1:0\] bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATMPX {
    ///0: Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected
    External = 0,
    ///1: Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\[15:0\] part of DFSDM_CHyDATINR register
    Adc = 1,
    ///2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\[1:0\] bit field setting
    Internal = 2,
}
impl From<DATMPX> for u8 {
    #[inline(always)]
    fn from(variant: DATMPX) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATMPX {
    type Ux = u8;
}
impl crate::IsEnum for DATMPX {}
///Field `DATMPX` reader - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\[1:0\] bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type DATMPX_R = crate::FieldReader<DATMPX>;
impl DATMPX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATMPX> {
        match self.bits {
            0 => Some(DATMPX::External),
            1 => Some(DATMPX::Adc),
            2 => Some(DATMPX::Internal),
            _ => None,
        }
    }
    ///Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == DATMPX::External
    }
    ///Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\[15:0\] part of DFSDM_CHyDATINR register
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == DATMPX::Adc
    }
    ///Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\[1:0\] bit field setting
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == DATMPX::Internal
    }
}
///Field `DATMPX` writer - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\[1:0\] bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type DATMPX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATMPX>;
impl<'a, REG> DATMPX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(DATMPX::External)
    }
    ///Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\[15:0\] part of DFSDM_CHyDATINR register
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(DATMPX::Adc)
    }
    ///Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\[1:0\] bit field setting
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(DATMPX::Internal)
    }
}
/**Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\[15:0\] part is read as first sample and then INDAT1\[15:0\] part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\[1:0\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATPACK {
    ///0: Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\[15:0\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y
    Standard = 0,
    ///1: : Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample in INDAT0\[15:0\] (assigned to channel y) –second sample INDAT1\[15:0\] (assigned to channel y)
    Interleaved = 1,
    ///2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample INDAT0\[15:0\] (assigned to channel y) –second sample INDAT1\[15:0\] (assigned to channel y+1)
    Dual = 2,
}
impl From<DATPACK> for u8 {
    #[inline(always)]
    fn from(variant: DATPACK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATPACK {
    type Ux = u8;
}
impl crate::IsEnum for DATPACK {}
///Field `DATPACK` reader - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\[15:0\] part is read as first sample and then INDAT1\[15:0\] part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\[1:0\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type DATPACK_R = crate::FieldReader<DATPACK>;
impl DATPACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATPACK> {
        match self.bits {
            0 => Some(DATPACK::Standard),
            1 => Some(DATPACK::Interleaved),
            2 => Some(DATPACK::Dual),
            _ => None,
        }
    }
    ///Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\[15:0\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DATPACK::Standard
    }
    ///: Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample in INDAT0\[15:0\] (assigned to channel y) –second sample INDAT1\[15:0\] (assigned to channel y)
    #[inline(always)]
    pub fn is_interleaved(&self) -> bool {
        *self == DATPACK::Interleaved
    }
    ///Dual: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample INDAT0\[15:0\] (assigned to channel y) –second sample INDAT1\[15:0\] (assigned to channel y+1)
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DATPACK::Dual
    }
}
///Field `DATPACK` writer - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\[15:0\] part is read as first sample and then INDAT1\[15:0\] part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\[1:0\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type DATPACK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATPACK>;
impl<'a, REG> DATPACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\[15:0\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(DATPACK::Standard)
    }
    ///: Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample in INDAT0\[15:0\] (assigned to channel y) –second sample INDAT1\[15:0\] (assigned to channel y)
    #[inline(always)]
    pub fn interleaved(self) -> &'a mut crate::W<REG> {
        self.variant(DATPACK::Interleaved)
    }
    ///Dual: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample INDAT0\[15:0\] (assigned to channel y) –second sample INDAT1\[15:0\] (assigned to channel y+1)
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(DATPACK::Dual)
    }
}
///Field `CKOUTDIV` reader - Output serial clock divider 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2-
pub type CKOUTDIV_R = crate::FieldReader;
///Field `CKOUTDIV` writer - Output serial clock divider 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2-
pub type CKOUTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
/**Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKOUTSRC {
    ///0: Source for output clock is from system clock
    Sysclk = 0,
    ///1: Source for output clock is from audio clock
    Audclk = 1,
}
impl From<CKOUTSRC> for bool {
    #[inline(always)]
    fn from(variant: CKOUTSRC) -> Self {
        variant as u8 != 0
    }
}
///Field `CKOUTSRC` reader - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)
pub type CKOUTSRC_R = crate::BitReader<CKOUTSRC>;
impl CKOUTSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKOUTSRC {
        match self.bits {
            false => CKOUTSRC::Sysclk,
            true => CKOUTSRC::Audclk,
        }
    }
    ///Source for output clock is from system clock
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CKOUTSRC::Sysclk
    }
    ///Source for output clock is from audio clock
    #[inline(always)]
    pub fn is_audclk(&self) -> bool {
        *self == CKOUTSRC::Audclk
    }
}
///Field `CKOUTSRC` writer - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)
pub type CKOUTSRC_W<'a, REG> = crate::BitWriter<'a, REG, CKOUTSRC>;
impl<'a, REG> CKOUTSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Source for output clock is from system clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUTSRC::Sysclk)
    }
    ///Source for output clock is from audio clock
    #[inline(always)]
    pub fn audclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUTSRC::Audclk)
    }
}
/**Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDMEN {
    ///0: DFSDM interface disabled
    Disabled = 0,
    ///1: DFSDM interface enabled
    Enabled = 1,
}
impl From<DFSDMEN> for bool {
    #[inline(always)]
    fn from(variant: DFSDMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DFSDMEN` reader - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)
pub type DFSDMEN_R = crate::BitReader<DFSDMEN>;
impl DFSDMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFSDMEN {
        match self.bits {
            false => DFSDMEN::Disabled,
            true => DFSDMEN::Enabled,
        }
    }
    ///DFSDM interface disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFSDMEN::Disabled
    }
    ///DFSDM interface enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFSDMEN::Enabled
    }
}
///Field `DFSDMEN` writer - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)
pub type DFSDMEN_W<'a, REG> = crate::BitWriter<'a, REG, DFSDMEN>;
impl<'a, REG> DFSDMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DFSDM interface disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDMEN::Disabled)
    }
    ///DFSDM interface enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDMEN::Enabled)
    }
}
impl R {
    ///Bits 0:1 - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 5 - Short-circuit detector enable on channel y
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Clock absence detector enable on channel y
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting.
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:13 - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\[1:0\] bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\[15:0\] part is read as first sample and then INDAT1\[15:0\] part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\[1:0\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:23 - Output serial clock divider 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2-
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 30 - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("sitp", &self.sitp())
            .field("spicksel", &self.spicksel())
            .field("scden", &self.scden())
            .field("ckaben", &self.ckaben())
            .field("chen", &self.chen())
            .field("chinsel", &self.chinsel())
            .field("datmpx", &self.datmpx())
            .field("datpack", &self.datpack())
            .field("ckoutdiv", &self.ckoutdiv())
            .field("ckoutsrc", &self.ckoutsrc())
            .field("dfsdmen", &self.dfsdmen())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn sitp(&mut self) -> SITP_W<'_, CFGR1rs> {
        SITP_W::new(self, 0)
    }
    ///Bits 2:3 - SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn spicksel(&mut self) -> SPICKSEL_W<'_, CFGR1rs> {
        SPICKSEL_W::new(self, 2)
    }
    ///Bit 5 - Short-circuit detector enable on channel y
    #[inline(always)]
    pub fn scden(&mut self) -> SCDEN_W<'_, CFGR1rs> {
        SCDEN_W::new(self, 5)
    }
    ///Bit 6 - Clock absence detector enable on channel y
    #[inline(always)]
    pub fn ckaben(&mut self) -> CKABEN_W<'_, CFGR1rs> {
        CKABEN_W::new(self, 6)
    }
    ///Bit 7 - Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting.
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W<'_, CFGR1rs> {
        CHEN_W::new(self, 7)
    }
    ///Bit 8 - Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn chinsel(&mut self) -> CHINSEL_W<'_, CFGR1rs> {
        CHINSEL_W::new(self, 8)
    }
    ///Bits 12:13 - Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\[1:0\] bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn datmpx(&mut self) -> DATMPX_W<'_, CFGR1rs> {
        DATMPX_W::new(self, 12)
    }
    ///Bits 14:15 - Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\[15:0\] part is read as first sample and then INDAT1\[15:0\] part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\[15:0\] (assigned to channel y) second sample INDAT1\[15:0\] (assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\[1:0\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn datpack(&mut self) -> DATPACK_W<'_, CFGR1rs> {
        DATPACK_W::new(self, 14)
    }
    ///Bits 16:23 - Output serial clock divider 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2-
    #[inline(always)]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W<'_, CFGR1rs> {
        CKOUTDIV_W::new(self, 16)
    }
    ///Bit 30 - Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)
    #[inline(always)]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W<'_, CFGR1rs> {
        CKOUTSRC_W::new(self, 30)
    }
    ///Bit 31 - Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<'_, CFGR1rs> {
        DFSDMEN_W::new(self, 31)
    }
}
/**DFSDM channel 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
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
