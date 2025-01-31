///Register `HSPI_PIR` reader
pub type R = crate::R<HSPI_PIRrs>;
///Register `HSPI_PIR` writer
pub type W = crate::W<HSPI_PIRrs>;
///Field `INTERVAL` reader - 15: 0\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases
pub type INTERVAL_R = crate::FieldReader<u16>;
///Field `INTERVAL` writer - 15: 0\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases
pub type INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - 15: 0\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSPI_PIR")
            .field("interval", &self.interval())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - 15: 0\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases
    #[inline(always)]
    pub fn interval(&mut self) -> INTERVAL_W<HSPI_PIRrs> {
        INTERVAL_W::new(self, 0)
    }
}
/**HSPI polling interval register

You can [`read`](crate::Reg::read) this register and get [`hspi_pir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspi_pir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#HSPI1:HSPI_PIR)*/
pub struct HSPI_PIRrs;
impl crate::RegisterSpec for HSPI_PIRrs {
    type Ux = u32;
}
///`read()` method returns [`hspi_pir::R`](R) reader structure
impl crate::Readable for HSPI_PIRrs {}
///`write(|w| ..)` method takes [`hspi_pir::W`](W) writer structure
impl crate::Writable for HSPI_PIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSPI_PIR to value 0
impl crate::Resettable for HSPI_PIRrs {
    const RESET_VALUE: u32 = 0;
}
