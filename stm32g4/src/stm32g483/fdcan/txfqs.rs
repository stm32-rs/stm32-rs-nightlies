///Register `TXFQS` reader
pub type R = crate::R<TXFQSrs>;
///Field `TFFL` reader - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\[TFQM\] = 1).
pub type TFFL_R = crate::FieldReader;
///Field `TFGI` reader - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)
pub type TFGI_R = crate::FieldReader;
///Field `TFQPI` reader - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3
pub type TFQPI_R = crate::FieldReader;
///Field `TFQF` reader - Tx FIFO/queue full
pub type TFQF_R = crate::BitReader;
impl R {
    ///Bits 0:2 - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\[TFQM\] = 1).
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:9 - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3
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
        f.debug_struct("TXFQS")
            .field("tffl", &self.tffl())
            .field("tfgi", &self.tfgi())
            .field("tfqpi", &self.tfqpi())
            .field("tfqf", &self.tfqf())
            .finish()
    }
}
/**FDCAN Tx FIFO/queue status register

You can [`read`](crate::Reg::read) this register and get [`txfqs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#FDCAN:TXFQS)*/
pub struct TXFQSrs;
impl crate::RegisterSpec for TXFQSrs {
    type Ux = u32;
}
///`read()` method returns [`txfqs::R`](R) reader structure
impl crate::Readable for TXFQSrs {}
///`reset()` method sets TXFQS to value 0x03
impl crate::Resettable for TXFQSrs {
    const RESET_VALUE: u32 = 0x03;
}
