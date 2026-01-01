///Register `CCIPR13` reader
pub type R = crate::R<CCIPR13rs>;
///Register `CCIPR13` writer
pub type W = crate::W<CCIPR13rs>;
///Field `USART1SEL` reader - Source selection for the USART1 kernel clock
pub type USART1SEL_R = crate::FieldReader;
///Field `USART1SEL` writer - Source selection for the USART1 kernel clock
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `USART2SEL` reader - Source selection for the USART2 kernel clock
pub type USART2SEL_R = crate::FieldReader;
///Field `USART2SEL` writer - Source selection for the USART2 kernel clock
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `USART3SEL` reader - Source selection for the USART3 kernel clock
pub type USART3SEL_R = crate::FieldReader;
///Field `USART3SEL` writer - Source selection for the USART3 kernel clock
pub type USART3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `UART4SEL` reader - Source selection for the UART4 kernel clock
pub type UART4SEL_R = crate::FieldReader;
///Field `UART4SEL` writer - Source selection for the UART4 kernel clock
pub type UART4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `UART5SEL` reader - Source selection for the UART5 kernel clock
pub type UART5SEL_R = crate::FieldReader;
///Field `UART5SEL` writer - Source selection for the UART5 kernel clock
pub type UART5SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `USART6SEL` reader - Source selection for the USART6 kernel clock
pub type USART6SEL_R = crate::FieldReader;
///Field `USART6SEL` writer - Source selection for the USART6 kernel clock
pub type USART6SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `UART7SEL` reader - Source selection for the UART7 kernel clock
pub type UART7SEL_R = crate::FieldReader;
///Field `UART7SEL` writer - Source selection for the UART7 kernel clock
pub type UART7SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `UART8SEL` reader - Source selection for the UART8 kernel clock
pub type UART8SEL_R = crate::FieldReader;
///Field `UART8SEL` writer - Source selection for the UART8 kernel clock
pub type UART8SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Source selection for the USART1 kernel clock
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Source selection for the USART2 kernel clock
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - Source selection for the USART3 kernel clock
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - Source selection for the UART4 kernel clock
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - Source selection for the UART5 kernel clock
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Source selection for the USART6 kernel clock
    #[inline(always)]
    pub fn usart6sel(&self) -> USART6SEL_R {
        USART6SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - Source selection for the UART7 kernel clock
    #[inline(always)]
    pub fn uart7sel(&self) -> UART7SEL_R {
        UART7SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - Source selection for the UART8 kernel clock
    #[inline(always)]
    pub fn uart8sel(&self) -> UART8SEL_R {
        UART8SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR13")
            .field("usart1sel", &self.usart1sel())
            .field("usart2sel", &self.usart2sel())
            .field("usart3sel", &self.usart3sel())
            .field("uart4sel", &self.uart4sel())
            .field("uart5sel", &self.uart5sel())
            .field("usart6sel", &self.usart6sel())
            .field("uart7sel", &self.uart7sel())
            .field("uart8sel", &self.uart8sel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Source selection for the USART1 kernel clock
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W<'_, CCIPR13rs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 4:6 - Source selection for the USART2 kernel clock
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W<'_, CCIPR13rs> {
        USART2SEL_W::new(self, 4)
    }
    ///Bits 8:10 - Source selection for the USART3 kernel clock
    #[inline(always)]
    pub fn usart3sel(&mut self) -> USART3SEL_W<'_, CCIPR13rs> {
        USART3SEL_W::new(self, 8)
    }
    ///Bits 12:14 - Source selection for the UART4 kernel clock
    #[inline(always)]
    pub fn uart4sel(&mut self) -> UART4SEL_W<'_, CCIPR13rs> {
        UART4SEL_W::new(self, 12)
    }
    ///Bits 16:18 - Source selection for the UART5 kernel clock
    #[inline(always)]
    pub fn uart5sel(&mut self) -> UART5SEL_W<'_, CCIPR13rs> {
        UART5SEL_W::new(self, 16)
    }
    ///Bits 20:22 - Source selection for the USART6 kernel clock
    #[inline(always)]
    pub fn usart6sel(&mut self) -> USART6SEL_W<'_, CCIPR13rs> {
        USART6SEL_W::new(self, 20)
    }
    ///Bits 24:26 - Source selection for the UART7 kernel clock
    #[inline(always)]
    pub fn uart7sel(&mut self) -> UART7SEL_W<'_, CCIPR13rs> {
        UART7SEL_W::new(self, 24)
    }
    ///Bits 28:30 - Source selection for the UART8 kernel clock
    #[inline(always)]
    pub fn uart8sel(&mut self) -> UART8SEL_W<'_, CCIPR13rs> {
        UART8SEL_W::new(self, 28)
    }
}
/**RCC clock configuration for independent peripheral register13

You can [`read`](crate::Reg::read) this register and get [`ccipr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:CCIPR13)*/
pub struct CCIPR13rs;
impl crate::RegisterSpec for CCIPR13rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr13::R`](R) reader structure
impl crate::Readable for CCIPR13rs {}
///`write(|w| ..)` method takes [`ccipr13::W`](W) writer structure
impl crate::Writable for CCIPR13rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR13 to value 0
impl crate::Resettable for CCIPR13rs {}
