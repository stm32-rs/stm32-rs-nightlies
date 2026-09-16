///Register `UART1CKSELR` reader
pub type R = crate::R<UART1CKSELRrs>;
///Register `UART1CKSELR` writer
pub type W = crate::W<UART1CKSELRrs>;
///Field `UART1SRC` reader - UART1SRC
pub type UART1SRC_R = crate::FieldReader;
///Field `UART1SRC` writer - UART1SRC
pub type UART1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - UART1SRC
    #[inline(always)]
    pub fn uart1src(&self) -> UART1SRC_R {
        UART1SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1CKSELR")
            .field("uart1src", &self.uart1src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - UART1SRC
    #[inline(always)]
    pub fn uart1src(&mut self) -> UART1SRC_W<'_, UART1CKSELRrs> {
        UART1SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`uart1ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart1ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:UART1CKSELR)*/
pub struct UART1CKSELRrs;
impl crate::RegisterSpec for UART1CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`uart1ckselr::R`](R) reader structure
impl crate::Readable for UART1CKSELRrs {}
///`write(|w| ..)` method takes [`uart1ckselr::W`](W) writer structure
impl crate::Writable for UART1CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UART1CKSELR to value 0
impl crate::Resettable for UART1CKSELRrs {}
