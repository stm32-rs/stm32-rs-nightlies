///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///7-bit Address Detection/4-bit Address Detection
pub use crate::stm32g483::usart1::cr2::ADDM7;
///Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection
pub use crate::stm32g483::usart1::cr2::ADDM7_R;
///Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection
pub use crate::stm32g483::usart1::cr2::ADDM7_W;
/**STOP bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP {
    ///0: 1 stop bit
    Stop1 = 0,
    ///2: 2 stop bit
    Stop2 = 2,
}
impl From<STOP> for u8 {
    #[inline(always)]
    fn from(variant: STOP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOP {
    type Ux = u8;
}
impl crate::IsEnum for STOP {}
///Field `STOP` reader - STOP bits
pub type STOP_R = crate::FieldReader<STOP>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STOP> {
        match self.bits {
            0 => Some(STOP::Stop1),
            2 => Some(STOP::Stop2),
            _ => None,
        }
    }
    ///1 stop bit
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP::Stop1
    }
    ///2 stop bit
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP::Stop2
    }
}
///Field `STOP` writer - STOP bits
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STOP>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 stop bit
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1)
    }
    ///2 stop bit
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop2)
    }
}
///Binary data inversion
pub use crate::stm32g483::usart1::cr2::DATAINV;
///Field `DATAINV` reader - Binary data inversion
pub use crate::stm32g483::usart1::cr2::DATAINV_R;
///Field `DATAINV` writer - Binary data inversion
pub use crate::stm32g483::usart1::cr2::DATAINV_W;
///Most significant bit first
pub use crate::stm32g483::usart1::cr2::MSBFIRST;
///Field `MSBFIRST` reader - Most significant bit first
pub use crate::stm32g483::usart1::cr2::MSBFIRST_R;
///Field `MSBFIRST` writer - Most significant bit first
pub use crate::stm32g483::usart1::cr2::MSBFIRST_W;
///RX pin active level inversion
pub use crate::stm32g483::usart1::cr2::RXINV;
///Field `RXINV` reader - RX pin active level inversion
pub use crate::stm32g483::usart1::cr2::RXINV_R;
///Field `RXINV` writer - RX pin active level inversion
pub use crate::stm32g483::usart1::cr2::RXINV_W;
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
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
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
            .field("msbfirst", &self.msbfirst())
            .field("datainv", &self.datainv())
            .field("txinv", &self.txinv())
            .field("rxinv", &self.rxinv())
            .field("swap", &self.swap())
            .field("stop", &self.stop())
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
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<'_, CR2rs> {
        STOP_W::new(self, 12)
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
    ///Bits 24:31 - Address of the USART node
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, CR2rs> {
        ADD_W::new(self, 24)
    }
}
/**Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#LPUART1:CR2)*/
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
