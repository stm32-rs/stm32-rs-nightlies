///Register `UART35CKSELR` reader
pub type R = crate::R<UART35CKSELRrs>;
///Register `UART35CKSELR` writer
pub type W = crate::W<UART35CKSELRrs>;
///Field `UART35SRC` reader - UART35SRC
pub type UART35SRC_R = crate::FieldReader;
///Field `UART35SRC` writer - UART35SRC
pub type UART35SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - UART35SRC
    #[inline(always)]
    pub fn uart35src(&self) -> UART35SRC_R {
        UART35SRC_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART35CKSELR")
            .field("uart35src", &self.uart35src())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - UART35SRC
    #[inline(always)]
    pub fn uart35src(&mut self) -> UART35SRC_W<'_, UART35CKSELRrs> {
        UART35SRC_W::new(self, 0)
    }
}
/**This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`uart35ckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart35ckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:UART35CKSELR)*/
pub struct UART35CKSELRrs;
impl crate::RegisterSpec for UART35CKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`uart35ckselr::R`](R) reader structure
impl crate::Readable for UART35CKSELRrs {}
///`write(|w| ..)` method takes [`uart35ckselr::W`](W) writer structure
impl crate::Writable for UART35CKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets UART35CKSELR to value 0
impl crate::Resettable for UART35CKSELRrs {}
