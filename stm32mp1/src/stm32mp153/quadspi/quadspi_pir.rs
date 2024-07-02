///Register `QUADSPI_PIR` reader
pub type R = crate::R<QUADSPI_PIRrs>;
///Register `QUADSPI_PIR` writer
pub type W = crate::W<QUADSPI_PIRrs>;
///Field `INTERVAL` reader - INTERVAL
pub type INTERVAL_R = crate::FieldReader<u16>;
///Field `INTERVAL` writer - INTERVAL
pub type INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - INTERVAL
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUADSPI_PIR")
            .field("interval", &self.interval())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - INTERVAL
    #[inline(always)]
    #[must_use]
    pub fn interval(&mut self) -> INTERVAL_W<QUADSPI_PIRrs> {
        INTERVAL_W::new(self, 0)
    }
}
/**QUADSPI polling interval register

You can [`read`](crate::Reg::read) this register and get [`quadspi_pir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quadspi_pir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#QUADSPI:QUADSPI_PIR)*/
pub struct QUADSPI_PIRrs;
impl crate::RegisterSpec for QUADSPI_PIRrs {
    type Ux = u32;
}
///`read()` method returns [`quadspi_pir::R`](R) reader structure
impl crate::Readable for QUADSPI_PIRrs {}
///`write(|w| ..)` method takes [`quadspi_pir::W`](W) writer structure
impl crate::Writable for QUADSPI_PIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QUADSPI_PIR to value 0
impl crate::Resettable for QUADSPI_PIRrs {
    const RESET_VALUE: u32 = 0;
}
