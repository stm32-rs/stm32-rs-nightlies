///Register `RCC_SPDIFCKSELR` reader
pub type R = crate::R<RCC_SPDIFCKSELRrs>;
///Register `RCC_SPDIFCKSELR` writer
pub type W = crate::W<RCC_SPDIFCKSELRrs>;
///Field `SPDIFSRC` reader - SPDIFSRC
pub type SPDIFSRC_R = crate::FieldReader;
///Field `SPDIFSRC` writer - SPDIFSRC
pub type SPDIFSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - SPDIFSRC
    #[inline(always)]
    pub fn spdifsrc(&self) -> SPDIFSRC_R {
        SPDIFSRC_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SPDIFCKSELR")
            .field("spdifsrc", &self.spdifsrc())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SPDIFSRC
    #[inline(always)]
    #[must_use]
    pub fn spdifsrc(&mut self) -> SPDIFSRC_W<RCC_SPDIFCKSELRrs> {
        SPDIFSRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`rcc_spdifckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_spdifckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RCC_SPDIFCKSELR)*/
pub struct RCC_SPDIFCKSELRrs;
impl crate::RegisterSpec for RCC_SPDIFCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_spdifckselr::R`](R) reader structure
impl crate::Readable for RCC_SPDIFCKSELRrs {}
///`write(|w| ..)` method takes [`rcc_spdifckselr::W`](W) writer structure
impl crate::Writable for RCC_SPDIFCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_SPDIFCKSELR to value 0
impl crate::Resettable for RCC_SPDIFCKSELRrs {
    const RESET_VALUE: u32 = 0;
}
