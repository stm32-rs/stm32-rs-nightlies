///Register `FDCAN_TXFQS` reader
pub type R = crate::R<FDCAN_TXFQSrs>;
///Field `TFFL` reader - Tx FIFO free level
pub type TFFL_R = crate::FieldReader;
///Field `TFGI` reader - Tx FIFO get index
pub type TFGI_R = crate::FieldReader;
///Field `TFQPI` reader - Tx FIFO/queue put index
pub type TFQPI_R = crate::FieldReader;
/**Tx FIFO/queue full

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFQF {
    ///0: Tx FIFO/queue not full
    B0x0 = 0,
    ///1: Tx FIFO/queue full
    B0x1 = 1,
}
impl From<TFQF> for bool {
    #[inline(always)]
    fn from(variant: TFQF) -> Self {
        variant as u8 != 0
    }
}
///Field `TFQF` reader - Tx FIFO/queue full
pub type TFQF_R = crate::BitReader<TFQF>;
impl TFQF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFQF {
        match self.bits {
            false => TFQF::B0x0,
            true => TFQF::B0x1,
        }
    }
    ///Tx FIFO/queue not full
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TFQF::B0x0
    }
    ///Tx FIFO/queue full
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TFQF::B0x1
    }
}
impl R {
    ///Bits 0:2 - Tx FIFO free level
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:9 - Tx FIFO get index
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - Tx FIFO/queue put index
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 21 - Tx FIFO/queue full
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXFQS")
            .field("tffl", &self.tffl())
            .field("tfgi", &self.tfgi())
            .field("tfqpi", &self.tfqpi())
            .field("tfqf", &self.tfqf())
            .finish()
    }
}
/**FDCAN Tx FIFO/queue status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txfqs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TXFQS)*/
pub struct FDCAN_TXFQSrs;
impl crate::RegisterSpec for FDCAN_TXFQSrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txfqs::R`](R) reader structure
impl crate::Readable for FDCAN_TXFQSrs {}
///`reset()` method sets FDCAN_TXFQS to value 0x03
impl crate::Resettable for FDCAN_TXFQSrs {
    const RESET_VALUE: u32 = 0x03;
}
