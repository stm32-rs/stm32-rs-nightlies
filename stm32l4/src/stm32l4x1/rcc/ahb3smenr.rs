///Register `AHB3SMENR` reader
pub type R = crate::R<AHB3SMENRrs>;
///Register `AHB3SMENR` writer
pub type W = crate::W<AHB3SMENRrs>;
///Field `QSPISMEN` reader - QSPISMEN
pub type QSPISMEN_R = crate::BitReader;
///Field `QSPISMEN` writer - QSPISMEN
pub type QSPISMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - QSPISMEN
    #[inline(always)]
    pub fn qspismen(&self) -> QSPISMEN_R {
        QSPISMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3SMENR")
            .field("qspismen", &self.qspismen())
            .finish()
    }
}
impl W {
    ///Bit 8 - QSPISMEN
    #[inline(always)]
    pub fn qspismen(&mut self) -> QSPISMEN_W<'_, AHB3SMENRrs> {
        QSPISMEN_W::new(self, 8)
    }
}
/**AHB3 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#RCC:AHB3SMENR)*/
pub struct AHB3SMENRrs;
impl crate::RegisterSpec for AHB3SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3smenr::R`](R) reader structure
impl crate::Readable for AHB3SMENRrs {}
///`write(|w| ..)` method takes [`ahb3smenr::W`](W) writer structure
impl crate::Writable for AHB3SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3SMENR to value 0x0101
impl crate::Resettable for AHB3SMENRrs {
    const RESET_VALUE: u32 = 0x0101;
}
