///Register `MASK` reader
pub type R = crate::R<MASKrs>;
///Register `MASK` writer
pub type W = crate::W<MASKrs>;
///Field `CCRCFAILIE` reader - CCRCFAILIE
pub type CCRCFAILIE_R = crate::BitReader;
///Field `CCRCFAILIE` writer - CCRCFAILIE
pub type CCRCFAILIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCRCFAILIE` reader - DCRCFAILIE
pub type DCRCFAILIE_R = crate::BitReader;
///Field `DCRCFAILIE` writer - DCRCFAILIE
pub type DCRCFAILIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIMEOUTIE` reader - CTIMEOUTIE
pub type CTIMEOUTIE_R = crate::BitReader;
///Field `CTIMEOUTIE` writer - CTIMEOUTIE
pub type CTIMEOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTIMEOUTIE` reader - DTIMEOUTIE
pub type DTIMEOUTIE_R = crate::BitReader;
///Field `DTIMEOUTIE` writer - DTIMEOUTIE
pub type DTIMEOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUNDERRIE` reader - TXUNDERRIE
pub type TXUNDERRIE_R = crate::BitReader;
///Field `TXUNDERRIE` writer - TXUNDERRIE
pub type TXUNDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVERRIE` reader - RXOVERRIE
pub type RXOVERRIE_R = crate::BitReader;
///Field `RXOVERRIE` writer - RXOVERRIE
pub type RXOVERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDRENDIE` reader - CMDRENDIE
pub type CMDRENDIE_R = crate::BitReader;
///Field `CMDRENDIE` writer - CMDRENDIE
pub type CMDRENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDSENTIE` reader - CMDSENTIE
pub type CMDSENTIE_R = crate::BitReader;
///Field `CMDSENTIE` writer - CMDSENTIE
pub type CMDSENTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATAENDIE` reader - DATAENDIE
pub type DATAENDIE_R = crate::BitReader;
///Field `DATAENDIE` writer - DATAENDIE
pub type DATAENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STBITERRIE` reader - STBITERRIE
pub type STBITERRIE_R = crate::BitReader;
///Field `STBITERRIE` writer - STBITERRIE
pub type STBITERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBACKENDIE` reader - DBACKENDIE
pub type DBACKENDIE_R = crate::BitReader;
///Field `DBACKENDIE` writer - DBACKENDIE
pub type DBACKENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDACTIE` reader - CMDACTIE
pub type CMDACTIE_R = crate::BitReader;
///Field `CMDACTIE` writer - CMDACTIE
pub type CMDACTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXACTIE` reader - TXACTIE
pub type TXACTIE_R = crate::BitReader;
///Field `TXACTIE` writer - TXACTIE
pub type TXACTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXACTIE` reader - RXACTIE
pub type RXACTIE_R = crate::BitReader;
///Field `RXACTIE` writer - RXACTIE
pub type RXACTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFOHEIE` reader - TXFIFOHEIE
pub type TXFIFOHEIE_R = crate::BitReader;
///Field `TXFIFOHEIE` writer - TXFIFOHEIE
pub type TXFIFOHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFOHFIE` reader - RXFIFOHFIE
pub type RXFIFOHFIE_R = crate::BitReader;
///Field `RXFIFOHFIE` writer - RXFIFOHFIE
pub type RXFIFOHFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFOFIE` reader - TXFIFOFIE
pub type TXFIFOFIE_R = crate::BitReader;
///Field `TXFIFOFIE` writer - TXFIFOFIE
pub type TXFIFOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFOFIE` reader - RXFIFOFIE
pub type RXFIFOFIE_R = crate::BitReader;
///Field `RXFIFOFIE` writer - RXFIFOFIE
pub type RXFIFOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFOEIE` reader - TXFIFOEIE
pub type TXFIFOEIE_R = crate::BitReader;
///Field `TXFIFOEIE` writer - TXFIFOEIE
pub type TXFIFOEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFOEIE` reader - RXFIFOEIE
pub type RXFIFOEIE_R = crate::BitReader;
///Field `RXFIFOEIE` writer - RXFIFOEIE
pub type RXFIFOEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDAVLIE` reader - TXDAVLIE
pub type TXDAVLIE_R = crate::BitReader;
///Field `TXDAVLIE` writer - TXDAVLIE
pub type TXDAVLIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXDAVLIE` reader - RXDAVLIE
pub type RXDAVLIE_R = crate::BitReader;
///Field `RXDAVLIE` writer - RXDAVLIE
pub type RXDAVLIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIOITIE` reader - SDIOITIE
pub type SDIOITIE_R = crate::BitReader;
///Field `SDIOITIE` writer - SDIOITIE
pub type SDIOITIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEATENDIE` reader - CEATENDIE
pub type CEATENDIE_R = crate::BitReader;
///Field `CEATENDIE` writer - CEATENDIE
pub type CEATENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CCRCFAILIE
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DCRCFAILIE
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTIMEOUTIE
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DTIMEOUTIE
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXUNDERRIE
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXOVERRIE
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CMDRENDIE
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CMDSENTIE
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DATAENDIE
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - STBITERRIE
    #[inline(always)]
    pub fn stbiterrie(&self) -> STBITERRIE_R {
        STBITERRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBACKENDIE
    #[inline(always)]
    pub fn dbackendie(&self) -> DBACKENDIE_R {
        DBACKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CMDACTIE
    #[inline(always)]
    pub fn cmdactie(&self) -> CMDACTIE_R {
        CMDACTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TXACTIE
    #[inline(always)]
    pub fn txactie(&self) -> TXACTIE_R {
        TXACTIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RXACTIE
    #[inline(always)]
    pub fn rxactie(&self) -> RXACTIE_R {
        RXACTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TXFIFOHEIE
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RXFIFOHFIE
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TXFIFOFIE
    #[inline(always)]
    pub fn txfifofie(&self) -> TXFIFOFIE_R {
        TXFIFOFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RXFIFOFIE
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TXFIFOEIE
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RXFIFOEIE
    #[inline(always)]
    pub fn rxfifoeie(&self) -> RXFIFOEIE_R {
        RXFIFOEIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TXDAVLIE
    #[inline(always)]
    pub fn txdavlie(&self) -> TXDAVLIE_R {
        TXDAVLIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RXDAVLIE
    #[inline(always)]
    pub fn rxdavlie(&self) -> RXDAVLIE_R {
        RXDAVLIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIOITIE
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CEATENDIE
    #[inline(always)]
    pub fn ceatendie(&self) -> CEATENDIE_R {
        CEATENDIE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK")
            .field("ccrcfailie", &self.ccrcfailie())
            .field("dcrcfailie", &self.dcrcfailie())
            .field("ctimeoutie", &self.ctimeoutie())
            .field("dtimeoutie", &self.dtimeoutie())
            .field("txunderrie", &self.txunderrie())
            .field("rxoverrie", &self.rxoverrie())
            .field("cmdrendie", &self.cmdrendie())
            .field("cmdsentie", &self.cmdsentie())
            .field("dataendie", &self.dataendie())
            .field("stbiterrie", &self.stbiterrie())
            .field("dbackendie", &self.dbackendie())
            .field("cmdactie", &self.cmdactie())
            .field("txactie", &self.txactie())
            .field("rxactie", &self.rxactie())
            .field("txfifoheie", &self.txfifoheie())
            .field("rxfifohfie", &self.rxfifohfie())
            .field("txfifofie", &self.txfifofie())
            .field("rxfifofie", &self.rxfifofie())
            .field("txfifoeie", &self.txfifoeie())
            .field("rxfifoeie", &self.rxfifoeie())
            .field("txdavlie", &self.txdavlie())
            .field("rxdavlie", &self.rxdavlie())
            .field("sdioitie", &self.sdioitie())
            .field("ceatendie", &self.ceatendie())
            .finish()
    }
}
impl W {
    ///Bit 0 - CCRCFAILIE
    #[inline(always)]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W<MASKrs> {
        CCRCFAILIE_W::new(self, 0)
    }
    ///Bit 1 - DCRCFAILIE
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W<MASKrs> {
        DCRCFAILIE_W::new(self, 1)
    }
    ///Bit 2 - CTIMEOUTIE
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W<MASKrs> {
        CTIMEOUTIE_W::new(self, 2)
    }
    ///Bit 3 - DTIMEOUTIE
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W<MASKrs> {
        DTIMEOUTIE_W::new(self, 3)
    }
    ///Bit 4 - TXUNDERRIE
    #[inline(always)]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W<MASKrs> {
        TXUNDERRIE_W::new(self, 4)
    }
    ///Bit 5 - RXOVERRIE
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W<MASKrs> {
        RXOVERRIE_W::new(self, 5)
    }
    ///Bit 6 - CMDRENDIE
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W<MASKrs> {
        CMDRENDIE_W::new(self, 6)
    }
    ///Bit 7 - CMDSENTIE
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W<MASKrs> {
        CMDSENTIE_W::new(self, 7)
    }
    ///Bit 8 - DATAENDIE
    #[inline(always)]
    pub fn dataendie(&mut self) -> DATAENDIE_W<MASKrs> {
        DATAENDIE_W::new(self, 8)
    }
    ///Bit 9 - STBITERRIE
    #[inline(always)]
    pub fn stbiterrie(&mut self) -> STBITERRIE_W<MASKrs> {
        STBITERRIE_W::new(self, 9)
    }
    ///Bit 10 - DBACKENDIE
    #[inline(always)]
    pub fn dbackendie(&mut self) -> DBACKENDIE_W<MASKrs> {
        DBACKENDIE_W::new(self, 10)
    }
    ///Bit 11 - CMDACTIE
    #[inline(always)]
    pub fn cmdactie(&mut self) -> CMDACTIE_W<MASKrs> {
        CMDACTIE_W::new(self, 11)
    }
    ///Bit 12 - TXACTIE
    #[inline(always)]
    pub fn txactie(&mut self) -> TXACTIE_W<MASKrs> {
        TXACTIE_W::new(self, 12)
    }
    ///Bit 13 - RXACTIE
    #[inline(always)]
    pub fn rxactie(&mut self) -> RXACTIE_W<MASKrs> {
        RXACTIE_W::new(self, 13)
    }
    ///Bit 14 - TXFIFOHEIE
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W<MASKrs> {
        TXFIFOHEIE_W::new(self, 14)
    }
    ///Bit 15 - RXFIFOHFIE
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W<MASKrs> {
        RXFIFOHFIE_W::new(self, 15)
    }
    ///Bit 16 - TXFIFOFIE
    #[inline(always)]
    pub fn txfifofie(&mut self) -> TXFIFOFIE_W<MASKrs> {
        TXFIFOFIE_W::new(self, 16)
    }
    ///Bit 17 - RXFIFOFIE
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W<MASKrs> {
        RXFIFOFIE_W::new(self, 17)
    }
    ///Bit 18 - TXFIFOEIE
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W<MASKrs> {
        TXFIFOEIE_W::new(self, 18)
    }
    ///Bit 19 - RXFIFOEIE
    #[inline(always)]
    pub fn rxfifoeie(&mut self) -> RXFIFOEIE_W<MASKrs> {
        RXFIFOEIE_W::new(self, 19)
    }
    ///Bit 20 - TXDAVLIE
    #[inline(always)]
    pub fn txdavlie(&mut self) -> TXDAVLIE_W<MASKrs> {
        TXDAVLIE_W::new(self, 20)
    }
    ///Bit 21 - RXDAVLIE
    #[inline(always)]
    pub fn rxdavlie(&mut self) -> RXDAVLIE_W<MASKrs> {
        RXDAVLIE_W::new(self, 21)
    }
    ///Bit 22 - SDIOITIE
    #[inline(always)]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<MASKrs> {
        SDIOITIE_W::new(self, 22)
    }
    ///Bit 23 - CEATENDIE
    #[inline(always)]
    pub fn ceatendie(&mut self) -> CEATENDIE_W<MASKrs> {
        CEATENDIE_W::new(self, 23)
    }
}
/**SDIO mask register (SDIO_MASK)

You can [`read`](crate::Reg::read) this register and get [`mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#SDIO:MASK)*/
pub struct MASKrs;
impl crate::RegisterSpec for MASKrs {
    type Ux = u32;
}
///`read()` method returns [`mask::R`](R) reader structure
impl crate::Readable for MASKrs {}
///`write(|w| ..)` method takes [`mask::W`](W) writer structure
impl crate::Writable for MASKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MASK to value 0
impl crate::Resettable for MASKrs {}
