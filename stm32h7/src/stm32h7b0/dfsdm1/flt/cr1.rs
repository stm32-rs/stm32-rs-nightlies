///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
/**DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFEN {
    ///0: DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped
    Disabled = 0,
    ///1: DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting
    Enabled = 1,
}
impl From<DFEN> for bool {
    #[inline(always)]
    fn from(variant: DFEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DFEN` reader - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state
pub type DFEN_R = crate::BitReader<DFEN>;
impl DFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DFEN {
        match self.bits {
            false => DFEN::Disabled,
            true => DFEN::Enabled,
        }
    }
    ///DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFEN::Disabled
    }
    ///DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DFEN::Enabled
    }
}
///Field `DFEN` writer - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state
pub type DFEN_W<'a, REG> = crate::BitWriter<'a, REG, DFEN>;
impl<'a, REG> DFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFEN::Disabled)
    }
    ///DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DFEN::Enabled)
    }
}
/**Start a conversion of the injected group of channels This bit is always read as '0'.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTW {
    ///1: Writing ‘1’ makes a request to convert the channels in the injected conversion group, causing JCIP to become ‘1’ at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing ‘1’ has no effect if JSYNC=1
    Start = 1,
}
impl From<JSWSTARTW> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `JSWSTART` reader - Start a conversion of the injected group of channels This bit is always read as '0'.
pub type JSWSTART_R = crate::BitReader<JSWSTARTW>;
impl JSWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<JSWSTARTW> {
        match self.bits {
            true => Some(JSWSTARTW::Start),
            _ => None,
        }
    }
    ///Writing ‘1’ makes a request to convert the channels in the injected conversion group, causing JCIP to become ‘1’ at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing ‘1’ has no effect if JSYNC=1
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == JSWSTARTW::Start
    }
}
///Field `JSWSTART` writer - Start a conversion of the injected group of channels This bit is always read as '0'.
pub type JSWSTART_W<'a, REG> = crate::BitWriter<'a, REG, JSWSTARTW>;
impl<'a, REG> JSWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing ‘1’ makes a request to convert the channels in the injected conversion group, causing JCIP to become ‘1’ at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing ‘1’ has no effect if JSYNC=1
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(JSWSTARTW::Start)
    }
}
/**Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSYNC {
    ///0: Do not launch an injected conversion synchronously with DFSDM_FLT0
    Disabled = 0,
    ///1: Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger
    Enabled = 1,
}
impl From<JSYNC> for bool {
    #[inline(always)]
    fn from(variant: JSYNC) -> Self {
        variant as u8 != 0
    }
}
///Field `JSYNC` reader - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JSYNC_R = crate::BitReader<JSYNC>;
impl JSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JSYNC {
        match self.bits {
            false => JSYNC::Disabled,
            true => JSYNC::Enabled,
        }
    }
    ///Do not launch an injected conversion synchronously with DFSDM_FLT0
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JSYNC::Disabled
    }
    ///Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JSYNC::Enabled
    }
}
///Field `JSYNC` writer - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JSYNC_W<'a, REG> = crate::BitWriter<'a, REG, JSYNC>;
impl<'a, REG> JSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not launch an injected conversion synchronously with DFSDM_FLT0
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JSYNC::Disabled)
    }
    ///Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JSYNC::Enabled)
    }
}
/**Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSCAN {
    ///0: One channel conversion is performed from the injected channel group and next the selected channel from this group is selected
    Single = 0,
    ///1: The series of conversions for the injected group channels is executed, starting over with the lowest selected channel
    Series = 1,
}
impl From<JSCAN> for bool {
    #[inline(always)]
    fn from(variant: JSCAN) -> Self {
        variant as u8 != 0
    }
}
///Field `JSCAN` reader - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.
pub type JSCAN_R = crate::BitReader<JSCAN>;
impl JSCAN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JSCAN {
        match self.bits {
            false => JSCAN::Single,
            true => JSCAN::Series,
        }
    }
    ///One channel conversion is performed from the injected channel group and next the selected channel from this group is selected
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == JSCAN::Single
    }
    ///The series of conversions for the injected group channels is executed, starting over with the lowest selected channel
    #[inline(always)]
    pub fn is_series(&self) -> bool {
        *self == JSCAN::Series
    }
}
///Field `JSCAN` writer - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.
pub type JSCAN_W<'a, REG> = crate::BitWriter<'a, REG, JSCAN>;
impl<'a, REG> JSCAN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///One channel conversion is performed from the injected channel group and next the selected channel from this group is selected
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(JSCAN::Single)
    }
    ///The series of conversions for the injected group channels is executed, starting over with the lowest selected channel
    #[inline(always)]
    pub fn series(self) -> &'a mut crate::W<REG> {
        self.variant(JSCAN::Series)
    }
}
/**DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDMAEN {
    ///0: The DMA channel is not enabled to read injected data
    Disabled = 0,
    ///1: The DMA channel is enabled to read injected data
    Enabled = 1,
}
impl From<JDMAEN> for bool {
    #[inline(always)]
    fn from(variant: JDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `JDMAEN` reader - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JDMAEN_R = crate::BitReader<JDMAEN>;
impl JDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JDMAEN {
        match self.bits {
            false => JDMAEN::Disabled,
            true => JDMAEN::Enabled,
        }
    }
    ///The DMA channel is not enabled to read injected data
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDMAEN::Disabled
    }
    ///The DMA channel is enabled to read injected data
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDMAEN::Enabled
    }
}
///Field `JDMAEN` writer - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, JDMAEN>;
impl<'a, REG> JDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The DMA channel is not enabled to read injected data
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDMAEN::Disabled)
    }
    ///The DMA channel is enabled to read injected data
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JDMAEN::Enabled)
    }
}
///Field `JEXTSEL` reader - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger).
pub type JEXTSEL_R = crate::FieldReader;
///Field `JEXTSEL` writer - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger).
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTEN {
    ///0: Trigger detection is disabled
    Disabled = 0,
    ///1: Each rising edge on the selected trigger makes a request to launch an injected conversion
    RisingEdge = 1,
    ///2: Each falling edge on the selected trigger makes a request to launch an injected conversion
    FallingEdge = 2,
    ///3: Both rising edges and falling edges on the selected trigger make requests to launch injected conversions
    BothEdges = 3,
}
impl From<JEXTEN> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTEN {
    type Ux = u8;
}
impl crate::IsEnum for JEXTEN {}
///Field `JEXTEN` reader - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JEXTEN_R = crate::FieldReader<JEXTEN>;
impl JEXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEXTEN {
        match self.bits {
            0 => JEXTEN::Disabled,
            1 => JEXTEN::RisingEdge,
            2 => JEXTEN::FallingEdge,
            3 => JEXTEN::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Trigger detection is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN::Disabled
    }
    ///Each rising edge on the selected trigger makes a request to launch an injected conversion
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN::RisingEdge
    }
    ///Each falling edge on the selected trigger makes a request to launch an injected conversion
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN::FallingEdge
    }
    ///Both rising edges and falling edges on the selected trigger make requests to launch injected conversions
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN::BothEdges
    }
}
///Field `JEXTEN` writer - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, JEXTEN, crate::Safe>;
impl<'a, REG> JEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Trigger detection is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::Disabled)
    }
    ///Each rising edge on the selected trigger makes a request to launch an injected conversion
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::RisingEdge)
    }
    ///Each falling edge on the selected trigger makes a request to launch an injected conversion
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::FallingEdge)
    }
    ///Both rising edges and falling edges on the selected trigger make requests to launch injected conversions
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::BothEdges)
    }
}
/**Software start of a conversion on the regular channel This bit is always read as '0'.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSWSTARTW {
    ///1: Writing ‘1’ makes a request to start a conversion on the regular channel and causes RCIP to become ‘1’. If RCIP=1 already, writing to RSWSTART has no effect. Writing ‘1’ has no effect if RSYNC=1
    Start = 1,
}
impl From<RSWSTARTW> for bool {
    #[inline(always)]
    fn from(variant: RSWSTARTW) -> Self {
        variant as u8 != 0
    }
}
///Field `RSWSTART` reader - Software start of a conversion on the regular channel This bit is always read as '0'.
pub type RSWSTART_R = crate::BitReader<RSWSTARTW>;
impl RSWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RSWSTARTW> {
        match self.bits {
            true => Some(RSWSTARTW::Start),
            _ => None,
        }
    }
    ///Writing ‘1’ makes a request to start a conversion on the regular channel and causes RCIP to become ‘1’. If RCIP=1 already, writing to RSWSTART has no effect. Writing ‘1’ has no effect if RSYNC=1
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RSWSTARTW::Start
    }
}
///Field `RSWSTART` writer - Software start of a conversion on the regular channel This bit is always read as '0'.
pub type RSWSTART_W<'a, REG> = crate::BitWriter<'a, REG, RSWSTARTW>;
impl<'a, REG> RSWSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing ‘1’ makes a request to start a conversion on the regular channel and causes RCIP to become ‘1’. If RCIP=1 already, writing to RSWSTART has no effect. Writing ‘1’ has no effect if RSYNC=1
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RSWSTARTW::Start)
    }
}
/**Continuous mode selection for regular conversions Writing '0' to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCONT {
    ///0: The regular channel is converted just once for each conversion request
    Once = 0,
    ///1: The regular channel is converted repeatedly after each conversion request
    Continuous = 1,
}
impl From<RCONT> for bool {
    #[inline(always)]
    fn from(variant: RCONT) -> Self {
        variant as u8 != 0
    }
}
///Field `RCONT` reader - Continuous mode selection for regular conversions Writing '0' to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.
pub type RCONT_R = crate::BitReader<RCONT>;
impl RCONT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCONT {
        match self.bits {
            false => RCONT::Once,
            true => RCONT::Continuous,
        }
    }
    ///The regular channel is converted just once for each conversion request
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == RCONT::Once
    }
    ///The regular channel is converted repeatedly after each conversion request
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == RCONT::Continuous
    }
}
///Field `RCONT` writer - Continuous mode selection for regular conversions Writing '0' to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.
pub type RCONT_W<'a, REG> = crate::BitWriter<'a, REG, RCONT>;
impl<'a, REG> RCONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The regular channel is converted just once for each conversion request
    #[inline(always)]
    pub fn once(self) -> &'a mut crate::W<REG> {
        self.variant(RCONT::Once)
    }
    ///The regular channel is converted repeatedly after each conversion request
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(RCONT::Continuous)
    }
}
/**Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSYNC {
    ///0: Do not launch a regular conversion synchronously with DFSDM_FLT0
    NoLaunch = 0,
    ///1: Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0
    Launch = 1,
}
impl From<RSYNC> for bool {
    #[inline(always)]
    fn from(variant: RSYNC) -> Self {
        variant as u8 != 0
    }
}
///Field `RSYNC` reader - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type RSYNC_R = crate::BitReader<RSYNC>;
impl RSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSYNC {
        match self.bits {
            false => RSYNC::NoLaunch,
            true => RSYNC::Launch,
        }
    }
    ///Do not launch a regular conversion synchronously with DFSDM_FLT0
    #[inline(always)]
    pub fn is_no_launch(&self) -> bool {
        *self == RSYNC::NoLaunch
    }
    ///Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0
    #[inline(always)]
    pub fn is_launch(&self) -> bool {
        *self == RSYNC::Launch
    }
}
///Field `RSYNC` writer - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type RSYNC_W<'a, REG> = crate::BitWriter<'a, REG, RSYNC>;
impl<'a, REG> RSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not launch a regular conversion synchronously with DFSDM_FLT0
    #[inline(always)]
    pub fn no_launch(self) -> &'a mut crate::W<REG> {
        self.variant(RSYNC::NoLaunch)
    }
    ///Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0
    #[inline(always)]
    pub fn launch(self) -> &'a mut crate::W<REG> {
        self.variant(RSYNC::Launch)
    }
}
/**DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDMAEN {
    ///0: The DMA channel is not enabled to read regular data
    Disabled = 0,
    ///1: The DMA channel is enabled to read regular data
    Enabled = 1,
}
impl From<RDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RDMAEN` reader - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type RDMAEN_R = crate::BitReader<RDMAEN>;
impl RDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDMAEN {
        match self.bits {
            false => RDMAEN::Disabled,
            true => RDMAEN::Enabled,
        }
    }
    ///The DMA channel is not enabled to read regular data
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDMAEN::Disabled
    }
    ///The DMA channel is enabled to read regular data
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDMAEN::Enabled
    }
}
///Field `RDMAEN` writer - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
pub type RDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RDMAEN>;
impl<'a, REG> RDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The DMA channel is not enabled to read regular data
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDMAEN::Disabled)
    }
    ///The DMA channel is enabled to read regular data
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDMAEN::Enabled)
    }
}
/**Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RCH {
    ///0: Channel 0 is selected as regular channel
    Channel0 = 0,
    ///1: Channel 1 is selected as regular channel
    Channel1 = 1,
    ///2: Channel 2 is selected as regular channel
    Channel2 = 2,
    ///3: Channel 3 is selected as regular channel
    Channel3 = 3,
    ///4: Channel 4 is selected as regular channel
    Channel4 = 4,
    ///5: Channel 5 is selected as regular channel
    Channel5 = 5,
    ///6: Channel 6 is selected as regular channel
    Channel6 = 6,
    ///7: Channel 7 is selected as regular channel
    Channel7 = 7,
}
impl From<RCH> for u8 {
    #[inline(always)]
    fn from(variant: RCH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RCH {
    type Ux = u8;
}
impl crate::IsEnum for RCH {}
///Field `RCH` reader - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).
pub type RCH_R = crate::FieldReader<RCH>;
impl RCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCH {
        match self.bits {
            0 => RCH::Channel0,
            1 => RCH::Channel1,
            2 => RCH::Channel2,
            3 => RCH::Channel3,
            4 => RCH::Channel4,
            5 => RCH::Channel5,
            6 => RCH::Channel6,
            7 => RCH::Channel7,
            _ => unreachable!(),
        }
    }
    ///Channel 0 is selected as regular channel
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == RCH::Channel0
    }
    ///Channel 1 is selected as regular channel
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == RCH::Channel1
    }
    ///Channel 2 is selected as regular channel
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == RCH::Channel2
    }
    ///Channel 3 is selected as regular channel
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == RCH::Channel3
    }
    ///Channel 4 is selected as regular channel
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        *self == RCH::Channel4
    }
    ///Channel 5 is selected as regular channel
    #[inline(always)]
    pub fn is_channel5(&self) -> bool {
        *self == RCH::Channel5
    }
    ///Channel 6 is selected as regular channel
    #[inline(always)]
    pub fn is_channel6(&self) -> bool {
        *self == RCH::Channel6
    }
    ///Channel 7 is selected as regular channel
    #[inline(always)]
    pub fn is_channel7(&self) -> bool {
        *self == RCH::Channel7
    }
}
///Field `RCH` writer - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).
pub type RCH_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RCH, crate::Safe>;
impl<'a, REG> RCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Channel 0 is selected as regular channel
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel0)
    }
    ///Channel 1 is selected as regular channel
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel1)
    }
    ///Channel 2 is selected as regular channel
    #[inline(always)]
    pub fn channel2(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel2)
    }
    ///Channel 3 is selected as regular channel
    #[inline(always)]
    pub fn channel3(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel3)
    }
    ///Channel 4 is selected as regular channel
    #[inline(always)]
    pub fn channel4(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel4)
    }
    ///Channel 5 is selected as regular channel
    #[inline(always)]
    pub fn channel5(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel5)
    }
    ///Channel 6 is selected as regular channel
    #[inline(always)]
    pub fn channel6(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel6)
    }
    ///Channel 7 is selected as regular channel
    #[inline(always)]
    pub fn channel7(self) -> &'a mut crate::W<REG> {
        self.variant(RCH::Channel7)
    }
}
/**Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \[FOSR * (IOSR-1 + FORD) + FORD\] / fCKIN ..... for Sincx filters t = \[FOSR * (IOSR-1 + 4) + 2\] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \[FOSR * IOSR\] / fCKIN in case if FOSR = FOSR\[9:0\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAST {
    ///0: Fast conversion mode disabled
    Disabled = 0,
    ///1: Fast conversion mode enabled
    Enabled = 1,
}
impl From<FAST> for bool {
    #[inline(always)]
    fn from(variant: FAST) -> Self {
        variant as u8 != 0
    }
}
///Field `FAST` reader - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \[FOSR * (IOSR-1 + FORD) + FORD\] / fCKIN ..... for Sincx filters t = \[FOSR * (IOSR-1 + 4) + 2\] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \[FOSR * IOSR\] / fCKIN in case if FOSR = FOSR\[9:0\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.
pub type FAST_R = crate::BitReader<FAST>;
impl FAST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FAST {
        match self.bits {
            false => FAST::Disabled,
            true => FAST::Enabled,
        }
    }
    ///Fast conversion mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAST::Disabled
    }
    ///Fast conversion mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FAST::Enabled
    }
}
///Field `FAST` writer - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \[FOSR * (IOSR-1 + FORD) + FORD\] / fCKIN ..... for Sincx filters t = \[FOSR * (IOSR-1 + 4) + 2\] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \[FOSR * IOSR\] / fCKIN in case if FOSR = FOSR\[9:0\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.
pub type FAST_W<'a, REG> = crate::BitWriter<'a, REG, FAST>;
impl<'a, REG> FAST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fast conversion mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FAST::Disabled)
    }
    ///Fast conversion mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FAST::Enabled)
    }
}
/**Analog watchdog fast mode select

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWFSEL {
    ///0: Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift
    Output = 0,
    ///1: Analog watchdog on channel transceivers value (after watchdog filter)
    Transceiver = 1,
}
impl From<AWFSEL> for bool {
    #[inline(always)]
    fn from(variant: AWFSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `AWFSEL` reader - Analog watchdog fast mode select
pub type AWFSEL_R = crate::BitReader<AWFSEL>;
impl AWFSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWFSEL {
        match self.bits {
            false => AWFSEL::Output,
            true => AWFSEL::Transceiver,
        }
    }
    ///Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == AWFSEL::Output
    }
    ///Analog watchdog on channel transceivers value (after watchdog filter)
    #[inline(always)]
    pub fn is_transceiver(&self) -> bool {
        *self == AWFSEL::Transceiver
    }
}
///Field `AWFSEL` writer - Analog watchdog fast mode select
pub type AWFSEL_W<'a, REG> = crate::BitWriter<'a, REG, AWFSEL>;
impl<'a, REG> AWFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(AWFSEL::Output)
    }
    ///Analog watchdog on channel transceivers value (after watchdog filter)
    #[inline(always)]
    pub fn transceiver(self) -> &'a mut crate::W<REG> {
        self.variant(AWFSEL::Transceiver)
    }
}
impl R {
    ///Bit 0 - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start a conversion of the injected group of channels This bit is always read as '0'.
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.
    #[inline(always)]
    pub fn jscan(&self) -> JSCAN_R {
        JSCAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:12 - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger).
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 13:14 - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 17 - Software start of a conversion on the regular channel This bit is always read as '0'.
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Continuous mode selection for regular conversions Writing '0' to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 24:26 - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 29 - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \[FOSR * (IOSR-1 + FORD) + FORD\] / fCKIN ..... for Sincx filters t = \[FOSR * (IOSR-1 + 4) + 2\] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \[FOSR * IOSR\] / fCKIN in case if FOSR = FOSR\[9:0\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Analog watchdog fast mode select
    #[inline(always)]
    pub fn awfsel(&self) -> AWFSEL_R {
        AWFSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("dfen", &self.dfen())
            .field("jswstart", &self.jswstart())
            .field("jsync", &self.jsync())
            .field("jscan", &self.jscan())
            .field("jdmaen", &self.jdmaen())
            .field("jextsel", &self.jextsel())
            .field("jexten", &self.jexten())
            .field("rswstart", &self.rswstart())
            .field("rcont", &self.rcont())
            .field("rsync", &self.rsync())
            .field("rdmaen", &self.rdmaen())
            .field("rch", &self.rch())
            .field("fast", &self.fast())
            .field("awfsel", &self.awfsel())
            .finish()
    }
}
impl W {
    ///Bit 0 - DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state
    #[inline(always)]
    pub fn dfen(&mut self) -> DFEN_W<'_, CR1rs> {
        DFEN_W::new(self, 0)
    }
    ///Bit 1 - Start a conversion of the injected group of channels This bit is always read as '0'.
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W<'_, CR1rs> {
        JSWSTART_W::new(self, 1)
    }
    ///Bit 3 - Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jsync(&mut self) -> JSYNC_W<'_, CR1rs> {
        JSYNC_W::new(self, 3)
    }
    ///Bit 4 - Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel.
    #[inline(always)]
    pub fn jscan(&mut self) -> JSCAN_W<'_, CR1rs> {
        JSCAN_W::new(self, 4)
    }
    ///Bit 5 - DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jdmaen(&mut self) -> JDMAEN_W<'_, CR1rs> {
        JDMAEN_W::new(self, 5)
    }
    ///Bits 8:12 - Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger).
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<'_, CR1rs> {
        JEXTSEL_W::new(self, 8)
    }
    ///Bits 13:14 - Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W<'_, CR1rs> {
        JEXTEN_W::new(self, 13)
    }
    ///Bit 17 - Software start of a conversion on the regular channel This bit is always read as '0'.
    #[inline(always)]
    pub fn rswstart(&mut self) -> RSWSTART_W<'_, CR1rs> {
        RSWSTART_W::new(self, 17)
    }
    ///Bit 18 - Continuous mode selection for regular conversions Writing '0' to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately.
    #[inline(always)]
    pub fn rcont(&mut self) -> RCONT_W<'_, CR1rs> {
        RCONT_W::new(self, 18)
    }
    ///Bit 19 - Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn rsync(&mut self) -> RSYNC_W<'_, CR1rs> {
        RSYNC_W::new(self, 19)
    }
    ///Bit 21 - DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1).
    #[inline(always)]
    pub fn rdmaen(&mut self) -> RDMAEN_W<'_, CR1rs> {
        RDMAEN_W::new(self, 21)
    }
    ///Bits 24:26 - Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion).
    #[inline(always)]
    pub fn rch(&mut self) -> RCH_W<'_, CR1rs> {
        RCH_W::new(self, 24)
    }
    ///Bit 29 - Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \[FOSR * (IOSR-1 + FORD) + FORD\] / fCKIN ..... for Sincx filters t = \[FOSR * (IOSR-1 + 4) + 2\] / fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \[FOSR * IOSR\] / fCKIN in case if FOSR = FOSR\[9:0\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input.
    #[inline(always)]
    pub fn fast(&mut self) -> FAST_W<'_, CR1rs> {
        FAST_W::new(self, 29)
    }
    ///Bit 30 - Analog watchdog fast mode select
    #[inline(always)]
    pub fn awfsel(&mut self) -> AWFSEL_W<'_, CR1rs> {
        AWFSEL_W::new(self, 30)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
