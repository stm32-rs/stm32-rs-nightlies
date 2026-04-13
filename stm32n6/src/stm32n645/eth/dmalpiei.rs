///Register `DMALPIEI` reader
pub type R = crate::R<DMALPIEIrs>;
///Register `DMALPIEI` writer
pub type W = crate::W<DMALPIEIrs>;
///Field `LPIEI` reader - LPI Entry Interval
pub type LPIEI_R = crate::FieldReader;
///Field `LPIEI` writer - LPI Entry Interval
pub type LPIEI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - LPI Entry Interval
    #[inline(always)]
    pub fn lpiei(&self) -> LPIEI_R {
        LPIEI_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMALPIEI")
            .field("lpiei", &self.lpiei())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - LPI Entry Interval
    #[inline(always)]
    pub fn lpiei(&mut self) -> LPIEI_W<'_, DMALPIEIrs> {
        LPIEI_W::new(self, 0)
    }
}
/**AXI4 LPI Entry Interval register

You can [`read`](crate::Reg::read) this register and get [`dmalpiei::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmalpiei::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:DMALPIEI)*/
pub struct DMALPIEIrs;
impl crate::RegisterSpec for DMALPIEIrs {
    type Ux = u32;
}
///`read()` method returns [`dmalpiei::R`](R) reader structure
impl crate::Readable for DMALPIEIrs {}
///`write(|w| ..)` method takes [`dmalpiei::W`](W) writer structure
impl crate::Writable for DMALPIEIrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMALPIEI to value 0
impl crate::Resettable for DMALPIEIrs {}
