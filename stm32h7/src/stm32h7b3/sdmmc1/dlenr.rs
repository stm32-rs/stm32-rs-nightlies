///Register `DLENR` reader
pub type R = crate::R<DLENRrs>;
///Register `DLENR` writer
pub type W = crate::W<DLENRrs>;
///Field `DATALENGTH` reader - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0.
pub type DATALENGTH_R = crate::FieldReader<u32>;
///Field `DATALENGTH` writer - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0.
pub type DATALENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:24 - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0.
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLENR")
            .field("datalength", &self.datalength())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0.
    #[inline(always)]
    pub fn datalength(&mut self) -> DATALENGTH_W<'_, DLENRrs> {
        DATALENGTH_W::new(self, 0)
    }
}
/**The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.

You can [`read`](crate::Reg::read) this register and get [`dlenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#SDMMC1:DLENR)*/
pub struct DLENRrs;
impl crate::RegisterSpec for DLENRrs {
    type Ux = u32;
}
///`read()` method returns [`dlenr::R`](R) reader structure
impl crate::Readable for DLENRrs {}
///`write(|w| ..)` method takes [`dlenr::W`](W) writer structure
impl crate::Writable for DLENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLENR to value 0
impl crate::Resettable for DLENRrs {}
