#[doc = "Register `MACCR` reader"]
pub type R = crate::R<MACCRrs>;
#[doc = "Register `MACCR` writer"]
pub type W = crate::W<MACCRrs>;
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE {
    #[doc = "0: MAC receive state machine is disabled after the completion of the reception of the current frame"]
    Disabled = 0,
    #[doc = "1: MAC receive state machine is enabled"]
    Enabled = 1,
}
impl From<RE> for bool {
    #[inline(always)]
    fn from(variant: RE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader<RE>;
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RE {
        match self.bits {
            false => RE::Disabled,
            true => RE::Enabled,
        }
    }
    #[doc = "MAC receive state machine is disabled after the completion of the reception of the current frame"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE::Disabled
    }
    #[doc = "MAC receive state machine is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE::Enabled
    }
}
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG, RE>;
impl<'a, REG> RE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MAC receive state machine is disabled after the completion of the reception of the current frame"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE::Disabled)
    }
    #[doc = "MAC receive state machine is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE::Enabled)
    }
}
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE {
    #[doc = "0: MAC transmit state machine is disabled after completion of the transmission of the current frame"]
    Disabled = 0,
    #[doc = "1: MAC transmit state machine is enabled"]
    Enabled = 1,
}
impl From<TE> for bool {
    #[inline(always)]
    fn from(variant: TE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader<TE>;
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TE {
        match self.bits {
            false => TE::Disabled,
            true => TE::Enabled,
        }
    }
    #[doc = "MAC transmit state machine is disabled after completion of the transmission of the current frame"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE::Disabled
    }
    #[doc = "MAC transmit state machine is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE::Enabled
    }
}
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG, TE>;
impl<'a, REG> TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MAC transmit state machine is disabled after completion of the transmission of the current frame"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE::Disabled)
    }
    #[doc = "MAC transmit state machine is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE::Enabled)
    }
}
#[doc = "Deferral check\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DC {
    #[doc = "0: MAC defers until CRS signal goes inactive"]
    Disabled = 0,
    #[doc = "1: Deferral check function enabled"]
    Enabled = 1,
}
impl From<DC> for bool {
    #[inline(always)]
    fn from(variant: DC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DC` reader - Deferral check"]
pub type DC_R = crate::BitReader<DC>;
impl DC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DC {
        match self.bits {
            false => DC::Disabled,
            true => DC::Enabled,
        }
    }
    #[doc = "MAC defers until CRS signal goes inactive"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DC::Disabled
    }
    #[doc = "Deferral check function enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DC::Enabled
    }
}
#[doc = "Field `DC` writer - Deferral check"]
pub type DC_W<'a, REG> = crate::BitWriter<'a, REG, DC>;
impl<'a, REG> DC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MAC defers until CRS signal goes inactive"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DC::Disabled)
    }
    #[doc = "Deferral check function enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DC::Enabled)
    }
}
#[doc = "Back-off limit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BL {
    #[doc = "0: For retransmission n, wait up to 2^min(n, 10) time slots"]
    Bl10 = 0,
    #[doc = "1: For retransmission n, wait up to 2^min(n, 8) time slots"]
    Bl8 = 1,
    #[doc = "2: For retransmission n, wait up to 2^min(n, 4) time slots"]
    Bl4 = 2,
    #[doc = "3: For retransmission n, wait up to 2^min(n, 1) time slots"]
    Bl1 = 3,
}
impl From<BL> for u8 {
    #[inline(always)]
    fn from(variant: BL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BL {
    type Ux = u8;
}
#[doc = "Field `BL` reader - Back-off limit"]
pub type BL_R = crate::FieldReader<BL>;
impl BL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BL {
        match self.bits {
            0 => BL::Bl10,
            1 => BL::Bl8,
            2 => BL::Bl4,
            3 => BL::Bl1,
            _ => unreachable!(),
        }
    }
    #[doc = "For retransmission n, wait up to 2^min(n, 10) time slots"]
    #[inline(always)]
    pub fn is_bl10(&self) -> bool {
        *self == BL::Bl10
    }
    #[doc = "For retransmission n, wait up to 2^min(n, 8) time slots"]
    #[inline(always)]
    pub fn is_bl8(&self) -> bool {
        *self == BL::Bl8
    }
    #[doc = "For retransmission n, wait up to 2^min(n, 4) time slots"]
    #[inline(always)]
    pub fn is_bl4(&self) -> bool {
        *self == BL::Bl4
    }
    #[doc = "For retransmission n, wait up to 2^min(n, 1) time slots"]
    #[inline(always)]
    pub fn is_bl1(&self) -> bool {
        *self == BL::Bl1
    }
}
#[doc = "Field `BL` writer - Back-off limit"]
pub type BL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BL>;
impl<'a, REG> BL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "For retransmission n, wait up to 2^min(n, 10) time slots"]
    #[inline(always)]
    pub fn bl10(self) -> &'a mut crate::W<REG> {
        self.variant(BL::Bl10)
    }
    #[doc = "For retransmission n, wait up to 2^min(n, 8) time slots"]
    #[inline(always)]
    pub fn bl8(self) -> &'a mut crate::W<REG> {
        self.variant(BL::Bl8)
    }
    #[doc = "For retransmission n, wait up to 2^min(n, 4) time slots"]
    #[inline(always)]
    pub fn bl4(self) -> &'a mut crate::W<REG> {
        self.variant(BL::Bl4)
    }
    #[doc = "For retransmission n, wait up to 2^min(n, 1) time slots"]
    #[inline(always)]
    pub fn bl1(self) -> &'a mut crate::W<REG> {
        self.variant(BL::Bl1)
    }
}
#[doc = "Automatic pad/CRC stripping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APCS {
    #[doc = "0: MAC passes all incoming frames unmodified"]
    Disabled = 0,
    #[doc = "1: MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes"]
    Strip = 1,
}
impl From<APCS> for bool {
    #[inline(always)]
    fn from(variant: APCS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APCS` reader - Automatic pad/CRC stripping"]
pub type APCS_R = crate::BitReader<APCS>;
impl APCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> APCS {
        match self.bits {
            false => APCS::Disabled,
            true => APCS::Strip,
        }
    }
    #[doc = "MAC passes all incoming frames unmodified"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APCS::Disabled
    }
    #[doc = "MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes"]
    #[inline(always)]
    pub fn is_strip(&self) -> bool {
        *self == APCS::Strip
    }
}
#[doc = "Field `APCS` writer - Automatic pad/CRC stripping"]
pub type APCS_W<'a, REG> = crate::BitWriter<'a, REG, APCS>;
impl<'a, REG> APCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MAC passes all incoming frames unmodified"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(APCS::Disabled)
    }
    #[doc = "MAC strips the Pad/FCS field on incoming frames only for lengths less than or equal to 1500 bytes"]
    #[inline(always)]
    pub fn strip(self) -> &'a mut crate::W<REG> {
        self.variant(APCS::Strip)
    }
}
#[doc = "Retry disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD {
    #[doc = "0: MAC attempts retries based on the settings of BL"]
    Enabled = 0,
    #[doc = "1: MAC attempts only 1 transmission"]
    Disabled = 1,
}
impl From<RD> for bool {
    #[inline(always)]
    fn from(variant: RD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD` reader - Retry disable"]
pub type RD_R = crate::BitReader<RD>;
impl RD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RD {
        match self.bits {
            false => RD::Enabled,
            true => RD::Disabled,
        }
    }
    #[doc = "MAC attempts retries based on the settings of BL"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RD::Enabled
    }
    #[doc = "MAC attempts only 1 transmission"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RD::Disabled
    }
}
#[doc = "Field `RD` writer - Retry disable"]
pub type RD_W<'a, REG> = crate::BitWriter<'a, REG, RD>;
impl<'a, REG> RD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MAC attempts retries based on the settings of BL"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RD::Enabled)
    }
    #[doc = "MAC attempts only 1 transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RD::Disabled)
    }
}
#[doc = "IPv4 checksum offload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPCO {
    #[doc = "0: IPv4 checksum offload disabled"]
    Disabled = 0,
    #[doc = "1: IPv4 checksums are checked in received frames"]
    Offload = 1,
}
impl From<IPCO> for bool {
    #[inline(always)]
    fn from(variant: IPCO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPCO` reader - IPv4 checksum offload"]
pub type IPCO_R = crate::BitReader<IPCO>;
impl IPCO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IPCO {
        match self.bits {
            false => IPCO::Disabled,
            true => IPCO::Offload,
        }
    }
    #[doc = "IPv4 checksum offload disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IPCO::Disabled
    }
    #[doc = "IPv4 checksums are checked in received frames"]
    #[inline(always)]
    pub fn is_offload(&self) -> bool {
        *self == IPCO::Offload
    }
}
#[doc = "Field `IPCO` writer - IPv4 checksum offload"]
pub type IPCO_W<'a, REG> = crate::BitWriter<'a, REG, IPCO>;
impl<'a, REG> IPCO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IPv4 checksum offload disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IPCO::Disabled)
    }
    #[doc = "IPv4 checksums are checked in received frames"]
    #[inline(always)]
    pub fn offload(self) -> &'a mut crate::W<REG> {
        self.variant(IPCO::Offload)
    }
}
#[doc = "Duplex mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DM {
    #[doc = "0: MAC operates in half-duplex mode"]
    HalfDuplex = 0,
    #[doc = "1: MAC operates in full-duplex mode"]
    FullDuplex = 1,
}
impl From<DM> for bool {
    #[inline(always)]
    fn from(variant: DM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DM` reader - Duplex mode"]
pub type DM_R = crate::BitReader<DM>;
impl DM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DM {
        match self.bits {
            false => DM::HalfDuplex,
            true => DM::FullDuplex,
        }
    }
    #[doc = "MAC operates in half-duplex mode"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == DM::HalfDuplex
    }
    #[doc = "MAC operates in full-duplex mode"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == DM::FullDuplex
    }
}
#[doc = "Field `DM` writer - Duplex mode"]
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG, DM>;
impl<'a, REG> DM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MAC operates in half-duplex mode"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(DM::HalfDuplex)
    }
    #[doc = "MAC operates in full-duplex mode"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(DM::FullDuplex)
    }
}
#[doc = "Loopback mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LM {
    #[doc = "0: Normal mode"]
    Normal = 0,
    #[doc = "1: MAC operates in loopback mode at the MII"]
    Loopback = 1,
}
impl From<LM> for bool {
    #[inline(always)]
    fn from(variant: LM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LM` reader - Loopback mode"]
pub type LM_R = crate::BitReader<LM>;
impl LM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LM {
        match self.bits {
            false => LM::Normal,
            true => LM::Loopback,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LM::Normal
    }
    #[doc = "MAC operates in loopback mode at the MII"]
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        *self == LM::Loopback
    }
}
#[doc = "Field `LM` writer - Loopback mode"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG, LM>;
impl<'a, REG> LM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(LM::Normal)
    }
    #[doc = "MAC operates in loopback mode at the MII"]
    #[inline(always)]
    pub fn loopback(self) -> &'a mut crate::W<REG> {
        self.variant(LM::Loopback)
    }
}
#[doc = "Receive own disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROD {
    #[doc = "0: MAC receives all packets from PHY while transmitting"]
    Enabled = 0,
    #[doc = "1: MAC disables reception of frames in half-duplex mode"]
    Disabled = 1,
}
impl From<ROD> for bool {
    #[inline(always)]
    fn from(variant: ROD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROD` reader - Receive own disable"]
pub type ROD_R = crate::BitReader<ROD>;
impl ROD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROD {
        match self.bits {
            false => ROD::Enabled,
            true => ROD::Disabled,
        }
    }
    #[doc = "MAC receives all packets from PHY while transmitting"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROD::Enabled
    }
    #[doc = "MAC disables reception of frames in half-duplex mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROD::Disabled
    }
}
#[doc = "Field `ROD` writer - Receive own disable"]
pub type ROD_W<'a, REG> = crate::BitWriter<'a, REG, ROD>;
impl<'a, REG> ROD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MAC receives all packets from PHY while transmitting"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROD::Enabled)
    }
    #[doc = "MAC disables reception of frames in half-duplex mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ROD::Disabled)
    }
}
#[doc = "Fast Ethernet speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FES {
    #[doc = "0: 10 Mbit/s"]
    Fes10 = 0,
    #[doc = "1: 100 Mbit/s"]
    Fes100 = 1,
}
impl From<FES> for bool {
    #[inline(always)]
    fn from(variant: FES) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FES` reader - Fast Ethernet speed"]
pub type FES_R = crate::BitReader<FES>;
impl FES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FES {
        match self.bits {
            false => FES::Fes10,
            true => FES::Fes100,
        }
    }
    #[doc = "10 Mbit/s"]
    #[inline(always)]
    pub fn is_fes10(&self) -> bool {
        *self == FES::Fes10
    }
    #[doc = "100 Mbit/s"]
    #[inline(always)]
    pub fn is_fes100(&self) -> bool {
        *self == FES::Fes100
    }
}
#[doc = "Field `FES` writer - Fast Ethernet speed"]
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG, FES>;
impl<'a, REG> FES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10 Mbit/s"]
    #[inline(always)]
    pub fn fes10(self) -> &'a mut crate::W<REG> {
        self.variant(FES::Fes10)
    }
    #[doc = "100 Mbit/s"]
    #[inline(always)]
    pub fn fes100(self) -> &'a mut crate::W<REG> {
        self.variant(FES::Fes100)
    }
}
#[doc = "Carrier sense disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSD {
    #[doc = "0: Errors generated due to loss of carrier"]
    Enabled = 0,
    #[doc = "1: No error generated due to loss of carrier"]
    Disabled = 1,
}
impl From<CSD> for bool {
    #[inline(always)]
    fn from(variant: CSD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSD` reader - Carrier sense disable"]
pub type CSD_R = crate::BitReader<CSD>;
impl CSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSD {
        match self.bits {
            false => CSD::Enabled,
            true => CSD::Disabled,
        }
    }
    #[doc = "Errors generated due to loss of carrier"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSD::Enabled
    }
    #[doc = "No error generated due to loss of carrier"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSD::Disabled
    }
}
#[doc = "Field `CSD` writer - Carrier sense disable"]
pub type CSD_W<'a, REG> = crate::BitWriter<'a, REG, CSD>;
impl<'a, REG> CSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Errors generated due to loss of carrier"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSD::Enabled)
    }
    #[doc = "No error generated due to loss of carrier"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSD::Disabled)
    }
}
#[doc = "Interframe gap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IFG {
    #[doc = "0: 96 bit times"]
    Ifg96 = 0,
    #[doc = "1: 88 bit times"]
    Ifg88 = 1,
    #[doc = "2: 80 bit times"]
    Ifg80 = 2,
    #[doc = "3: 72 bit times"]
    Ifg72 = 3,
    #[doc = "4: 64 bit times"]
    Ifg64 = 4,
    #[doc = "5: 56 bit times"]
    Ifg56 = 5,
    #[doc = "6: 48 bit times"]
    Ifg48 = 6,
    #[doc = "7: 40 bit times"]
    Ifg40 = 7,
}
impl From<IFG> for u8 {
    #[inline(always)]
    fn from(variant: IFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IFG {
    type Ux = u8;
}
#[doc = "Field `IFG` reader - Interframe gap"]
pub type IFG_R = crate::FieldReader<IFG>;
impl IFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IFG {
        match self.bits {
            0 => IFG::Ifg96,
            1 => IFG::Ifg88,
            2 => IFG::Ifg80,
            3 => IFG::Ifg72,
            4 => IFG::Ifg64,
            5 => IFG::Ifg56,
            6 => IFG::Ifg48,
            7 => IFG::Ifg40,
            _ => unreachable!(),
        }
    }
    #[doc = "96 bit times"]
    #[inline(always)]
    pub fn is_ifg96(&self) -> bool {
        *self == IFG::Ifg96
    }
    #[doc = "88 bit times"]
    #[inline(always)]
    pub fn is_ifg88(&self) -> bool {
        *self == IFG::Ifg88
    }
    #[doc = "80 bit times"]
    #[inline(always)]
    pub fn is_ifg80(&self) -> bool {
        *self == IFG::Ifg80
    }
    #[doc = "72 bit times"]
    #[inline(always)]
    pub fn is_ifg72(&self) -> bool {
        *self == IFG::Ifg72
    }
    #[doc = "64 bit times"]
    #[inline(always)]
    pub fn is_ifg64(&self) -> bool {
        *self == IFG::Ifg64
    }
    #[doc = "56 bit times"]
    #[inline(always)]
    pub fn is_ifg56(&self) -> bool {
        *self == IFG::Ifg56
    }
    #[doc = "48 bit times"]
    #[inline(always)]
    pub fn is_ifg48(&self) -> bool {
        *self == IFG::Ifg48
    }
    #[doc = "40 bit times"]
    #[inline(always)]
    pub fn is_ifg40(&self) -> bool {
        *self == IFG::Ifg40
    }
}
#[doc = "Field `IFG` writer - Interframe gap"]
pub type IFG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, IFG>;
impl<'a, REG> IFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "96 bit times"]
    #[inline(always)]
    pub fn ifg96(self) -> &'a mut crate::W<REG> {
        self.variant(IFG::Ifg96)
    }
    #[doc = "88 bit times"]
    #[inline(always)]
    pub fn ifg88(self) -> &'a mut crate::W<REG> {
        self.variant(IFG::Ifg88)
    }
    #[doc = "80 bit times"]
    #[inline(always)]
    pub fn ifg80(self) -> &'a mut crate::W<REG> {
        self.variant(IFG::Ifg80)
    }
    #[doc = "72 bit times"]
    #[inline(always)]
    pub fn ifg72(self) -> &'a mut crate::W<REG> {
        self.variant(IFG::Ifg72)
    }
    #[doc = "64 bit times"]
    #[inline(always)]
    pub fn ifg64(self) -> &'a mut crate::W<REG> {
        self.variant(IFG::Ifg64)
    }
    #[doc = "56 bit times"]
    #[inline(always)]
    pub fn ifg56(self) -> &'a mut crate::W<REG> {
        self.variant(IFG::Ifg56)
    }
    #[doc = "48 bit times"]
    #[inline(always)]
    pub fn ifg48(self) -> &'a mut crate::W<REG> {
        self.variant(IFG::Ifg48)
    }
    #[doc = "40 bit times"]
    #[inline(always)]
    pub fn ifg40(self) -> &'a mut crate::W<REG> {
        self.variant(IFG::Ifg40)
    }
}
#[doc = "Jabber disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JD {
    #[doc = "0: Jabber enabled, transmit frames up to 2048 bytes"]
    Enabled = 0,
    #[doc = "1: Jabber disabled, transmit frames up to 16384 bytes"]
    Disabled = 1,
}
impl From<JD> for bool {
    #[inline(always)]
    fn from(variant: JD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JD` reader - Jabber disable"]
pub type JD_R = crate::BitReader<JD>;
impl JD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JD {
        match self.bits {
            false => JD::Enabled,
            true => JD::Disabled,
        }
    }
    #[doc = "Jabber enabled, transmit frames up to 2048 bytes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JD::Enabled
    }
    #[doc = "Jabber disabled, transmit frames up to 16384 bytes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JD::Disabled
    }
}
#[doc = "Field `JD` writer - Jabber disable"]
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG, JD>;
impl<'a, REG> JD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Jabber enabled, transmit frames up to 2048 bytes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(JD::Enabled)
    }
    #[doc = "Jabber disabled, transmit frames up to 16384 bytes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JD::Disabled)
    }
}
#[doc = "Watchdog disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WD {
    #[doc = "0: Watchdog enabled, receive frames limited to 2048 bytes"]
    Enabled = 0,
    #[doc = "1: Watchdog disabled, receive frames may be up to to 16384 bytes"]
    Disabled = 1,
}
impl From<WD> for bool {
    #[inline(always)]
    fn from(variant: WD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WD` reader - Watchdog disable"]
pub type WD_R = crate::BitReader<WD>;
impl WD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WD {
        match self.bits {
            false => WD::Enabled,
            true => WD::Disabled,
        }
    }
    #[doc = "Watchdog enabled, receive frames limited to 2048 bytes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WD::Enabled
    }
    #[doc = "Watchdog disabled, receive frames may be up to to 16384 bytes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WD::Disabled
    }
}
#[doc = "Field `WD` writer - Watchdog disable"]
pub type WD_W<'a, REG> = crate::BitWriter<'a, REG, WD>;
impl<'a, REG> WD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Watchdog enabled, receive frames limited to 2048 bytes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WD::Enabled)
    }
    #[doc = "Watchdog disabled, receive frames may be up to to 16384 bytes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WD::Disabled)
    }
}
#[doc = "CRC stripping for type frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTF {
    #[doc = "0: CRC not stripped"]
    Disabled = 0,
    #[doc = "1: CRC stripped"]
    Enabled = 1,
}
impl From<CSTF> for bool {
    #[inline(always)]
    fn from(variant: CSTF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSTF` reader - CRC stripping for type frames"]
pub type CSTF_R = crate::BitReader<CSTF>;
impl CSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSTF {
        match self.bits {
            false => CSTF::Disabled,
            true => CSTF::Enabled,
        }
    }
    #[doc = "CRC not stripped"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSTF::Disabled
    }
    #[doc = "CRC stripped"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSTF::Enabled
    }
}
#[doc = "Field `CSTF` writer - CRC stripping for type frames"]
pub type CSTF_W<'a, REG> = crate::BitWriter<'a, REG, CSTF>;
impl<'a, REG> CSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC not stripped"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSTF::Disabled)
    }
    #[doc = "CRC stripped"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CSTF::Enabled)
    }
}
impl R {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CRC stripping for type frames"]
    #[inline(always)]
    pub fn cstf(&self) -> CSTF_R {
        CSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<MACCRrs> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<MACCRrs> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<MACCRrs> {
        DC_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<MACCRrs> {
        BL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    #[must_use]
    pub fn apcs(&mut self) -> APCS_W<MACCRrs> {
        APCS_W::new(self, 7)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<MACCRrs> {
        RD_W::new(self, 9)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    #[must_use]
    pub fn ipco(&mut self) -> IPCO_W<MACCRrs> {
        IPCO_W::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<MACCRrs> {
        DM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<MACCRrs> {
        LM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    #[must_use]
    pub fn rod(&mut self) -> ROD_W<MACCRrs> {
        ROD_W::new(self, 13)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<MACCRrs> {
        FES_W::new(self, 14)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    #[must_use]
    pub fn csd(&mut self) -> CSD_W<MACCRrs> {
        CSD_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<MACCRrs> {
        IFG_W::new(self, 17)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<MACCRrs> {
        JD_W::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<MACCRrs> {
        WD_W::new(self, 23)
    }
    #[doc = "Bit 25 - CRC stripping for type frames"]
    #[inline(always)]
    #[must_use]
    pub fn cstf(&mut self) -> CSTF_W<MACCRrs> {
        CSTF_W::new(self, 25)
    }
}
#[doc = "Ethernet MAC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACCRrs;
impl crate::RegisterSpec for MACCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maccr::R`](R) reader structure"]
impl crate::Readable for MACCRrs {}
#[doc = "`write(|w| ..)` method takes [`maccr::W`](W) writer structure"]
impl crate::Writable for MACCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACCR to value 0x8000"]
impl crate::Resettable for MACCRrs {
    const RESET_VALUE: u32 = 0x8000;
}
