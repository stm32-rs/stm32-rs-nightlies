///Register `MASKR` reader
pub type R = crate::R<MASKRrs>;
///Register `MASKR` writer
pub type W = crate::W<MASKRrs>;
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
///Field `DHOLDIE` reader - DHOLDIE
pub type DHOLDIE_R = crate::BitReader;
///Field `DHOLDIE` writer - DHOLDIE
pub type DHOLDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBCKENDIE` reader - DBCKENDIE
pub type DBCKENDIE_R = crate::BitReader;
///Field `DBCKENDIE` writer - DBCKENDIE
pub type DBCKENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DABORTIE` reader - DABORTIE
pub type DABORTIE_R = crate::BitReader;
///Field `DABORTIE` writer - DABORTIE
pub type DABORTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFOHEIE` reader - TXFIFOHEIE
pub type TXFIFOHEIE_R = crate::BitReader;
///Field `TXFIFOHEIE` writer - TXFIFOHEIE
pub type TXFIFOHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFOHFIE` reader - RXFIFOHFIE
pub type RXFIFOHFIE_R = crate::BitReader;
///Field `RXFIFOHFIE` writer - RXFIFOHFIE
pub type RXFIFOHFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFOFIE` reader - RXFIFOFIE
pub type RXFIFOFIE_R = crate::BitReader;
///Field `RXFIFOFIE` writer - RXFIFOFIE
pub type RXFIFOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFOEIE` reader - TXFIFOEIE
pub type TXFIFOEIE_R = crate::BitReader;
///Field `TXFIFOEIE` writer - TXFIFOEIE
pub type TXFIFOEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSYD0ENDIE` reader - BUSYD0ENDIE
pub type BUSYD0ENDIE_R = crate::BitReader;
///Field `BUSYD0ENDIE` writer - BUSYD0ENDIE
pub type BUSYD0ENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIOITIE` reader - SDIOITIE
pub type SDIOITIE_R = crate::BitReader;
///Field `SDIOITIE` writer - SDIOITIE
pub type SDIOITIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACKFAILIE` reader - ACKFAILIE
pub type ACKFAILIE_R = crate::BitReader;
///Field `ACKFAILIE` writer - ACKFAILIE
pub type ACKFAILIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACKTIMEOUTIE` reader - ACKTIMEOUTIE
pub type ACKTIMEOUTIE_R = crate::BitReader;
///Field `ACKTIMEOUTIE` writer - ACKTIMEOUTIE
pub type ACKTIMEOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSWENDIE` reader - VSWENDIE
pub type VSWENDIE_R = crate::BitReader;
///Field `VSWENDIE` writer - VSWENDIE
pub type VSWENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKSTOPIE` reader - CKSTOPIE
pub type CKSTOPIE_R = crate::BitReader;
///Field `CKSTOPIE` writer - CKSTOPIE
pub type CKSTOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDMABTCIE` reader - IDMABTCIE
pub type IDMABTCIE_R = crate::BitReader;
///Field `IDMABTCIE` writer - IDMABTCIE
pub type IDMABTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 9 - DHOLDIE
    #[inline(always)]
    pub fn dholdie(&self) -> DHOLDIE_R {
        DHOLDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBCKENDIE
    #[inline(always)]
    pub fn dbckendie(&self) -> DBCKENDIE_R {
        DBCKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DABORTIE
    #[inline(always)]
    pub fn dabortie(&self) -> DABORTIE_R {
        DABORTIE_R::new(((self.bits >> 11) & 1) != 0)
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
    ///Bit 21 - BUSYD0ENDIE
    #[inline(always)]
    pub fn busyd0endie(&self) -> BUSYD0ENDIE_R {
        BUSYD0ENDIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIOITIE
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ACKFAILIE
    #[inline(always)]
    pub fn ackfailie(&self) -> ACKFAILIE_R {
        ACKFAILIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ACKTIMEOUTIE
    #[inline(always)]
    pub fn acktimeoutie(&self) -> ACKTIMEOUTIE_R {
        ACKTIMEOUTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VSWENDIE
    #[inline(always)]
    pub fn vswendie(&self) -> VSWENDIE_R {
        VSWENDIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CKSTOPIE
    #[inline(always)]
    pub fn ckstopie(&self) -> CKSTOPIE_R {
        CKSTOPIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - IDMABTCIE
    #[inline(always)]
    pub fn idmabtcie(&self) -> IDMABTCIE_R {
        IDMABTCIE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASKR")
            .field("ccrcfailie", &self.ccrcfailie())
            .field("dcrcfailie", &self.dcrcfailie())
            .field("ctimeoutie", &self.ctimeoutie())
            .field("dtimeoutie", &self.dtimeoutie())
            .field("txunderrie", &self.txunderrie())
            .field("rxoverrie", &self.rxoverrie())
            .field("cmdrendie", &self.cmdrendie())
            .field("cmdsentie", &self.cmdsentie())
            .field("dataendie", &self.dataendie())
            .field("dholdie", &self.dholdie())
            .field("dbckendie", &self.dbckendie())
            .field("dabortie", &self.dabortie())
            .field("txfifoheie", &self.txfifoheie())
            .field("rxfifohfie", &self.rxfifohfie())
            .field("rxfifofie", &self.rxfifofie())
            .field("txfifoeie", &self.txfifoeie())
            .field("busyd0endie", &self.busyd0endie())
            .field("sdioitie", &self.sdioitie())
            .field("ackfailie", &self.ackfailie())
            .field("acktimeoutie", &self.acktimeoutie())
            .field("vswendie", &self.vswendie())
            .field("ckstopie", &self.ckstopie())
            .field("idmabtcie", &self.idmabtcie())
            .finish()
    }
}
impl W {
    ///Bit 0 - CCRCFAILIE
    #[inline(always)]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W<'_, MASKRrs> {
        CCRCFAILIE_W::new(self, 0)
    }
    ///Bit 1 - DCRCFAILIE
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W<'_, MASKRrs> {
        DCRCFAILIE_W::new(self, 1)
    }
    ///Bit 2 - CTIMEOUTIE
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W<'_, MASKRrs> {
        CTIMEOUTIE_W::new(self, 2)
    }
    ///Bit 3 - DTIMEOUTIE
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W<'_, MASKRrs> {
        DTIMEOUTIE_W::new(self, 3)
    }
    ///Bit 4 - TXUNDERRIE
    #[inline(always)]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W<'_, MASKRrs> {
        TXUNDERRIE_W::new(self, 4)
    }
    ///Bit 5 - RXOVERRIE
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W<'_, MASKRrs> {
        RXOVERRIE_W::new(self, 5)
    }
    ///Bit 6 - CMDRENDIE
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W<'_, MASKRrs> {
        CMDRENDIE_W::new(self, 6)
    }
    ///Bit 7 - CMDSENTIE
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W<'_, MASKRrs> {
        CMDSENTIE_W::new(self, 7)
    }
    ///Bit 8 - DATAENDIE
    #[inline(always)]
    pub fn dataendie(&mut self) -> DATAENDIE_W<'_, MASKRrs> {
        DATAENDIE_W::new(self, 8)
    }
    ///Bit 9 - DHOLDIE
    #[inline(always)]
    pub fn dholdie(&mut self) -> DHOLDIE_W<'_, MASKRrs> {
        DHOLDIE_W::new(self, 9)
    }
    ///Bit 10 - DBCKENDIE
    #[inline(always)]
    pub fn dbckendie(&mut self) -> DBCKENDIE_W<'_, MASKRrs> {
        DBCKENDIE_W::new(self, 10)
    }
    ///Bit 11 - DABORTIE
    #[inline(always)]
    pub fn dabortie(&mut self) -> DABORTIE_W<'_, MASKRrs> {
        DABORTIE_W::new(self, 11)
    }
    ///Bit 14 - TXFIFOHEIE
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W<'_, MASKRrs> {
        TXFIFOHEIE_W::new(self, 14)
    }
    ///Bit 15 - RXFIFOHFIE
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W<'_, MASKRrs> {
        RXFIFOHFIE_W::new(self, 15)
    }
    ///Bit 17 - RXFIFOFIE
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W<'_, MASKRrs> {
        RXFIFOFIE_W::new(self, 17)
    }
    ///Bit 18 - TXFIFOEIE
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W<'_, MASKRrs> {
        TXFIFOEIE_W::new(self, 18)
    }
    ///Bit 21 - BUSYD0ENDIE
    #[inline(always)]
    pub fn busyd0endie(&mut self) -> BUSYD0ENDIE_W<'_, MASKRrs> {
        BUSYD0ENDIE_W::new(self, 21)
    }
    ///Bit 22 - SDIOITIE
    #[inline(always)]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<'_, MASKRrs> {
        SDIOITIE_W::new(self, 22)
    }
    ///Bit 23 - ACKFAILIE
    #[inline(always)]
    pub fn ackfailie(&mut self) -> ACKFAILIE_W<'_, MASKRrs> {
        ACKFAILIE_W::new(self, 23)
    }
    ///Bit 24 - ACKTIMEOUTIE
    #[inline(always)]
    pub fn acktimeoutie(&mut self) -> ACKTIMEOUTIE_W<'_, MASKRrs> {
        ACKTIMEOUTIE_W::new(self, 24)
    }
    ///Bit 25 - VSWENDIE
    #[inline(always)]
    pub fn vswendie(&mut self) -> VSWENDIE_W<'_, MASKRrs> {
        VSWENDIE_W::new(self, 25)
    }
    ///Bit 26 - CKSTOPIE
    #[inline(always)]
    pub fn ckstopie(&mut self) -> CKSTOPIE_W<'_, MASKRrs> {
        CKSTOPIE_W::new(self, 26)
    }
    ///Bit 28 - IDMABTCIE
    #[inline(always)]
    pub fn idmabtcie(&mut self) -> IDMABTCIE_W<'_, MASKRrs> {
        IDMABTCIE_W::new(self, 28)
    }
}
/**The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.

You can [`read`](crate::Reg::read) this register and get [`maskr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SDMMC1:MASKR)*/
pub struct MASKRrs;
impl crate::RegisterSpec for MASKRrs {
    type Ux = u32;
}
///`read()` method returns [`maskr::R`](R) reader structure
impl crate::Readable for MASKRrs {}
///`write(|w| ..)` method takes [`maskr::W`](W) writer structure
impl crate::Writable for MASKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MASKR to value 0
impl crate::Resettable for MASKRrs {}
