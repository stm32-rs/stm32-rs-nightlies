#[doc = "Register `RCC_SPI2S23CKSELR` reader"]
pub type R = crate::R<RCC_SPI2S23CKSELRrs>;
#[doc = "Register `RCC_SPI2S23CKSELR` writer"]
pub type W = crate::W<RCC_SPI2S23CKSELRrs>;
#[doc = "Field `SPI23SRC` reader - SPI23SRC"]
pub type SPI23SRC_R = crate::FieldReader;
#[doc = "Field `SPI23SRC` writer - SPI23SRC"]
pub type SPI23SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SPI23SRC"]
    #[inline(always)]
    pub fn spi23src(&self) -> SPI23SRC_R {
        SPI23SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI23SRC"]
    #[inline(always)]
    #[must_use]
    pub fn spi23src(&mut self) -> SPI23SRC_W<RCC_SPI2S23CKSELRrs> {
        SPI23SRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_spi2s23ckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_spi2s23ckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_SPI2S23CKSELRrs;
impl crate::RegisterSpec for RCC_SPI2S23CKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_spi2s23ckselr::R`](R) reader structure"]
impl crate::Readable for RCC_SPI2S23CKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_spi2s23ckselr::W`](W) writer structure"]
impl crate::Writable for RCC_SPI2S23CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_SPI2S23CKSELR to value 0"]
impl crate::Resettable for RCC_SPI2S23CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
