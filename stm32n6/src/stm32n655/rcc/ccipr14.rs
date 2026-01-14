///Register `CCIPR14` reader
pub type R = crate::R<CCIPR14rs>;
///Register `CCIPR14` writer
pub type W = crate::W<CCIPR14rs>;
///Field `UART9SEL` reader - Source selection for the UART9 kernel clock
pub type UART9SEL_R = crate::FieldReader;
///Field `UART9SEL` writer - Source selection for the UART9 kernel clock
pub type UART9SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `USART10SEL` reader - Source selection for the USART10 kernel clock
pub type USART10SEL_R = crate::FieldReader;
///Field `USART10SEL` writer - Source selection for the USART10 kernel clock
pub type USART10SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LPUART1SEL` reader - Source selection for the LPUART1 kernel clock
pub type LPUART1SEL_R = crate::FieldReader;
///Field `LPUART1SEL` writer - Source selection for the LPUART1 kernel clock
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Source selection for the UART9 kernel clock
    #[inline(always)]
    pub fn uart9sel(&self) -> UART9SEL_R {
        UART9SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Source selection for the USART10 kernel clock
    #[inline(always)]
    pub fn usart10sel(&self) -> USART10SEL_R {
        USART10SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Source selection for the LPUART1 kernel clock
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR14")
            .field("uart9sel", &self.uart9sel())
            .field("usart10sel", &self.usart10sel())
            .field("lpuart1sel", &self.lpuart1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Source selection for the UART9 kernel clock
    #[inline(always)]
    pub fn uart9sel(&mut self) -> UART9SEL_W<'_, CCIPR14rs> {
        UART9SEL_W::new(self, 0)
    }
    ///Bits 4:6 - Source selection for the USART10 kernel clock
    #[inline(always)]
    pub fn usart10sel(&mut self) -> USART10SEL_W<'_, CCIPR14rs> {
        USART10SEL_W::new(self, 4)
    }
    ///Bits 8:10 - Source selection for the LPUART1 kernel clock
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<'_, CCIPR14rs> {
        LPUART1SEL_W::new(self, 8)
    }
}
/**RCC clock configuration for independent peripheral register14

You can [`read`](crate::Reg::read) this register and get [`ccipr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:CCIPR14)*/
pub struct CCIPR14rs;
impl crate::RegisterSpec for CCIPR14rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr14::R`](R) reader structure
impl crate::Readable for CCIPR14rs {}
///`write(|w| ..)` method takes [`ccipr14::W`](W) writer structure
impl crate::Writable for CCIPR14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR14 to value 0
impl crate::Resettable for CCIPR14rs {}
