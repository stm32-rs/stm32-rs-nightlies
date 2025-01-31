///Register `FDCAN_TXEFS` reader
pub type R = crate::R<FDCAN_TXEFSrs>;
///Field `EFFL` reader - Event FIFO fill level
pub type EFFL_R = crate::FieldReader;
///Field `EFGI` reader - Event FIFO get index
pub type EFGI_R = crate::FieldReader;
///Field `EFPI` reader - Event FIFO put index
pub type EFPI_R = crate::FieldReader;
///Field `EFF` reader - Event FIFO full
pub type EFF_R = crate::BitReader;
///Field `TEFL` reader - Tx event FIFO element lost
pub type TEFL_R = crate::BitReader;
impl R {
    ///Bits 0:2 - Event FIFO fill level
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:9 - Event FIFO get index
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - Event FIFO put index
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - Event FIFO full
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Tx event FIFO element lost
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXEFS")
            .field("effl", &self.effl())
            .field("efgi", &self.efgi())
            .field("efpi", &self.efpi())
            .field("eff", &self.eff())
            .field("tefl", &self.tefl())
            .finish()
    }
}
/**FDCAN Tx event FIFO status register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txefs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FDCAN1:FDCAN_TXEFS)*/
pub struct FDCAN_TXEFSrs;
impl crate::RegisterSpec for FDCAN_TXEFSrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txefs::R`](R) reader structure
impl crate::Readable for FDCAN_TXEFSrs {}
///`reset()` method sets FDCAN_TXEFS to value 0
impl crate::Resettable for FDCAN_TXEFSrs {
    const RESET_VALUE: u32 = 0;
}
