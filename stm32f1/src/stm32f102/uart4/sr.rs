///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Framing error
pub use crate::stm32f102::usart1::sr::FE;
///Field `FE` reader - Framing error
pub use crate::stm32f102::usart1::sr::FE_R;
///IDLE line detected
pub use crate::stm32f102::usart1::sr::IDLE;
///Field `IDLE` reader - IDLE line detected
pub use crate::stm32f102::usart1::sr::IDLE_R;
///LIN break detection flag
pub use crate::stm32f102::usart1::sr::LBDR;
///LIN break detection flag
pub use crate::stm32f102::usart1::sr::LBDW;
///Field `LBD` reader - LIN break detection flag
pub use crate::stm32f102::usart1::sr::LBD_R;
///Field `LBD` writer - LIN break detection flag
pub use crate::stm32f102::usart1::sr::LBD_W;
///Noise error flag
pub use crate::stm32f102::usart1::sr::NE;
///Field `NE` reader - Noise error flag
pub use crate::stm32f102::usart1::sr::NE_R;
///Overrun error
pub use crate::stm32f102::usart1::sr::ORE;
///Field `ORE` reader - Overrun error
pub use crate::stm32f102::usart1::sr::ORE_R;
///Parity error
pub use crate::stm32f102::usart1::sr::PE;
///Field `PE` reader - Parity error
pub use crate::stm32f102::usart1::sr::PE_R;
///Read data register not empty
pub use crate::stm32f102::usart1::sr::RXNER;
///Read data register not empty
pub use crate::stm32f102::usart1::sr::RXNEW;
///Field `RXNE` reader - Read data register not empty
pub use crate::stm32f102::usart1::sr::RXNE_R;
///Field `RXNE` writer - Read data register not empty
pub use crate::stm32f102::usart1::sr::RXNE_W;
///Transmission complete
pub use crate::stm32f102::usart1::sr::TCR;
///Transmission complete
pub use crate::stm32f102::usart1::sr::TCW;
///Field `TC` reader - Transmission complete
pub use crate::stm32f102::usart1::sr::TC_R;
///Field `TC` writer - Transmission complete
pub use crate::stm32f102::usart1::sr::TC_W;
///Transmit data register empty
pub use crate::stm32f102::usart1::sr::TXE;
///Field `TXE` reader - Transmit data register empty
pub use crate::stm32f102::usart1::sr::TXE_R;
impl R {
    ///Bit 0 - Parity error
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Noise error flag
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE line detected
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Read data register not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit data register empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LIN break detection flag
    #[inline(always)]
    pub fn lbd(&self) -> LBD_R {
        LBD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("pe", &self.pe())
            .field("fe", &self.fe())
            .field("ne", &self.ne())
            .field("ore", &self.ore())
            .field("idle", &self.idle())
            .field("rxne", &self.rxne())
            .field("tc", &self.tc())
            .field("txe", &self.txe())
            .field("lbd", &self.lbd())
            .finish()
    }
}
impl W {
    ///Bit 5 - Read data register not empty
    #[inline(always)]
    pub fn rxne(&mut self) -> RXNE_W<'_, SRrs> {
        RXNE_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<'_, SRrs> {
        TC_W::new(self, 6)
    }
    ///Bit 8 - LIN break detection flag
    #[inline(always)]
    pub fn lbd(&mut self) -> LBD_W<'_, SRrs> {
        LBD_W::new(self, 8)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#UART4:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x0160;
}
///`reset()` method sets SR to value 0xc0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u16 = 0xc0;
}
