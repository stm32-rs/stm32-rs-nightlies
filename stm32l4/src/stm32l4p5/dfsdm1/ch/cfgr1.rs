#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1rs>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1rs>;
#[doc = "SITP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SITP {
    #[doc = "0: SPI with rising edge to strobe data"]
    SpirisingEdge = 0,
    #[doc = "1: SPI with falling edge to strobe data"]
    SpifallingEdge = 1,
    #[doc = "2: Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1"]
    Manchester = 2,
    #[doc = "3: Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0"]
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
#[doc = "Field `SITP` reader - SITP"]
pub type SITP_R = crate::FieldReader<SITP>;
impl SITP_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "SPI with rising edge to strobe data"]
    #[inline(always)]
    pub fn is_spirising_edge(&self) -> bool {
        *self == SITP::SpirisingEdge
    }
    #[doc = "SPI with falling edge to strobe data"]
    #[inline(always)]
    pub fn is_spifalling_edge(&self) -> bool {
        *self == SITP::SpifallingEdge
    }
    #[doc = "Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1"]
    #[inline(always)]
    pub fn is_manchester(&self) -> bool {
        *self == SITP::Manchester
    }
    #[doc = "Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0"]
    #[inline(always)]
    pub fn is_manchester_inverted(&self) -> bool {
        *self == SITP::ManchesterInverted
    }
}
#[doc = "Field `SITP` writer - SITP"]
pub type SITP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SITP>;
impl<'a, REG> SITP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI with rising edge to strobe data"]
    #[inline(always)]
    pub fn spirising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SITP::SpirisingEdge)
    }
    #[doc = "SPI with falling edge to strobe data"]
    #[inline(always)]
    pub fn spifalling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SITP::SpifallingEdge)
    }
    #[doc = "Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1"]
    #[inline(always)]
    pub fn manchester(self) -> &'a mut crate::W<REG> {
        self.variant(SITP::Manchester)
    }
    #[doc = "Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0"]
    #[inline(always)]
    pub fn manchester_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(SITP::ManchesterInverted)
    }
}
#[doc = "SPICKSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPICKSEL {
    #[doc = "0: Clock coming from external CKINy input - sampling point according SITP\\[1:0\\]"]
    Ckin = 0,
    #[doc = "1: Clock coming from internal CKOUT output - sampling point according SITP\\[1:0\\]"]
    Ckout = 1,
    #[doc = "2: Clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge)"]
    CkoutsecondFalling = 2,
    #[doc = "3: Clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge)"]
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
#[doc = "Field `SPICKSEL` reader - SPICKSEL"]
pub type SPICKSEL_R = crate::FieldReader<SPICKSEL>;
impl SPICKSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Clock coming from external CKINy input - sampling point according SITP\\[1:0\\]"]
    #[inline(always)]
    pub fn is_ckin(&self) -> bool {
        *self == SPICKSEL::Ckin
    }
    #[doc = "Clock coming from internal CKOUT output - sampling point according SITP\\[1:0\\]"]
    #[inline(always)]
    pub fn is_ckout(&self) -> bool {
        *self == SPICKSEL::Ckout
    }
    #[doc = "Clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge)"]
    #[inline(always)]
    pub fn is_ckoutsecond_falling(&self) -> bool {
        *self == SPICKSEL::CkoutsecondFalling
    }
    #[doc = "Clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge)"]
    #[inline(always)]
    pub fn is_ckoutsecond_rising(&self) -> bool {
        *self == SPICKSEL::CkoutsecondRising
    }
}
#[doc = "Field `SPICKSEL` writer - SPICKSEL"]
pub type SPICKSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SPICKSEL>;
impl<'a, REG> SPICKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock coming from external CKINy input - sampling point according SITP\\[1:0\\]"]
    #[inline(always)]
    pub fn ckin(self) -> &'a mut crate::W<REG> {
        self.variant(SPICKSEL::Ckin)
    }
    #[doc = "Clock coming from internal CKOUT output - sampling point according SITP\\[1:0\\]"]
    #[inline(always)]
    pub fn ckout(self) -> &'a mut crate::W<REG> {
        self.variant(SPICKSEL::Ckout)
    }
    #[doc = "Clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge)"]
    #[inline(always)]
    pub fn ckoutsecond_falling(self) -> &'a mut crate::W<REG> {
        self.variant(SPICKSEL::CkoutsecondFalling)
    }
    #[doc = "Clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external Σ∆ modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge)"]
    #[inline(always)]
    pub fn ckoutsecond_rising(self) -> &'a mut crate::W<REG> {
        self.variant(SPICKSEL::CkoutsecondRising)
    }
}
#[doc = "SCDEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCDEN {
    #[doc = "0: Input channel y will not be guarded by the short-circuit detector"]
    Disabled = 0,
    #[doc = "1: Input channel y will be continuously guarded by the short-circuit detector"]
    Enabled = 1,
}
impl From<SCDEN> for bool {
    #[inline(always)]
    fn from(variant: SCDEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCDEN` reader - SCDEN"]
pub type SCDEN_R = crate::BitReader<SCDEN>;
impl SCDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCDEN {
        match self.bits {
            false => SCDEN::Disabled,
            true => SCDEN::Enabled,
        }
    }
    #[doc = "Input channel y will not be guarded by the short-circuit detector"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCDEN::Disabled
    }
    #[doc = "Input channel y will be continuously guarded by the short-circuit detector"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCDEN::Enabled
    }
}
#[doc = "Field `SCDEN` writer - SCDEN"]
pub type SCDEN_W<'a, REG> = crate::BitWriter<'a, REG, SCDEN>;
impl<'a, REG> SCDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input channel y will not be guarded by the short-circuit detector"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCDEN::Disabled)
    }
    #[doc = "Input channel y will be continuously guarded by the short-circuit detector"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCDEN::Enabled)
    }
}
#[doc = "CKABEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKABEN {
    #[doc = "0: Clock absence detector disabled on channel y"]
    Disabled = 0,
    #[doc = "1: Clock absence detector enabled on channel y"]
    Enabled = 1,
}
impl From<CKABEN> for bool {
    #[inline(always)]
    fn from(variant: CKABEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKABEN` reader - CKABEN"]
pub type CKABEN_R = crate::BitReader<CKABEN>;
impl CKABEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKABEN {
        match self.bits {
            false => CKABEN::Disabled,
            true => CKABEN::Enabled,
        }
    }
    #[doc = "Clock absence detector disabled on channel y"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CKABEN::Disabled
    }
    #[doc = "Clock absence detector enabled on channel y"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CKABEN::Enabled
    }
}
#[doc = "Field `CKABEN` writer - CKABEN"]
pub type CKABEN_W<'a, REG> = crate::BitWriter<'a, REG, CKABEN>;
impl<'a, REG> CKABEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock absence detector disabled on channel y"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CKABEN::Disabled)
    }
    #[doc = "Clock absence detector enabled on channel y"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CKABEN::Enabled)
    }
}
#[doc = "CHEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHEN {
    #[doc = "0: Channel y disabled"]
    Disabled = 0,
    #[doc = "1: Channel y enabled"]
    Enabled = 1,
}
impl From<CHEN> for bool {
    #[inline(always)]
    fn from(variant: CHEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHEN` reader - CHEN"]
pub type CHEN_R = crate::BitReader<CHEN>;
impl CHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHEN {
        match self.bits {
            false => CHEN::Disabled,
            true => CHEN::Enabled,
        }
    }
    #[doc = "Channel y disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHEN::Disabled
    }
    #[doc = "Channel y enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHEN::Enabled
    }
}
#[doc = "Field `CHEN` writer - CHEN"]
pub type CHEN_W<'a, REG> = crate::BitWriter<'a, REG, CHEN>;
impl<'a, REG> CHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel y disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHEN::Disabled)
    }
    #[doc = "Channel y enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CHEN::Enabled)
    }
}
#[doc = "CHINSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHINSEL {
    #[doc = "0: Channel inputs are taken from pins of the same channel y"]
    SameChannel = 0,
    #[doc = "1: Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)"]
    FollowingChannel = 1,
}
impl From<CHINSEL> for bool {
    #[inline(always)]
    fn from(variant: CHINSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHINSEL` reader - CHINSEL"]
pub type CHINSEL_R = crate::BitReader<CHINSEL>;
impl CHINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHINSEL {
        match self.bits {
            false => CHINSEL::SameChannel,
            true => CHINSEL::FollowingChannel,
        }
    }
    #[doc = "Channel inputs are taken from pins of the same channel y"]
    #[inline(always)]
    pub fn is_same_channel(&self) -> bool {
        *self == CHINSEL::SameChannel
    }
    #[doc = "Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)"]
    #[inline(always)]
    pub fn is_following_channel(&self) -> bool {
        *self == CHINSEL::FollowingChannel
    }
}
#[doc = "Field `CHINSEL` writer - CHINSEL"]
pub type CHINSEL_W<'a, REG> = crate::BitWriter<'a, REG, CHINSEL>;
impl<'a, REG> CHINSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel inputs are taken from pins of the same channel y"]
    #[inline(always)]
    pub fn same_channel(self) -> &'a mut crate::W<REG> {
        self.variant(CHINSEL::SameChannel)
    }
    #[doc = "Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)"]
    #[inline(always)]
    pub fn following_channel(self) -> &'a mut crate::W<REG> {
        self.variant(CHINSEL::FollowingChannel)
    }
}
#[doc = "DATMPX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATMPX {
    #[doc = "0: Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected"]
    External = 0,
    #[doc = "1: Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\\[15:0\\]
