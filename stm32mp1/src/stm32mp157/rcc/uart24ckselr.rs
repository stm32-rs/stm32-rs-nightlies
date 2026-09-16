///Register `UART24CKSELR` reader
pub type R = crate::R<UART24CKSELRrs>;
///Register `UART24CKSELR` writer
pub type W = crate::W<UART24CKSELRrs>;
///Field `UART24SRC` reader - UART24SRC
pub type UART24SRC_R = crate::FieldReader;
///Field `UART24SRC` writer - UART24SRC
pub type UART24SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - UART24SRC
    #[inline(always)]
    pub fn uart24src(&self) -> UART24SRC_R {
        UART24SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART24CKSELR")
            .field("uart24src", &self.uart24src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - UART24SRC
    #[inline(always)]
    pub fn uart24src(&mut self) -> UART24SRC_W<'_, UART24CKSELRrs> {
        UART24SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`uart24ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart24ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:UART24CKSELR)*/
pub struct UART24CKSELRrs;
impl crate::RegisterSpec for UART24CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`uart24ckselr::R`](R) reader structure
impl crate::Readable for UART24CKSELRrs {}
///`write(|w| ..)` method takes [`uart24ckselr::W`](W) writer structure
impl crate::Writable for UART24CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UART24CKSELR to value 0
impl crate::Resettable for UART24CKSELRrs {}
