///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
/**Receive buffer not empty

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE {
    ///0: Rx buffer empty
    Empty = 0,
    ///1: Rx buffer not empty
    NotEmpty = 1,
}
impl From<RXNE> for bool {
    #[inline(always)]
    fn from(variant: RXNE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNE` reader - Receive buffer not empty
pub type RXNE_R = crate::BitReader<RXNE>;
impl RXNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXNE {
        match self.bits {
            false => RXNE::Empty,
            true => RXNE::NotEmpty,
        }
    }
    ///Rx buffer empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE::Empty
    }
    ///Rx buffer not empty
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE::NotEmpty
    }
}
/**Transmit buffer empty

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE {
    ///0: Tx buffer not empty
    NotEmpty = 0,
    ///1: Tx buffer empty
    Empty = 1,
}
impl From<TXE> for bool {
    #[inline(always)]
    fn from(variant: TXE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXE` reader - Transmit buffer empty
pub type TXE_R = crate::BitReader<TXE>;
impl TXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXE {
        match self.bits {
            false => TXE::NotEmpty,
            true => TXE::Empty,
        }
    }
    ///Tx buffer not empty
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE::NotEmpty
    }
    ///Tx buffer empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXE::Empty
    }
}
/**Channel side

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSIDE {
    ///0: Channel left has to be transmitted or has been received
    Left = 0,
    ///1: Channel right has to be transmitted or has been received
    Right = 1,
}
impl From<CHSIDE> for bool {
    #[inline(always)]
    fn from(variant: CHSIDE) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSIDE` reader - Channel side
pub type CHSIDE_R = crate::BitReader<CHSIDE>;
impl CHSIDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSIDE {
        match self.bits {
            false => CHSIDE::Left,
            true => CHSIDE::Right,
        }
    }
    ///Channel left has to be transmitted or has been received
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == CHSIDE::Left
    }
    ///Channel right has to be transmitted or has been received
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == CHSIDE::Right
    }
}
/**Underrun flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRR {
    ///0: No underrun occurred
    NoUnderrun = 0,
    ///1: Underrun occurred
    Underrun = 1,
}
impl From<UDRR> for bool {
    #[inline(always)]
    fn from(variant: UDRR) -> Self {
        variant as u8 != 0
    }
}
///Field `UDR` reader - Underrun flag
pub type UDR_R = crate::BitReader<UDRR>;
impl UDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDRR {
        match self.bits {
            false => UDRR::NoUnderrun,
            true => UDRR::Underrun,
        }
    }
    ///No underrun occurred
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == UDRR::NoUnderrun
    }
    ///Underrun occurred
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == UDRR::Underrun
    }
}
/**CRC error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERRR {
    ///0: CRC value received matches the SPIx_RXCRCR value
    Match = 0,
    ///1: CRC value received does not match the SPIx_RXCRCR value
    NoMatch = 1,
}
impl From<CRCERRR> for bool {
    #[inline(always)]
    fn from(variant: CRCERRR) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCERR` reader - CRC error flag
pub type CRCERR_R = crate::BitReader<CRCERRR>;
impl CRCERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCERRR {
        match self.bits {
            false => CRCERRR::Match,
            true => CRCERRR::NoMatch,
        }
    }
    ///CRC value received matches the SPIx_RXCRCR value
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CRCERRR::Match
    }
    ///CRC value received does not match the SPIx_RXCRCR value
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CRCERRR::NoMatch
    }
}
/**CRC error flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERRW {
    ///0: Clear flag
    Clear = 0,
}
impl From<CRCERRW> for bool {
    #[inline(always)]
    fn from(variant: CRCERRW) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCERR` writer - CRC error flag
pub type CRCERR_W<'a, REG> = crate::BitWriter0C<'a, REG, CRCERRW>;
impl<'a, REG> CRCERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CRCERRW::Clear)
    }
}
/**Mode fault

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFR {
    ///0: No mode fault occurred
    NoFault = 0,
    ///1: Mode fault occurred
    Fault = 1,
}
impl From<MODFR> for bool {
    #[inline(always)]
    fn from(variant: MODFR) -> Self {
        variant as u8 != 0
    }
}
///Field `MODF` reader - Mode fault
pub type MODF_R = crate::BitReader<MODFR>;
impl MODF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODFR {
        match self.bits {
            false => MODFR::NoFault,
            true => MODFR::Fault,
        }
    }
    ///No mode fault occurred
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == MODFR::NoFault
    }
    ///Mode fault occurred
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == MODFR::Fault
    }
}
/**Overrun flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR {
    ///0: No overrun occurred
    NoOverrun = 0,
    ///1: Overrun occurred
    Overrun = 1,
}
impl From<OVRR> for bool {
    #[inline(always)]
    fn from(variant: OVRR) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - Overrun flag
pub type OVR_R = crate::BitReader<OVRR>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRR {
        match self.bits {
            false => OVRR::NoOverrun,
            true => OVRR::Overrun,
        }
    }
    ///No overrun occurred
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR::NoOverrun
    }
    ///Overrun occurred
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR::Overrun
    }
}
/**Busy flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYR {
    ///0: SPI not busy
    NotBusy = 0,
    ///1: SPI busy
    Busy = 1,
}
impl From<BSYR> for bool {
    #[inline(always)]
    fn from(variant: BSYR) -> Self {
        variant as u8 != 0
    }
}
///Field `BSY` reader - Busy flag
pub type BSY_R = crate::BitReader<BSYR>;
impl BSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSYR {
        match self.bits {
            false => BSYR::NotBusy,
            true => BSYR::Busy,
        }
    }
    ///SPI not busy
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSYR::NotBusy
    }
    ///SPI busy
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSYR::Busy
    }
}
/**TI frame format error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRER {
    ///0: No frame format error
    NoError = 0,
    ///1: A frame format error occurred
    Error = 1,
}
impl From<FRER> for bool {
    #[inline(always)]
    fn from(variant: FRER) -> Self {
        variant as u8 != 0
    }
}
///Field `FRE` reader - TI frame format error
pub type FRE_R = crate::BitReader<FRER>;
impl FRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRER {
        match self.bits {
            false => FRER::NoError,
            true => FRER::Error,
        }
    }
    ///No frame format error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FRER::NoError
    }
    ///A frame format error occurred
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FRER::Error
    }
}
impl R {
    ///Bit 0 - Receive buffer not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit buffer empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel side
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Underrun flag
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC error flag
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mode fault
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overrun flag
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Busy flag
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TI frame format error
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("fre", &self.fre())
            .field("bsy", &self.bsy())
            .field("ovr", &self.ovr())
            .field("modf", &self.modf())
            .field("crcerr", &self.crcerr())
            .field("udr", &self.udr())
            .field("chside", &self.chside())
            .field("txe", &self.txe())
            .field("rxne", &self.rxne())
            .finish()
    }
}
impl W {
    ///Bit 4 - CRC error flag
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W<'_, SRrs> {
        CRCERR_W::new(self, 4)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#SPI1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x10;
}
///`reset()` method sets SR to value 0x02
impl crate::Resettable for SRrs {
    const RESET_VALUE: u16 = 0x02;
}
