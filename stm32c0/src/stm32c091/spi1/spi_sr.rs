///Register `SPI_SR` reader
pub type R = crate::R<SPI_SRrs>;
///Register `SPI_SR` writer
pub type W = crate::W<SPI_SRrs>;
/**Receive buffer not empty

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE {
    ///0: Rx buffer empty
    B0x0 = 0,
    ///1: Rx buffer not empty
    B0x1 = 1,
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
            false => RXNE::B0x0,
            true => RXNE::B0x1,
        }
    }
    ///Rx buffer empty
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXNE::B0x0
    }
    ///Rx buffer not empty
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXNE::B0x1
    }
}
/**Transmit buffer empty

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE {
    ///0: Tx buffer not empty
    B0x0 = 0,
    ///1: Tx buffer empty
    B0x1 = 1,
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
            false => TXE::B0x0,
            true => TXE::B0x1,
        }
    }
    ///Tx buffer not empty
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXE::B0x0
    }
    ///Tx buffer empty
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXE::B0x1
    }
}
/**Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSIDE {
    ///0: Channel Left has to be transmitted or has been received
    B0x0 = 0,
    ///1: Channel Right has to be transmitted or has been received
    B0x1 = 1,
}
impl From<CHSIDE> for bool {
    #[inline(always)]
    fn from(variant: CHSIDE) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSIDE` reader - Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode.
pub type CHSIDE_R = crate::BitReader<CHSIDE>;
impl CHSIDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHSIDE {
        match self.bits {
            false => CHSIDE::B0x0,
            true => CHSIDE::B0x1,
        }
    }
    ///Channel Left has to be transmitted or has been received
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSIDE::B0x0
    }
    ///Channel Right has to be transmitted or has been received
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSIDE::B0x1
    }
}
/**Underrun flag This flag is set by hardware and reset by a software sequence. Refer to I2S error flags on page 821 for the software sequence. Note: This bit is not used in SPI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDR {
    ///0: No underrun occurred
    B0x0 = 0,
    ///1: Underrun occurred
    B0x1 = 1,
}
impl From<UDR> for bool {
    #[inline(always)]
    fn from(variant: UDR) -> Self {
        variant as u8 != 0
    }
}
///Field `UDR` reader - Underrun flag This flag is set by hardware and reset by a software sequence. Refer to I2S error flags on page 821 for the software sequence. Note: This bit is not used in SPI mode.
pub type UDR_R = crate::BitReader<UDR>;
impl UDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDR {
        match self.bits {
            false => UDR::B0x0,
            true => UDR::B0x1,
        }
    }
    ///No underrun occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UDR::B0x0
    }
    ///Underrun occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UDR::B0x1
    }
}
/**CRC error flag Note: This flag is set by hardware and cleared by software writing 0. Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERR {
    ///0: CRC value received matches the SPIx_RXCRCR value
    B0x0 = 0,
    ///1: CRC value received does not match the SPIx_RXCRCR value
    B0x1 = 1,
}
impl From<CRCERR> for bool {
    #[inline(always)]
    fn from(variant: CRCERR) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCERR` reader - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. Note: This bit is not used in I<sup>2</sup>S mode.
pub type CRCERR_R = crate::BitReader<CRCERR>;
impl CRCERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCERR {
        match self.bits {
            false => CRCERR::B0x0,
            true => CRCERR::B0x1,
        }
    }
    ///CRC value received matches the SPIx_RXCRCR value
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCERR::B0x0
    }
    ///CRC value received does not match the SPIx_RXCRCR value
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCERR::B0x1
    }
}
///Field `CRCERR` writer - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. Note: This bit is not used in I<sup>2</sup>S mode.
pub type CRCERR_W<'a, REG> = crate::BitWriter<'a, REG, CRCERR>;
impl<'a, REG> CRCERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC value received matches the SPIx_RXCRCR value
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCERR::B0x0)
    }
    ///CRC value received does not match the SPIx_RXCRCR value
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCERR::B0x1)
    }
}
/**Mode fault This flag is set by hardware and reset by a software sequence. Refer to Section : Mode fault (MODF) on page 799 for the software sequence. Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODF {
    ///0: No mode fault occurred
    B0x0 = 0,
    ///1: Mode fault occurred
    B0x1 = 1,
}
impl From<MODF> for bool {
    #[inline(always)]
    fn from(variant: MODF) -> Self {
        variant as u8 != 0
    }
}
///Field `MODF` reader - Mode fault This flag is set by hardware and reset by a software sequence. Refer to Section : Mode fault (MODF) on page 799 for the software sequence. Note: This bit is not used in I<sup>2</sup>S mode.
pub type MODF_R = crate::BitReader<MODF>;
impl MODF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODF {
        match self.bits {
            false => MODF::B0x0,
            true => MODF::B0x1,
        }
    }
    ///No mode fault occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODF::B0x0
    }
    ///Mode fault occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODF::B0x1
    }
}
/**Overrun flag This flag is set by hardware and reset by a software sequence. Refer to I2S error flags on page 821 for the software sequence.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR {
    ///0: No overrun occurred
    B0x0 = 0,
    ///1: Overrun occurred
    B0x1 = 1,
}
impl From<OVR> for bool {
    #[inline(always)]
    fn from(variant: OVR) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - Overrun flag This flag is set by hardware and reset by a software sequence. Refer to I2S error flags on page 821 for the software sequence.
pub type OVR_R = crate::BitReader<OVR>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVR {
        match self.bits {
            false => OVR::B0x0,
            true => OVR::B0x1,
        }
    }
    ///No overrun occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVR::B0x0
    }
    ///Overrun occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVR::B0x1
    }
}
/**Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to Section 27.5.10: SPI status flags and Procedure for disabling the SPI on page 789.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSY {
    ///0: SPI (or I2S) not busy
    B0x0 = 0,
    ///1: SPI (or I2S) is busy in communication or Tx buffer is not empty
    B0x1 = 1,
}
impl From<BSY> for bool {
    #[inline(always)]
    fn from(variant: BSY) -> Self {
        variant as u8 != 0
    }
}
///Field `BSY` reader - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to Section 27.5.10: SPI status flags and Procedure for disabling the SPI on page 789.
pub type BSY_R = crate::BitReader<BSY>;
impl BSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSY {
        match self.bits {
            false => BSY::B0x0,
            true => BSY::B0x1,
        }
    }
    ///SPI (or I2S) not busy
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BSY::B0x0
    }
    ///SPI (or I2S) is busy in communication or Tx buffer is not empty
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BSY::B0x1
    }
}
/**Frame format error This flag is used for SPI in TI slave mode and I<sup>2</sup>S slave mode. Refer to Section 27.5.11: SPI error flags and Section 27.7.8: I2S error flags. This flag is set by hardware and reset when SPIx_SR is read by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRE {
    ///0: No frame format error
    B0x0 = 0,
    ///1: A frame format error occurred
    B0x1 = 1,
}
impl From<FRE> for bool {
    #[inline(always)]
    fn from(variant: FRE) -> Self {
        variant as u8 != 0
    }
}
///Field `FRE` reader - Frame format error This flag is used for SPI in TI slave mode and I<sup>2</sup>S slave mode. Refer to Section 27.5.11: SPI error flags and Section 27.7.8: I2S error flags. This flag is set by hardware and reset when SPIx_SR is read by software.
pub type FRE_R = crate::BitReader<FRE>;
impl FRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRE {
        match self.bits {
            false => FRE::B0x0,
            true => FRE::B0x1,
        }
    }
    ///No frame format error
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FRE::B0x0
    }
    ///A frame format error occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FRE::B0x1
    }
}
/**FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in I S mode and in SPI receive-only mode while CRC calculation is enabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRLVL {
    ///0: FIFO empty
    B0x0 = 0,
    ///1: 1/4 FIFO
    B0x1 = 1,
    ///2: 1/2 FIFO
    B0x2 = 2,
    ///3: FIFO full
    B0x3 = 3,
}
impl From<FRLVL> for u8 {
    #[inline(always)]
    fn from(variant: FRLVL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FRLVL {
    type Ux = u8;
}
impl crate::IsEnum for FRLVL {}
///Field `FRLVL` reader - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in I S mode and in SPI receive-only mode while CRC calculation is enabled.
pub type FRLVL_R = crate::FieldReader<FRLVL>;
impl FRLVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRLVL {
        match self.bits {
            0 => FRLVL::B0x0,
            1 => FRLVL::B0x1,
            2 => FRLVL::B0x2,
            3 => FRLVL::B0x3,
            _ => unreachable!(),
        }
    }
    ///FIFO empty
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FRLVL::B0x0
    }
    ///1/4 FIFO
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FRLVL::B0x1
    }
    ///1/2 FIFO
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == FRLVL::B0x2
    }
    ///FIFO full
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == FRLVL::B0x3
    }
}
/**FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTLVL {
    ///0: FIFO empty
    B0x0 = 0,
    ///1: 1/4 FIFO
    B0x1 = 1,
    ///2: 1/2 FIFO
    B0x2 = 2,
    ///3: FIFO full (considered as FULL when the FIFO threshold is greater than 1/2)
    B0x3 = 3,
}
impl From<FTLVL> for u8 {
    #[inline(always)]
    fn from(variant: FTLVL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FTLVL {
    type Ux = u8;
}
impl crate::IsEnum for FTLVL {}
///Field `FTLVL` reader - FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I<sup>2</sup>S mode.
pub type FTLVL_R = crate::FieldReader<FTLVL>;
impl FTLVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FTLVL {
        match self.bits {
            0 => FTLVL::B0x0,
            1 => FTLVL::B0x1,
            2 => FTLVL::B0x2,
            3 => FTLVL::B0x3,
            _ => unreachable!(),
        }
    }
    ///FIFO empty
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FTLVL::B0x0
    }
    ///1/4 FIFO
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FTLVL::B0x1
    }
    ///1/2 FIFO
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == FTLVL::B0x2
    }
    ///FIFO full (considered as FULL when the FIFO threshold is greater than 1/2)
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == FTLVL::B0x3
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
    ///Bit 2 - Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode.
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Underrun flag This flag is set by hardware and reset by a software sequence. Refer to I2S error flags on page 821 for the software sequence. Note: This bit is not used in SPI mode.
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mode fault This flag is set by hardware and reset by a software sequence. Refer to Section : Mode fault (MODF) on page 799 for the software sequence. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Overrun flag This flag is set by hardware and reset by a software sequence. Refer to I2S error flags on page 821 for the software sequence.
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to Section 27.5.10: SPI status flags and Procedure for disabling the SPI on page 789.
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Frame format error This flag is used for SPI in TI slave mode and I<sup>2</sup>S slave mode. Refer to Section 27.5.11: SPI error flags and Section 27.7.8: I2S error flags. This flag is set by hardware and reset when SPIx_SR is read by software.
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in I S mode and in SPI receive-only mode while CRC calculation is enabled.
    #[inline(always)]
    pub fn frlvl(&self) -> FRLVL_R {
        FRLVL_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bits 11:12 - FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn ftlvl(&self) -> FTLVL_R {
        FTLVL_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SR")
            .field("rxne", &self.rxne())
            .field("txe", &self.txe())
            .field("chside", &self.chside())
            .field("udr", &self.udr())
            .field("crcerr", &self.crcerr())
            .field("modf", &self.modf())
            .field("ovr", &self.ovr())
            .field("bsy", &self.bsy())
            .field("fre", &self.fre())
            .field("frlvl", &self.frlvl())
            .field("ftlvl", &self.ftlvl())
            .finish()
    }
}
impl W {
    ///Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W<'_, SPI_SRrs> {
        CRCERR_W::new(self, 4)
    }
}
/**SPI status register

You can [`read`](crate::Reg::read) this register and get [`spi_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SPI1:SPI_SR)*/
pub struct SPI_SRrs;
impl crate::RegisterSpec for SPI_SRrs {
    type Ux = u16;
}
///`read()` method returns [`spi_sr::R`](R) reader structure
impl crate::Readable for SPI_SRrs {}
///`write(|w| ..)` method takes [`spi_sr::W`](W) writer structure
impl crate::Writable for SPI_SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI_SR to value 0x02
impl crate::Resettable for SPI_SRrs {
    const RESET_VALUE: u16 = 0x02;
}
