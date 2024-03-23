#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM7_R = crate::BitReader;
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP bits"]
pub type STOP_R = crate::FieldReader;
#[doc = "Field `STOP` writer - STOP bits"]
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWAP` reader - Swap TX/RX pins"]
pub type SWAP_R = crate::BitReader;
#[doc = "Field `SWAP` writer - Swap TX/RX pins"]
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - RX pin active level inversion"]
pub type RXINV_R = crate::BitReader;
#[doc = "Field `RXINV` writer - RX pin active level inversion"]
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - TX pin active level inversion"]
pub type TXINV_R = crate::BitReader;
#[doc = "Field `TXINV` writer - TX pin active level inversion"]
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAINV` reader - Binary data inversion"]
pub type DATAINV_R = crate::BitReader;
#[doc = "Field `DATAINV` writer - Binary data inversion"]
pub type DATAINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBFIRST` reader - Most significant bit first"]
pub type MSBFIRST_R = crate::BitReader;
#[doc = "Field `MSBFIRST` writer - Most significant bit first"]
pub type MSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD` reader - Address of the USART node"]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - Address of the USART node"]
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    #[must_use]
    pub fn addm7(&mut self) -> ADDM7_W<CR2rs> {
        ADDM7_W::new(self, 4)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 12)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<CR2rs> {
        SWAP_W::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<CR2rs> {
        RXINV_W::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<CR2rs> {
        TXINV_W::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    #[must_use]
    pub fn datainv(&mut self) -> DATAINV_W<CR2rs> {
        DATAINV_W::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<CR2rs> {
        MSBFIRST_W::new(self, 19)
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<CR2rs> {
        ADD_W::new(self, 24)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
