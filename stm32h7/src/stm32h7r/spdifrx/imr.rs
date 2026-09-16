///Register `IMR` reader
pub type R = crate::R<IMRrs>;
///Register `IMR` writer
pub type W = crate::W<IMRrs>;
///Field `RXNEIE` reader - RXNE interrupt enable This bit is set and cleared by software.
pub type RXNEIE_R = crate::BitReader;
///Field `RXNEIE` writer - RXNE interrupt enable This bit is set and cleared by software.
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSRNEIE` reader - Control buffer ready interrupt enable This bit is set and cleared by software.
pub type CSRNEIE_R = crate::BitReader;
///Field `CSRNEIE` writer - Control buffer ready interrupt enable This bit is set and cleared by software.
pub type CSRNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERRIE` reader - Parity error interrupt enable This bit is set and cleared by software.
pub type PERRIE_R = crate::BitReader;
///Field `PERRIE` writer - Parity error interrupt enable This bit is set and cleared by software.
pub type PERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRIE` reader - Overrun error interrupt enable This bit is set and cleared by software.
pub type OVRIE_R = crate::BitReader;
///Field `OVRIE` writer - Overrun error interrupt enable This bit is set and cleared by software.
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBLKIE` reader - Synchronization block detected interrupt enable This bit is set and cleared by software.
pub type SBLKIE_R = crate::BitReader;
///Field `SBLKIE` writer - Synchronization block detected interrupt enable This bit is set and cleared by software.
pub type SBLKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCDIE` reader - Synchronization done This bit is set and cleared by software.
pub type SYNCDIE_R = crate::BitReader;
///Field `SYNCDIE` writer - Synchronization done This bit is set and cleared by software.
pub type SYNCDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IFEIE` reader - Serial interface error interrupt enable This bit is set and cleared by software.
pub type IFEIE_R = crate::BitReader;
///Field `IFEIE` writer - Serial interface error interrupt enable This bit is set and cleared by software.
pub type IFEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RXNE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Control buffer ready interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn csrneie(&self) -> CSRNEIE_R {
        CSRNEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Parity error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Synchronization block detected interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn sblkie(&self) -> SBLKIE_R {
        SBLKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Synchronization done This bit is set and cleared by software.
    #[inline(always)]
    pub fn syncdie(&self) -> SYNCDIE_R {
        SYNCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Serial interface error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ifeie(&self) -> IFEIE_R {
        IFEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR")
            .field("rxneie", &self.rxneie())
            .field("csrneie", &self.csrneie())
            .field("perrie", &self.perrie())
            .field("ovrie", &self.ovrie())
            .field("sblkie", &self.sblkie())
            .field("syncdie", &self.syncdie())
            .field("ifeie", &self.ifeie())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXNE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<'_, IMRrs> {
        RXNEIE_W::new(self, 0)
    }
    ///Bit 1 - Control buffer ready interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn csrneie(&mut self) -> CSRNEIE_W<'_, IMRrs> {
        CSRNEIE_W::new(self, 1)
    }
    ///Bit 2 - Parity error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn perrie(&mut self) -> PERRIE_W<'_, IMRrs> {
        PERRIE_W::new(self, 2)
    }
    ///Bit 3 - Overrun error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<'_, IMRrs> {
        OVRIE_W::new(self, 3)
    }
    ///Bit 4 - Synchronization block detected interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn sblkie(&mut self) -> SBLKIE_W<'_, IMRrs> {
        SBLKIE_W::new(self, 4)
    }
    ///Bit 5 - Synchronization done This bit is set and cleared by software.
    #[inline(always)]
    pub fn syncdie(&mut self) -> SYNCDIE_W<'_, IMRrs> {
        SYNCDIE_W::new(self, 5)
    }
    ///Bit 6 - Serial interface error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ifeie(&mut self) -> IFEIE_W<'_, IMRrs> {
        IFEIE_W::new(self, 6)
    }
}
/**SPDIFRX interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SPDIFRX:IMR)*/
pub struct IMRrs;
impl crate::RegisterSpec for IMRrs {
    type Ux = u32;
}
///`read()` method returns [`imr::R`](R) reader structure
impl crate::Readable for IMRrs {}
///`write(|w| ..)` method takes [`imr::W`](W) writer structure
impl crate::Writable for IMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR to value 0
impl crate::Resettable for IMRrs {}
