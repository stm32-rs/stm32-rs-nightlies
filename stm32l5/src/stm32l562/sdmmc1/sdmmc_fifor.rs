///Register `SDMMC_FIFOR` reader
pub type R = crate::R<SDMMC_FIFORrs>;
///Register `SDMMC_FIFOR` writer
pub type W = crate::W<SDMMC_FIFORrs>;
///Field `FIFODATA` reader - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words.
pub type FIFODATA_R = crate::FieldReader<u32>;
///Field `FIFODATA` writer - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words.
pub type FIFODATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words.
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_FIFOR")
            .field("fifodata", &self.fifodata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Receive and transmit FIFO data This register can only be read or written by firmware when the DPSM is active (DPSMACT=1). The FIFO data occupies 16 entries of 32-bit words.
    #[inline(always)]
    pub fn fifodata(&mut self) -> FIFODATA_W<SDMMC_FIFORrs> {
        FIFODATA_W::new(self, 0)
    }
}
/**The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_fifor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_fifor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SDMMC1:SDMMC_FIFOR)*/
pub struct SDMMC_FIFORrs;
impl crate::RegisterSpec for SDMMC_FIFORrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_fifor::R`](R) reader structure
impl crate::Readable for SDMMC_FIFORrs {}
///`write(|w| ..)` method takes [`sdmmc_fifor::W`](W) writer structure
impl crate::Writable for SDMMC_FIFORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDMMC_FIFOR to value 0
impl crate::Resettable for SDMMC_FIFORrs {
    const RESET_VALUE: u32 = 0;
}