part of DFSDM_CHyDATINR register"]
    Adc = 1,
    #[doc = "2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting"]
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
#[doc = "Field `DATMPX` reader - DATMPX"]
pub type DATMPX_R = crate::FieldReader<DATMPX>;
impl DATMPX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATMPX> {
        match self.bits {
            0 => Some(DATMPX::External),
            1 => Some(DATMPX::Adc),
            2 => Some(DATMPX::Internal),
            _ => None,
        }
    }
    #[doc = "Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == DATMPX::External
    }
    #[doc = "Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\\[15:0\\]
part of DFSDM_CHyDATINR register"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == DATMPX::Adc
    }
    #[doc = "Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == DATMPX::Internal
    }
}
#[doc = "Field `DATMPX` writer - DATMPX"]
pub type DATMPX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATMPX>;
impl<'a, REG> DATMPX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(DATMPX::External)
    }
    #[doc = "Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\\[15:0\\]
part of DFSDM_CHyDATINR register"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(DATMPX::Adc)
    }
    #[doc = "Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(DATMPX::Internal)
    }
}
#[doc = "DATPACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATPACK {
    #[doc = "0: Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\\[15:0\\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y"]
    Standard = 0,
    #[doc = "1: : Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample in INDAT0\\[15:0\\]
(assigned to channel y) –second sample INDAT1\\[15:0\\]
(assigned to channel y)"]
    Interleaved = 1,
    #[doc = "2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample INDAT0\\[15:0\\]
