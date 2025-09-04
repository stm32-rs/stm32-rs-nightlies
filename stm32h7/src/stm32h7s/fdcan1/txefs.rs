///Register `TXEFS` reader
pub type R = crate::R<TXEFSrs>;
///Field `EFFL` reader - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3.
pub type EFFL_R = crate::FieldReader;
///Field `EFGI` reader - Event FIFO get index Tx event FIFO read index pointer, range 0 to 3.
pub type EFGI_R = crate::FieldReader;
///Field `EFPI` reader - Event FIFO put index Tx event FIFO write index pointer, range 0 to 3.
pub type EFPI_R = crate::FieldReader;
///Field `EFF` reader - Event FIFO full
pub type EFF_R = crate::BitReader;
///Field `TEFL` reader - Tx event FIFO element lost This bit is a copy of interrupt flag IR\[TEFL\]. When IR\[TEFL\] is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0.
pub type TEFL_R = crate::BitReader;
impl R {
    ///Bits 0:2 - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3.
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:9 - Event FIFO get index Tx event FIFO read index pointer, range 0 to 3.
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - Event FIFO put index Tx event FIFO write index pointer, range 0 to 3.
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - Event FIFO full
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Tx event FIFO element lost This bit is a copy of interrupt flag IR\[TEFL\]. When IR\[TEFL\] is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0.
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXEFS")
            .field("effl", &self.effl())
            .field("efgi", &self.efgi())
            .field("efpi", &self.efpi())
            .field("eff", &self.eff())
            .field("tefl", &self.tefl())
            .finish()
    }
}
/**FDCAN Tx Event FIFO Status Register

You can [`read`](crate::Reg::read) this register and get [`txefs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FDCAN1:TXEFS)*/
pub struct TXEFSrs;
impl crate::RegisterSpec for TXEFSrs {
    type Ux = u32;
}
///`read()` method returns [`txefs::R`](R) reader structure
impl crate::Readable for TXEFSrs {}
///`reset()` method sets TXEFS to value 0
impl crate::Resettable for TXEFSrs {}
