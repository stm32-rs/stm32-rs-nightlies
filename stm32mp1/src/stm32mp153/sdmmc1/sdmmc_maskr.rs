#[doc = "Register `SDMMC_MASKR` reader"]
pub type R = crate::R<SDMMC_MASKRrs>;
#[doc = "Register `SDMMC_MASKR` writer"]
pub type W = crate::W<SDMMC_MASKRrs>;
#[doc = "Field `CCRCFAILIE` reader - CCRCFAILIE"]
pub type CCRCFAILIE_R = crate::BitReader;
#[doc = "Field `CCRCFAILIE` writer - CCRCFAILIE"]
pub type CCRCFAILIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCFAILIE` reader - DCRCFAILIE"]
pub type DCRCFAILIE_R = crate::BitReader;
#[doc = "Field `DCRCFAILIE` writer - DCRCFAILIE"]
pub type DCRCFAILIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIMEOUTIE` reader - CTIMEOUTIE"]
pub type CTIMEOUTIE_R = crate::BitReader;
#[doc = "Field `CTIMEOUTIE` writer - CTIMEOUTIE"]
pub type CTIMEOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIMEOUTIE` reader - DTIMEOUTIE"]
pub type DTIMEOUTIE_R = crate::BitReader;
#[doc = "Field `DTIMEOUTIE` writer - DTIMEOUTIE"]
pub type DTIMEOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRIE` reader - TXUNDERRIE"]
pub type TXUNDERRIE_R = crate::BitReader;
#[doc = "Field `TXUNDERRIE` writer - TXUNDERRIE"]
pub type TXUNDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRIE` reader - RXOVERRIE"]
pub type RXOVERRIE_R = crate::BitReader;
#[doc = "Field `RXOVERRIE` writer - RXOVERRIE"]
pub type RXOVERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRENDIE` reader - CMDRENDIE"]
pub type CMDRENDIE_R = crate::BitReader;
#[doc = "Field `CMDRENDIE` writer - CMDRENDIE"]
pub type CMDRENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSENTIE` reader - CMDSENTIE"]
pub type CMDSENTIE_R = crate::BitReader;
#[doc = "Field `CMDSENTIE` writer - CMDSENTIE"]
pub type CMDSENTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDIE` reader - DATAENDIE"]
pub type DATAENDIE_R = crate::BitReader;
#[doc = "Field `DATAENDIE` writer - DATAENDIE"]
pub type DATAENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHOLDIE` reader - DHOLDIE"]
pub type DHOLDIE_R = crate::BitReader;
#[doc = "Field `DHOLDIE` writer - DHOLDIE"]
pub type DHOLDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCKENDIE` reader - DBCKENDIE"]
pub type DBCKENDIE_R = crate::BitReader;
#[doc = "Field `DBCKENDIE` writer - DBCKENDIE"]
pub type DBCKENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DABORTIE` reader - DABORTIE"]
pub type DABORTIE_R = crate::BitReader;
#[doc = "Field `DABORTIE` writer - DABORTIE"]
pub type DABORTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOHEIE` reader - TXFIFOHEIE"]
pub type TXFIFOHEIE_R = crate::BitReader;
#[doc = "Field `TXFIFOHEIE` writer - TXFIFOHEIE"]
pub type TXFIFOHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOHFIE` reader - RXFIFOHFIE"]
pub type RXFIFOHFIE_R = crate::BitReader;
#[doc = "Field `RXFIFOHFIE` writer - RXFIFOHFIE"]
pub type RXFIFOHFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOFIE` reader - RXFIFOFIE"]
pub type RXFIFOFIE_R = crate::BitReader;
#[doc = "Field `RXFIFOFIE` writer - RXFIFOFIE"]
pub type RXFIFOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOEIE` reader - TXFIFOEIE"]
pub type TXFIFOEIE_R = crate::BitReader;
#[doc = "Field `TXFIFOEIE` writer - TXFIFOEIE"]
pub type TXFIFOEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSYD0ENDIE` reader - BUSYD0ENDIE"]
pub type BUSYD0ENDIE_R = crate::BitReader;
#[doc = "Field `BUSYD0ENDIE` writer - BUSYD0ENDIE"]
pub type BUSYD0ENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITIE` reader - SDIOITIE"]
pub type SDIOITIE_R = crate::BitReader;
#[doc = "Field `SDIOITIE` writer - SDIOITIE"]
pub type SDIOITIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAILIE` reader - ACKFAILIE"]
pub type ACKFAILIE_R = crate::BitReader;
#[doc = "Field `ACKFAILIE` writer - ACKFAILIE"]
pub type ACKFAILIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKTIMEOUTIE` reader - ACKTIMEOUTIE"]
pub type ACKTIMEOUTIE_R = crate::BitReader;
#[doc = "Field `ACKTIMEOUTIE` writer - ACKTIMEOUTIE"]
pub type ACKTIMEOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSWENDIE` reader - VSWENDIE"]
pub type VSWENDIE_R = crate::BitReader;
#[doc = "Field `VSWENDIE` writer - VSWENDIE"]
pub type VSWENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSTOPIE` reader - CKSTOPIE"]
pub type CKSTOPIE_R = crate::BitReader;
#[doc = "Field `CKSTOPIE` writer - CKSTOPIE"]
pub type CKSTOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMABTCIE` reader - IDMABTCIE"]
pub type IDMABTCIE_R = crate::BitReader;
#[doc = "Field `IDMABTCIE` writer - IDMABTCIE"]
pub type IDMABTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DHOLDIE"]
    #[inline(always)]
    pub fn dholdie(&self) -> DHOLDIE_R {
        DHOLDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKENDIE"]
    #[inline(always)]
    pub fn dbckendie(&self) -> DBCKENDIE_R {
        DBCKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DABORTIE"]
    #[inline(always)]
    pub fn dabortie(&self) -> DABORTIE_R {
        DABORTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - BUSYD0ENDIE"]
    #[inline(always)]
    pub fn busyd0endie(&self) -> BUSYD0ENDIE_R {
        BUSYD0ENDIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ACKFAILIE"]
    #[inline(always)]
    pub fn ackfailie(&self) -> ACKFAILIE_R {
        ACKFAILIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ACKTIMEOUTIE"]
    #[inline(always)]
    pub fn acktimeoutie(&self) -> ACKTIMEOUTIE_R {
        ACKTIMEOUTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VSWENDIE"]
    #[inline(always)]
    pub fn vswendie(&self) -> VSWENDIE_R {
        VSWENDIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CKSTOPIE"]
    #[inline(always)]
    pub fn ckstopie(&self) -> CKSTOPIE_R {
        CKSTOPIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - IDMABTCIE"]
    #[inline(always)]
    pub fn idmabtcie(&self) -> IDMABTCIE_R {
        IDMABTCIE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W<SDMMC_MASKRrs> {
        CCRCFAILIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W<SDMMC_MASKRrs> {
        DCRCFAILIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W<SDMMC_MASKRrs> {
        CTIMEOUTIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W<SDMMC_MASKRrs> {
        DTIMEOUTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W<SDMMC_MASKRrs> {
        TXUNDERRIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W<SDMMC_MASKRrs> {
        RXOVERRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W<SDMMC_MASKRrs> {
        CMDRENDIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W<SDMMC_MASKRrs> {
        CMDSENTIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn dataendie(&mut self) -> DATAENDIE_W<SDMMC_MASKRrs> {
        DATAENDIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - DHOLDIE"]
    #[inline(always)]
    #[must_use]
    pub fn dholdie(&mut self) -> DHOLDIE_W<SDMMC_MASKRrs> {
        DHOLDIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - DBCKENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn dbckendie(&mut self) -> DBCKENDIE_W<SDMMC_MASKRrs> {
        DBCKENDIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - DABORTIE"]
    #[inline(always)]
    #[must_use]
    pub fn dabortie(&mut self) -> DABORTIE_W<SDMMC_MASKRrs> {
        DABORTIE_W::new(self, 11)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W<SDMMC_MASKRrs> {
        TXFIFOHEIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W<SDMMC_MASKRrs> {
        RXFIFOHFIE_W::new(self, 15)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W<SDMMC_MASKRrs> {
        RXFIFOFIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W<SDMMC_MASKRrs> {
        TXFIFOEIE_W::new(self, 18)
    }
    #[doc = "Bit 21 - BUSYD0ENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn busyd0endie(&mut self) -> BUSYD0ENDIE_W<SDMMC_MASKRrs> {
        BUSYD0ENDIE_W::new(self, 21)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    #[must_use]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<SDMMC_MASKRrs> {
        SDIOITIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - ACKFAILIE"]
    #[inline(always)]
    #[must_use]
    pub fn ackfailie(&mut self) -> ACKFAILIE_W<SDMMC_MASKRrs> {
        ACKFAILIE_W::new(self, 23)
    }
    #[doc = "Bit 24 - ACKTIMEOUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn acktimeoutie(&mut self) -> ACKTIMEOUTIE_W<SDMMC_MASKRrs> {
        ACKTIMEOUTIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - VSWENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn vswendie(&mut self) -> VSWENDIE_W<SDMMC_MASKRrs> {
        VSWENDIE_W::new(self, 25)
    }
    #[doc = "Bit 26 - CKSTOPIE"]
    #[inline(always)]
    #[must_use]
    pub fn ckstopie(&mut self) -> CKSTOPIE_W<SDMMC_MASKRrs> {
        CKSTOPIE_W::new(self, 26)
    }
    #[doc = "Bit 28 - IDMABTCIE"]
    #[inline(always)]
    #[must_use]
    pub fn idmabtcie(&mut self) -> IDMABTCIE_W<SDMMC_MASKRrs> {
        IDMABTCIE_W::new(self, 28)
    }
}
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_maskr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_maskr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_MASKRrs;
impl crate::RegisterSpec for SDMMC_MASKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_maskr::R`](R) reader structure"]
impl crate::Readable for SDMMC_MASKRrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_maskr::W`](W) writer structure"]
impl crate::Writable for SDMMC_MASKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_MASKR to value 0"]
impl crate::Resettable for SDMMC_MASKRrs {
    const RESET_VALUE: u32 = 0;
}
