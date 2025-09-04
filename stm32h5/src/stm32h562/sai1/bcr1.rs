///Register `BCR1` reader
pub type R = crate::R<BCR1rs>;
///Register `BCR1` writer
pub type W = crate::W<BCR1rs>;
/**SAIx audio block mode These bits are set and cleared by software. They must be configured when SAIx audio block is disabled. Note: When the audio block is configured in SPDIF mode, the master transmitter mode is forced (MODE\[1:0\] = 00). In Master transmitter mode, the audio block starts generating the FS and the clocks immediately.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: Master transmitter
    MasterTx = 0,
    ///1: Master receiver
    MasterRx = 1,
    ///2: Slave transmitter
    SlaveTx = 2,
    ///3: Slave receiver
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
impl crate::IsEnum for MODE {}
///Field `MODE` reader - SAIx audio block mode These bits are set and cleared by software. They must be configured when SAIx audio block is disabled. Note: When the audio block is configured in SPDIF mode, the master transmitter mode is forced (MODE\[1:0\] = 00). In Master transmitter mode, the audio block starts generating the FS and the clocks immediately.
pub type MODE_R = crate::FieldReader<MODE>;
impl MODE_R {
    ///Get enumerated values variant
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
    ///Master transmitter
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        *self == MODE::MasterTx
    }
    ///Master receiver
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        *self == MODE::MasterRx
    }
    ///Slave transmitter
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        *self == MODE::SlaveTx
    }
    ///Slave receiver
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        *self == MODE::SlaveRx
    }
}
///Field `MODE` writer - SAIx audio block mode These bits are set and cleared by software. They must be configured when SAIx audio block is disabled. Note: When the audio block is configured in SPDIF mode, the master transmitter mode is forced (MODE\[1:0\] = 00). In Master transmitter mode, the audio block starts generating the FS and the clocks immediately.
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Master transmitter
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MasterTx)
    }
    ///Master receiver
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::MasterRx)
    }
    ///Slave transmitter
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::SlaveTx)
    }
    ///Slave receiver
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::SlaveRx)
    }
}
/**Protocol configuration These bits are set and cleared by software. These bits have to be configured when the audio block is disabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRTCFG {
    ///0: Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol
    Free = 0,
    ///1: SPDIF protocol
    Spdif = 1,
    ///2: AC’97 protocol
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
impl crate::IsEnum for PRTCFG {}
///Field `PRTCFG` reader - Protocol configuration These bits are set and cleared by software. These bits have to be configured when the audio block is disabled.
pub type PRTCFG_R = crate::FieldReader<PRTCFG>;
impl PRTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRTCFG> {
        match self.bits {
            0 => Some(PRTCFG::Free),
            1 => Some(PRTCFG::Spdif),
            2 => Some(PRTCFG::Ac97),
            _ => None,
        }
    }
    ///Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == PRTCFG::Free
    }
    ///SPDIF protocol
    #[inline(always)]
    pub fn is_spdif(&self) -> bool {
        *self == PRTCFG::Spdif
    }
    ///AC’97 protocol
    #[inline(always)]
    pub fn is_ac97(&self) -> bool {
        *self == PRTCFG::Ac97
    }
}
///Field `PRTCFG` writer - Protocol configuration These bits are set and cleared by software. These bits have to be configured when the audio block is disabled.
pub type PRTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PRTCFG>;
impl<'a, REG> PRTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(PRTCFG::Free)
    }
    ///SPDIF protocol
    #[inline(always)]
    pub fn spdif(self) -> &'a mut crate::W<REG> {
        self.variant(PRTCFG::Spdif)
    }
    ///AC’97 protocol
    #[inline(always)]
    pub fn ac97(self) -> &'a mut crate::W<REG> {
        self.variant(PRTCFG::Ac97)
    }
}
/**Data size These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\[1:0\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\[1:0\] bits, DS\[1:0\] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled.

Value on reset: 2*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DS {
    ///2: 8 bits
    Bit8 = 2,
    ///3: 10 bits
    Bit10 = 3,
    ///4: 16 bits
    Bit16 = 4,
    ///5: 20 bits
    Bit20 = 5,
    ///6: 24 bits
    Bit24 = 6,
    ///7: 32 bits
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
impl crate::IsEnum for DS {}
///Field `DS` reader - Data size These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\[1:0\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\[1:0\] bits, DS\[1:0\] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled.
pub type DS_R = crate::FieldReader<DS>;
impl DS_R {
    ///Get enumerated values variant
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
    ///8 bits
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == DS::Bit8
    }
    ///10 bits
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == DS::Bit10
    }
    ///16 bits
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == DS::Bit16
    }
    ///20 bits
    #[inline(always)]
    pub fn is_bit20(&self) -> bool {
        *self == DS::Bit20
    }
    ///24 bits
    #[inline(always)]
    pub fn is_bit24(&self) -> bool {
        *self == DS::Bit24
    }
    ///32 bits
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == DS::Bit32
    }
}
///Field `DS` writer - Data size These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\[1:0\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\[1:0\] bits, DS\[1:0\] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled.
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DS>;
impl<'a, REG> DS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit8)
    }
    ///10 bits
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit10)
    }
    ///16 bits
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit16)
    }
    ///20 bits
    #[inline(always)]
    pub fn bit20(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit20)
    }
    ///24 bits
    #[inline(always)]
    pub fn bit24(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit24)
    }
    ///32 bits
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(DS::Bit32)
    }
}
/**Least significant bit first This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC'97 audio protocol since AC'97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFIRST {
    ///0: Data are transferred with MSB first
    MsbFirst = 0,
    ///1: Data are transferred with LSB first
    LsbFirst = 1,
}
impl From<LSBFIRST> for bool {
    #[inline(always)]
    fn from(variant: LSBFIRST) -> Self {
        variant as u8 != 0
    }
}
///Field `LSBFIRST` reader - Least significant bit first This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC'97 audio protocol since AC'97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first.
pub type LSBFIRST_R = crate::BitReader<LSBFIRST>;
impl LSBFIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSBFIRST {
        match self.bits {
            false => LSBFIRST::MsbFirst,
            true => LSBFIRST::LsbFirst,
        }
    }
    ///Data are transferred with MSB first
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == LSBFIRST::MsbFirst
    }
    ///Data are transferred with LSB first
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == LSBFIRST::LsbFirst
    }
}
///Field `LSBFIRST` writer - Least significant bit first This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC'97 audio protocol since AC'97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first.
pub type LSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG, LSBFIRST>;
impl<'a, REG> LSBFIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data are transferred with MSB first
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFIRST::MsbFirst)
    }
    ///Data are transferred with LSB first
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFIRST::LsbFirst)
    }
}
/**Clock strobing edge This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKSTR {
    ///0: Data strobing edge is falling edge of SCK
    FallingEdge = 0,
    ///1: Data strobing edge is rising edge of SCK
    RisingEdge = 1,
}
impl From<CKSTR> for bool {
    #[inline(always)]
    fn from(variant: CKSTR) -> Self {
        variant as u8 != 0
    }
}
///Field `CKSTR` reader - Clock strobing edge This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol.
pub type CKSTR_R = crate::BitReader<CKSTR>;
impl CKSTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKSTR {
        match self.bits {
            false => CKSTR::FallingEdge,
            true => CKSTR::RisingEdge,
        }
    }
    ///Data strobing edge is falling edge of SCK
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CKSTR::FallingEdge
    }
    ///Data strobing edge is rising edge of SCK
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CKSTR::RisingEdge
    }
}
///Field `CKSTR` writer - Clock strobing edge This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol.
pub type CKSTR_W<'a, REG> = crate::BitWriter<'a, REG, CKSTR>;
impl<'a, REG> CKSTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data strobing edge is falling edge of SCK
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKSTR::FallingEdge)
    }
    ///Data strobing edge is rising edge of SCK
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKSTR::RisingEdge)
    }
}
/**Synchronization enable These bits are set and cleared by software. They must be configured when the audio subblock is disabled. Note: The audio subblock should be configured as asynchronous when SPDIF mode is enabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCEN {
    ///0: audio sub-block in asynchronous mode
    Asynchronous = 0,
    ///1: audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode
    Internal = 1,
    ///2: audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode
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
impl crate::IsEnum for SYNCEN {}
///Field `SYNCEN` reader - Synchronization enable These bits are set and cleared by software. They must be configured when the audio subblock is disabled. Note: The audio subblock should be configured as asynchronous when SPDIF mode is enabled.
pub type SYNCEN_R = crate::FieldReader<SYNCEN>;
impl SYNCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCEN> {
        match self.bits {
            0 => Some(SYNCEN::Asynchronous),
            1 => Some(SYNCEN::Internal),
            2 => Some(SYNCEN::External),
            _ => None,
        }
    }
    ///audio sub-block in asynchronous mode
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == SYNCEN::Asynchronous
    }
    ///audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == SYNCEN::Internal
    }
    ///audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SYNCEN::External
    }
}
///Field `SYNCEN` writer - Synchronization enable These bits are set and cleared by software. They must be configured when the audio subblock is disabled. Note: The audio subblock should be configured as asynchronous when SPDIF mode is enabled.
pub type SYNCEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCEN>;
impl<'a, REG> SYNCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///audio sub-block in asynchronous mode
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCEN::Asynchronous)
    }
    ///audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCEN::Internal)
    }
    ///audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCEN::External)
    }
}
/**Mono mode This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to for more details.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONO {
    ///0: Stereo mode
    Stereo = 0,
    ///1: Mono mode
    Mono = 1,
}
impl From<MONO> for bool {
    #[inline(always)]
    fn from(variant: MONO) -> Self {
        variant as u8 != 0
    }
}
///Field `MONO` reader - Mono mode This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to for more details.
pub type MONO_R = crate::BitReader<MONO>;
impl MONO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MONO {
        match self.bits {
            false => MONO::Stereo,
            true => MONO::Mono,
        }
    }
    ///Stereo mode
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == MONO::Stereo
    }
    ///Mono mode
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == MONO::Mono
    }
}
///Field `MONO` writer - Mono mode This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to for more details.
pub type MONO_W<'a, REG> = crate::BitWriter<'a, REG, MONO>;
impl<'a, REG> MONO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stereo mode
    #[inline(always)]
    pub fn stereo(self) -> &'a mut crate::W<REG> {
        self.variant(MONO::Stereo)
    }
    ///Mono mode
    #[inline(always)]
    pub fn mono(self) -> &'a mut crate::W<REG> {
        self.variant(MONO::Mono)
    }
}
/**Output drive This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTDRIV {
    ///0: Audio block output driven when SAIEN is set
    OnStart = 0,
    ///1: Audio block output driven immediately after the setting of this bit
    Immediately = 1,
}
impl From<OUTDRIV> for bool {
    #[inline(always)]
    fn from(variant: OUTDRIV) -> Self {
        variant as u8 != 0
    }
}
///Field `OUTDRIV` reader - Output drive This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration.
pub type OUTDRIV_R = crate::BitReader<OUTDRIV>;
impl OUTDRIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OUTDRIV {
        match self.bits {
            false => OUTDRIV::OnStart,
            true => OUTDRIV::Immediately,
        }
    }
    ///Audio block output driven when SAIEN is set
    #[inline(always)]
    pub fn is_on_start(&self) -> bool {
        *self == OUTDRIV::OnStart
    }
    ///Audio block output driven immediately after the setting of this bit
    #[inline(always)]
    pub fn is_immediately(&self) -> bool {
        *self == OUTDRIV::Immediately
    }
}
///Field `OUTDRIV` writer - Output drive This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration.
pub type OUTDRIV_W<'a, REG> = crate::BitWriter<'a, REG, OUTDRIV>;
impl<'a, REG> OUTDRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Audio block output driven when SAIEN is set
    #[inline(always)]
    pub fn on_start(self) -> &'a mut crate::W<REG> {
        self.variant(OUTDRIV::OnStart)
    }
    ///Audio block output driven immediately after the setting of this bit
    #[inline(always)]
    pub fn immediately(self) -> &'a mut crate::W<REG> {
        self.variant(OUTDRIV::Immediately)
    }
}
/**Audio block enable This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command is not taken into account. This bit allows controlling the state of the SAI audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the SAI block input before setting SAIEN bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIEN {
    ///0: SAI audio block disabled
    Disabled = 0,
    ///1: SAI audio block enabled
    Enabled = 1,
}
impl From<SAIEN> for bool {
    #[inline(always)]
    fn from(variant: SAIEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIEN` reader - Audio block enable This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command is not taken into account. This bit allows controlling the state of the SAI audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the SAI block input before setting SAIEN bit.
pub type SAIEN_R = crate::BitReader<SAIEN>;
impl SAIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAIEN {
        match self.bits {
            false => SAIEN::Disabled,
            true => SAIEN::Enabled,
        }
    }
    ///SAI audio block disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAIEN::Disabled
    }
    ///SAI audio block enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAIEN::Enabled
    }
}
///Field `SAIEN` writer - Audio block enable This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command is not taken into account. This bit allows controlling the state of the SAI audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the SAI block input before setting SAIEN bit.
pub type SAIEN_W<'a, REG> = crate::BitWriter<'a, REG, SAIEN>;
impl<'a, REG> SAIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SAI audio block disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAIEN::Disabled)
    }
    ///SAI audio block enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAIEN::Enabled)
    }
}
/**DMA enable This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\[1:0\] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode.

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
///Field `DMAEN` reader - DMA enable This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\[1:0\] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode.
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
///Field `DMAEN` writer - DMA enable This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\[1:0\] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode.
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
/**No divider This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NODIV {
    ///0: MCLK output is enabled. Forces the ratio between FS and MCLK to 256 or 512 according to the OSR value
    MasterClock = 0,
    ///1: MCLK output enable set by the MCKEN bit (where present, else 0). Ratio between FS and MCLK depends on FRL.
    NoDiv = 1,
}
impl From<NODIV> for bool {
    #[inline(always)]
    fn from(variant: NODIV) -> Self {
        variant as u8 != 0
    }
}
///Field `NODIV` reader - No divider This bit is set and cleared by software.
pub type NODIV_R = crate::BitReader<NODIV>;
impl NODIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NODIV {
        match self.bits {
            false => NODIV::MasterClock,
            true => NODIV::NoDiv,
        }
    }
    ///MCLK output is enabled. Forces the ratio between FS and MCLK to 256 or 512 according to the OSR value
    #[inline(always)]
    pub fn is_master_clock(&self) -> bool {
        *self == NODIV::MasterClock
    }
    ///MCLK output enable set by the MCKEN bit (where present, else 0). Ratio between FS and MCLK depends on FRL.
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == NODIV::NoDiv
    }
}
///Field `NODIV` writer - No divider This bit is set and cleared by software.
pub type NODIV_W<'a, REG> = crate::BitWriter<'a, REG, NODIV>;
impl<'a, REG> NODIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MCLK output is enabled. Forces the ratio between FS and MCLK to 256 or 512 according to the OSR value
    #[inline(always)]
    pub fn master_clock(self) -> &'a mut crate::W<REG> {
        self.variant(NODIV::MasterClock)
    }
    ///MCLK output enable set by the MCKEN bit (where present, else 0). Ratio between FS and MCLK depends on FRL.
    #[inline(always)]
    pub fn no_div(self) -> &'a mut crate::W<REG> {
        self.variant(NODIV::NoDiv)
    }
}
///Field `MCKDIV` reader - Master clock divider These bits are set and cleared by software. Otherwise, The master clock frequency is calculated according to the formula given in . These bits have no meaning when the audio block is slave. They have to be configured when the audio block is disabled.
pub type MCKDIV_R = crate::FieldReader;
///Field `MCKDIV` writer - Master clock divider These bits are set and cleared by software. Otherwise, The master clock frequency is calculated according to the formula given in . These bits have no meaning when the audio block is slave. They have to be configured when the audio block is disabled.
pub type MCKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `OSR` reader - Oversampling ratio for master clock This bit is meaningful only when NODIV bit is set to 0.
pub type OSR_R = crate::BitReader;
///Field `OSR` writer - Oversampling ratio for master clock This bit is meaningful only when NODIV bit is set to 0.
pub type OSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCKEN` reader - Master clock generation enable
pub type MCKEN_R = crate::BitReader;
///Field `MCKEN` writer - Master clock generation enable
pub type MCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - SAIx audio block mode These bits are set and cleared by software. They must be configured when SAIx audio block is disabled. Note: When the audio block is configured in SPDIF mode, the master transmitter mode is forced (MODE\[1:0\] = 00). In Master transmitter mode, the audio block starts generating the FS and the clocks immediately.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Protocol configuration These bits are set and cleared by software. These bits have to be configured when the audio block is disabled.
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:7 - Data size These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\[1:0\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\[1:0\] bits, DS\[1:0\] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Least significant bit first This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC'97 audio protocol since AC'97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first.
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clock strobing edge This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol.
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - Synchronization enable These bits are set and cleared by software. They must be configured when the audio subblock is disabled. Note: The audio subblock should be configured as asynchronous when SPDIF mode is enabled.
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Mono mode This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to for more details.
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Output drive This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration.
    #[inline(always)]
    pub fn outdriv(&self) -> OUTDRIV_R {
        OUTDRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Audio block enable This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command is not taken into account. This bit allows controlling the state of the SAI audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the SAI block input before setting SAIEN bit.
    #[inline(always)]
    pub fn saien(&self) -> SAIEN_R {
        SAIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMA enable This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\[1:0\] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode.
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - No divider This bit is set and cleared by software.
    #[inline(always)]
    pub fn nodiv(&self) -> NODIV_R {
        NODIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:25 - Master clock divider These bits are set and cleared by software. Otherwise, The master clock frequency is calculated according to the formula given in . These bits have no meaning when the audio block is slave. They have to be configured when the audio block is disabled.
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 26 - Oversampling ratio for master clock This bit is meaningful only when NODIV bit is set to 0.
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Master clock generation enable
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR1")
            .field("mode", &self.mode())
            .field("prtcfg", &self.prtcfg())
            .field("ds", &self.ds())
            .field("lsbfirst", &self.lsbfirst())
            .field("ckstr", &self.ckstr())
            .field("syncen", &self.syncen())
            .field("mono", &self.mono())
            .field("outdriv", &self.outdriv())
            .field("saien", &self.saien())
            .field("dmaen", &self.dmaen())
            .field("nodiv", &self.nodiv())
            .field("mckdiv", &self.mckdiv())
            .field("osr", &self.osr())
            .field("mcken", &self.mcken())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SAIx audio block mode These bits are set and cleared by software. They must be configured when SAIx audio block is disabled. Note: When the audio block is configured in SPDIF mode, the master transmitter mode is forced (MODE\[1:0\] = 00). In Master transmitter mode, the audio block starts generating the FS and the clocks immediately.
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<BCR1rs> {
        MODE_W::new(self, 0)
    }
    ///Bits 2:3 - Protocol configuration These bits are set and cleared by software. These bits have to be configured when the audio block is disabled.
    #[inline(always)]
    pub fn prtcfg(&mut self) -> PRTCFG_W<BCR1rs> {
        PRTCFG_W::new(self, 2)
    }
    ///Bits 5:7 - Data size These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\[1:0\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\[1:0\] bits, DS\[1:0\] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W<BCR1rs> {
        DS_W::new(self, 5)
    }
    ///Bit 8 - Least significant bit first This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC'97 audio protocol since AC'97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first.
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<BCR1rs> {
        LSBFIRST_W::new(self, 8)
    }
    ///Bit 9 - Clock strobing edge This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol.
    #[inline(always)]
    pub fn ckstr(&mut self) -> CKSTR_W<BCR1rs> {
        CKSTR_W::new(self, 9)
    }
    ///Bits 10:11 - Synchronization enable These bits are set and cleared by software. They must be configured when the audio subblock is disabled. Note: The audio subblock should be configured as asynchronous when SPDIF mode is enabled.
    #[inline(always)]
    pub fn syncen(&mut self) -> SYNCEN_W<BCR1rs> {
        SYNCEN_W::new(self, 10)
    }
    ///Bit 12 - Mono mode This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to for more details.
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W<BCR1rs> {
        MONO_W::new(self, 12)
    }
    ///Bit 13 - Output drive This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration.
    #[inline(always)]
    pub fn outdriv(&mut self) -> OUTDRIV_W<BCR1rs> {
        OUTDRIV_W::new(self, 13)
    }
    ///Bit 16 - Audio block enable This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command is not taken into account. This bit allows controlling the state of the SAI audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the SAI block input before setting SAIEN bit.
    #[inline(always)]
    pub fn saien(&mut self) -> SAIEN_W<BCR1rs> {
        SAIEN_W::new(self, 16)
    }
    ///Bit 17 - DMA enable This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\[1:0\] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode.
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<BCR1rs> {
        DMAEN_W::new(self, 17)
    }
    ///Bit 19 - No divider This bit is set and cleared by software.
    #[inline(always)]
    pub fn nodiv(&mut self) -> NODIV_W<BCR1rs> {
        NODIV_W::new(self, 19)
    }
    ///Bits 20:25 - Master clock divider These bits are set and cleared by software. Otherwise, The master clock frequency is calculated according to the formula given in . These bits have no meaning when the audio block is slave. They have to be configured when the audio block is disabled.
    #[inline(always)]
    pub fn mckdiv(&mut self) -> MCKDIV_W<BCR1rs> {
        MCKDIV_W::new(self, 20)
    }
    ///Bit 26 - Oversampling ratio for master clock This bit is meaningful only when NODIV bit is set to 0.
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W<BCR1rs> {
        OSR_W::new(self, 26)
    }
    ///Bit 27 - Master clock generation enable
    #[inline(always)]
    pub fn mcken(&mut self) -> MCKEN_W<BCR1rs> {
        MCKEN_W::new(self, 27)
    }
}
/**SAI configuration register 1

You can [`read`](crate::Reg::read) this register and get [`bcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#SAI1:BCR1)*/
pub struct BCR1rs;
impl crate::RegisterSpec for BCR1rs {
    type Ux = u32;
}
///`read()` method returns [`bcr1::R`](R) reader structure
impl crate::Readable for BCR1rs {}
///`write(|w| ..)` method takes [`bcr1::W`](W) writer structure
impl crate::Writable for BCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCR1 to value 0x40
impl crate::Resettable for BCR1rs {
    const RESET_VALUE: u32 = 0x40;
}
