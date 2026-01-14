///Register `FDCAN_CCCR` reader
pub type R = crate::R<FDCAN_CCCRrs>;
///Register `FDCAN_CCCR` writer
pub type W = crate::W<FDCAN_CCCRrs>;
/**Initialization

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT {
    ///0: Normal operation
    B0x0 = 0,
    ///1: Initialization started
    B0x1 = 1,
}
impl From<INIT> for bool {
    #[inline(always)]
    fn from(variant: INIT) -> Self {
        variant as u8 != 0
    }
}
///Field `INIT` reader - Initialization
pub type INIT_R = crate::BitReader<INIT>;
impl INIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INIT {
        match self.bits {
            false => INIT::B0x0,
            true => INIT::B0x1,
        }
    }
    ///Normal operation
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == INIT::B0x0
    }
    ///Initialization started
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == INIT::B0x1
    }
}
///Field `INIT` writer - Initialization
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG, INIT>;
impl<'a, REG> INIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal operation
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(INIT::B0x0)
    }
    ///Initialization started
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(INIT::B0x1)
    }
}
/**Configuration change enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCE {
    ///0: The CPU has no write access to the protected configuration registers.
    B0x0 = 0,
    ///1: The CPU has write access to the protected configuration registers (while INIT set in FDCAN_CCCR).
    B0x1 = 1,
}
impl From<CCE> for bool {
    #[inline(always)]
    fn from(variant: CCE) -> Self {
        variant as u8 != 0
    }
}
///Field `CCE` reader - Configuration change enable
pub type CCE_R = crate::BitReader<CCE>;
impl CCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCE {
        match self.bits {
            false => CCE::B0x0,
            true => CCE::B0x1,
        }
    }
    ///The CPU has no write access to the protected configuration registers.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCE::B0x0
    }
    ///The CPU has write access to the protected configuration registers (while INIT set in FDCAN_CCCR).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCE::B0x1
    }
}
///Field `CCE` writer - Configuration change enable
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG, CCE>;
impl<'a, REG> CCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The CPU has no write access to the protected configuration registers.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCE::B0x0)
    }
    ///The CPU has write access to the protected configuration registers (while INIT set in FDCAN_CCCR).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCE::B0x1)
    }
}
/**ASM restricted operation mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASM {
    ///0: Normal CAN operation
    B0x0 = 0,
    ///1: Restricted operation mode active
    B0x1 = 1,
}
impl From<ASM> for bool {
    #[inline(always)]
    fn from(variant: ASM) -> Self {
        variant as u8 != 0
    }
}
///Field `ASM` reader - ASM restricted operation mode
pub type ASM_R = crate::BitReader<ASM>;
impl ASM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASM {
        match self.bits {
            false => ASM::B0x0,
            true => ASM::B0x1,
        }
    }
    ///Normal CAN operation
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ASM::B0x0
    }
    ///Restricted operation mode active
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ASM::B0x1
    }
}
///Field `ASM` writer - ASM restricted operation mode
pub type ASM_W<'a, REG> = crate::BitWriter<'a, REG, ASM>;
impl<'a, REG> ASM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal CAN operation
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ASM::B0x0)
    }
    ///Restricted operation mode active
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ASM::B0x1)
    }
}
/**Clock stop acknowledge

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSA {
    ///0: No clock stop acknowledged
    B0x0 = 0,
    ///1: FDCAN can be set in power-down by stopping APB clock and kernel clock.
    B0x1 = 1,
}
impl From<CSA> for bool {
    #[inline(always)]
    fn from(variant: CSA) -> Self {
        variant as u8 != 0
    }
}
///Field `CSA` reader - Clock stop acknowledge
pub type CSA_R = crate::BitReader<CSA>;
impl CSA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSA {
        match self.bits {
            false => CSA::B0x0,
            true => CSA::B0x1,
        }
    }
    ///No clock stop acknowledged
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSA::B0x0
    }
    ///FDCAN can be set in power-down by stopping APB clock and kernel clock.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSA::B0x1
    }
}
/**Clock stop request

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSR {
    ///0: No clock stop requested
    B0x0 = 0,
    ///1: Clock stop requested.
    B0x1 = 1,
}
impl From<CSR> for bool {
    #[inline(always)]
    fn from(variant: CSR) -> Self {
        variant as u8 != 0
    }
}
///Field `CSR` reader - Clock stop request
pub type CSR_R = crate::BitReader<CSR>;
impl CSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSR {
        match self.bits {
            false => CSR::B0x0,
            true => CSR::B0x1,
        }
    }
    ///No clock stop requested
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSR::B0x0
    }
    ///Clock stop requested.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSR::B0x1
    }
}
///Field `CSR` writer - Clock stop request
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG, CSR>;
impl<'a, REG> CSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No clock stop requested
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CSR::B0x0)
    }
    ///Clock stop requested.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CSR::B0x1)
    }
}
/**Bus monitoring mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MON {
    ///0: Bus monitoring mode disabled
    B0x0 = 0,
    ///1: Bus monitoring mode enabled
    B0x1 = 1,
}
impl From<MON> for bool {
    #[inline(always)]
    fn from(variant: MON) -> Self {
        variant as u8 != 0
    }
}
///Field `MON` reader - Bus monitoring mode
pub type MON_R = crate::BitReader<MON>;
impl MON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MON {
        match self.bits {
            false => MON::B0x0,
            true => MON::B0x1,
        }
    }
    ///Bus monitoring mode disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MON::B0x0
    }
    ///Bus monitoring mode enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MON::B0x1
    }
}
///Field `MON` writer - Bus monitoring mode
pub type MON_W<'a, REG> = crate::BitWriter<'a, REG, MON>;
impl<'a, REG> MON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus monitoring mode disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MON::B0x0)
    }
    ///Bus monitoring mode enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MON::B0x1)
    }
}
/**Disable automatic retransmission

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAR {
    ///0: Automatic retransmission of messages not transmitted successfully enabled
    B0x0 = 0,
    ///1: Automatic retransmission disabled
    B0x1 = 1,
}
impl From<DAR> for bool {
    #[inline(always)]
    fn from(variant: DAR) -> Self {
        variant as u8 != 0
    }
}
///Field `DAR` reader - Disable automatic retransmission
pub type DAR_R = crate::BitReader<DAR>;
impl DAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAR {
        match self.bits {
            false => DAR::B0x0,
            true => DAR::B0x1,
        }
    }
    ///Automatic retransmission of messages not transmitted successfully enabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAR::B0x0
    }
    ///Automatic retransmission disabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAR::B0x1
    }
}
///Field `DAR` writer - Disable automatic retransmission
pub type DAR_W<'a, REG> = crate::BitWriter<'a, REG, DAR>;
impl<'a, REG> DAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Automatic retransmission of messages not transmitted successfully enabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DAR::B0x0)
    }
    ///Automatic retransmission disabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DAR::B0x1)
    }
}
/**Test mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEST {
    ///0: Normal operation, FDCAN_TEST holds reset values
    B0x0 = 0,
    ///1: Test mode, write access to FDCAN_TEST enabled
    B0x1 = 1,
}
impl From<TEST> for bool {
    #[inline(always)]
    fn from(variant: TEST) -> Self {
        variant as u8 != 0
    }
}
///Field `TEST` reader - Test mode enable
pub type TEST_R = crate::BitReader<TEST>;
impl TEST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEST {
        match self.bits {
            false => TEST::B0x0,
            true => TEST::B0x1,
        }
    }
    ///Normal operation, FDCAN_TEST holds reset values
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEST::B0x0
    }
    ///Test mode, write access to FDCAN_TEST enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEST::B0x1
    }
}
///Field `TEST` writer - Test mode enable
pub type TEST_W<'a, REG> = crate::BitWriter<'a, REG, TEST>;
impl<'a, REG> TEST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal operation, FDCAN_TEST holds reset values
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEST::B0x0)
    }
    ///Test mode, write access to FDCAN_TEST enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEST::B0x1)
    }
}
/**FD operation enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDOE {
    ///0: FD operation disabled
    B0x0 = 0,
    ///1: FD operation enabled
    B0x1 = 1,
}
impl From<FDOE> for bool {
    #[inline(always)]
    fn from(variant: FDOE) -> Self {
        variant as u8 != 0
    }
}
///Field `FDOE` reader - FD operation enable
pub type FDOE_R = crate::BitReader<FDOE>;
impl FDOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FDOE {
        match self.bits {
            false => FDOE::B0x0,
            true => FDOE::B0x1,
        }
    }
    ///FD operation disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FDOE::B0x0
    }
    ///FD operation enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FDOE::B0x1
    }
}
///Field `FDOE` writer - FD operation enable
pub type FDOE_W<'a, REG> = crate::BitWriter<'a, REG, FDOE>;
impl<'a, REG> FDOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FD operation disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FDOE::B0x0)
    }
    ///FD operation enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FDOE::B0x1)
    }
}
/**FDCAN bit rate switching

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRSE {
    ///0: Bit rate switching for transmissions disabled
    B0x0 = 0,
    ///1: Bit rate switching for transmissions enabled
    B0x1 = 1,
}
impl From<BRSE> for bool {
    #[inline(always)]
    fn from(variant: BRSE) -> Self {
        variant as u8 != 0
    }
}
///Field `BRSE` reader - FDCAN bit rate switching
pub type BRSE_R = crate::BitReader<BRSE>;
impl BRSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRSE {
        match self.bits {
            false => BRSE::B0x0,
            true => BRSE::B0x1,
        }
    }
    ///Bit rate switching for transmissions disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRSE::B0x0
    }
    ///Bit rate switching for transmissions enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRSE::B0x1
    }
}
///Field `BRSE` writer - FDCAN bit rate switching
pub type BRSE_W<'a, REG> = crate::BitWriter<'a, REG, BRSE>;
impl<'a, REG> BRSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bit rate switching for transmissions disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BRSE::B0x0)
    }
    ///Bit rate switching for transmissions enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BRSE::B0x1)
    }
}
/**Protocol exception handling disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PXHD {
    ///0: Protocol exception handling enabled
    B0x0 = 0,
    ///1: Protocol exception handling disabled
    B0x1 = 1,
}
impl From<PXHD> for bool {
    #[inline(always)]
    fn from(variant: PXHD) -> Self {
        variant as u8 != 0
    }
}
///Field `PXHD` reader - Protocol exception handling disable
pub type PXHD_R = crate::BitReader<PXHD>;
impl PXHD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PXHD {
        match self.bits {
            false => PXHD::B0x0,
            true => PXHD::B0x1,
        }
    }
    ///Protocol exception handling enabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PXHD::B0x0
    }
    ///Protocol exception handling disabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PXHD::B0x1
    }
}
///Field `PXHD` writer - Protocol exception handling disable
pub type PXHD_W<'a, REG> = crate::BitWriter<'a, REG, PXHD>;
impl<'a, REG> PXHD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Protocol exception handling enabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PXHD::B0x0)
    }
    ///Protocol exception handling disabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PXHD::B0x1)
    }
}
/**Edge filtering during bus integration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EFBI {
    ///0: Edge filtering disabled
    B0x0 = 0,
    ///1: Two consecutive dominant tless thansub>qless than/sub> required to detect an edge for hard synchronization
    B0x1 = 1,
}
impl From<EFBI> for bool {
    #[inline(always)]
    fn from(variant: EFBI) -> Self {
        variant as u8 != 0
    }
}
///Field `EFBI` reader - Edge filtering during bus integration
pub type EFBI_R = crate::BitReader<EFBI>;
impl EFBI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EFBI {
        match self.bits {
            false => EFBI::B0x0,
            true => EFBI::B0x1,
        }
    }
    ///Edge filtering disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EFBI::B0x0
    }
    ///Two consecutive dominant tless thansub>qless than/sub> required to detect an edge for hard synchronization
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EFBI::B0x1
    }
}
///Field `EFBI` writer - Edge filtering during bus integration
pub type EFBI_W<'a, REG> = crate::BitWriter<'a, REG, EFBI>;
impl<'a, REG> EFBI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Edge filtering disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EFBI::B0x0)
    }
    ///Two consecutive dominant tless thansub>qless than/sub> required to detect an edge for hard synchronization
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EFBI::B0x1)
    }
}
/**Transmit pause enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXP {
    ///0: Disabled
    B0x0 = 0,
    ///1: Enabled
    B0x1 = 1,
}
impl From<TXP> for bool {
    #[inline(always)]
    fn from(variant: TXP) -> Self {
        variant as u8 != 0
    }
}
///Field `TXP` reader - Transmit pause enable
pub type TXP_R = crate::BitReader<TXP>;
impl TXP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXP {
        match self.bits {
            false => TXP::B0x0,
            true => TXP::B0x1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXP::B0x0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXP::B0x1
    }
}
///Field `TXP` writer - Transmit pause enable
pub type TXP_W<'a, REG> = crate::BitWriter<'a, REG, TXP>;
impl<'a, REG> TXP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXP::B0x0)
    }
    ///Enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXP::B0x1)
    }
}
/**Non-ISO operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NISO {
    ///0: CAN FD frame format according to ISO11898-1
    B0x0 = 0,
    ///1: CAN FD frame format according to Bosch CAN FD Specification V1.
    B0x1 = 1,
}
impl From<NISO> for bool {
    #[inline(always)]
    fn from(variant: NISO) -> Self {
        variant as u8 != 0
    }
}
///Field `NISO` reader - Non-ISO operation
pub type NISO_R = crate::BitReader<NISO>;
impl NISO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NISO {
        match self.bits {
            false => NISO::B0x0,
            true => NISO::B0x1,
        }
    }
    ///CAN FD frame format according to ISO11898-1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NISO::B0x0
    }
    ///CAN FD frame format according to Bosch CAN FD Specification V1.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NISO::B0x1
    }
}
///Field `NISO` writer - Non-ISO operation
pub type NISO_W<'a, REG> = crate::BitWriter<'a, REG, NISO>;
impl<'a, REG> NISO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CAN FD frame format according to ISO11898-1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NISO::B0x0)
    }
    ///CAN FD frame format according to Bosch CAN FD Specification V1.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NISO::B0x1)
    }
}
impl R {
    ///Bit 0 - Initialization
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Configuration change enable
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ASM restricted operation mode
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clock stop acknowledge
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clock stop request
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bus monitoring mode
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Disable automatic retransmission
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Test mode enable
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - FD operation enable
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FDCAN bit rate switching
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - Protocol exception handling disable
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Edge filtering during bus integration
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Transmit pause enable
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Non-ISO operation
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_CCCR")
            .field("init", &self.init())
            .field("cce", &self.cce())
            .field("asm", &self.asm())
            .field("csa", &self.csa())
            .field("csr", &self.csr())
            .field("mon", &self.mon())
            .field("dar", &self.dar())
            .field("test", &self.test())
            .field("fdoe", &self.fdoe())
            .field("brse", &self.brse())
            .field("pxhd", &self.pxhd())
            .field("efbi", &self.efbi())
            .field("txp", &self.txp())
            .field("niso", &self.niso())
            .finish()
    }
}
impl W {
    ///Bit 0 - Initialization
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, FDCAN_CCCRrs> {
        INIT_W::new(self, 0)
    }
    ///Bit 1 - Configuration change enable
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W<'_, FDCAN_CCCRrs> {
        CCE_W::new(self, 1)
    }
    ///Bit 2 - ASM restricted operation mode
    #[inline(always)]
    pub fn asm(&mut self) -> ASM_W<'_, FDCAN_CCCRrs> {
        ASM_W::new(self, 2)
    }
    ///Bit 4 - Clock stop request
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W<'_, FDCAN_CCCRrs> {
        CSR_W::new(self, 4)
    }
    ///Bit 5 - Bus monitoring mode
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W<'_, FDCAN_CCCRrs> {
        MON_W::new(self, 5)
    }
    ///Bit 6 - Disable automatic retransmission
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W<'_, FDCAN_CCCRrs> {
        DAR_W::new(self, 6)
    }
    ///Bit 7 - Test mode enable
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W<'_, FDCAN_CCCRrs> {
        TEST_W::new(self, 7)
    }
    ///Bit 8 - FD operation enable
    #[inline(always)]
    pub fn fdoe(&mut self) -> FDOE_W<'_, FDCAN_CCCRrs> {
        FDOE_W::new(self, 8)
    }
    ///Bit 9 - FDCAN bit rate switching
    #[inline(always)]
    pub fn brse(&mut self) -> BRSE_W<'_, FDCAN_CCCRrs> {
        BRSE_W::new(self, 9)
    }
    ///Bit 12 - Protocol exception handling disable
    #[inline(always)]
    pub fn pxhd(&mut self) -> PXHD_W<'_, FDCAN_CCCRrs> {
        PXHD_W::new(self, 12)
    }
    ///Bit 13 - Edge filtering during bus integration
    #[inline(always)]
    pub fn efbi(&mut self) -> EFBI_W<'_, FDCAN_CCCRrs> {
        EFBI_W::new(self, 13)
    }
    ///Bit 14 - Transmit pause enable
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W<'_, FDCAN_CCCRrs> {
        TXP_W::new(self, 14)
    }
    ///Bit 15 - Non-ISO operation
    #[inline(always)]
    pub fn niso(&mut self) -> NISO_W<'_, FDCAN_CCCRrs> {
        NISO_W::new(self, 15)
    }
}
/**FDCAN CC control register

You can [`read`](crate::Reg::read) this register and get [`fdcan_cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_CCCR)*/
pub struct FDCAN_CCCRrs;
impl crate::RegisterSpec for FDCAN_CCCRrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_cccr::R`](R) reader structure
impl crate::Readable for FDCAN_CCCRrs {}
///`write(|w| ..)` method takes [`fdcan_cccr::W`](W) writer structure
impl crate::Writable for FDCAN_CCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_CCCR to value 0x01
impl crate::Resettable for FDCAN_CCCRrs {
    const RESET_VALUE: u32 = 0x01;
}
