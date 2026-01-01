///Register `UART6CKSELR` reader
pub type R = crate::R<UART6CKSELRrs>;
///Register `UART6CKSELR` writer
pub type W = crate::W<UART6CKSELRrs>;
///Field `UART6SRC` reader - UART6SRC
pub type UART6SRC_R = crate::FieldReader;
///Field `UART6SRC` writer - UART6SRC
pub type UART6SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - UART6SRC
    #[inline(always)]
    pub fn uart6src(&self) -> UART6SRC_R {
        UART6SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART6CKSELR")
            .field("uart6src", &self.uart6src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - UART6SRC
    #[inline(always)]
    pub fn uart6src(&mut self) -> UART6SRC_W<'_, UART6CKSELRrs> {
        UART6SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`uart6ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart6ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:UART6CKSELR)*/
pub struct UART6CKSELRrs;
impl crate::RegisterSpec for UART6CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`uart6ckselr::R`](R) reader structure
impl crate::Readable for UART6CKSELRrs {}
///`write(|w| ..)` method takes [`uart6ckselr::W`](W) writer structure
impl crate::Writable for UART6CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UART6CKSELR to value 0
impl crate::Resettable for UART6CKSELRrs {}
