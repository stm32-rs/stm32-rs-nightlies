///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection
pub type ADDM7_R = crate::BitReader;
///Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` reader - STOP bits
pub type STOP_R = crate::FieldReader;
///Field `STOP` writer - STOP bits
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SWAP` reader - Swap TX/RX pins
pub type SWAP_R = crate::BitReader;
///Field `SWAP` writer - Swap TX/RX pins
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXINV` reader - RX pin active level inversion
pub type RXINV_R = crate::BitReader;
///Field `RXINV` writer - RX pin active level inversion
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXINV` reader - TX pin active level inversion
pub type TXINV_R = crate::BitReader;
///Field `TXINV` writer - TX pin active level inversion
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAINV` reader - Binary data inversion
pub type TAINV_R = crate::BitReader;
///Field `TAINV` writer - Binary data inversion
pub type TAINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSBFIRST` reader - Most significant bit first
pub type MSBFIRST_R = crate::BitReader;
///Field `MSBFIRST` writer - Most significant bit first
pub type MSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD0_3` reader - Address of the USART node
pub type ADD0_3_R = crate::FieldReader;
///Field `ADD0_3` writer - Address of the USART node
pub type ADD0_3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADD4_7` reader - Address of the USART node
pub type ADD4_7_R = crate::FieldReader;
///Field `ADD4_7` writer - Address of the USART node
pub type ADD4_7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    pub fn tainv(&self) -> TAINV_R {
        TAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 24:27 - Address of the USART node
    #[inline(always)]
    pub fn add0_3(&self) -> ADD0_3_R {
        ADD0_3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Address of the USART node
    #[inline(always)]
    pub fn add4_7(&self) -> ADD4_7_R {
        ADD4_7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("add4_7", &self.add4_7())
            .field("add0_3", &self.add0_3())
            .field("msbfirst", &self.msbfirst())
            .field("tainv", &self.tainv())
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
    #[must_use]
    pub fn addm7(&mut self) -> ADDM7_W<CR2rs> {
        ADDM7_W::new(self, 4)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 12)
    }
    ///Bit 15 - Swap TX/RX pins
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<CR2rs> {
        SWAP_W::new(self, 15)
    }
    ///Bit 16 - RX pin active level inversion
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<CR2rs> {
        RXINV_W::new(self, 16)
    }
    ///Bit 17 - TX pin active level inversion
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<CR2rs> {
        TXINV_W::new(self, 17)
    }
    ///Bit 18 - Binary data inversion
    #[inline(always)]
    #[must_use]
    pub fn tainv(&mut self) -> TAINV_W<CR2rs> {
        TAINV_W::new(self, 18)
    }
    ///Bit 19 - Most significant bit first
    #[inline(always)]
    #[must_use]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<CR2rs> {
        MSBFIRST_W::new(self, 19)
    }
    ///Bits 24:27 - Address of the USART node
    #[inline(always)]
    #[must_use]
    pub fn add0_3(&mut self) -> ADD0_3_W<CR2rs> {
        ADD0_3_W::new(self, 24)
    }
    ///Bits 28:31 - Address of the USART node
    #[inline(always)]
    #[must_use]
    pub fn add4_7(&mut self) -> ADD4_7_W<CR2rs> {
        ADD4_7_W::new(self, 28)
    }
}
/**Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#LPUART:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
