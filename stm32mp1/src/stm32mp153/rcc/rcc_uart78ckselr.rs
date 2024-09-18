///Register `RCC_UART78CKSELR` reader
pub type R = crate::R<RCC_UART78CKSELRrs>;
///Register `RCC_UART78CKSELR` writer
pub type W = crate::W<RCC_UART78CKSELRrs>;
///Field `UART78SRC` reader - UART78SRC
pub type UART78SRC_R = crate::FieldReader;
///Field `UART78SRC` writer - UART78SRC
pub type UART78SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - UART78SRC
    #[inline(always)]
    pub fn uart78src(&self) -> UART78SRC_R {
        UART78SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_UART78CKSELR")
            .field("uart78src", &self.uart78src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - UART78SRC
    #[inline(always)]
    #[must_use]
    pub fn uart78src(&mut self) -> UART78SRC_W<RCC_UART78CKSELRrs> {
        UART78SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`rcc_uart78ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_uart78ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCC_UART78CKSELR)*/
pub struct RCC_UART78CKSELRrs;
impl crate::RegisterSpec for RCC_UART78CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_uart78ckselr::R`](R) reader structure
impl crate::Readable for RCC_UART78CKSELRrs {}
///`write(|w| ..)` method takes [`rcc_uart78ckselr::W`](W) writer structure
impl crate::Writable for RCC_UART78CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_UART78CKSELR to value 0
impl crate::Resettable for RCC_UART78CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
