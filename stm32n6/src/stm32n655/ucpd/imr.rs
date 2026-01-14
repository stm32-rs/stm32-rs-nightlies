///Register `IMR` reader
pub type R = crate::R<IMRrs>;
///Register `IMR` writer
pub type W = crate::W<IMRrs>;
///Field `TXISIE` reader - TXIS interrupt enable
pub type TXISIE_R = crate::BitReader;
///Field `TXISIE` writer - TXIS interrupt enable
pub type TXISIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMSGDISCIE` reader - TXMSGDISC interrupt enable
pub type TXMSGDISCIE_R = crate::BitReader;
///Field `TXMSGDISCIE` writer - TXMSGDISC interrupt enable
pub type TXMSGDISCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMSGSENTIE` reader - TXMSGSENT interrupt enable
pub type TXMSGSENTIE_R = crate::BitReader;
///Field `TXMSGSENTIE` writer - TXMSGSENT interrupt enable
pub type TXMSGSENTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMSGABTIE` reader - TXMSGABT interrupt enable
pub type TXMSGABTIE_R = crate::BitReader;
///Field `TXMSGABTIE` writer - TXMSGABT interrupt enable
pub type TXMSGABTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HRSTDISCIE` reader - HRSTDISC interrupt enable
pub type HRSTDISCIE_R = crate::BitReader;
///Field `HRSTDISCIE` writer - HRSTDISC interrupt enable
pub type HRSTDISCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HRSTSENTIE` reader - HRSTSENT interrupt enable
pub type HRSTSENTIE_R = crate::BitReader;
///Field `HRSTSENTIE` writer - HRSTSENT interrupt enable
pub type HRSTSENTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUNDIE` reader - TXUND interrupt enable
pub type TXUNDIE_R = crate::BitReader;
///Field `TXUNDIE` writer - TXUND interrupt enable
pub type TXUNDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXNEIE` reader - RXNE interrupt enable
pub type RXNEIE_R = crate::BitReader;
///Field `RXNEIE` writer - RXNE interrupt enable
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXORDDETIE` reader - RXORDDET interrupt enable
pub type RXORDDETIE_R = crate::BitReader;
///Field `RXORDDETIE` writer - RXORDDET interrupt enable
pub type RXORDDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXHRSTDETIE` reader - RXHRSTDET interrupt enable
pub type RXHRSTDETIE_R = crate::BitReader;
///Field `RXHRSTDETIE` writer - RXHRSTDET interrupt enable
pub type RXHRSTDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVRIE` reader - RXOVR interrupt enable
pub type RXOVRIE_R = crate::BitReader;
///Field `RXOVRIE` writer - RXOVR interrupt enable
pub type RXOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXMSGENDIE` reader - RXMSGEND interrupt enable
pub type RXMSGENDIE_R = crate::BitReader;
///Field `RXMSGENDIE` writer - RXMSGEND interrupt enable
pub type RXMSGENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TYPECEVT1IE` reader - TYPECEVT1 interrupt enable
pub type TYPECEVT1IE_R = crate::BitReader;
///Field `TYPECEVT1IE` writer - TYPECEVT1 interrupt enable
pub type TYPECEVT1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TYPECEVT2IE` reader - TYPECEVT2 interrupt enable
pub type TYPECEVT2IE_R = crate::BitReader;
///Field `TYPECEVT2IE` writer - TYPECEVT2 interrupt enable
pub type TYPECEVT2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRSEVTIE` reader - FRSEVT interrupt enable
pub type FRSEVTIE_R = crate::BitReader;
impl R {
    ///Bit 0 - TXIS interrupt enable
    #[inline(always)]
    pub fn txisie(&self) -> TXISIE_R {
        TXISIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXMSGDISC interrupt enable
    #[inline(always)]
    pub fn txmsgdiscie(&self) -> TXMSGDISCIE_R {
        TXMSGDISCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TXMSGSENT interrupt enable
    #[inline(always)]
    pub fn txmsgsentie(&self) -> TXMSGSENTIE_R {
        TXMSGSENTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TXMSGABT interrupt enable
    #[inline(always)]
    pub fn txmsgabtie(&self) -> TXMSGABTIE_R {
        TXMSGABTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HRSTDISC interrupt enable
    #[inline(always)]
    pub fn hrstdiscie(&self) -> HRSTDISCIE_R {
        HRSTDISCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HRSTSENT interrupt enable
    #[inline(always)]
    pub fn hrstsentie(&self) -> HRSTSENTIE_R {
        HRSTSENTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TXUND interrupt enable
    #[inline(always)]
    pub fn txundie(&self) -> TXUNDIE_R {
        TXUNDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RXORDDET interrupt enable
    #[inline(always)]
    pub fn rxorddetie(&self) -> RXORDDETIE_R {
        RXORDDETIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RXHRSTDET interrupt enable
    #[inline(always)]
    pub fn rxhrstdetie(&self) -> RXHRSTDETIE_R {
        RXHRSTDETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RXOVR interrupt enable
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RXMSGEND interrupt enable
    #[inline(always)]
    pub fn rxmsgendie(&self) -> RXMSGENDIE_R {
        RXMSGENDIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - TYPECEVT1 interrupt enable
    #[inline(always)]
    pub fn typecevt1ie(&self) -> TYPECEVT1IE_R {
        TYPECEVT1IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TYPECEVT2 interrupt enable
    #[inline(always)]
    pub fn typecevt2ie(&self) -> TYPECEVT2IE_R {
        TYPECEVT2IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - FRSEVT interrupt enable
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
    ///Bit 0 - TXIS interrupt enable
    #[inline(always)]
    pub fn txisie(&mut self) -> TXISIE_W<'_, IMRrs> {
        TXISIE_W::new(self, 0)
    }
    ///Bit 1 - TXMSGDISC interrupt enable
    #[inline(always)]
    pub fn txmsgdiscie(&mut self) -> TXMSGDISCIE_W<'_, IMRrs> {
        TXMSGDISCIE_W::new(self, 1)
    }
    ///Bit 2 - TXMSGSENT interrupt enable
    #[inline(always)]
    pub fn txmsgsentie(&mut self) -> TXMSGSENTIE_W<'_, IMRrs> {
        TXMSGSENTIE_W::new(self, 2)
    }
    ///Bit 3 - TXMSGABT interrupt enable
    #[inline(always)]
    pub fn txmsgabtie(&mut self) -> TXMSGABTIE_W<'_, IMRrs> {
        TXMSGABTIE_W::new(self, 3)
    }
    ///Bit 4 - HRSTDISC interrupt enable
    #[inline(always)]
    pub fn hrstdiscie(&mut self) -> HRSTDISCIE_W<'_, IMRrs> {
        HRSTDISCIE_W::new(self, 4)
    }
    ///Bit 5 - HRSTSENT interrupt enable
    #[inline(always)]
    pub fn hrstsentie(&mut self) -> HRSTSENTIE_W<'_, IMRrs> {
        HRSTSENTIE_W::new(self, 5)
    }
    ///Bit 6 - TXUND interrupt enable
    #[inline(always)]
    pub fn txundie(&mut self) -> TXUNDIE_W<'_, IMRrs> {
        TXUNDIE_W::new(self, 6)
    }
    ///Bit 8 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<'_, IMRrs> {
        RXNEIE_W::new(self, 8)
    }
    ///Bit 9 - RXORDDET interrupt enable
    #[inline(always)]
    pub fn rxorddetie(&mut self) -> RXORDDETIE_W<'_, IMRrs> {
        RXORDDETIE_W::new(self, 9)
    }
    ///Bit 10 - RXHRSTDET interrupt enable
    #[inline(always)]
    pub fn rxhrstdetie(&mut self) -> RXHRSTDETIE_W<'_, IMRrs> {
        RXHRSTDETIE_W::new(self, 10)
    }
    ///Bit 11 - RXOVR interrupt enable
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<'_, IMRrs> {
        RXOVRIE_W::new(self, 11)
    }
    ///Bit 12 - RXMSGEND interrupt enable
    #[inline(always)]
    pub fn rxmsgendie(&mut self) -> RXMSGENDIE_W<'_, IMRrs> {
        RXMSGENDIE_W::new(self, 12)
    }
    ///Bit 14 - TYPECEVT1 interrupt enable
    #[inline(always)]
    pub fn typecevt1ie(&mut self) -> TYPECEVT1IE_W<'_, IMRrs> {
        TYPECEVT1IE_W::new(self, 14)
    }
    ///Bit 15 - TYPECEVT2 interrupt enable
    #[inline(always)]
    pub fn typecevt2ie(&mut self) -> TYPECEVT2IE_W<'_, IMRrs> {
        TYPECEVT2IE_W::new(self, 15)
    }
}
/**UCPD interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#UCPD:IMR)*/
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
