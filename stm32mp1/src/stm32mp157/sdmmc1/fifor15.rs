///Register `FIFOR15` reader
pub type R = crate::R<FIFOR15rs>;
///Register `FIFOR15` writer
pub type W = crate::W<FIFOR15rs>;
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
        f.debug_struct("FIFOR15")
            .field("fifodata", &self.fifodata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - FIFODATA
    #[inline(always)]
    pub fn fifodata(&mut self) -> FIFODATA_W<'_, FIFOR15rs> {
        FIFODATA_W::new(self, 0)
    }
}
/**The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`fifor15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifor15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SDMMC1:FIFOR15)*/
pub struct FIFOR15rs;
impl crate::RegisterSpec for FIFOR15rs {
    type Ux = u32;
}
///`read()` method returns [`fifor15::R`](R) reader structure
impl crate::Readable for FIFOR15rs {}
///`write(|w| ..)` method takes [`fifor15::W`](W) writer structure
impl crate::Writable for FIFOR15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FIFOR15 to value 0
impl crate::Resettable for FIFOR15rs {}
