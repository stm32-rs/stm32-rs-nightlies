///Register `RQR` writer
pub type W = crate::W<RQRrs>;
///Mute mode request
pub use crate::stm32g471::usart1::rqr::MMRQ;
///Field `MMRQ` writer - Mute mode request
pub use crate::stm32g471::usart1::rqr::MMRQ_W;
///Receive data flush request
pub use crate::stm32g471::usart1::rqr::RXFRQ;
///Field `RXFRQ` writer - Receive data flush request
pub use crate::stm32g471::usart1::rqr::RXFRQ_W;
///Send break request
pub use crate::stm32g471::usart1::rqr::SBKRQ;
///Field `SBKRQ` writer - Send break request
pub use crate::stm32g471::usart1::rqr::SBKRQ_W;
///TXFRQ
pub use crate::stm32g471::usart1::rqr::TXFRQ;
///Field `TXFRQ` writer - TXFRQ
pub use crate::stm32g471::usart1::rqr::TXFRQ_W;
impl core::fmt::Debug for crate::generic::Reg<RQRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 1 - Send break request
    #[inline(always)]
    pub fn sbkrq(&mut self) -> SBKRQ_W<'_, RQRrs> {
        SBKRQ_W::new(self, 1)
    }
    ///Bit 2 - Mute mode request
    #[inline(always)]
    pub fn mmrq(&mut self) -> MMRQ_W<'_, RQRrs> {
        MMRQ_W::new(self, 2)
    }
    ///Bit 3 - Receive data flush request
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RXFRQ_W<'_, RQRrs> {
        RXFRQ_W::new(self, 3)
    }
    ///Bit 4 - TXFRQ
    #[inline(always)]
    pub fn txfrq(&mut self) -> TXFRQ_W<'_, RQRrs> {
        TXFRQ_W::new(self, 4)
    }
}
/**Request register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471.html#LPUART1:RQR)*/
pub struct RQRrs;
impl crate::RegisterSpec for RQRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rqr::W`](W) writer structure
impl crate::Writable for RQRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RQR to value 0
impl crate::Resettable for RQRrs {}
