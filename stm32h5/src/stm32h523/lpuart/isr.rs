///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `PE` reader - Parity error
pub type PE_R = crate::BitReader;
///Field `FE` reader - Framing error
pub type FE_R = crate::BitReader;
///Field `NE` reader - Start bit noise detection flag
pub type NE_R = crate::BitReader;
///Field `ORE` reader - Overrun error
pub type ORE_R = crate::BitReader;
///Field `IDLE` reader - Idle line detected
pub type IDLE_R = crate::BitReader;
///Field `RXFNE` reader - RXFIFO not empty
pub type RXFNE_R = crate::BitReader;
///Field `TC` reader - Transmission complete
pub type TC_R = crate::BitReader;
///Field `TXFNF` reader - TXFIFO not full
pub type TXFNF_R = crate::BitReader;
///Field `CTSIF` reader - CTS interrupt flag
pub type CTSIF_R = crate::BitReader;
///Field `CTS` reader - CTS flag
pub type CTS_R = crate::BitReader;
///Field `BUSY` reader - Busy flag
pub type BUSY_R = crate::BitReader;
///Field `CMF` reader - Character match flag
pub type CMF_R = crate::BitReader;
///Field `SBKF` reader - Send break flag
pub type SBKF_R = crate::BitReader;
/**Receiver wake-up from mute mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWU {
    ///0: Receiver in Active mode
    Active = 0,
    ///1: Receiver in Mute mode
    Mute = 1,
}
impl From<RWU> for bool {
    #[inline(always)]
    fn from(variant: RWU) -> Self {
        variant as u8 != 0
    }
}
///Field `RWU` reader - Receiver wake-up from mute mode
pub type RWU_R = crate::BitReader<RWU>;
impl RWU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWU {
        match self.bits {
            false => RWU::Active,
            true => RWU::Mute,
        }
    }
    ///Receiver in Active mode
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RWU::Active
    }
    ///Receiver in Mute mode
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == RWU::Mute
    }
}
///Field `WUF` reader - Wake-up from low-power mode flag
pub type WUF_R = crate::BitReader;
///Field `TEACK` reader - Transmit enable acknowledge flag
pub type TEACK_R = crate::BitReader;
///Field `REACK` reader - Receive enable acknowledge flag
pub type REACK_R = crate::BitReader;
/**TXFIFO Empty

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFE {
    ///0: TXFIFO not empty.
    NotEmpty = 0,
    ///1: TXFIFO empty.
    Empty = 1,
}
impl From<TXFE> for bool {
    #[inline(always)]
    fn from(variant: TXFE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFE` reader - TXFIFO Empty
pub type TXFE_R = crate::BitReader<TXFE>;
impl TXFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFE {
        match self.bits {
            false => TXFE::NotEmpty,
            true => TXFE::Empty,
        }
    }
    ///TXFIFO not empty.
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXFE::NotEmpty
    }
    ///TXFIFO empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFE::Empty
    }
}
/**RXFIFO Full

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFF {
    ///0: RXFIFO not full.
    NotFull = 0,
    ///1: RXFIFO Full.
    Full = 1,
}
impl From<RXFF> for bool {
    #[inline(always)]
    fn from(variant: RXFF) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFF` reader - RXFIFO Full
pub type RXFF_R = crate::BitReader<RXFF>;
impl RXFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFF {
        match self.bits {
            false => RXFF::NotFull,
            true => RXFF::Full,
        }
    }
    ///RXFIFO not full.
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RXFF::NotFull
    }
    ///RXFIFO Full.
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFF::Full
    }
}
/**RXFIFO threshold flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFT {
    ///0: Receive FIFO does not reach the programmed threshold.
    NotReached = 0,
    ///1: Receive FIFO reached the programmed threshold.
    Reached = 1,
}
impl From<RXFT> for bool {
    #[inline(always)]
    fn from(variant: RXFT) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFT` reader - RXFIFO threshold flag
pub type RXFT_R = crate::BitReader<RXFT>;
impl RXFT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFT {
        match self.bits {
            false => RXFT::NotReached,
            true => RXFT::Reached,
        }
    }
    ///Receive FIFO does not reach the programmed threshold.
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == RXFT::NotReached
    }
    ///Receive FIFO reached the programmed threshold.
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == RXFT::Reached
    }
}
/**TXFIFO threshold flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFT {
    ///0: TXFIFO does not reach the programmed threshold.
    NotReached = 0,
    ///1: TXFIFO reached the programmed threshold.
    Reached = 1,
}
impl From<TXFT> for bool {
    #[inline(always)]
    fn from(variant: TXFT) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFT` reader - TXFIFO threshold flag
pub type TXFT_R = crate::BitReader<TXFT>;
impl TXFT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFT {
        match self.bits {
            false => TXFT::NotReached,
            true => TXFT::Reached,
        }
    }
    ///TXFIFO does not reach the programmed threshold.
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == TXFT::NotReached
    }
    ///TXFIFO reached the programmed threshold.
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == TXFT::Reached
    }
}
impl R {
    ///Bit 0 - Parity error
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Start bit noise detection flag
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Idle line detected
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXFIFO not empty
    #[inline(always)]
    pub fn rxfne(&self) -> RXFNE_R {
        RXFNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXFIFO not full
    #[inline(always)]
    pub fn txfnf(&self) -> TXFNF_R {
        TXFNF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - CTS interrupt flag
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS flag
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - Busy flag
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Character match flag
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Send break flag
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Receiver wake-up from mute mode
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Wake-up from low-power mode flag
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Transmit enable acknowledge flag
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Receive enable acknowledge flag
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFIFO Empty
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - RXFIFO Full
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - RXFIFO threshold flag
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TXFIFO threshold flag
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("pe", &self.pe())
            .field("fe", &self.fe())
            .field("ne", &self.ne())
            .field("ore", &self.ore())
            .field("idle", &self.idle())
            .field("rxfne", &self.rxfne())
            .field("tc", &self.tc())
            .field("txfnf", &self.txfnf())
            .field("ctsif", &self.ctsif())
            .field("cts", &self.cts())
            .field("busy", &self.busy())
            .field("cmf", &self.cmf())
            .field("sbkf", &self.sbkf())
            .field("rwu", &self.rwu())
            .field("wuf", &self.wuf())
            .field("teack", &self.teack())
            .field("reack", &self.reack())
            .field("txfe", &self.txfe())
            .field("rxff", &self.rxff())
            .field("rxft", &self.rxft())
            .field("txft", &self.txft())
            .finish()
    }
}
/**LPUART interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#LPUART:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0x0080_00c0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x0080_00c0;
}
