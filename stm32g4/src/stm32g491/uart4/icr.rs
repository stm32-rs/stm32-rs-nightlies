///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Character match clear flag
pub use crate::stm32g491::usart1::icr::CMCF;
///Field `CMCF` writer - Character match clear flag
pub use crate::stm32g491::usart1::icr::CMCF_W;
///CTS clear flag
pub use crate::stm32g491::usart1::icr::CTSCF;
///Field `CTSCF` writer - CTS clear flag
pub use crate::stm32g491::usart1::icr::CTSCF_W;
///Framing error clear flag
pub use crate::stm32g491::usart1::icr::FECF;
///Field `FECF` writer - Framing error clear flag
pub use crate::stm32g491::usart1::icr::FECF_W;
///Idle line detected clear flag
pub use crate::stm32g491::usart1::icr::IDLECF;
///Field `IDLECF` writer - Idle line detected clear flag
pub use crate::stm32g491::usart1::icr::IDLECF_W;
///LIN break detection clear flag
pub use crate::stm32g491::usart1::icr::LBDCF;
///Field `LBDCF` writer - LIN break detection clear flag
pub use crate::stm32g491::usart1::icr::LBDCF_W;
///Noise detected clear flag
pub use crate::stm32g491::usart1::icr::NCF;
///Field `NCF` writer - Noise detected clear flag
pub use crate::stm32g491::usart1::icr::NCF_W;
///Overrun error clear flag
pub use crate::stm32g491::usart1::icr::ORECF;
///Field `ORECF` writer - Overrun error clear flag
pub use crate::stm32g491::usart1::icr::ORECF_W;
///Parity error clear flag
pub use crate::stm32g491::usart1::icr::PECF;
///Field `PECF` writer - Parity error clear flag
pub use crate::stm32g491::usart1::icr::PECF_W;
///Receiver timeout clear flag
pub use crate::stm32g491::usart1::icr::RTOCF;
///Field `RTOCF` writer - Receiver timeout clear flag
pub use crate::stm32g491::usart1::icr::RTOCF_W;
///TCBGTCF
pub use crate::stm32g491::usart1::icr::TCBGTCF;
///Field `TCBGTCF` writer - TCBGTCF
pub use crate::stm32g491::usart1::icr::TCBGTCF_W;
///Transmission complete clear flag
pub use crate::stm32g491::usart1::icr::TCCF;
///Field `TCCF` writer - Transmission complete clear flag
pub use crate::stm32g491::usart1::icr::TCCF_W;
///TXFECF
pub use crate::stm32g491::usart1::icr::TXFECF;
///Field `TXFECF` writer - TXFECF
pub use crate::stm32g491::usart1::icr::TXFECF_W;
///Wakeup from Stop mode clear flag
pub use crate::stm32g491::usart1::icr::WUCF;
///Field `WUCF` writer - Wakeup from Stop mode clear flag
pub use crate::stm32g491::usart1::icr::WUCF_W;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Parity error clear flag
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W<'_, ICRrs> {
        PECF_W::new(self, 0)
    }
    ///Bit 1 - Framing error clear flag
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W<'_, ICRrs> {
        FECF_W::new(self, 1)
    }
    ///Bit 2 - Noise detected clear flag
    #[inline(always)]
    pub fn ncf(&mut self) -> NCF_W<'_, ICRrs> {
        NCF_W::new(self, 2)
    }
    ///Bit 3 - Overrun error clear flag
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W<'_, ICRrs> {
        ORECF_W::new(self, 3)
    }
    ///Bit 4 - Idle line detected clear flag
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W<'_, ICRrs> {
        IDLECF_W::new(self, 4)
    }
    ///Bit 5 - TXFECF
    #[inline(always)]
    pub fn txfecf(&mut self) -> TXFECF_W<'_, ICRrs> {
        TXFECF_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete clear flag
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W<'_, ICRrs> {
        TCCF_W::new(self, 6)
    }
    ///Bit 7 - TCBGTCF
    #[inline(always)]
    pub fn tcbgtcf(&mut self) -> TCBGTCF_W<'_, ICRrs> {
        TCBGTCF_W::new(self, 7)
    }
    ///Bit 8 - LIN break detection clear flag
    #[inline(always)]
    pub fn lbdcf(&mut self) -> LBDCF_W<'_, ICRrs> {
        LBDCF_W::new(self, 8)
    }
    ///Bit 9 - CTS clear flag
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<'_, ICRrs> {
        CTSCF_W::new(self, 9)
    }
    ///Bit 11 - Receiver timeout clear flag
    #[inline(always)]
    pub fn rtocf(&mut self) -> RTOCF_W<'_, ICRrs> {
        RTOCF_W::new(self, 11)
    }
    ///Bit 17 - Character match clear flag
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W<'_, ICRrs> {
        CMCF_W::new(self, 17)
    }
    ///Bit 20 - Wakeup from Stop mode clear flag
    #[inline(always)]
    pub fn wucf(&mut self) -> WUCF_W<'_, ICRrs> {
        WUCF_W::new(self, 20)
    }
}
/**Interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G491.html#UART4:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0012_0bff;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
