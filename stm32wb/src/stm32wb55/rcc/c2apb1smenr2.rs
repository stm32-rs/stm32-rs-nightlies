#[doc = "Register `C2APB1SMENR2` reader"]
pub type R = crate::R<C2APB1SMENR2rs>;
#[doc = "Register `C2APB1SMENR2` writer"]
pub type W = crate::W<C2APB1SMENR2rs>;
#[doc = "Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during CPU2 Sleep mode"]
pub type LPUART1SMEN_R = crate::BitReader;
#[doc = "Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during CPU2 Sleep mode"]
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2SMEN` reader - Low power timer 2 clocks enable during CPU2 Sleep mode"]
pub type LPTIM2SMEN_R = crate::BitReader;
#[doc = "Field `LPTIM2SMEN` writer - Low power timer 2 clocks enable during CPU2 Sleep mode"]
pub type LPTIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Low power timer 2 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<C2APB1SMENR2rs> {
        LPUART1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 5 - Low power timer 2 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<C2APB1SMENR2rs> {
        LPTIM2SMEN_W::new(self, 5)
    }
}
#[doc = "CPU2 APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2apb1smenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2apb1smenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2APB1SMENR2rs;
impl crate::RegisterSpec for C2APB1SMENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2apb1smenr2::R`](R) reader structure"]
impl crate::Readable for C2APB1SMENR2rs {}
#[doc = "`write(|w| ..)` method takes [`c2apb1smenr2::W`](W) writer structure"]
impl crate::Writable for C2APB1SMENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2APB1SMENR2 to value 0x21"]
impl crate::Resettable for C2APB1SMENR2rs {
    const RESET_VALUE: u32 = 0x21;
}