(assigned to channel y) –second sample INDAT1\\[15:0\\]
(assigned to channel y+1)"]
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
#[doc = "Field `DATPACK` reader - DATPACK"]
pub type DATPACK_R = crate::FieldReader<DATPACK>;
impl DATPACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATPACK> {
        match self.bits {
            0 => Some(DATPACK::Standard),
            1 => Some(DATPACK::Interleaved),
            2 => Some(DATPACK::Dual),
            _ => None,
        }
    }
    #[doc = "Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\\[15:0\\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DATPACK::Standard
    }
    #[doc = ": Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample in INDAT0\\[15:0\\]
(assigned to channel y) –second sample INDAT1\\[15:0\\]
(assigned to channel y)"]
    #[inline(always)]
    pub fn is_interleaved(&self) -> bool {
        *self == DATPACK::Interleaved
    }
    #[doc = "Dual: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample INDAT0\\[15:0\\]
(assigned to channel y) –second sample INDAT1\\[15:0\\]
(assigned to channel y+1)"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DATPACK::Dual
    }
}
#[doc = "Field `DATPACK` writer - DATPACK"]
pub type DATPACK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATPACK>;
impl<'a, REG> DATPACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\\[15:0\\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(DATPACK::Standard)
    }
    #[doc = ": Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample in INDAT0\\[15:0\\]
(assigned to channel y) –second sample INDAT1\\[15:0\\]
(assigned to channel y)"]
    #[inline(always)]
    pub fn interleaved(self) -> &'a mut crate::W<REG> {
        self.variant(DATPACK::Interleaved)
    }
    #[doc = "Dual: input data in DFSDM_CHyDATINR register are stored as two samples: –first sample INDAT0\\[15:0\\]
