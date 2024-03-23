#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Audio block mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    #[doc = "0: Master transmitter"]
    MasterTx = 0,
    #[doc = "1: Master receiver"]
    MasterRx = 1,
    #[doc = "2: Slave transmitter"]
    SlaveTx = 2,
    #[doc = "3: Slave receiver"]
    SlaveRx = 3,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
#[doc = "Field `MODE` reader - Audio block mode"]
pub type MODE_R = crate::FieldReader<MODE>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE {
        match self.bits {
            0 => MODE::MasterTx,
            1 => MODE::MasterRx,
            2 => MODE::SlaveTx,
            3 => MODE::SlaveRx,
            _ => unreachable!(),
        }
    }
    #[doc = "Master transmitter"]
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        *self == MODE::MasterTx
    }
    #[doc = "Master receiver"]
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        *self == MODE::MasterRx
    }
    #[doc = "Slave transmitter"]
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        *self == MODE::SlaveTx
    }
    #[doc = "Slave receiver"]
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        *self == MODE::SlaveRx
    }
}
#[doc = "Field `MODE` writer - Audio block mode"]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master transmitter"]
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MasterTx)
    }
    #[doc = "Master receiver"]
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MasterRx)
    }
    #[doc = "Slave transmitter"]
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::SlaveTx)
    }
    #[doc = "Slave receiver"]
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::SlaveRx)
    }
}
#[doc = "Protocol configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRTCFG {
    #[doc = "0: Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol"]
    Free = 0,
    #[doc = "1: SPDIF protocol"]
    Spdif = 1,
    #[doc = "2: AC’97 protocol"]
    Ac97 = 2,
}
impl From<PRTCFG> for u8 {
    #[inline(always)]
    fn from(variant: PRTCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRTCFG {
    type Ux = u8;
}
#[doc = "Field `PRTCFG` reader - Protocol configuration"]
pub type PRTCFG_R = crate::FieldReader<PRTCFG>;
impl PRTCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRTCFG> {
        match self.bits {
            0 => Some(PRTCFG::Free),
            1 => Some(PRTCFG::Spdif),
            2 => Some(PRTCFG::Ac97),
            _ => None,
        }
    }
    #[doc = "Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == PRTCFG::Free
    }
    #[doc = "SPDIF protocol"]
    #[inline(always)]
    pub fn is_spdif(&self) -> bool {
        *self == PRTCFG::Spdif
    }
    #[doc = "AC’97 protocol"]
    #[inline(always)]
    pub fn is_ac97(&self) -> bool {
        *self == PRTCFG::Ac97
    }
}
#[doc = "Field `PRTCFG` writer - Protocol configuration"]
pub type PRTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PRTCFG>;
impl<'a, REG> PRTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol"]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(PRTCFG::Free)
    }
    #[doc = "SPDIF protocol"]
    #[inline(always)]
    pub fn spdif(self) -> &'a mut crate::W<REG> {
        self.variant(PRTCFG::Spdif)
    }
    #[doc = "AC’97 protocol"]
    #[inline(always)]
    pub fn ac97(self) -> &'a mut crate::W<REG> {
        self.variant(PRTCFG::Ac97)
    }
}
#[doc = "Data size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DS {
    #[doc = "2: 8 bits"]
    Bit8 = 2,
    #[doc = "3: 10 bits"]
    Bit10 = 3,
    #[doc = "4: 16 bits"]
    Bit16 = 4,
    #[doc = "5: 20 bits"]
    Bit20 = 5,
    #[doc = "6: 24 bits"]
    Bit24 = 6,
    #[doc = "7: 32 bits"]
    Bit32 = 7,
}
impl From<DS> for u8 {
    #[inline(always)]
    fn from(variant: DS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DS {
    type Ux = u8;
}
#[doc = "Field `DS` reader - Data size"]
pub type DS_R = crate::FieldReader<DS>;
impl DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DS> {
        match self.bits {
            2 => Some(DS::Bit8),
            3 => Some(DS::Bit10),
            4 => Some(DS::Bit16),
            5 => Some(DS::Bit20),
            6 => Some(DS::Bit24),
            7 => Some(DS::Bit32),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == DS::Bit8
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == DS::Bit10
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == DS::Bit16
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn is_bit20(&self) -> bool {
        *self == DS::Bit20
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn is_bit24(&self) -> bool {
        *self == DS::Bit24
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == DS::Bit32
    }
}
#[doc = "Field `DS` writer - Data size"]
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DS>;
impl<'a, REG> DS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit8)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit10)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit16)
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn bit20(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit20)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn bit24(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit24)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit32)
    }
}
#[doc = "Least significant bit first\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFIRST {
    #[doc = "0: Data are transferred with MSB first"]
    MsbFirst = 0,
    #[doc = "1: Data are transferred with LSB first"]
    LsbFirst = 1,
}
impl From<LSBFIRST> for bool {
    #[inline(always)]
    fn from(variant: LSBFIRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBFIRST` reader - Least significant bit first"]
pub type LSBFIRST_R = crate::BitReader<LSBFIRST>;
impl LSBFIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSBFIRST {
        match self.bits {
            false => LSBFIRST::MsbFirst,
            true => LSBFIRST::LsbFirst,
        }
    }
    #[doc = "Data are transferred with MSB first"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == LSBFIRST::MsbFirst
    }
    #[doc = "Data are transferred with LSB first"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == LSBFIRST::LsbFirst
    }
}
#[doc = "Field `LSBFIRST` writer - Least significant bit first"]
pub type LSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG, LSBFIRST>;
impl<'a, REG> LSBFIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data are transferred with MSB first"]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFIRST::MsbFirst)
    }
    #[doc = "Data are transferred with LSB first"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFIRST::LsbFirst)
    }
}
#[doc = "Clock strobing edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKSTR {
    #[doc = "0: Data strobing edge is falling edge of SCK"]
    FallingEdge = 0,
    #[doc = "1: Data strobing edge is rising edge of SCK"]
    RisingEdge = 1,
}
impl From<CKSTR> for bool {
    #[inline(always)]
    fn from(variant: CKSTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKSTR` reader - Clock strobing edge"]
pub type CKSTR_R = crate::BitReader<CKSTR>;
impl CKSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKSTR {
        match self.bits {
            false => CKSTR::FallingEdge,
            true => CKSTR::RisingEdge,
        }
    }
    #[doc = "Data strobing edge is falling edge of SCK"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CKSTR::FallingEdge
    }
    #[doc = "Data strobing edge is rising edge of SCK"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CKSTR::RisingEdge
    }
}
#[doc = "Field `CKSTR` writer - Clock strobing edge"]
pub type CKSTR_W<'a, REG> = crate::BitWriter<'a, REG, CKSTR>;
impl<'a, REG> CKSTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data strobing edge is falling edge of SCK"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKSTR::FallingEdge)
    }
    #[doc = "Data strobing edge is rising edge of SCK"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKSTR::RisingEdge)
    }
}
#[doc = "Synchronization enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCEN {
    #[doc = "0: audio sub-block in asynchronous mode"]
    Asynchronous = 0,
    #[doc = "1: audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode"]
    Internal = 1,
    #[doc = "2: audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode"]
    External = 2,
}
impl From<SYNCEN> for u8 {
    #[inline(always)]
    fn from(variant: SYNCEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCEN {
    type Ux = u8;
}
#[doc = "Field `SYNCEN` reader - Synchronization enable"]
pub type SYNCEN_R = crate::FieldReader<SYNCEN>;
impl SYNCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCEN> {
        match self.bits {
            0 => Some(SYNCEN::Asynchronous),
            1 => Some(SYNCEN::Internal),
            2 => Some(SYNCEN::External),
            _ => None,
        }
    }
    #[doc = "audio sub-block in asynchronous mode"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == SYNCEN::Asynchronous
    }
    #[doc = "audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == SYNCEN::Internal
    }
    #[doc = "audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SYNCEN::External
    }
}
#[doc = "Field `SYNCEN` writer - Synchronization enable"]
pub type SYNCEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCEN>;
impl<'a, REG> SYNCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "audio sub-block in asynchronous mode"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCEN::Asynchronous)
    }
    #[doc = "audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCEN::Internal)
    }
    #[doc = "audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCEN::External)
    }
}
#[doc = "Mono mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONO {
    #[doc = "0: Stereo mode"]
    Stereo = 0,
    #[doc = "1: Mono mode"]
    Mono = 1,
}
impl From<MONO> for bool {
    #[inline(always)]
    fn from(variant: MONO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONO` reader - Mono mode"]
pub type MONO_R = crate::BitReader<MONO>;
impl MONO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MONO {
        match self.bits {
            false => MONO::Stereo,
            true => MONO::Mono,
        }
    }
    #[doc = "Stereo mode"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == MONO::Stereo
    }
    #[doc = "Mono mode"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == MONO::Mono
    }
}
#[doc = "Field `MONO` writer - Mono mode"]
pub type MONO_W<'a, REG> = crate::BitWriter<'a, REG, MONO>;
impl<'a, REG> MONO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stereo mode"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut crate::W<REG> {
        self.variant(MONO::Stereo)
    }
    #[doc = "Mono mode"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut crate::W<REG> {
        self.variant(MONO::Mono)
    }
}
#[doc = "Output drive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTDRIV {
    #[doc = "0: Audio block output driven when SAIEN is set"]
    OnStart = 0,
    #[doc = "1: Audio block output driven immediately after the setting of this bit"]
    Immediately = 1,
}
impl From<OUTDRIV> for bool {
    #[inline(always)]
    fn from(variant: OUTDRIV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTDRIV` reader - Output drive"]
pub type OUTDRIV_R = crate::BitReader<OUTDRIV>;
impl OUTDRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUTDRIV {
        match self.bits {
            false => OUTDRIV::OnStart,
            true => OUTDRIV::Immediately,
        }
    }
    #[doc = "Audio block output driven when SAIEN is set"]
    #[inline(always)]
    pub fn is_on_start(&self) -> bool {
        *self == OUTDRIV::OnStart
    }
    #[doc = "Audio block output driven immediately after the setting of this bit"]
    #[inline(always)]
    pub fn is_immediately(&self) -> bool {
        *self == OUTDRIV::Immediately
    }
}
#[doc = "Field `OUTDRIV` writer - Output drive"]
pub type OUTDRIV_W<'a, REG> = crate::BitWriter<'a, REG, OUTDRIV>;
impl<'a, REG> OUTDRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Audio block output driven when SAIEN is set"]
    #[inline(always)]
    pub fn on_start(self) -> &'a mut crate::W<REG> {
        self.variant(OUTDRIV::OnStart)
    }
    #[doc = "Audio block output driven immediately after the setting of this bit"]
    #[inline(always)]
    pub fn immediately(self) -> &'a mut crate::W<REG> {
        self.variant(OUTDRIV::Immediately)
    }
}
#[doc = "Audio block A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIEN {
    #[doc = "0: SAI audio block disabled"]
    Disabled = 0,
    #[doc = "1: SAI audio block enabled"]
    Enabled = 1,
}
impl From<SAIEN> for bool {
    #[inline(always)]
    fn from(variant: SAIEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAIEN` reader - Audio block A enable"]
pub type SAIEN_R = crate::BitReader<SAIEN>;
impl SAIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAIEN {
        match self.bits {
            false => SAIEN::Disabled,
            true => SAIEN::Enabled,
        }
    }
    #[doc = "SAI audio block disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAIEN::Disabled
    }
    #[doc = "SAI audio block enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAIEN::Enabled
    }
}
#[doc = "Field `SAIEN` writer - Audio block A enable"]
pub type SAIEN_W<'a, REG> = crate::BitWriter<'a, REG, SAIEN>;
impl<'a, REG> SAIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAI audio block disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAIEN::Disabled)
    }
    #[doc = "SAI audio block enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAIEN::Enabled)
    }
}
#[doc = "DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    #[doc = "0: DMA disabled"]
    Disabled = 0,
    #[doc = "1: DMA enabled"]
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
#[doc = "No divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NODIV {
    #[doc = "0: MCLK output is enabled. Forces the ratio between FS and MCLK to 256 or 512 according to the OSR value"]
    MasterClock = 0,
    #[doc = "1: MCLK output enable set by the MCKEN bit (where present, else 0). Ratio between FS and MCLK depends on FRL."]
    NoDiv = 1,
}
impl From<NODIV> for bool {
    #[inline(always)]
    fn from(variant: NODIV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NODIV` reader - No divider"]
pub type NODIV_R = crate::BitReader<NODIV>;
impl NODIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NODIV {
        match self.bits {
            false => NODIV::MasterClock,
            true => NODIV::NoDiv,
        }
    }
    #[doc = "MCLK output is enabled. Forces the ratio between FS and MCLK to 256 or 512 according to the OSR value"]
    #[inline(always)]
    pub fn is_master_clock(&self) -> bool {
        *self == NODIV::MasterClock
    }
    #[doc = "MCLK output enable set by the MCKEN bit (where present, else 0). Ratio between FS and MCLK depends on FRL."]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == NODIV::NoDiv
    }
}
#[doc = "Field `NODIV` writer - No divider"]
pub type NODIV_W<'a, REG> = crate::BitWriter<'a, REG, NODIV>;
impl<'a, REG> NODIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCLK output is enabled. Forces the ratio between FS and MCLK to 256 or 512 according to the OSR value"]
    #[inline(always)]
    pub fn master_clock(self) -> &'a mut crate::W<REG> {
        self.variant(NODIV::MasterClock)
    }
    #[doc = "MCLK output enable set by the MCKEN bit (where present, else 0). Ratio between FS and MCLK depends on FRL."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut crate::W<REG> {
        self.variant(NODIV::NoDiv)
    }
}
#[doc = "Field `MCKDIV` reader - Master clock divider"]
pub type MCKDIV_R = crate::FieldReader;
#[doc = "Field `MCKDIV` writer - Master clock divider"]
pub type MCKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OSR` reader - OSR"]
pub type OSR_R = crate::BitReader;
#[doc = "Field `OSR` writer - OSR"]
pub type OSR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Audio block mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Protocol configuration"]
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Data size"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock strobing edge"]
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Mono mode"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output drive"]
    #[inline(always)]
    pub fn outdriv(&self) -> OUTDRIV_R {
        OUTDRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Audio block A enable"]
    #[inline(always)]
    pub fn saien(&self) -> SAIEN_R {
        SAIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nodiv(&self) -> NODIV_R {
        NODIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Master clock divider"]
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - OSR"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Audio block mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CR1rs> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Protocol configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prtcfg(&mut self) -> PRTCFG_W<CR1rs> {
        PRTCFG_W::new(self, 2)
    }
    #[doc = "Bits 5:7 - Data size"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<CR1rs> {
        DS_W::new(self, 5)
    }
    #[doc = "Bit 8 - Least significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<CR1rs> {
        LSBFIRST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock strobing edge"]
    #[inline(always)]
    #[must_use]
    pub fn ckstr(&mut self) -> CKSTR_W<CR1rs> {
        CKSTR_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Synchronization enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<CR1rs> {
        SYNCEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - Mono mode"]
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<CR1rs> {
        MONO_W::new(self, 12)
    }
    #[doc = "Bit 13 - Output drive"]
    #[inline(always)]
    #[must_use]
    pub fn outdriv(&mut self) -> OUTDRIV_W<CR1rs> {
        OUTDRIV_W::new(self, 13)
    }
    #[doc = "Bit 16 - Audio block A enable"]
    #[inline(always)]
    #[must_use]
    pub fn saien(&mut self) -> SAIEN_W<CR1rs> {
        SAIEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CR1rs> {
        DMAEN_W::new(self, 17)
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    #[must_use]
    pub fn nodiv(&mut self) -> NODIV_W<CR1rs> {
        NODIV_W::new(self, 19)
    }
    #[doc = "Bits 20:23 - Master clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn mckdiv(&mut self) -> MCKDIV_W<CR1rs> {
        MCKDIV_W::new(self, 20)
    }
    #[doc = "Bit 26 - OSR"]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OSR_W<CR1rs> {
        OSR_W::new(self, 26)
    }
}
#[doc = "AConfiguration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x40"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x40;
}
