///Register `FDCAN_PSR` reader
pub type R = crate::R<FDCAN_PSRrs>;
///Register `FDCAN_PSR` writer
pub type W = crate::W<FDCAN_PSRrs>;
/**Last error code

Value on reset: 7*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEC {
    ///0: No error occurred since LEC\[2:0\] has been cleared by successful reception or transmission.
    B0x0 = 0,
    ///1: Stuff error.
    B0x1 = 1,
    ///2: Form error.
    B0x2 = 2,
    ///3: Ack error.
    B0x3 = 3,
    ///4: Bit1 error.
    B0x4 = 4,
    ///5: Bit0 error.
    B0x5 = 5,
    ///6: CRC error.
    B0x6 = 6,
    ///7: No change.
    B0x7 = 7,
}
impl From<LEC> for u8 {
    #[inline(always)]
    fn from(variant: LEC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEC {
    type Ux = u8;
}
impl crate::IsEnum for LEC {}
///Field `LEC` reader - Last error code
pub type LEC_R = crate::FieldReader<LEC>;
impl LEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LEC {
        match self.bits {
            0 => LEC::B0x0,
            1 => LEC::B0x1,
            2 => LEC::B0x2,
            3 => LEC::B0x3,
            4 => LEC::B0x4,
            5 => LEC::B0x5,
            6 => LEC::B0x6,
            7 => LEC::B0x7,
            _ => unreachable!(),
        }
    }
    ///No error occurred since LEC\[2:0\] has been cleared by successful reception or transmission.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LEC::B0x0
    }
    ///Stuff error.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LEC::B0x1
    }
    ///Form error.
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == LEC::B0x2
    }
    ///Ack error.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LEC::B0x3
    }
    ///Bit1 error.
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == LEC::B0x4
    }
    ///Bit0 error.
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == LEC::B0x5
    }
    ///CRC error.
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == LEC::B0x6
    }
    ///No change.
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == LEC::B0x7
    }
}
///Field `LEC` writer - Last error code
pub type LEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LEC, crate::Safe>;
impl<'a, REG> LEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No error occurred since LEC\[2:0\] has been cleared by successful reception or transmission.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::B0x0)
    }
    ///Stuff error.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::B0x1)
    }
    ///Form error.
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::B0x2)
    }
    ///Ack error.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::B0x3)
    }
    ///Bit1 error.
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::B0x4)
    }
    ///Bit0 error.
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::B0x5)
    }
    ///CRC error.
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::B0x6)
    }
    ///No change.
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(LEC::B0x7)
    }
}
/**Activity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACT {
    ///0: Synchronizing: node is synchronizing on CAN communication.
    B0x0 = 0,
    ///1: Idle: node is neither receiver nor transmitter.
    B0x1 = 1,
    ///2: Receiver: node is operating as receiver.
    B0x2 = 2,
    ///3: Transmitter: node is operating as transmitter.
    B0x3 = 3,
}
impl From<ACT> for u8 {
    #[inline(always)]
    fn from(variant: ACT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACT {
    type Ux = u8;
}
impl crate::IsEnum for ACT {}
///Field `ACT` reader - Activity
pub type ACT_R = crate::FieldReader<ACT>;
impl ACT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACT {
        match self.bits {
            0 => ACT::B0x0,
            1 => ACT::B0x1,
            2 => ACT::B0x2,
            3 => ACT::B0x3,
            _ => unreachable!(),
        }
    }
    ///Synchronizing: node is synchronizing on CAN communication.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ACT::B0x0
    }
    ///Idle: node is neither receiver nor transmitter.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ACT::B0x1
    }
    ///Receiver: node is operating as receiver.
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ACT::B0x2
    }
    ///Transmitter: node is operating as transmitter.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ACT::B0x3
    }
}
/**Error passive

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP {
    ///0: The FDCAN is in the error-active state.
    B0x0 = 0,
    ///1: The FDCAN is in the error-passive state.
    B0x1 = 1,
}
impl From<EP> for bool {
    #[inline(always)]
    fn from(variant: EP) -> Self {
        variant as u8 != 0
    }
}
///Field `EP` reader - Error passive
pub type EP_R = crate::BitReader<EP>;
impl EP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EP {
        match self.bits {
            false => EP::B0x0,
            true => EP::B0x1,
        }
    }
    ///The FDCAN is in the error-active state.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EP::B0x0
    }
    ///The FDCAN is in the error-passive state.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EP::B0x1
    }
}
/**Warning status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EW {
    ///0: Both error counters are below the error-warning limit of 96.
    B0x0 = 0,
    ///1: At least one of error counter has reached the error-warning limit of 96.
    B0x1 = 1,
}
impl From<EW> for bool {
    #[inline(always)]
    fn from(variant: EW) -> Self {
        variant as u8 != 0
    }
}
///Field `EW` reader - Warning status
pub type EW_R = crate::BitReader<EW>;
impl EW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EW {
        match self.bits {
            false => EW::B0x0,
            true => EW::B0x1,
        }
    }
    ///Both error counters are below the error-warning limit of 96.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EW::B0x0
    }
    ///At least one of error counter has reached the error-warning limit of 96.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EW::B0x1
    }
}
/**Bus-off status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BO {
    ///0: The FDCAN is not in bus-off state.
    B0x0 = 0,
    ///1: The FDCAN is in bus-off state.
    B0x1 = 1,
}
impl From<BO> for bool {
    #[inline(always)]
    fn from(variant: BO) -> Self {
        variant as u8 != 0
    }
}
///Field `BO` reader - Bus-off status
pub type BO_R = crate::BitReader<BO>;
impl BO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BO {
        match self.bits {
            false => BO::B0x0,
            true => BO::B0x1,
        }
    }
    ///The FDCAN is not in bus-off state.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BO::B0x0
    }
    ///The FDCAN is in bus-off state.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BO::B0x1
    }
}
///Field `DLEC` reader - Data last error code
pub type DLEC_R = crate::FieldReader;
///Field `DLEC` writer - Data last error code
pub type DLEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**ESI flag of last received FDCAN message

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESI {
    ///0: Last received FDCAN message did not have its ESI flag set.
    B0x0 = 0,
    ///1: Last received FDCAN message had its ESI flag set.
    B0x1 = 1,
}
impl From<RESI> for bool {
    #[inline(always)]
    fn from(variant: RESI) -> Self {
        variant as u8 != 0
    }
}
///Field `RESI` reader - ESI flag of last received FDCAN message
pub type RESI_R = crate::BitReader<RESI>;
impl RESI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RESI {
        match self.bits {
            false => RESI::B0x0,
            true => RESI::B0x1,
        }
    }
    ///Last received FDCAN message did not have its ESI flag set.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RESI::B0x0
    }
    ///Last received FDCAN message had its ESI flag set.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RESI::B0x1
    }
}
///Field `RESI` writer - ESI flag of last received FDCAN message
pub type RESI_W<'a, REG> = crate::BitWriter<'a, REG, RESI>;
impl<'a, REG> RESI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Last received FDCAN message did not have its ESI flag set.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RESI::B0x0)
    }
    ///Last received FDCAN message had its ESI flag set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RESI::B0x1)
    }
}
/**BRS flag of last received FDCAN message

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBRS {
    ///0: Last received FDCAN message did not have its BRS flag set.
    B0x0 = 0,
    ///1: Last received FDCAN message had its BRS flag set.
    B0x1 = 1,
}
impl From<RBRS> for bool {
    #[inline(always)]
    fn from(variant: RBRS) -> Self {
        variant as u8 != 0
    }
}
///Field `RBRS` reader - BRS flag of last received FDCAN message
pub type RBRS_R = crate::BitReader<RBRS>;
impl RBRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RBRS {
        match self.bits {
            false => RBRS::B0x0,
            true => RBRS::B0x1,
        }
    }
    ///Last received FDCAN message did not have its BRS flag set.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RBRS::B0x0
    }
    ///Last received FDCAN message had its BRS flag set.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RBRS::B0x1
    }
}
///Field `RBRS` writer - BRS flag of last received FDCAN message
pub type RBRS_W<'a, REG> = crate::BitWriter<'a, REG, RBRS>;
impl<'a, REG> RBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Last received FDCAN message did not have its BRS flag set.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RBRS::B0x0)
    }
    ///Last received FDCAN message had its BRS flag set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RBRS::B0x1)
    }
}
/**Received FDCAN message

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REDL {
    ///0: Since this bit was cleared by the CPU, no FDCAN message has been received.
    B0x0 = 0,
    ///1: Message in FDCAN format with EDL flag set has been received.
    B0x1 = 1,
}
impl From<REDL> for bool {
    #[inline(always)]
    fn from(variant: REDL) -> Self {
        variant as u8 != 0
    }
}
///Field `REDL` reader - Received FDCAN message
pub type REDL_R = crate::BitReader<REDL>;
impl REDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REDL {
        match self.bits {
            false => REDL::B0x0,
            true => REDL::B0x1,
        }
    }
    ///Since this bit was cleared by the CPU, no FDCAN message has been received.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REDL::B0x0
    }
    ///Message in FDCAN format with EDL flag set has been received.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REDL::B0x1
    }
}
///Field `REDL` writer - Received FDCAN message
pub type REDL_W<'a, REG> = crate::BitWriter<'a, REG, REDL>;
impl<'a, REG> REDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Since this bit was cleared by the CPU, no FDCAN message has been received.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REDL::B0x0)
    }
    ///Message in FDCAN format with EDL flag set has been received.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REDL::B0x1)
    }
}
/**Protocol exception event

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PXE {
    ///0: No protocol exception event occurred since last read access
    B0x0 = 0,
    ///1: Protocol exception event occurred
    B0x1 = 1,
}
impl From<PXE> for bool {
    #[inline(always)]
    fn from(variant: PXE) -> Self {
        variant as u8 != 0
    }
}
///Field `PXE` reader - Protocol exception event
pub type PXE_R = crate::BitReader<PXE>;
impl PXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PXE {
        match self.bits {
            false => PXE::B0x0,
            true => PXE::B0x1,
        }
    }
    ///No protocol exception event occurred since last read access
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PXE::B0x0
    }
    ///Protocol exception event occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PXE::B0x1
    }
}
///Field `PXE` writer - Protocol exception event
pub type PXE_W<'a, REG> = crate::BitWriter<'a, REG, PXE>;
impl<'a, REG> PXE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No protocol exception event occurred since last read access
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PXE::B0x0)
    }
    ///Protocol exception event occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PXE::B0x1)
    }
}
///Field `TDCV` reader - Transmitter delay compensation value
pub type TDCV_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Last error code
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - Activity
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - Error passive
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Warning status
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Bus-off status
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Data last error code
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - ESI flag of last received FDCAN message
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - BRS flag of last received FDCAN message
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Received FDCAN message
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Protocol exception event
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:22 - Transmitter delay compensation value
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_PSR")
            .field("lec", &self.lec())
            .field("act", &self.act())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("dlec", &self.dlec())
            .field("resi", &self.resi())
            .field("rbrs", &self.rbrs())
            .field("redl", &self.redl())
            .field("pxe", &self.pxe())
            .field("tdcv", &self.tdcv())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Last error code
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W<'_, FDCAN_PSRrs> {
        LEC_W::new(self, 0)
    }
    ///Bits 8:10 - Data last error code
    #[inline(always)]
    pub fn dlec(&mut self) -> DLEC_W<'_, FDCAN_PSRrs> {
        DLEC_W::new(self, 8)
    }
    ///Bit 11 - ESI flag of last received FDCAN message
    #[inline(always)]
    pub fn resi(&mut self) -> RESI_W<'_, FDCAN_PSRrs> {
        RESI_W::new(self, 11)
    }
    ///Bit 12 - BRS flag of last received FDCAN message
    #[inline(always)]
    pub fn rbrs(&mut self) -> RBRS_W<'_, FDCAN_PSRrs> {
        RBRS_W::new(self, 12)
    }
    ///Bit 13 - Received FDCAN message
    #[inline(always)]
    pub fn redl(&mut self) -> REDL_W<'_, FDCAN_PSRrs> {
        REDL_W::new(self, 13)
    }
    ///Bit 14 - Protocol exception event
    #[inline(always)]
    pub fn pxe(&mut self) -> PXE_W<'_, FDCAN_PSRrs> {
        PXE_W::new(self, 14)
    }
}
/**FDCAN protocol status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_psr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_psr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_PSR)*/
pub struct FDCAN_PSRrs;
impl crate::RegisterSpec for FDCAN_PSRrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_psr::R`](R) reader structure
impl crate::Readable for FDCAN_PSRrs {}
///`write(|w| ..)` method takes [`fdcan_psr::W`](W) writer structure
impl crate::Writable for FDCAN_PSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_PSR to value 0x0707
impl crate::Resettable for FDCAN_PSRrs {
    const RESET_VALUE: u32 = 0x0707;
}
