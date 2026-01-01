///Register `MASK` reader
pub type R = crate::R<MASKrs>;
///Register `MASK` writer
pub type W = crate::W<MASKrs>;
/**Command CRC fail interrupt enable

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
///Field `CCRCFAILIE` reader - Command CRC fail interrupt enable
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
///Field `CCRCFAILIE` writer - Command CRC fail interrupt enable
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
///Field `DCRCFAILIE` reader - Data CRC fail interrupt enable
pub use CCRCFAILIE_R as DCRCFAILIE_R;
///Field `CTIMEOUTIE` reader - Command timeout interrupt enable
pub use CCRCFAILIE_R as CTIMEOUTIE_R;
///Field `DTIMEOUTIE` reader - Data timeout interrupt enable
pub use CCRCFAILIE_R as DTIMEOUTIE_R;
///Field `TXUNDERRIE` reader - Tx FIFO underrun error interrupt enable
pub use CCRCFAILIE_R as TXUNDERRIE_R;
///Field `RXOVERRIE` reader - Rx FIFO overrun error interrupt enable
pub use CCRCFAILIE_R as RXOVERRIE_R;
///Field `CMDRENDIE` reader - Command response received interrupt enable
pub use CCRCFAILIE_R as CMDRENDIE_R;
///Field `CMDSENTIE` reader - Command sent interrupt enable
pub use CCRCFAILIE_R as CMDSENTIE_R;
///Field `DATAENDIE` reader - Data end interrupt enable
pub use CCRCFAILIE_R as DATAENDIE_R;
///Field `STBITERRIE` reader - Start bit error interrupt enable
pub use CCRCFAILIE_R as STBITERRIE_R;
///Field `DBCKENDIE` reader - Data block end interrupt enable
pub use CCRCFAILIE_R as DBCKENDIE_R;
///Field `CMDACTIE` reader - Command acting interrupt enable
pub use CCRCFAILIE_R as CMDACTIE_R;
///Field `TXACTIE` reader - Data transmit acting interrupt enable
pub use CCRCFAILIE_R as TXACTIE_R;
///Field `RXACTIE` reader - Data receive acting interrupt enable
pub use CCRCFAILIE_R as RXACTIE_R;
///Field `TXFIFOHEIE` reader - Tx FIFO half empty interrupt enable
pub use CCRCFAILIE_R as TXFIFOHEIE_R;
///Field `RXFIFOHFIE` reader - Rx FIFO half full interrupt enable
pub use CCRCFAILIE_R as RXFIFOHFIE_R;
///Field `TXFIFOFIE` reader - Tx FIFO full interrupt enable
pub use CCRCFAILIE_R as TXFIFOFIE_R;
///Field `RXFIFOFIE` reader - Rx FIFO full interrupt enable
pub use CCRCFAILIE_R as RXFIFOFIE_R;
///Field `TXFIFOEIE` reader - Tx FIFO empty interrupt enable
pub use CCRCFAILIE_R as TXFIFOEIE_R;
///Field `RXFIFOEIE` reader - Rx FIFO empty interrupt enable
pub use CCRCFAILIE_R as RXFIFOEIE_R;
///Field `TXDAVLIE` reader - Data available in Tx FIFO interrupt enable
pub use CCRCFAILIE_R as TXDAVLIE_R;
///Field `RXDAVLIE` reader - Data available in Rx FIFO interrupt enable
pub use CCRCFAILIE_R as RXDAVLIE_R;
///Field `SDIOITIE` reader - SDIO mode interrupt received interrupt enable
pub use CCRCFAILIE_R as SDIOITIE_R;
///Field `CEATAENDIE` reader - CE-ATA command completion signal received interrupt enable
pub use CCRCFAILIE_R as CEATAENDIE_R;
///Field `DCRCFAILIE` writer - Data CRC fail interrupt enable
pub use CCRCFAILIE_W as DCRCFAILIE_W;
///Field `CTIMEOUTIE` writer - Command timeout interrupt enable
pub use CCRCFAILIE_W as CTIMEOUTIE_W;
///Field `DTIMEOUTIE` writer - Data timeout interrupt enable
pub use CCRCFAILIE_W as DTIMEOUTIE_W;
///Field `TXUNDERRIE` writer - Tx FIFO underrun error interrupt enable
pub use CCRCFAILIE_W as TXUNDERRIE_W;
///Field `RXOVERRIE` writer - Rx FIFO overrun error interrupt enable
pub use CCRCFAILIE_W as RXOVERRIE_W;
///Field `CMDRENDIE` writer - Command response received interrupt enable
pub use CCRCFAILIE_W as CMDRENDIE_W;
///Field `CMDSENTIE` writer - Command sent interrupt enable
pub use CCRCFAILIE_W as CMDSENTIE_W;
///Field `DATAENDIE` writer - Data end interrupt enable
pub use CCRCFAILIE_W as DATAENDIE_W;
///Field `STBITERRIE` writer - Start bit error interrupt enable
pub use CCRCFAILIE_W as STBITERRIE_W;
///Field `DBCKENDIE` writer - Data block end interrupt enable
pub use CCRCFAILIE_W as DBCKENDIE_W;
///Field `CMDACTIE` writer - Command acting interrupt enable
pub use CCRCFAILIE_W as CMDACTIE_W;
///Field `TXACTIE` writer - Data transmit acting interrupt enable
pub use CCRCFAILIE_W as TXACTIE_W;
///Field `RXACTIE` writer - Data receive acting interrupt enable
pub use CCRCFAILIE_W as RXACTIE_W;
///Field `TXFIFOHEIE` writer - Tx FIFO half empty interrupt enable
pub use CCRCFAILIE_W as TXFIFOHEIE_W;
///Field `RXFIFOHFIE` writer - Rx FIFO half full interrupt enable
pub use CCRCFAILIE_W as RXFIFOHFIE_W;
///Field `TXFIFOFIE` writer - Tx FIFO full interrupt enable
pub use CCRCFAILIE_W as TXFIFOFIE_W;
///Field `RXFIFOFIE` writer - Rx FIFO full interrupt enable
pub use CCRCFAILIE_W as RXFIFOFIE_W;
///Field `TXFIFOEIE` writer - Tx FIFO empty interrupt enable
pub use CCRCFAILIE_W as TXFIFOEIE_W;
///Field `RXFIFOEIE` writer - Rx FIFO empty interrupt enable
pub use CCRCFAILIE_W as RXFIFOEIE_W;
///Field `TXDAVLIE` writer - Data available in Tx FIFO interrupt enable
pub use CCRCFAILIE_W as TXDAVLIE_W;
///Field `RXDAVLIE` writer - Data available in Rx FIFO interrupt enable
pub use CCRCFAILIE_W as RXDAVLIE_W;
///Field `SDIOITIE` writer - SDIO mode interrupt received interrupt enable
pub use CCRCFAILIE_W as SDIOITIE_W;
///Field `CEATAENDIE` writer - CE-ATA command completion signal received interrupt enable
pub use CCRCFAILIE_W as CEATAENDIE_W;
impl R {
    ///Bit 0 - Command CRC fail interrupt enable
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data CRC fail interrupt enable
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command timeout interrupt enable
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Data timeout interrupt enable
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Tx FIFO underrun error interrupt enable
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO overrun error interrupt enable
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Command response received interrupt enable
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Command sent interrupt enable
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Data end interrupt enable
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Start bit error interrupt enable
    #[inline(always)]
    pub fn stbiterrie(&self) -> STBITERRIE_R {
        STBITERRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data block end interrupt enable
    #[inline(always)]
    pub fn dbckendie(&self) -> DBCKENDIE_R {
        DBCKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Command acting interrupt enable
    #[inline(always)]
    pub fn cmdactie(&self) -> CMDACTIE_R {
        CMDACTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Data transmit acting interrupt enable
    #[inline(always)]
    pub fn txactie(&self) -> TXACTIE_R {
        TXACTIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Data receive acting interrupt enable
    #[inline(always)]
    pub fn rxactie(&self) -> RXACTIE_R {
        RXACTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Tx FIFO half empty interrupt enable
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rx FIFO half full interrupt enable
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Tx FIFO full interrupt enable
    #[inline(always)]
    pub fn txfifofie(&self) -> TXFIFOFIE_R {
        TXFIFOFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Rx FIFO full interrupt enable
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Tx FIFO empty interrupt enable
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Rx FIFO empty interrupt enable
    #[inline(always)]
    pub fn rxfifoeie(&self) -> RXFIFOEIE_R {
        RXFIFOEIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Data available in Tx FIFO interrupt enable
    #[inline(always)]
    pub fn txdavlie(&self) -> TXDAVLIE_R {
        TXDAVLIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Data available in Rx FIFO interrupt enable
    #[inline(always)]
    pub fn rxdavlie(&self) -> RXDAVLIE_R {
        RXDAVLIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIO mode interrupt received interrupt enable
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CE-ATA command completion signal received interrupt enable
    #[inline(always)]
    pub fn ceataendie(&self) -> CEATAENDIE_R {
        CEATAENDIE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK")
            .field("ccrcfailie", &self.ccrcfailie())
            .field("ceataendie", &self.ceataendie())
            .field("sdioitie", &self.sdioitie())
            .field("rxdavlie", &self.rxdavlie())
            .field("txdavlie", &self.txdavlie())
            .field("rxfifoeie", &self.rxfifoeie())
            .field("txfifoeie", &self.txfifoeie())
            .field("rxfifofie", &self.rxfifofie())
            .field("txfifofie", &self.txfifofie())
            .field("rxfifohfie", &self.rxfifohfie())
            .field("txfifoheie", &self.txfifoheie())
            .field("rxactie", &self.rxactie())
            .field("txactie", &self.txactie())
            .field("cmdactie", &self.cmdactie())
            .field("dbckendie", &self.dbckendie())
            .field("stbiterrie", &self.stbiterrie())
            .field("dataendie", &self.dataendie())
            .field("cmdsentie", &self.cmdsentie())
            .field("cmdrendie", &self.cmdrendie())
            .field("rxoverrie", &self.rxoverrie())
            .field("txunderrie", &self.txunderrie())
            .field("dtimeoutie", &self.dtimeoutie())
            .field("ctimeoutie", &self.ctimeoutie())
            .field("dcrcfailie", &self.dcrcfailie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Command CRC fail interrupt enable
    #[inline(always)]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W<'_, MASKrs> {
        CCRCFAILIE_W::new(self, 0)
    }
    ///Bit 1 - Data CRC fail interrupt enable
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W<'_, MASKrs> {
        DCRCFAILIE_W::new(self, 1)
    }
    ///Bit 2 - Command timeout interrupt enable
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W<'_, MASKrs> {
        CTIMEOUTIE_W::new(self, 2)
    }
    ///Bit 3 - Data timeout interrupt enable
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W<'_, MASKrs> {
        DTIMEOUTIE_W::new(self, 3)
    }
    ///Bit 4 - Tx FIFO underrun error interrupt enable
    #[inline(always)]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W<'_, MASKrs> {
        TXUNDERRIE_W::new(self, 4)
    }
    ///Bit 5 - Rx FIFO overrun error interrupt enable
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W<'_, MASKrs> {
        RXOVERRIE_W::new(self, 5)
    }
    ///Bit 6 - Command response received interrupt enable
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W<'_, MASKrs> {
        CMDRENDIE_W::new(self, 6)
    }
    ///Bit 7 - Command sent interrupt enable
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W<'_, MASKrs> {
        CMDSENTIE_W::new(self, 7)
    }
    ///Bit 8 - Data end interrupt enable
    #[inline(always)]
    pub fn dataendie(&mut self) -> DATAENDIE_W<'_, MASKrs> {
        DATAENDIE_W::new(self, 8)
    }
    ///Bit 9 - Start bit error interrupt enable
    #[inline(always)]
    pub fn stbiterrie(&mut self) -> STBITERRIE_W<'_, MASKrs> {
        STBITERRIE_W::new(self, 9)
    }
    ///Bit 10 - Data block end interrupt enable
    #[inline(always)]
    pub fn dbckendie(&mut self) -> DBCKENDIE_W<'_, MASKrs> {
        DBCKENDIE_W::new(self, 10)
    }
    ///Bit 11 - Command acting interrupt enable
    #[inline(always)]
    pub fn cmdactie(&mut self) -> CMDACTIE_W<'_, MASKrs> {
        CMDACTIE_W::new(self, 11)
    }
    ///Bit 12 - Data transmit acting interrupt enable
    #[inline(always)]
    pub fn txactie(&mut self) -> TXACTIE_W<'_, MASKrs> {
        TXACTIE_W::new(self, 12)
    }
    ///Bit 13 - Data receive acting interrupt enable
    #[inline(always)]
    pub fn rxactie(&mut self) -> RXACTIE_W<'_, MASKrs> {
        RXACTIE_W::new(self, 13)
    }
    ///Bit 14 - Tx FIFO half empty interrupt enable
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W<'_, MASKrs> {
        TXFIFOHEIE_W::new(self, 14)
    }
    ///Bit 15 - Rx FIFO half full interrupt enable
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W<'_, MASKrs> {
        RXFIFOHFIE_W::new(self, 15)
    }
    ///Bit 16 - Tx FIFO full interrupt enable
    #[inline(always)]
    pub fn txfifofie(&mut self) -> TXFIFOFIE_W<'_, MASKrs> {
        TXFIFOFIE_W::new(self, 16)
    }
    ///Bit 17 - Rx FIFO full interrupt enable
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W<'_, MASKrs> {
        RXFIFOFIE_W::new(self, 17)
    }
    ///Bit 18 - Tx FIFO empty interrupt enable
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W<'_, MASKrs> {
        TXFIFOEIE_W::new(self, 18)
    }
    ///Bit 19 - Rx FIFO empty interrupt enable
    #[inline(always)]
    pub fn rxfifoeie(&mut self) -> RXFIFOEIE_W<'_, MASKrs> {
        RXFIFOEIE_W::new(self, 19)
    }
    ///Bit 20 - Data available in Tx FIFO interrupt enable
    #[inline(always)]
    pub fn txdavlie(&mut self) -> TXDAVLIE_W<'_, MASKrs> {
        TXDAVLIE_W::new(self, 20)
    }
    ///Bit 21 - Data available in Rx FIFO interrupt enable
    #[inline(always)]
    pub fn rxdavlie(&mut self) -> RXDAVLIE_W<'_, MASKrs> {
        RXDAVLIE_W::new(self, 21)
    }
    ///Bit 22 - SDIO mode interrupt received interrupt enable
    #[inline(always)]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<'_, MASKrs> {
        SDIOITIE_W::new(self, 22)
    }
    ///Bit 23 - CE-ATA command completion signal received interrupt enable
    #[inline(always)]
    pub fn ceataendie(&mut self) -> CEATAENDIE_W<'_, MASKrs> {
        CEATAENDIE_W::new(self, 23)
    }
}
/**mask register

You can [`read`](crate::Reg::read) this register and get [`mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#SDIO:MASK)*/
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
