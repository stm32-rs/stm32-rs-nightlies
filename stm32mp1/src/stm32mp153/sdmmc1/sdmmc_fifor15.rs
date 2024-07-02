///Register `SDMMC_FIFOR15` reader
pub type R = crate::R<SDMMC_FIFOR15rs>;
///Register `SDMMC_FIFOR15` writer
pub type W = crate::W<SDMMC_FIFOR15rs>;
///Field `FIFODATA` reader - FIFODATA
pub type FIFODATA_R = crate::FieldReader<u32>;
///Field `FIFODATA` writer - FIFODATA
pub type FIFODATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - FIFODATA
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_FIFOR15")
            .field("fifodata", &self.fifodata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - FIFODATA
    #[inline(always)]
    #[must_use]
    pub fn fifodata(&mut self) -> FIFODATA_W<SDMMC_FIFOR15rs> {
        FIFODATA_W::new(self, 0)
    }
}
/**The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SDMMC1:SDMMC_FIFOR15)*/
pub struct SDMMC_FIFOR15rs;
impl crate::RegisterSpec for SDMMC_FIFOR15rs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_fifor15::R`](R) reader structure
impl crate::Readable for SDMMC_FIFOR15rs {}
///`write(|w| ..)` method takes [`sdmmc_fifor15::W`](W) writer structure
impl crate::Writable for SDMMC_FIFOR15rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDMMC_FIFOR15 to value 0
impl crate::Resettable for SDMMC_FIFOR15rs {
    const RESET_VALUE: u32 = 0;
}
