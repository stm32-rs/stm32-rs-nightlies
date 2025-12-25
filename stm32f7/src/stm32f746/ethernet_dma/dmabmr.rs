///Register `DMABMR` reader
pub type R = crate::R<DMABMRrs>;
///Register `DMABMR` writer
pub type W = crate::W<DMABMRrs>;
/**Software reset

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR {
    ///1: Reset all MAC subsystem internal registers and logic. Cleared automatically
    Reset = 1,
}
impl From<SR> for bool {
    #[inline(always)]
    fn from(variant: SR) -> Self {
        variant as u8 != 0
    }
}
///Field `SR` reader - Software reset
pub type SR_R = crate::BitReader<SR>;
impl SR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SR> {
        match self.bits {
            true => Some(SR::Reset),
            _ => None,
        }
    }
    ///Reset all MAC subsystem internal registers and logic. Cleared automatically
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SR::Reset
    }
}
///Field `SR` writer - Software reset
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG, SR>;
impl<'a, REG> SR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset all MAC subsystem internal registers and logic. Cleared automatically
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SR::Reset)
    }
}
/**DMA arbitration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DA {
    ///0: Round-robin with Rx:Tx priority given by PM
    RoundRobin = 0,
    ///1: Rx has priority over Tx
    RxPriority = 1,
}
impl From<DA> for bool {
    #[inline(always)]
    fn from(variant: DA) -> Self {
        variant as u8 != 0
    }
}
///Field `DA` reader - DMA arbitration
pub type DA_R = crate::BitReader<DA>;
impl DA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DA {
        match self.bits {
            false => DA::RoundRobin,
            true => DA::RxPriority,
        }
    }
    ///Round-robin with Rx:Tx priority given by PM
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        *self == DA::RoundRobin
    }
    ///Rx has priority over Tx
    #[inline(always)]
    pub fn is_rx_priority(&self) -> bool {
        *self == DA::RxPriority
    }
}
///Field `DA` writer - DMA arbitration
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG, DA>;
impl<'a, REG> DA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Round-robin with Rx:Tx priority given by PM
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut crate::W<REG> {
        self.variant(DA::RoundRobin)
    }
    ///Rx has priority over Tx
    #[inline(always)]
    pub fn rx_priority(self) -> &'a mut crate::W<REG> {
        self.variant(DA::RxPriority)
    }
}
///Field `DSL` reader - Descriptor skip length
pub type DSL_R = crate::FieldReader;
///Field `DSL` writer - Descriptor skip length
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Enhanced descriptor format enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDFE {
    ///0: Normal descriptor format
    Disabled = 0,
    ///1: Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload
    Enabled = 1,
}
impl From<EDFE> for bool {
    #[inline(always)]
    fn from(variant: EDFE) -> Self {
        variant as u8 != 0
    }
}
///Field `EDFE` reader - Enhanced descriptor format enable
pub type EDFE_R = crate::BitReader<EDFE>;
impl EDFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EDFE {
        match self.bits {
            false => EDFE::Disabled,
            true => EDFE::Enabled,
        }
    }
    ///Normal descriptor format
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDFE::Disabled
    }
    ///Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDFE::Enabled
    }
}
///Field `EDFE` writer - Enhanced descriptor format enable
pub type EDFE_W<'a, REG> = crate::BitWriter<'a, REG, EDFE>;
impl<'a, REG> EDFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal descriptor format
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EDFE::Disabled)
    }
    ///Enhanced 32-byte descriptor format, required for timestamping and IPv4 checksum offload
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EDFE::Enabled)
    }
}
/**Programmable burst length

Value on reset: 33*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PBL {
    ///1: Maximum of 1 beat per DMA transaction
    Pbl1 = 1,
    ///2: Maximum of 2 beats per DMA transaction
    Pbl2 = 2,
    ///4: Maximum of 4 beats per DMA transaction
    Pbl4 = 4,
    ///8: Maximum of 8 beats per DMA transaction
    Pbl8 = 8,
    ///16: Maximum of 16 beats per DMA transaction
    Pbl16 = 16,
    ///32: Maximum of 32 beats per DMA transaction
    Pbl32 = 32,
}
impl From<PBL> for u8 {
    #[inline(always)]
    fn from(variant: PBL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PBL {
    type Ux = u8;
}
impl crate::IsEnum for PBL {}
///Field `PBL` reader - Programmable burst length
pub type PBL_R = crate::FieldReader<PBL>;
impl PBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PBL> {
        match self.bits {
            1 => Some(PBL::Pbl1),
            2 => Some(PBL::Pbl2),
            4 => Some(PBL::Pbl4),
            8 => Some(PBL::Pbl8),
            16 => Some(PBL::Pbl16),
            32 => Some(PBL::Pbl32),
            _ => None,
        }
    }
    ///Maximum of 1 beat per DMA transaction
    #[inline(always)]
    pub fn is_pbl1(&self) -> bool {
        *self == PBL::Pbl1
    }
    ///Maximum of 2 beats per DMA transaction
    #[inline(always)]
    pub fn is_pbl2(&self) -> bool {
        *self == PBL::Pbl2
    }
    ///Maximum of 4 beats per DMA transaction
    #[inline(always)]
    pub fn is_pbl4(&self) -> bool {
        *self == PBL::Pbl4
    }
    ///Maximum of 8 beats per DMA transaction
    #[inline(always)]
    pub fn is_pbl8(&self) -> bool {
        *self == PBL::Pbl8
    }
    ///Maximum of 16 beats per DMA transaction
    #[inline(always)]
    pub fn is_pbl16(&self) -> bool {
        *self == PBL::Pbl16
    }
    ///Maximum of 32 beats per DMA transaction
    #[inline(always)]
    pub fn is_pbl32(&self) -> bool {
        *self == PBL::Pbl32
    }
}
///Field `PBL` writer - Programmable burst length
pub type PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6, PBL>;
impl<'a, REG> PBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Maximum of 1 beat per DMA transaction
    #[inline(always)]
    pub fn pbl1(self) -> &'a mut crate::W<REG> {
        self.variant(PBL::Pbl1)
    }
    ///Maximum of 2 beats per DMA transaction
    #[inline(always)]
    pub fn pbl2(self) -> &'a mut crate::W<REG> {
        self.variant(PBL::Pbl2)
    }
    ///Maximum of 4 beats per DMA transaction
    #[inline(always)]
    pub fn pbl4(self) -> &'a mut crate::W<REG> {
        self.variant(PBL::Pbl4)
    }
    ///Maximum of 8 beats per DMA transaction
    #[inline(always)]
    pub fn pbl8(self) -> &'a mut crate::W<REG> {
        self.variant(PBL::Pbl8)
    }
    ///Maximum of 16 beats per DMA transaction
    #[inline(always)]
    pub fn pbl16(self) -> &'a mut crate::W<REG> {
        self.variant(PBL::Pbl16)
    }
    ///Maximum of 32 beats per DMA transaction
    #[inline(always)]
    pub fn pbl32(self) -> &'a mut crate::W<REG> {
        self.variant(PBL::Pbl32)
    }
}
/**Rx-Tx priority ratio

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PM {
    ///0: RxDMA priority over TxDMA is 1:1
    OneToOne = 0,
    ///1: RxDMA priority over TxDMA is 2:1
    TwoToOne = 1,
    ///2: RxDMA priority over TxDMA is 3:1
    ThreeToOne = 2,
    ///3: RxDMA priority over TxDMA is 4:1
    FourToOne = 3,
}
impl From<PM> for u8 {
    #[inline(always)]
    fn from(variant: PM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PM {
    type Ux = u8;
}
impl crate::IsEnum for PM {}
///Field `PM` reader - Rx-Tx priority ratio
pub type PM_R = crate::FieldReader<PM>;
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PM {
        match self.bits {
            0 => PM::OneToOne,
            1 => PM::TwoToOne,
            2 => PM::ThreeToOne,
            3 => PM::FourToOne,
            _ => unreachable!(),
        }
    }
    ///RxDMA priority over TxDMA is 1:1
    #[inline(always)]
    pub fn is_one_to_one(&self) -> bool {
        *self == PM::OneToOne
    }
    ///RxDMA priority over TxDMA is 2:1
    #[inline(always)]
    pub fn is_two_to_one(&self) -> bool {
        *self == PM::TwoToOne
    }
    ///RxDMA priority over TxDMA is 3:1
    #[inline(always)]
    pub fn is_three_to_one(&self) -> bool {
        *self == PM::ThreeToOne
    }
    ///RxDMA priority over TxDMA is 4:1
    #[inline(always)]
    pub fn is_four_to_one(&self) -> bool {
        *self == PM::FourToOne
    }
}
///Field `PM` writer - Rx-Tx priority ratio
pub type PM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PM, crate::Safe>;
impl<'a, REG> PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///RxDMA priority over TxDMA is 1:1
    #[inline(always)]
    pub fn one_to_one(self) -> &'a mut crate::W<REG> {
        self.variant(PM::OneToOne)
    }
    ///RxDMA priority over TxDMA is 2:1
    #[inline(always)]
    pub fn two_to_one(self) -> &'a mut crate::W<REG> {
        self.variant(PM::TwoToOne)
    }
    ///RxDMA priority over TxDMA is 3:1
    #[inline(always)]
    pub fn three_to_one(self) -> &'a mut crate::W<REG> {
        self.variant(PM::ThreeToOne)
    }
    ///RxDMA priority over TxDMA is 4:1
    #[inline(always)]
    pub fn four_to_one(self) -> &'a mut crate::W<REG> {
        self.variant(PM::FourToOne)
    }
}
/**Fixed burst

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FB {
    ///0: AHB uses SINGLE and INCR burst transfers
    Variable = 0,
    ///1: AHB uses only fixed burst transfers
    Fixed = 1,
}
impl From<FB> for bool {
    #[inline(always)]
    fn from(variant: FB) -> Self {
        variant as u8 != 0
    }
}
///Field `FB` reader - Fixed burst
pub type FB_R = crate::BitReader<FB>;
impl FB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FB {
        match self.bits {
            false => FB::Variable,
            true => FB::Fixed,
        }
    }
    ///AHB uses SINGLE and INCR burst transfers
    #[inline(always)]
    pub fn is_variable(&self) -> bool {
        *self == FB::Variable
    }
    ///AHB uses only fixed burst transfers
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == FB::Fixed
    }
}
///Field `FB` writer - Fixed burst
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG, FB>;
impl<'a, REG> FB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AHB uses SINGLE and INCR burst transfers
    #[inline(always)]
    pub fn variable(self) -> &'a mut crate::W<REG> {
        self.variant(FB::Variable)
    }
    ///AHB uses only fixed burst transfers
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(FB::Fixed)
    }
}
/**Rx DMA PBL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDP {
    ///1: 1 beat per RxDMA transaction
    Rdp1 = 1,
    ///2: 2 beats per RxDMA transaction
    Rdp2 = 2,
    ///4: 4 beats per RxDMA transaction
    Rdp4 = 4,
    ///8: 8 beats per RxDMA transaction
    Rdp8 = 8,
    ///16: 16 beats per RxDMA transaction
    Rdp16 = 16,
    ///32: 32 beats per RxDMA transaction
    Rdp32 = 32,
}
impl From<RDP> for u8 {
    #[inline(always)]
    fn from(variant: RDP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RDP {
    type Ux = u8;
}
impl crate::IsEnum for RDP {}
///Field `RDP` reader - Rx DMA PBL
pub type RDP_R = crate::FieldReader<RDP>;
impl RDP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RDP> {
        match self.bits {
            1 => Some(RDP::Rdp1),
            2 => Some(RDP::Rdp2),
            4 => Some(RDP::Rdp4),
            8 => Some(RDP::Rdp8),
            16 => Some(RDP::Rdp16),
            32 => Some(RDP::Rdp32),
            _ => None,
        }
    }
    ///1 beat per RxDMA transaction
    #[inline(always)]
    pub fn is_rdp1(&self) -> bool {
        *self == RDP::Rdp1
    }
    ///2 beats per RxDMA transaction
    #[inline(always)]
    pub fn is_rdp2(&self) -> bool {
        *self == RDP::Rdp2
    }
    ///4 beats per RxDMA transaction
    #[inline(always)]
    pub fn is_rdp4(&self) -> bool {
        *self == RDP::Rdp4
    }
    ///8 beats per RxDMA transaction
    #[inline(always)]
    pub fn is_rdp8(&self) -> bool {
        *self == RDP::Rdp8
    }
    ///16 beats per RxDMA transaction
    #[inline(always)]
    pub fn is_rdp16(&self) -> bool {
        *self == RDP::Rdp16
    }
    ///32 beats per RxDMA transaction
    #[inline(always)]
    pub fn is_rdp32(&self) -> bool {
        *self == RDP::Rdp32
    }
}
///Field `RDP` writer - Rx DMA PBL
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 6, RDP>;
impl<'a, REG> RDP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 beat per RxDMA transaction
    #[inline(always)]
    pub fn rdp1(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Rdp1)
    }
    ///2 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp2(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Rdp2)
    }
    ///4 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp4(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Rdp4)
    }
    ///8 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp8(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Rdp8)
    }
    ///16 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp16(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Rdp16)
    }
    ///32 beats per RxDMA transaction
    #[inline(always)]
    pub fn rdp32(self) -> &'a mut crate::W<REG> {
        self.variant(RDP::Rdp32)
    }
}
/**Use separate PBL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USP {
    ///0: PBL value used for both Rx and Tx DMA
    Combined = 0,
    ///1: RxDMA uses RDP value, TxDMA uses PBL value
    Separate = 1,
}
impl From<USP> for bool {
    #[inline(always)]
    fn from(variant: USP) -> Self {
        variant as u8 != 0
    }
}
///Field `USP` reader - Use separate PBL
pub type USP_R = crate::BitReader<USP>;
impl USP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USP {
        match self.bits {
            false => USP::Combined,
            true => USP::Separate,
        }
    }
    ///PBL value used for both Rx and Tx DMA
    #[inline(always)]
    pub fn is_combined(&self) -> bool {
        *self == USP::Combined
    }
    ///RxDMA uses RDP value, TxDMA uses PBL value
    #[inline(always)]
    pub fn is_separate(&self) -> bool {
        *self == USP::Separate
    }
}
///Field `USP` writer - Use separate PBL
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG, USP>;
impl<'a, REG> USP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PBL value used for both Rx and Tx DMA
    #[inline(always)]
    pub fn combined(self) -> &'a mut crate::W<REG> {
        self.variant(USP::Combined)
    }
    ///RxDMA uses RDP value, TxDMA uses PBL value
    #[inline(always)]
    pub fn separate(self) -> &'a mut crate::W<REG> {
        self.variant(USP::Separate)
    }
}
/**4xPBL mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPM {
    ///0: PBL values used as-is
    X1 = 0,
    ///1: PBL values multiplied by 4
    X4 = 1,
}
impl From<FPM> for bool {
    #[inline(always)]
    fn from(variant: FPM) -> Self {
        variant as u8 != 0
    }
}
///Field `FPM` reader - 4xPBL mode
pub type FPM_R = crate::BitReader<FPM>;
impl FPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FPM {
        match self.bits {
            false => FPM::X1,
            true => FPM::X4,
        }
    }
    ///PBL values used as-is
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == FPM::X1
    }
    ///PBL values multiplied by 4
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == FPM::X4
    }
}
///Field `FPM` writer - 4xPBL mode
pub type FPM_W<'a, REG> = crate::BitWriter<'a, REG, FPM>;
impl<'a, REG> FPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PBL values used as-is
    #[inline(always)]
    pub fn x1(self) -> &'a mut crate::W<REG> {
        self.variant(FPM::X1)
    }
    ///PBL values multiplied by 4
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(FPM::X4)
    }
}
/**Address-aligned beats

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AAB {
    ///0: Bursts are not aligned
    Unaligned = 0,
    ///1: Align bursts to start address LS bits. First burst alignment depends on FB bit
    Aligned = 1,
}
impl From<AAB> for bool {
    #[inline(always)]
    fn from(variant: AAB) -> Self {
        variant as u8 != 0
    }
}
///Field `AAB` reader - Address-aligned beats
pub type AAB_R = crate::BitReader<AAB>;
impl AAB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AAB {
        match self.bits {
            false => AAB::Unaligned,
            true => AAB::Aligned,
        }
    }
    ///Bursts are not aligned
    #[inline(always)]
    pub fn is_unaligned(&self) -> bool {
        *self == AAB::Unaligned
    }
    ///Align bursts to start address LS bits. First burst alignment depends on FB bit
    #[inline(always)]
    pub fn is_aligned(&self) -> bool {
        *self == AAB::Aligned
    }
}
///Field `AAB` writer - Address-aligned beats
pub type AAB_W<'a, REG> = crate::BitWriter<'a, REG, AAB>;
impl<'a, REG> AAB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bursts are not aligned
    #[inline(always)]
    pub fn unaligned(self) -> &'a mut crate::W<REG> {
        self.variant(AAB::Unaligned)
    }
    ///Align bursts to start address LS bits. First burst alignment depends on FB bit
    #[inline(always)]
    pub fn aligned(self) -> &'a mut crate::W<REG> {
        self.variant(AAB::Aligned)
    }
}
/**Mixed burst

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB {
    ///0: Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below
    Normal = 0,
    ///1: If FB is low, start all bursts greater than 16 with INCR (undefined burst)
    Mixed = 1,
}
impl From<MB> for bool {
    #[inline(always)]
    fn from(variant: MB) -> Self {
        variant as u8 != 0
    }
}
///Field `MB` reader - Mixed burst
pub type MB_R = crate::BitReader<MB>;
impl MB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB {
        match self.bits {
            false => MB::Normal,
            true => MB::Mixed,
        }
    }
    ///Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MB::Normal
    }
    ///If FB is low, start all bursts greater than 16 with INCR (undefined burst)
    #[inline(always)]
    pub fn is_mixed(&self) -> bool {
        *self == MB::Mixed
    }
}
///Field `MB` writer - Mixed burst
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG, MB>;
impl<'a, REG> MB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fixed burst transfers (INCRx and SINGLE) for burst lengths of 16 and below
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(MB::Normal)
    }
    ///If FB is low, start all bursts greater than 16 with INCR (undefined burst)
    #[inline(always)]
    pub fn mixed(self) -> &'a mut crate::W<REG> {
        self.variant(MB::Mixed)
    }
}
impl R {
    ///Bit 0 - Software reset
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA arbitration
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:6 - Descriptor skip length
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bit 7 - Enhanced descriptor format enable
    #[inline(always)]
    pub fn edfe(&self) -> EDFE_R {
        EDFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - Programmable burst length
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:15 - Rx-Tx priority ratio
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Fixed burst
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:22 - Rx DMA PBL
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    ///Bit 23 - Use separate PBL
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - 4xPBL mode
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Address-aligned beats
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Mixed burst
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMABMR")
            .field("sr", &self.sr())
            .field("da", &self.da())
            .field("dsl", &self.dsl())
            .field("edfe", &self.edfe())
            .field("pbl", &self.pbl())
            .field("pm", &self.pm())
            .field("fb", &self.fb())
            .field("rdp", &self.rdp())
            .field("usp", &self.usp())
            .field("fpm", &self.fpm())
            .field("aab", &self.aab())
            .field("mb", &self.mb())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software reset
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<'_, DMABMRrs> {
        SR_W::new(self, 0)
    }
    ///Bit 1 - DMA arbitration
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<'_, DMABMRrs> {
        DA_W::new(self, 1)
    }
    ///Bits 2:6 - Descriptor skip length
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W<'_, DMABMRrs> {
        DSL_W::new(self, 2)
    }
    ///Bit 7 - Enhanced descriptor format enable
    #[inline(always)]
    pub fn edfe(&mut self) -> EDFE_W<'_, DMABMRrs> {
        EDFE_W::new(self, 7)
    }
    ///Bits 8:13 - Programmable burst length
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W<'_, DMABMRrs> {
        PBL_W::new(self, 8)
    }
    ///Bits 14:15 - Rx-Tx priority ratio
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<'_, DMABMRrs> {
        PM_W::new(self, 14)
    }
    ///Bit 16 - Fixed burst
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<'_, DMABMRrs> {
        FB_W::new(self, 16)
    }
    ///Bits 17:22 - Rx DMA PBL
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, DMABMRrs> {
        RDP_W::new(self, 17)
    }
    ///Bit 23 - Use separate PBL
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W<'_, DMABMRrs> {
        USP_W::new(self, 23)
    }
    ///Bit 24 - 4xPBL mode
    #[inline(always)]
    pub fn fpm(&mut self) -> FPM_W<'_, DMABMRrs> {
        FPM_W::new(self, 24)
    }
    ///Bit 25 - Address-aligned beats
    #[inline(always)]
    pub fn aab(&mut self) -> AAB_W<'_, DMABMRrs> {
        AAB_W::new(self, 25)
    }
    ///Bit 26 - Mixed burst
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<'_, DMABMRrs> {
        MB_W::new(self, 26)
    }
}
/**Ethernet DMA bus mode register

You can [`read`](crate::Reg::read) this register and get [`dmabmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#Ethernet_DMA:DMABMR)*/
pub struct DMABMRrs;
impl crate::RegisterSpec for DMABMRrs {
    type Ux = u32;
}
///`read()` method returns [`dmabmr::R`](R) reader structure
impl crate::Readable for DMABMRrs {}
///`write(|w| ..)` method takes [`dmabmr::W`](W) writer structure
impl crate::Writable for DMABMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMABMR to value 0x2101
impl crate::Resettable for DMABMRrs {
    const RESET_VALUE: u32 = 0x2101;
}
