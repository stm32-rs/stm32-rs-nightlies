///Register `DR` reader
pub type R = crate::R<DRrs>;
///Register `DR` writer
pub type W = crate::W<DRrs>;
///Field `DATA` reader - Data Data to be sent/received to/from the external SPI device. In indirect write mode, data written to this register is stored on the FIFO before it is sent to the Flash memory during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written. In indirect read mode, reading this register gives (via the FIFO) the data which was received from the Flash memory. If the FIFO does not have as many bytes as requested by the read operation and if BUSY=1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first. In automatic polling mode, this register contains the last data read from the Flash memory (without masking). Word, halfword, and byte accesses to this register are supported. In indirect write mode, a byte write adds 1 byte to the FIFO, a halfword write 2, and a word write 4. Similarly, in indirect read mode, a byte read removes 1 byte from the FIFO, a halfword read 2, and a word read 4. Accesses in indirect mode must be aligned to the bottom of this register: a byte read must read DATA\[7:0\] and a halfword read must read DATA\[15:0\].
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - Data Data to be sent/received to/from the external SPI device. In indirect write mode, data written to this register is stored on the FIFO before it is sent to the Flash memory during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written. In indirect read mode, reading this register gives (via the FIFO) the data which was received from the Flash memory. If the FIFO does not have as many bytes as requested by the read operation and if BUSY=1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first. In automatic polling mode, this register contains the last data read from the Flash memory (without masking). Word, halfword, and byte accesses to this register are supported. In indirect write mode, a byte write adds 1 byte to the FIFO, a halfword write 2, and a word write 4. Similarly, in indirect read mode, a byte read removes 1 byte from the FIFO, a halfword read 2, and a word read 4. Accesses in indirect mode must be aligned to the bottom of this register: a byte read must read DATA\[7:0\] and a halfword read must read DATA\[15:0\].
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Data Data to be sent/received to/from the external SPI device. In indirect write mode, data written to this register is stored on the FIFO before it is sent to the Flash memory during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written. In indirect read mode, reading this register gives (via the FIFO) the data which was received from the Flash memory. If the FIFO does not have as many bytes as requested by the read operation and if BUSY=1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first. In automatic polling mode, this register contains the last data read from the Flash memory (without masking). Word, halfword, and byte accesses to this register are supported. In indirect write mode, a byte write adds 1 byte to the FIFO, a halfword write 2, and a word write 4. Similarly, in indirect read mode, a byte read removes 1 byte from the FIFO, a halfword read 2, and a word read 4. Accesses in indirect mode must be aligned to the bottom of this register: a byte read must read DATA\[7:0\] and a halfword read must read DATA\[15:0\].
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR").field("data", &self.data()).finish()
    }
}
impl W {
    ///Bits 0:31 - Data Data to be sent/received to/from the external SPI device. In indirect write mode, data written to this register is stored on the FIFO before it is sent to the Flash memory during the data phase. If the FIFO is too full, a write operation is stalled until the FIFO has enough space to accept the amount of data being written. In indirect read mode, reading this register gives (via the FIFO) the data which was received from the Flash memory. If the FIFO does not have as many bytes as requested by the read operation and if BUSY=1, the read operation is stalled until enough data is present or until the transfer is complete, whichever happens first. In automatic polling mode, this register contains the last data read from the Flash memory (without masking). Word, halfword, and byte accesses to this register are supported. In indirect write mode, a byte write adds 1 byte to the FIFO, a halfword write 2, and a word write 4. Similarly, in indirect read mode, a byte read removes 1 byte from the FIFO, a halfword read 2, and a word read 4. Accesses in indirect mode must be aligned to the bottom of this register: a byte read must read DATA\[7:0\] and a halfword read must read DATA\[15:0\].
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<'_, DRrs> {
        DATA_W::new(self, 0)
    }
}
/**Data register: full word (32 bit) access

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#QUADSPI:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`write(|w| ..)` method takes [`dr::W`](W) writer structure
impl crate::Writable for DRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {}
