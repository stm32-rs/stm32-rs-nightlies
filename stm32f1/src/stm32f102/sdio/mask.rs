#[doc = "Register `MASK` reader"]
pub type R = crate::R<MASKrs>;
#[doc = "Register `MASK` writer"]
pub type W = crate::W<MASKrs>;
#[doc = "Field `CCRCFAILIE` reader - Command CRC fail interrupt enable"]
pub type CCRCFAILIE_R = crate::BitReader;
#[doc = "Field `CCRCFAILIE` writer - Command CRC fail interrupt enable"]
pub type CCRCFAILIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCFAILIE` reader - Data CRC fail interrupt enable"]
pub type DCRCFAILIE_R = crate::BitReader;
#[doc = "Field `DCRCFAILIE` writer - Data CRC fail interrupt enable"]
pub type DCRCFAILIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIMEOUTIE` reader - Command timeout interrupt enable"]
pub type CTIMEOUTIE_R = crate::BitReader;
#[doc = "Field `CTIMEOUTIE` writer - Command timeout interrupt enable"]
pub type CTIMEOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIMEOUTIE` reader - Data timeout interrupt enable"]
pub type DTIMEOUTIE_R = crate::BitReader;
#[doc = "Field `DTIMEOUTIE` writer - Data timeout interrupt enable"]
pub type DTIMEOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRIE` reader - Tx FIFO underrun error interrupt enable"]
pub type TXUNDERRIE_R = crate::BitReader;
#[doc = "Field `TXUNDERRIE` writer - Tx FIFO underrun error interrupt enable"]
pub type TXUNDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRIE` reader - Rx FIFO overrun error interrupt enable"]
pub type RXOVERRIE_R = crate::BitReader;
#[doc = "Field `RXOVERRIE` writer - Rx FIFO overrun error interrupt enable"]
pub type RXOVERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRENDIE` reader - Command response received interrupt enable"]
pub type CMDRENDIE_R = crate::BitReader;
#[doc = "Field `CMDRENDIE` writer - Command response received interrupt enable"]
pub type CMDRENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSENTIE` reader - Command sent interrupt enable"]
pub type CMDSENTIE_R = crate::BitReader;
#[doc = "Field `CMDSENTIE` writer - Command sent interrupt enable"]
pub type CMDSENTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDIE` reader - Data end interrupt enable"]
pub type DATAENDIE_R = crate::BitReader;
#[doc = "Field `DATAENDIE` writer - Data end interrupt enable"]
pub type DATAENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBITERRIE` reader - Start bit error interrupt enable"]
pub type STBITERRIE_R = crate::BitReader;
#[doc = "Field `STBITERRIE` writer - Start bit error interrupt enable"]
pub type STBITERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCKENDIE` reader - Data block end interrupt enable"]
pub type DBCKENDIE_R = crate::BitReader;
#[doc = "Field `DBCKENDIE` writer - Data block end interrupt enable"]
pub type DBCKENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDACTIE` reader - Command acting interrupt enable"]
pub type CMDACTIE_R = crate::BitReader;
#[doc = "Field `CMDACTIE` writer - Command acting interrupt enable"]
pub type CMDACTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXACTIE` reader - Data transmit acting interrupt enable"]
pub type TXACTIE_R = crate::BitReader;
#[doc = "Field `TXACTIE` writer - Data transmit acting interrupt enable"]
pub type TXACTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXACTIE` reader - Data receive acting interrupt enable"]
pub type RXACTIE_R = crate::BitReader;
#[doc = "Field `RXACTIE` writer - Data receive acting interrupt enable"]
pub type RXACTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOHEIE` reader - Tx FIFO half empty interrupt enable"]
pub type TXFIFOHEIE_R = crate::BitReader;
#[doc = "Field `TXFIFOHEIE` writer - Tx FIFO half empty interrupt enable"]
pub type TXFIFOHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOHFIE` reader - Rx FIFO half full interrupt enable"]
pub type RXFIFOHFIE_R = crate::BitReader;
#[doc = "Field `RXFIFOHFIE` writer - Rx FIFO half full interrupt enable"]
pub type RXFIFOHFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOFIE` reader - Tx FIFO full interrupt enable"]
pub type TXFIFOFIE_R = crate::BitReader;
#[doc = "Field `TXFIFOFIE` writer - Tx FIFO full interrupt enable"]
pub type TXFIFOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOFIE` reader - Rx FIFO full interrupt enable"]
pub type RXFIFOFIE_R = crate::BitReader;
#[doc = "Field `RXFIFOFIE` writer - Rx FIFO full interrupt enable"]
pub type RXFIFOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOEIE` reader - Tx FIFO empty interrupt enable"]
pub type TXFIFOEIE_R = crate::BitReader;
#[doc = "Field `TXFIFOEIE` writer - Tx FIFO empty interrupt enable"]
pub type TXFIFOEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOEIE` reader - Rx FIFO empty interrupt enable"]
pub type RXFIFOEIE_R = crate::BitReader;
#[doc = "Field `RXFIFOEIE` writer - Rx FIFO empty interrupt enable"]
pub type RXFIFOEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDAVLIE` reader - Data available in Tx FIFO interrupt enable"]
pub type TXDAVLIE_R = crate::BitReader;
#[doc = "Field `TXDAVLIE` writer - Data available in Tx FIFO interrupt enable"]
pub type TXDAVLIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDAVLIE` reader - Data available in Rx FIFO interrupt enable"]
pub type RXDAVLIE_R = crate::BitReader;
#[doc = "Field `RXDAVLIE` writer - Data available in Rx FIFO interrupt enable"]
pub type RXDAVLIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITIE` reader - SDIO mode interrupt received interrupt enable"]
pub type SDIOITIE_R = crate::BitReader;
#[doc = "Field `SDIOITIE` writer - SDIO mode interrupt received interrupt enable"]
pub type SDIOITIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEATAENDIE` reader - CE-ATA command completion signal received interrupt enable"]
pub type CEATAENDIE_R = crate::BitReader;
#[doc = "Field `CEATAENDIE` writer - CE-ATA command completion signal received interrupt enable"]
pub type CEATAENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command CRC fail interrupt enable"]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn stbiterrie(&self) -> STBITERRIE_R {
        STBITERRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    pub fn dbckendie(&self) -> DBCKENDIE_R {
        DBCKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command acting interrupt enable"]
    #[inline(always)]
    pub fn cmdactie(&self) -> CMDACTIE_R {
        CMDACTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable"]
    #[inline(always)]
    pub fn txactie(&self) -> TXACTIE_R {
        TXACTIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable"]
    #[inline(always)]
    pub fn rxactie(&self) -> RXACTIE_R {
        RXACTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable"]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx FIFO full interrupt enable"]
    #[inline(always)]
    pub fn txfifofie(&self) -> TXFIFOFIE_R {
        TXFIFOFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn rxfifoeie(&self) -> RXFIFOEIE_R {
        RXFIFOEIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data available in Tx FIFO interrupt enable"]
    #[inline(always)]
    pub fn txdavlie(&self) -> TXDAVLIE_R {
        TXDAVLIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data available in Rx FIFO interrupt enable"]
    #[inline(always)]
    pub fn rxdavlie(&self) -> RXDAVLIE_R {
        RXDAVLIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable"]
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received interrupt enable"]
    #[inline(always)]
    pub fn ceataendie(&self) -> CEATAENDIE_R {
        CEATAENDIE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command CRC fail interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W<MASKrs> {
        CCRCFAILIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W<MASKrs> {
        DCRCFAILIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W<MASKrs> {
        CTIMEOUTIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W<MASKrs> {
        DTIMEOUTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W<MASKrs> {
        TXUNDERRIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W<MASKrs> {
        RXOVERRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W<MASKrs> {
        CMDRENDIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W<MASKrs> {
        CMDSENTIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataendie(&mut self) -> DATAENDIE_W<MASKrs> {
        DATAENDIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stbiterrie(&mut self) -> STBITERRIE_W<MASKrs> {
        STBITERRIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbckendie(&mut self) -> DBCKENDIE_W<MASKrs> {
        DBCKENDIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Command acting interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmdactie(&mut self) -> CMDACTIE_W<MASKrs> {
        CMDACTIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txactie(&mut self) -> TXACTIE_W<MASKrs> {
        TXACTIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxactie(&mut self) -> RXACTIE_W<MASKrs> {
        RXACTIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W<MASKrs> {
        TXFIFOHEIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W<MASKrs> {
        RXFIFOHFIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Tx FIFO full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txfifofie(&mut self) -> TXFIFOFIE_W<MASKrs> {
        TXFIFOFIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W<MASKrs> {
        RXFIFOFIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W<MASKrs> {
        TXFIFOEIE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Rx FIFO empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifoeie(&mut self) -> RXFIFOEIE_W<MASKrs> {
        RXFIFOEIE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Data available in Tx FIFO interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdavlie(&mut self) -> TXDAVLIE_W<MASKrs> {
        TXDAVLIE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Data available in Rx FIFO interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdavlie(&mut self) -> RXDAVLIE_W<MASKrs> {
        RXDAVLIE_W::new(self, 21)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<MASKrs> {
        SDIOITIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ceataendie(&mut self) -> CEATAENDIE_W<MASKrs> {
        CEATAENDIE_W::new(self, 23)
    }
}
#[doc = "mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASKrs;
impl crate::RegisterSpec for MASKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask::R`](R) reader structure"]
impl crate::Readable for MASKrs {}
#[doc = "`write(|w| ..)` method takes [`mask::W`](W) writer structure"]
impl crate::Writable for MASKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MASKrs {
    const RESET_VALUE: u32 = 0;
}
