///Register `RCC_SDMMC12CKSELR` reader
pub type R = crate::R<RCC_SDMMC12CKSELRrs>;
///Register `RCC_SDMMC12CKSELR` writer
pub type W = crate::W<RCC_SDMMC12CKSELRrs>;
///Field `SDMMC12SRC` reader - SDMMC12SRC
pub type SDMMC12SRC_R = crate::FieldReader;
///Field `SDMMC12SRC` writer - SDMMC12SRC
pub type SDMMC12SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - SDMMC12SRC
    #[inline(always)]
    pub fn sdmmc12src(&self) -> SDMMC12SRC_R {
        SDMMC12SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SDMMC12CKSELR")
            .field("sdmmc12src", &self.sdmmc12src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SDMMC12SRC
    #[inline(always)]
    #[must_use]
    pub fn sdmmc12src(&mut self) -> SDMMC12SRC_W<RCC_SDMMC12CKSELRrs> {
        SDMMC12SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`rcc_sdmmc12ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_sdmmc12ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCC_SDMMC12CKSELR)*/
pub struct RCC_SDMMC12CKSELRrs;
impl crate::RegisterSpec for RCC_SDMMC12CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_sdmmc12ckselr::R`](R) reader structure
impl crate::Readable for RCC_SDMMC12CKSELRrs {}
///`write(|w| ..)` method takes [`rcc_sdmmc12ckselr::W`](W) writer structure
impl crate::Writable for RCC_SDMMC12CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_SDMMC12CKSELR to value 0x03
impl crate::Resettable for RCC_SDMMC12CKSELRrs {
    const RESET_VALUE: u32 = 0x03;
}
