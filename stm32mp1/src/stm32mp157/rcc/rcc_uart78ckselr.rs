#[doc = "Register `RCC_UART78CKSELR` reader"]
pub type R = crate::R<RCC_UART78CKSELRrs>;
#[doc = "Register `RCC_UART78CKSELR` writer"]
pub type W = crate::W<RCC_UART78CKSELRrs>;
#[doc = "Field `UART78SRC` reader - UART78SRC"]
pub type UART78SRC_R = crate::FieldReader;
#[doc = "Field `UART78SRC` writer - UART78SRC"]
pub type UART78SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - UART78SRC"]
    #[inline(always)]
    pub fn uart78src(&self) -> UART78SRC_R {
        UART78SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART78SRC"]
    #[inline(always)]
    #[must_use]
    pub fn uart78src(&mut self) -> UART78SRC_W<RCC_UART78CKSELRrs> {
        UART78SRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_uart78ckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_uart78ckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_UART78CKSELRrs;
impl crate::RegisterSpec for RCC_UART78CKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_uart78ckselr::R`](R) reader structure"]
impl crate::Readable for RCC_UART78CKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_uart78ckselr::W`](W) writer structure"]
impl crate::Writable for RCC_UART78CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_UART78CKSELR to value 0"]
impl crate::Resettable for RCC_UART78CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