(assigned to channel y) –second sample INDAT1\\[15:0\\]
(assigned to channel y+1)"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(DATPACK::Dual)
    }
}
#[doc = "Field `CKOUTDIV` reader - CKOUTDIV"]
pub type CKOUTDIV_R = crate::FieldReader;
#[doc = "Field `CKOUTDIV` writer - CKOUTDIV"]
pub type CKOUTDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "CKOUTSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKOUTSRC {
    #[doc = "0: Source for output clock is from system clock"]
    Sysclk = 0,
    #[doc = "1: Source for output clock is from audio clock"]
    Audclk = 1,
}
impl From<CKOUTSRC> for bool {
    #[inline(always)]
    fn from(variant: CKOUTSRC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKOUTSRC` reader - CKOUTSRC"]
pub type CKOUTSRC_R = crate::BitReader<CKOUTSRC>;
impl CKOUTSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKOUTSRC {
        match self.bits {
            false => CKOUTSRC::Sysclk,
            true => CKOUTSRC::Audclk,
        }
    }
    #[doc = "Source for output clock is from system clock"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CKOUTSRC::Sysclk
    }
    #[doc = "Source for output clock is from audio clock"]
    #[inline(always)]
    pub fn is_audclk(&self) -> bool {
        *self == CKOUTSRC::Audclk
    }
}
#[doc = "Field `CKOUTSRC` writer - CKOUTSRC"]
pub type CKOUTSRC_W<'a, REG> = crate::BitWriter<'a, REG, CKOUTSRC>;
impl<'a, REG> CKOUTSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Source for output clock is from system clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUTSRC::Sysclk)
    }
    #[doc = "Source for output clock is from audio clock"]
    #[inline(always)]
    pub fn audclk(self) -> &'a mut crate::W<REG> {
        self.variant(CKOUTSRC::Audclk)
    }
}
#[doc = "DFSDMEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFSDMEN {
    #[doc = "0: DFSDM interface disabled"]
    Disabled = 0,
    #[doc = "1: DFSDM interface enabled"]
    Enabled = 1,
}
impl From<DFSDMEN> for bool {
    #[inline(always)]
    fn from(variant: DFSDMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFSDMEN` reader - DFSDMEN"]
pub type DFSDMEN_R = crate::BitReader<DFSDMEN>;
impl DFSDMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DFSDMEN {
        match self.bits {
            false => DFSDMEN::Disabled,
            true => DFSDMEN::Enabled,
        }
    }
    #[doc = "DFSDM interface disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFSDMEN::Disabled
    }
    #[doc = "DFSDM interface enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFSDMEN::Enabled
    }
}
#[doc = "Field `DFSDMEN` writer - DFSDMEN"]
pub type DFSDMEN_W<'a, REG> = crate::BitWriter<'a, REG, DFSDMEN>;
impl<'a, REG> DFSDMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DFSDM interface disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDMEN::Disabled)
    }
    #[doc = "DFSDM interface enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFSDMEN::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - CKOUTDIV"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    #[must_use]
    pub fn sitp(&mut self) -> SITP_W<CFGR1rs> {
        SITP_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn spicksel(&mut self) -> SPICKSEL_W<CFGR1rs> {
        SPICKSEL_W::new(self, 2)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    #[must_use]
    pub fn scden(&mut self) -> SCDEN_W<CFGR1rs> {
        SCDEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    #[must_use]
    pub fn ckaben(&mut self) -> CKABEN_W<CFGR1rs> {
        CKABEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<CFGR1rs> {
        CHEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    #[must_use]
    pub fn chinsel(&mut self) -> CHINSEL_W<CFGR1rs> {
        CHINSEL_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    #[must_use]
    pub fn datmpx(&mut self) -> DATMPX_W<CFGR1rs> {
        DATMPX_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    #[must_use]
    pub fn datpack(&mut self) -> DATPACK_W<CFGR1rs> {
        DATPACK_W::new(self, 14)
    }
    #[doc = "Bits 16:23 - CKOUTDIV"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W<CFGR1rs> {
        CKOUTDIV_W::new(self, 16)
    }
    #[doc = "Bit 30 - CKOUTSRC"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W<CFGR1rs> {
        CKOUTSRC_W::new(self, 30)
    }
    #[doc = "Bit 31 - DFSDMEN"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<CFGR1rs> {
        DFSDMEN_W::new(self, 31)
    }
}
#[doc = "channel configuration y register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
