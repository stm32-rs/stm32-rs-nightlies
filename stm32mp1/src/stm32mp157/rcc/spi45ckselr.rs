///Register `SPI45CKSELR` reader
pub type R = crate::R<SPI45CKSELRrs>;
///Register `SPI45CKSELR` writer
pub type W = crate::W<SPI45CKSELRrs>;
///Field `SPI45SRC` reader - SPI45SRC
pub type SPI45SRC_R = crate::FieldReader;
///Field `SPI45SRC` writer - SPI45SRC
pub type SPI45SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SPI45SRC
    #[inline(always)]
    pub fn spi45src(&self) -> SPI45SRC_R {
        SPI45SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI45CKSELR")
            .field("spi45src", &self.spi45src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SPI45SRC
    #[inline(always)]
    pub fn spi45src(&mut self) -> SPI45SRC_W<'_, SPI45CKSELRrs> {
        SPI45SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`spi45ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi45ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:SPI45CKSELR)*/
pub struct SPI45CKSELRrs;
impl crate::RegisterSpec for SPI45CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`spi45ckselr::R`](R) reader structure
impl crate::Readable for SPI45CKSELRrs {}
///`write(|w| ..)` method takes [`spi45ckselr::W`](W) writer structure
impl crate::Writable for SPI45CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI45CKSELR to value 0
impl crate::Resettable for SPI45CKSELRrs {}
