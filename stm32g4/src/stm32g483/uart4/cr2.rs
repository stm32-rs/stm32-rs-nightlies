///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Auto baud rate enable
pub use crate::stm32g483::usart1::cr2::ABREN;
///Field `ABREN` reader - Auto baud rate enable
pub use crate::stm32g483::usart1::cr2::ABREN_R;
///Field `ABREN` writer - Auto baud rate enable
pub use crate::stm32g483::usart1::cr2::ABREN_W;
///Auto baud rate mode
pub use crate::stm32g483::usart1::cr2::ABRMOD;
///Field `ABRMOD` reader - Auto baud rate mode
pub use crate::stm32g483::usart1::cr2::ABRMOD_R;
///Field `ABRMOD` writer - Auto baud rate mode
pub use crate::stm32g483::usart1::cr2::ABRMOD_W;
///7-bit Address Detection/4-bit Address Detection
pub use crate::stm32g483::usart1::cr2::ADDM7;
///Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection
pub use crate::stm32g483::usart1::cr2::ADDM7_R;
///Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection
pub use crate::stm32g483::usart1::cr2::ADDM7_W;
///Binary data inversion
pub use crate::stm32g483::usart1::cr2::DATAINV;
///Field `DATAINV` reader - Binary data inversion
pub use crate::stm32g483::usart1::cr2::DATAINV_R;
///Field `DATAINV` writer - Binary data inversion
pub use crate::stm32g483::usart1::cr2::DATAINV_W;
///LIN break detection interrupt enable
pub use crate::stm32g483::usart1::cr2::LBDIE;
///Field `LBDIE` reader - LIN break detection interrupt enable
pub use crate::stm32g483::usart1::cr2::LBDIE_R;
///Field `LBDIE` writer - LIN break detection interrupt enable
pub use crate::stm32g483::usart1::cr2::LBDIE_W;
///LIN break detection length
pub use crate::stm32g483::usart1::cr2::LBDL;
///Field `LBDL` reader - LIN break detection length
pub use crate::stm32g483::usart1::cr2::LBDL_R;
///Field `LBDL` writer - LIN break detection length
pub use crate::stm32g483::usart1::cr2::LBDL_W;
///LIN mode enable
pub use crate::stm32g483::usart1::cr2::LINEN;
///Field `LINEN` reader - LIN mode enable
pub use crate::stm32g483::usart1::cr2::LINEN_R;
///Field `LINEN` writer - LIN mode enable
pub use crate::stm32g483::usart1::cr2::LINEN_W;
///Most significant bit first
pub use crate::stm32g483::usart1::cr2::MSBFIRST;
///Field `MSBFIRST` reader - Most significant bit first
pub use crate::stm32g483::usart1::cr2::MSBFIRST_R;
///Field `MSBFIRST` writer - Most significant bit first
pub use crate::stm32g483::usart1::cr2::MSBFIRST_W;
///Receiver timeout enable
pub use crate::stm32g483::usart1::cr2::RTOEN;
///Field `RTOEN` reader - Receiver timeout enable
pub use crate::stm32g483::usart1::cr2::RTOEN_R;
///Field `RTOEN` writer - Receiver timeout enable
pub use crate::stm32g483::usart1::cr2::RTOEN_W;
///RX pin active level inversion
pub use crate::stm32g483::usart1::cr2::RXINV;
///Field `RXINV` reader - RX pin active level inversion
pub use crate::stm32g483::usart1::cr2::RXINV_R;
///Field `RXINV` writer - RX pin active level inversion
pub use crate::stm32g483::usart1::cr2::RXINV_W;
///STOP bits
pub use crate::stm32g483::usart1::cr2::STOP;
///Field `STOP` reader - STOP bits
pub use crate::stm32g483::usart1::cr2::STOP_R;
///Field `STOP` writer - STOP bits
pub use crate::stm32g483::usart1::cr2::STOP_W;
///Swap TX/RX pins
pub use crate::stm32g483::usart1::cr2::SWAP;
///Field `SWAP` reader - Swap TX/RX pins
pub use crate::stm32g483::usart1::cr2::SWAP_R;
///Field `SWAP` writer - Swap TX/RX pins
pub use crate::stm32g483::usart1::cr2::SWAP_W;
///TX pin active level inversion
pub use crate::stm32g483::usart1::cr2::TXINV;
///Field `TXINV` reader - TX pin active level inversion
pub use crate::stm32g483::usart1::cr2::TXINV_R;
///Field `TXINV` writer - TX pin active level inversion
pub use crate::stm32g483::usart1::cr2::TXINV_W;
///Field `ADD` reader - Address of the USART node
pub type ADD_R = crate::FieldReader;
///Field `ADD` writer - Address of the USART node
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LIN break detection length
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Auto baud rate enable
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Auto baud rate mode
    #[inline(always)]
    pub fn abrmod(&self) -> ABRMOD_R {
        ABRMOD_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Receiver timeout enable
    #[inline(always)]
    pub fn rtoen(&self) -> RTOEN_R {
        RTOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("add", &self.add())
            .field("rtoen", &self.rtoen())
            .field("abrmod", &self.abrmod())
            .field("abren", &self.abren())
            .field("msbfirst", &self.msbfirst())
            .field("datainv", &self.datainv())
            .field("txinv", &self.txinv())
            .field("rxinv", &self.rxinv())
            .field("swap", &self.swap())
            .field("linen", &self.linen())
            .field("stop", &self.stop())
            .field("lbdie", &self.lbdie())
            .field("lbdl", &self.lbdl())
            .field("addm7", &self.addm7())
            .finish()
    }
}
impl W {
    ///Bit 4 - 7-bit Address Detection/4-bit Address Detection
    #[inline(always)]
    pub fn addm7(&mut self) -> ADDM7_W<'_, CR2rs> {
        ADDM7_W::new(self, 4)
    }
    ///Bit 5 - LIN break detection length
    #[inline(always)]
    pub fn lbdl(&mut self) -> LBDL_W<'_, CR2rs> {
        LBDL_W::new(self, 5)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W<'_, CR2rs> {
        LBDIE_W::new(self, 6)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<'_, CR2rs> {
        STOP_W::new(self, 12)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&mut self) -> LINEN_W<'_, CR2rs> {
        LINEN_W::new(self, 14)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W<'_, CR2rs> {
        SWAP_W::new(self, 15)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W<'_, CR2rs> {
        RXINV_W::new(self, 16)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W<'_, CR2rs> {
        TXINV_W::new(self, 17)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    pub fn datainv(&mut self) -> DATAINV_W<'_, CR2rs> {
        DATAINV_W::new(self, 18)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<'_, CR2rs> {
        MSBFIRST_W::new(self, 19)
    }
    ///Bit 20 - Auto baud rate enable
    #[inline(always)]
    pub fn abren(&mut self) -> ABREN_W<'_, CR2rs> {
        ABREN_W::new(self, 20)
    }
    ///Bits 21:22 - Auto baud rate mode
    #[inline(always)]
    pub fn abrmod(&mut self) -> ABRMOD_W<'_, CR2rs> {
        ABRMOD_W::new(self, 21)
    }
    ///Bit 23 - Receiver timeout enable
    #[inline(always)]
    pub fn rtoen(&mut self) -> RTOEN_W<'_, CR2rs> {
        RTOEN_W::new(self, 23)
    }
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, CR2rs> {
        ADD_W::new(self, 24)
    }
}
/**Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#UART4:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
