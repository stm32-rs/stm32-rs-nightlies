///Register `SPI2S1CKSELR` reader
pub type R = crate::R<SPI2S1CKSELRrs>;
///Register `SPI2S1CKSELR` writer
pub type W = crate::W<SPI2S1CKSELRrs>;
///Field `SPI1SRC` reader - SPI1SRC
pub type SPI1SRC_R = crate::FieldReader;
///Field `SPI1SRC` writer - SPI1SRC
pub type SPI1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SPI1SRC
    #[inline(always)]
    pub fn spi1src(&self) -> SPI1SRC_R {
        SPI1SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2S1CKSELR")
            .field("spi1src", &self.spi1src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SPI1SRC
    #[inline(always)]
    pub fn spi1src(&mut self) -> SPI1SRC_W<'_, SPI2S1CKSELRrs> {
        SPI1SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`spi2s1ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s1ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:SPI2S1CKSELR)*/
pub struct SPI2S1CKSELRrs;
impl crate::RegisterSpec for SPI2S1CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`spi2s1ckselr::R`](R) reader structure
impl crate::Readable for SPI2S1CKSELRrs {}
///`write(|w| ..)` method takes [`spi2s1ckselr::W`](W) writer structure
impl crate::Writable for SPI2S1CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI2S1CKSELR to value 0
impl crate::Resettable for SPI2S1CKSELRrs {}
