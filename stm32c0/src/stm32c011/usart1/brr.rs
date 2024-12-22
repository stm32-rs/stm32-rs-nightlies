///Register `BRR` reader
pub type R = crate::R<BRRrs>;
///Register `BRR` writer
pub type W = crate::W<BRRrs>;
/**Field `BRR` reader - USART baud rate BRR\[15:4\]
BRR\[15:4\]
= USARTDIV\[15:4\]
BRR\[3:0\]
When OVER8 = 0, BRR\[3:0\]
= USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\]
= USARTDIV\[3:0\]
shifted 1 bit to the right. BRR\[3\]
must be kept cleared.*/
pub type BRR_R = crate::FieldReader<u16>;
/**Field `BRR` writer - USART baud rate BRR\[15:4\]
BRR\[15:4\]
= USARTDIV\[15:4\]
BRR\[3:0\]
When OVER8 = 0, BRR\[3:0\]
= USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\]
= USARTDIV\[3:0\]
shifted 1 bit to the right. BRR\[3\]
must be kept cleared.*/
pub type BRR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    /**Bits 0:15 - USART baud rate BRR\[15:4\]
    BRR\[15:4\]
    = USARTDIV\[15:4\]
    BRR\[3:0\]
    When OVER8 = 0, BRR\[3:0\]
    = USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\]
    = USARTDIV\[3:0\]
    shifted 1 bit to the right. BRR\[3\]
    must be kept cleared.*/
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRR").field("brr", &self.brr()).finish()
    }
}
impl W {
    /**Bits 0:15 - USART baud rate BRR\[15:4\]
    BRR\[15:4\]
    = USARTDIV\[15:4\]
    BRR\[3:0\]
    When OVER8 = 0, BRR\[3:0\]
    = USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\]
    = USARTDIV\[3:0\]
    shifted 1 bit to the right. BRR\[3\]
    must be kept cleared.*/
    #[inline(always)]
    pub fn brr(&mut self) -> BRR_W<BRRrs> {
        BRR_W::new(self, 0)
    }
}
/**USART baud rate register

You can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#USART1:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`read()` method returns [`brr::R`](R) reader structure
impl crate::Readable for BRRrs {}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {
    const RESET_VALUE: u32 = 0;
}
