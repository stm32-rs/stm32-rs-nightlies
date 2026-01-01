///Register `USART_BRR` reader
pub type R = crate::R<USART_BRRrs>;
///Register `USART_BRR` writer
pub type W = crate::W<USART_BRRrs>;
///Field `BRR` reader - USART baud rate BRR\[15:4\] BRR\[15:4\] = USARTDIV\[15:4\] BRR\[3:0\] When OVER8 = 0, BRR\[3:0\] = USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\] = USARTDIV\[3:0\] shifted 1 bit to the right. BRR\[3\] must be kept cleared.
pub type BRR_R = crate::FieldReader<u16>;
///Field `BRR` writer - USART baud rate BRR\[15:4\] BRR\[15:4\] = USARTDIV\[15:4\] BRR\[3:0\] When OVER8 = 0, BRR\[3:0\] = USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\] = USARTDIV\[3:0\] shifted 1 bit to the right. BRR\[3\] must be kept cleared.
pub type BRR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - USART baud rate BRR\[15:4\] BRR\[15:4\] = USARTDIV\[15:4\] BRR\[3:0\] When OVER8 = 0, BRR\[3:0\] = USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\] = USARTDIV\[3:0\] shifted 1 bit to the right. BRR\[3\] must be kept cleared.
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_BRR")
            .field("brr", &self.brr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - USART baud rate BRR\[15:4\] BRR\[15:4\] = USARTDIV\[15:4\] BRR\[3:0\] When OVER8 = 0, BRR\[3:0\] = USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\] = USARTDIV\[3:0\] shifted 1 bit to the right. BRR\[3\] must be kept cleared.
    #[inline(always)]
    pub fn brr(&mut self) -> BRR_W<'_, USART_BRRrs> {
        BRR_W::new(self, 0)
    }
}
/**USART baud rate register

You can [`read`](crate::Reg::read) this register and get [`usart_brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_BRR)*/
pub struct USART_BRRrs;
impl crate::RegisterSpec for USART_BRRrs {
    type Ux = u32;
}
///`read()` method returns [`usart_brr::R`](R) reader structure
impl crate::Readable for USART_BRRrs {}
///`write(|w| ..)` method takes [`usart_brr::W`](W) writer structure
impl crate::Writable for USART_BRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USART_BRR to value 0
impl crate::Resettable for USART_BRRrs {}
