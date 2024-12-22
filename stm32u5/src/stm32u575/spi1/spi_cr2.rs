///Register `SPI_CR2` reader
pub type R = crate::R<SPI_CR2rs>;
///Register `SPI_CR2` writer
pub type W = crate::W<SPI_CR2rs>;
/**Field `TSIZE` reader - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\[15:10\]
bits are reserved at limited feature set instances and must be kept at reset value.*/
pub type TSIZE_R = crate::FieldReader<u16>;
/**Field `TSIZE` writer - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\[15:10\]
bits are reserved at limited feature set instances and must be kept at reset value.*/
pub type TSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    /**Bits 0:15 - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\[15:10\]
    bits are reserved at limited feature set instances and must be kept at reset value.*/
    #[inline(always)]
    pub fn tsize(&self) -> TSIZE_R {
        TSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CR2")
            .field("tsize", &self.tsize())
            .finish()
    }
}
impl W {
    /**Bits 0:15 - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\[15:10\]
    bits are reserved at limited feature set instances and must be kept at reset value.*/
    #[inline(always)]
    pub fn tsize(&mut self) -> TSIZE_W<SPI_CR2rs> {
        TSIZE_W::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`spi_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#SPI1:SPI_CR2)*/
pub struct SPI_CR2rs;
impl crate::RegisterSpec for SPI_CR2rs {
    type Ux = u32;
}
///`read()` method returns [`spi_cr2::R`](R) reader structure
impl crate::Readable for SPI_CR2rs {}
///`write(|w| ..)` method takes [`spi_cr2::W`](W) writer structure
impl crate::Writable for SPI_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SPI_CR2 to value 0
impl crate::Resettable for SPI_CR2rs {
    const RESET_VALUE: u32 = 0;
}
