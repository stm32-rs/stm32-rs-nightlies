///Register `IMR` reader
pub type R = crate::R<IMRrs>;
///Register `IMR` writer
pub type W = crate::W<IMRrs>;
///Field `TXISIE` reader - TXISIE
pub type TXISIE_R = crate::BitReader;
///Field `TXISIE` writer - TXISIE
pub type TXISIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMSGDISCIE` reader - TXMSGDISCIE
pub type TXMSGDISCIE_R = crate::BitReader;
///Field `TXMSGDISCIE` writer - TXMSGDISCIE
pub type TXMSGDISCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMSGSENTIE` reader - TXMSGSENTIE
pub type TXMSGSENTIE_R = crate::BitReader;
///Field `TXMSGSENTIE` writer - TXMSGSENTIE
pub type TXMSGSENTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMSGABTIE` reader - TXMSGABTIE
pub type TXMSGABTIE_R = crate::BitReader;
///Field `TXMSGABTIE` writer - TXMSGABTIE
pub type TXMSGABTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HRSTDISCIE` reader - HRSTDISCIE
pub type HRSTDISCIE_R = crate::BitReader;
///Field `HRSTDISCIE` writer - HRSTDISCIE
pub type HRSTDISCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HRSTSENTIE` reader - HRSTSENTIE
pub type HRSTSENTIE_R = crate::BitReader;
///Field `HRSTSENTIE` writer - HRSTSENTIE
pub type HRSTSENTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUNDIE` reader - TXUNDIE
pub type TXUNDIE_R = crate::BitReader;
///Field `TXUNDIE` writer - TXUNDIE
pub type TXUNDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXNEIE` reader - RXNEIE
pub type RXNEIE_R = crate::BitReader;
///Field `RXNEIE` writer - RXNEIE
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDDETIE` reader - RXORDDETIE
pub type RXORDDETIE_R = crate::BitReader;
///Field `RXORDDETIE` writer - RXORDDETIE
pub type RXORDDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXHRSTDETIE` reader - RXHRSTDETIE
pub type RXHRSTDETIE_R = crate::BitReader;
///Field `RXHRSTDETIE` writer - RXHRSTDETIE
pub type RXHRSTDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVRIE` reader - RXOVRIE
pub type RXOVRIE_R = crate::BitReader;
///Field `RXOVRIE` writer - RXOVRIE
pub type RXOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXMSGENDIE` reader - RXMSGENDIE
pub type RXMSGENDIE_R = crate::BitReader;
///Field `RXMSGENDIE` writer - RXMSGENDIE
pub type RXMSGENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TYPECEVT1IE` reader - TYPECEVT1IE
pub type TYPECEVT1IE_R = crate::BitReader;
///Field `TYPECEVT1IE` writer - TYPECEVT1IE
pub type TYPECEVT1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TYPECEVT2IE` reader - TYPECEVT2IE
pub type TYPECEVT2IE_R = crate::BitReader;
///Field `TYPECEVT2IE` writer - TYPECEVT2IE
pub type TYPECEVT2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRSEVTIE` reader - FRSEVTIE
pub type FRSEVTIE_R = crate::BitReader;
///Field `FRSEVTIE` writer - FRSEVTIE
pub type FRSEVTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TXISIE
    #[inline(always)]
    pub fn txisie(&self) -> TXISIE_R {
        TXISIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXMSGDISCIE
    #[inline(always)]
    pub fn txmsgdiscie(&self) -> TXMSGDISCIE_R {
        TXMSGDISCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TXMSGSENTIE
    #[inline(always)]
    pub fn txmsgsentie(&self) -> TXMSGSENTIE_R {
        TXMSGSENTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TXMSGABTIE
    #[inline(always)]
    pub fn txmsgabtie(&self) -> TXMSGABTIE_R {
        TXMSGABTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HRSTDISCIE
    #[inline(always)]
    pub fn hrstdiscie(&self) -> HRSTDISCIE_R {
        HRSTDISCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HRSTSENTIE
    #[inline(always)]
    pub fn hrstsentie(&self) -> HRSTSENTIE_R {
        HRSTSENTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TXUNDIE
    #[inline(always)]
    pub fn txundie(&self) -> TXUNDIE_R {
        TXUNDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - RXNEIE
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RXORDDETIE
    #[inline(always)]
    pub fn rxorddetie(&self) -> RXORDDETIE_R {
        RXORDDETIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RXHRSTDETIE
    #[inline(always)]
    pub fn rxhrstdetie(&self) -> RXHRSTDETIE_R {
        RXHRSTDETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RXOVRIE
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RXMSGENDIE
    #[inline(always)]
    pub fn rxmsgendie(&self) -> RXMSGENDIE_R {
        RXMSGENDIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - TYPECEVT1IE
    #[inline(always)]
    pub fn typecevt1ie(&self) -> TYPECEVT1IE_R {
        TYPECEVT1IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TYPECEVT2IE
    #[inline(always)]
    pub fn typecevt2ie(&self) -> TYPECEVT2IE_R {
        TYPECEVT2IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - FRSEVTIE
    #[inline(always)]
    pub fn frsevtie(&self) -> FRSEVTIE_R {
        FRSEVTIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR")
            .field("txisie", &self.txisie())
            .field("txmsgdiscie", &self.txmsgdiscie())
            .field("txmsgsentie", &self.txmsgsentie())
            .field("txmsgabtie", &self.txmsgabtie())
            .field("hrstdiscie", &self.hrstdiscie())
            .field("hrstsentie", &self.hrstsentie())
            .field("txundie", &self.txundie())
            .field("rxneie", &self.rxneie())
            .field("rxorddetie", &self.rxorddetie())
            .field("rxhrstdetie", &self.rxhrstdetie())
            .field("rxovrie", &self.rxovrie())
            .field("rxmsgendie", &self.rxmsgendie())
            .field("typecevt1ie", &self.typecevt1ie())
            .field("typecevt2ie", &self.typecevt2ie())
            .field("frsevtie", &self.frsevtie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TXISIE
    #[inline(always)]
    #[must_use]
    pub fn txisie(&mut self) -> TXISIE_W<IMRrs> {
        TXISIE_W::new(self, 0)
    }
    ///Bit 1 - TXMSGDISCIE
    #[inline(always)]
    #[must_use]
    pub fn txmsgdiscie(&mut self) -> TXMSGDISCIE_W<IMRrs> {
        TXMSGDISCIE_W::new(self, 1)
    }
    ///Bit 2 - TXMSGSENTIE
    #[inline(always)]
    #[must_use]
    pub fn txmsgsentie(&mut self) -> TXMSGSENTIE_W<IMRrs> {
        TXMSGSENTIE_W::new(self, 2)
    }
    ///Bit 3 - TXMSGABTIE
    #[inline(always)]
    #[must_use]
    pub fn txmsgabtie(&mut self) -> TXMSGABTIE_W<IMRrs> {
        TXMSGABTIE_W::new(self, 3)
    }
    ///Bit 4 - HRSTDISCIE
    #[inline(always)]
    #[must_use]
    pub fn hrstdiscie(&mut self) -> HRSTDISCIE_W<IMRrs> {
        HRSTDISCIE_W::new(self, 4)
    }
    ///Bit 5 - HRSTSENTIE
    #[inline(always)]
    #[must_use]
    pub fn hrstsentie(&mut self) -> HRSTSENTIE_W<IMRrs> {
        HRSTSENTIE_W::new(self, 5)
    }
    ///Bit 6 - TXUNDIE
    #[inline(always)]
    #[must_use]
    pub fn txundie(&mut self) -> TXUNDIE_W<IMRrs> {
        TXUNDIE_W::new(self, 6)
    }
    ///Bit 8 - RXNEIE
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<IMRrs> {
        RXNEIE_W::new(self, 8)
    }
    ///Bit 9 - RXORDDETIE
    #[inline(always)]
    #[must_use]
    pub fn rxorddetie(&mut self) -> RXORDDETIE_W<IMRrs> {
        RXORDDETIE_W::new(self, 9)
    }
    ///Bit 10 - RXHRSTDETIE
    #[inline(always)]
    #[must_use]
    pub fn rxhrstdetie(&mut self) -> RXHRSTDETIE_W<IMRrs> {
        RXHRSTDETIE_W::new(self, 10)
    }
    ///Bit 11 - RXOVRIE
    #[inline(always)]
    #[must_use]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<IMRrs> {
        RXOVRIE_W::new(self, 11)
    }
    ///Bit 12 - RXMSGENDIE
    #[inline(always)]
    #[must_use]
    pub fn rxmsgendie(&mut self) -> RXMSGENDIE_W<IMRrs> {
        RXMSGENDIE_W::new(self, 12)
    }
    ///Bit 14 - TYPECEVT1IE
    #[inline(always)]
    #[must_use]
    pub fn typecevt1ie(&mut self) -> TYPECEVT1IE_W<IMRrs> {
        TYPECEVT1IE_W::new(self, 14)
    }
    ///Bit 15 - TYPECEVT2IE
    #[inline(always)]
    #[must_use]
    pub fn typecevt2ie(&mut self) -> TYPECEVT2IE_W<IMRrs> {
        TYPECEVT2IE_W::new(self, 15)
    }
    ///Bit 20 - FRSEVTIE
    #[inline(always)]
    #[must_use]
    pub fn frsevtie(&mut self) -> FRSEVTIE_W<IMRrs> {
        FRSEVTIE_W::new(self, 20)
    }
}
/**UCPD Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#UCPD1:IMR)*/
pub struct IMRrs;
impl crate::RegisterSpec for IMRrs {
    type Ux = u32;
}
///`read()` method returns [`imr::R`](R) reader structure
impl crate::Readable for IMRrs {}
///`write(|w| ..)` method takes [`imr::W`](W) writer structure
impl crate::Writable for IMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IMR to value 0
impl crate::Resettable for IMRrs {
    const RESET_VALUE: u32 = 0;
}
