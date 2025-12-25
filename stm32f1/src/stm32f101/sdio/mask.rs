///Register `MASK` reader
pub type R = crate::R<MASKrs>;
///Register `MASK` writer
pub type W = crate::W<MASKrs>;
/**CCRCFAILIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRCFAILIE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<CCRCFAILIE> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAILIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CCRCFAILIE` reader - CCRCFAILIE
pub type CCRCFAILIE_R = crate::BitReader<CCRCFAILIE>;
impl CCRCFAILIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCRCFAILIE {
        match self.bits {
            false => CCRCFAILIE::Disabled,
            true => CCRCFAILIE::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCRCFAILIE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCRCFAILIE::Enabled
    }
}
///Field `CCRCFAILIE` writer - CCRCFAILIE
pub type CCRCFAILIE_W<'a, REG> = crate::BitWriter<'a, REG, CCRCFAILIE>;
impl<'a, REG> CCRCFAILIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCRCFAILIE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCRCFAILIE::Enabled)
    }
}
///Field `DCRCFAILIE` reader - DCRCFAILIE
pub use CCRCFAILIE_R as DCRCFAILIE_R;
///Field `CTIMEOUTIE` reader - CTIMEOUTIE
pub use CCRCFAILIE_R as CTIMEOUTIE_R;
///Field `DTIMEOUTIE` reader - DTIMEOUTIE
pub use CCRCFAILIE_R as DTIMEOUTIE_R;
///Field `TXUNDERRIE` reader - TXUNDERRIE
pub use CCRCFAILIE_R as TXUNDERRIE_R;
///Field `RXOVERRIE` reader - RXOVERRIE
pub use CCRCFAILIE_R as RXOVERRIE_R;
///Field `CMDRENDIE` reader - CMDRENDIE
pub use CCRCFAILIE_R as CMDRENDIE_R;
///Field `CMDSENTIE` reader - CMDSENTIE
pub use CCRCFAILIE_R as CMDSENTIE_R;
///Field `DATAENDIE` reader - DATAENDIE
pub use CCRCFAILIE_R as DATAENDIE_R;
///Field `STBITERRIE` reader - STBITERRIE
pub use CCRCFAILIE_R as STBITERRIE_R;
///Field `DBACKENDIE` reader - DBACKENDIE
pub use CCRCFAILIE_R as DBACKENDIE_R;
///Field `CMDACTIE` reader - CMDACTIE
pub use CCRCFAILIE_R as CMDACTIE_R;
///Field `TXACTIE` reader - TXACTIE
pub use CCRCFAILIE_R as TXACTIE_R;
///Field `RXACTIE` reader - RXACTIE
pub use CCRCFAILIE_R as RXACTIE_R;
///Field `TXFIFOHEIE` reader - TXFIFOHEIE
pub use CCRCFAILIE_R as TXFIFOHEIE_R;
///Field `RXFIFOHFIE` reader - RXFIFOHFIE
pub use CCRCFAILIE_R as RXFIFOHFIE_R;
///Field `TXFIFOFIE` reader - TXFIFOFIE
pub use CCRCFAILIE_R as TXFIFOFIE_R;
///Field `RXFIFOFIE` reader - RXFIFOFIE
pub use CCRCFAILIE_R as RXFIFOFIE_R;
///Field `TXFIFOEIE` reader - TXFIFOEIE
pub use CCRCFAILIE_R as TXFIFOEIE_R;
///Field `RXFIFOEIE` reader - RXFIFOEIE
pub use CCRCFAILIE_R as RXFIFOEIE_R;
///Field `TXDAVLIE` reader - TXDAVLIE
pub use CCRCFAILIE_R as TXDAVLIE_R;
///Field `RXDAVLIE` reader - RXDAVLIE
pub use CCRCFAILIE_R as RXDAVLIE_R;
///Field `SDIOITIE` reader - SDIOITIE
pub use CCRCFAILIE_R as SDIOITIE_R;
///Field `CEATENDIE` reader - CEATENDIE
pub use CCRCFAILIE_R as CEATENDIE_R;
///Field `DCRCFAILIE` writer - DCRCFAILIE
pub use CCRCFAILIE_W as DCRCFAILIE_W;
///Field `CTIMEOUTIE` writer - CTIMEOUTIE
pub use CCRCFAILIE_W as CTIMEOUTIE_W;
///Field `DTIMEOUTIE` writer - DTIMEOUTIE
pub use CCRCFAILIE_W as DTIMEOUTIE_W;
///Field `TXUNDERRIE` writer - TXUNDERRIE
pub use CCRCFAILIE_W as TXUNDERRIE_W;
///Field `RXOVERRIE` writer - RXOVERRIE
pub use CCRCFAILIE_W as RXOVERRIE_W;
///Field `CMDRENDIE` writer - CMDRENDIE
pub use CCRCFAILIE_W as CMDRENDIE_W;
///Field `CMDSENTIE` writer - CMDSENTIE
pub use CCRCFAILIE_W as CMDSENTIE_W;
///Field `DATAENDIE` writer - DATAENDIE
pub use CCRCFAILIE_W as DATAENDIE_W;
///Field `STBITERRIE` writer - STBITERRIE
pub use CCRCFAILIE_W as STBITERRIE_W;
///Field `DBACKENDIE` writer - DBACKENDIE
pub use CCRCFAILIE_W as DBACKENDIE_W;
///Field `CMDACTIE` writer - CMDACTIE
pub use CCRCFAILIE_W as CMDACTIE_W;
///Field `TXACTIE` writer - TXACTIE
pub use CCRCFAILIE_W as TXACTIE_W;
///Field `RXACTIE` writer - RXACTIE
pub use CCRCFAILIE_W as RXACTIE_W;
///Field `TXFIFOHEIE` writer - TXFIFOHEIE
pub use CCRCFAILIE_W as TXFIFOHEIE_W;
///Field `RXFIFOHFIE` writer - RXFIFOHFIE
pub use CCRCFAILIE_W as RXFIFOHFIE_W;
///Field `TXFIFOFIE` writer - TXFIFOFIE
pub use CCRCFAILIE_W as TXFIFOFIE_W;
///Field `RXFIFOFIE` writer - RXFIFOFIE
pub use CCRCFAILIE_W as RXFIFOFIE_W;
///Field `TXFIFOEIE` writer - TXFIFOEIE
pub use CCRCFAILIE_W as TXFIFOEIE_W;
///Field `RXFIFOEIE` writer - RXFIFOEIE
pub use CCRCFAILIE_W as RXFIFOEIE_W;
///Field `TXDAVLIE` writer - TXDAVLIE
pub use CCRCFAILIE_W as TXDAVLIE_W;
///Field `RXDAVLIE` writer - RXDAVLIE
pub use CCRCFAILIE_W as RXDAVLIE_W;
///Field `SDIOITIE` writer - SDIOITIE
pub use CCRCFAILIE_W as SDIOITIE_W;
///Field `CEATENDIE` writer - CEATENDIE
pub use CCRCFAILIE_W as CEATENDIE_W;
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
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W<'_, MASKrs> {
        CCRCFAILIE_W::new(self, 0)
    }
    ///Bit 1 - DCRCFAILIE
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W<'_, MASKrs> {
        DCRCFAILIE_W::new(self, 1)
    }
    ///Bit 2 - CTIMEOUTIE
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W<'_, MASKrs> {
        CTIMEOUTIE_W::new(self, 2)
    }
    ///Bit 3 - DTIMEOUTIE
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W<'_, MASKrs> {
        DTIMEOUTIE_W::new(self, 3)
    }
    ///Bit 4 - TXUNDERRIE
    #[inline(always)]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W<'_, MASKrs> {
        TXUNDERRIE_W::new(self, 4)
    }
    ///Bit 5 - RXOVERRIE
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W<'_, MASKrs> {
        RXOVERRIE_W::new(self, 5)
    }
    ///Bit 6 - CMDRENDIE
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W<'_, MASKrs> {
        CMDRENDIE_W::new(self, 6)
    }
    ///Bit 7 - CMDSENTIE
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W<'_, MASKrs> {
        CMDSENTIE_W::new(self, 7)
    }
    ///Bit 8 - DATAENDIE
    #[inline(always)]
    pub fn dataendie(&mut self) -> DATAENDIE_W<'_, MASKrs> {
        DATAENDIE_W::new(self, 8)
    }
    ///Bit 9 - STBITERRIE
    #[inline(always)]
    pub fn stbiterrie(&mut self) -> STBITERRIE_W<'_, MASKrs> {
        STBITERRIE_W::new(self, 9)
    }
    ///Bit 10 - DBACKENDIE
    #[inline(always)]
    pub fn dbackendie(&mut self) -> DBACKENDIE_W<'_, MASKrs> {
        DBACKENDIE_W::new(self, 10)
    }
    ///Bit 11 - CMDACTIE
    #[inline(always)]
    pub fn cmdactie(&mut self) -> CMDACTIE_W<'_, MASKrs> {
        CMDACTIE_W::new(self, 11)
    }
    ///Bit 12 - TXACTIE
    #[inline(always)]
    pub fn txactie(&mut self) -> TXACTIE_W<'_, MASKrs> {
        TXACTIE_W::new(self, 12)
    }
    ///Bit 13 - RXACTIE
    #[inline(always)]
    pub fn rxactie(&mut self) -> RXACTIE_W<'_, MASKrs> {
        RXACTIE_W::new(self, 13)
    }
    ///Bit 14 - TXFIFOHEIE
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W<'_, MASKrs> {
        TXFIFOHEIE_W::new(self, 14)
    }
    ///Bit 15 - RXFIFOHFIE
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W<'_, MASKrs> {
        RXFIFOHFIE_W::new(self, 15)
    }
    ///Bit 16 - TXFIFOFIE
    #[inline(always)]
    pub fn txfifofie(&mut self) -> TXFIFOFIE_W<'_, MASKrs> {
        TXFIFOFIE_W::new(self, 16)
    }
    ///Bit 17 - RXFIFOFIE
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W<'_, MASKrs> {
        RXFIFOFIE_W::new(self, 17)
    }
    ///Bit 18 - TXFIFOEIE
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W<'_, MASKrs> {
        TXFIFOEIE_W::new(self, 18)
    }
    ///Bit 19 - RXFIFOEIE
    #[inline(always)]
    pub fn rxfifoeie(&mut self) -> RXFIFOEIE_W<'_, MASKrs> {
        RXFIFOEIE_W::new(self, 19)
    }
    ///Bit 20 - TXDAVLIE
    #[inline(always)]
    pub fn txdavlie(&mut self) -> TXDAVLIE_W<'_, MASKrs> {
        TXDAVLIE_W::new(self, 20)
    }
    ///Bit 21 - RXDAVLIE
    #[inline(always)]
    pub fn rxdavlie(&mut self) -> RXDAVLIE_W<'_, MASKrs> {
        RXDAVLIE_W::new(self, 21)
    }
    ///Bit 22 - SDIOITIE
    #[inline(always)]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<'_, MASKrs> {
        SDIOITIE_W::new(self, 22)
    }
    ///Bit 23 - CEATENDIE
    #[inline(always)]
    pub fn ceatendie(&mut self) -> CEATENDIE_W<'_, MASKrs> {
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
