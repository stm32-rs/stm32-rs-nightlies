///Register `RCC_SPI6CKSELR` reader
pub type R = crate::R<RCC_SPI6CKSELRrs>;
///Register `RCC_SPI6CKSELR` writer
pub type W = crate::W<RCC_SPI6CKSELRrs>;
///Field `SPI6SRC` reader - SPI6SRC
pub type SPI6SRC_R = crate::FieldReader;
///Field `SPI6SRC` writer - SPI6SRC
pub type SPI6SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SPI6SRC
    #[inline(always)]
    pub fn spi6src(&self) -> SPI6SRC_R {
        SPI6SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SPI6CKSELR")
            .field("spi6src", &self.spi6src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SPI6SRC
    #[inline(always)]
    #[must_use]
    pub fn spi6src(&mut self) -> SPI6SRC_W<RCC_SPI6CKSELRrs> {
        SPI6SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rcc_spi6ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_spi6ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCC_SPI6CKSELR)*/
pub struct RCC_SPI6CKSELRrs;
impl crate::RegisterSpec for RCC_SPI6CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_spi6ckselr::R`](R) reader structure
impl crate::Readable for RCC_SPI6CKSELRrs {}
///`write(|w| ..)` method takes [`rcc_spi6ckselr::W`](W) writer structure
impl crate::Writable for RCC_SPI6CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_SPI6CKSELR to value 0
impl crate::Resettable for RCC_SPI6CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
