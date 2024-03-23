#[doc = "Register `SDMMC_ICR` reader"]
pub type R = crate::R<SDMMC_ICRrs>;
#[doc = "Register `SDMMC_ICR` writer"]
pub type W = crate::W<SDMMC_ICRrs>;
#[doc = "Field `CCRCFAILC` reader - CCRCFAILC"]
pub type CCRCFAILC_R = crate::BitReader;
#[doc = "Field `CCRCFAILC` writer - CCRCFAILC"]
pub type CCRCFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCFAILC` reader - DCRCFAILC"]
pub type DCRCFAILC_R = crate::BitReader;
#[doc = "Field `DCRCFAILC` writer - DCRCFAILC"]
pub type DCRCFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIMEOUTC` reader - CTIMEOUTC"]
pub type CTIMEOUTC_R = crate::BitReader;
#[doc = "Field `CTIMEOUTC` writer - CTIMEOUTC"]
pub type CTIMEOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIMEOUTC` reader - DTIMEOUTC"]
pub type DTIMEOUTC_R = crate::BitReader;
#[doc = "Field `DTIMEOUTC` writer - DTIMEOUTC"]
pub type DTIMEOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRC` reader - TXUNDERRC"]
pub type TXUNDERRC_R = crate::BitReader;
#[doc = "Field `TXUNDERRC` writer - TXUNDERRC"]
pub type TXUNDERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRC` reader - RXOVERRC"]
pub type RXOVERRC_R = crate::BitReader;
#[doc = "Field `RXOVERRC` writer - RXOVERRC"]
pub type RXOVERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRENDC` reader - CMDRENDC"]
pub type CMDRENDC_R = crate::BitReader;
#[doc = "Field `CMDRENDC` writer - CMDRENDC"]
pub type CMDRENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSENTC` reader - CMDSENTC"]
pub type CMDSENTC_R = crate::BitReader;
#[doc = "Field `CMDSENTC` writer - CMDSENTC"]
pub type CMDSENTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDC` reader - DATAENDC"]
pub type DATAENDC_R = crate::BitReader;
#[doc = "Field `DATAENDC` writer - DATAENDC"]
pub type DATAENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHOLDC` reader - DHOLDC"]
pub type DHOLDC_R = crate::BitReader;
#[doc = "Field `DHOLDC` writer - DHOLDC"]
pub type DHOLDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCKENDC` reader - DBCKENDC"]
pub type DBCKENDC_R = crate::BitReader;
#[doc = "Field `DBCKENDC` writer - DBCKENDC"]
pub type DBCKENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DABORTC` reader - DABORTC"]
pub type DABORTC_R = crate::BitReader;
#[doc = "Field `DABORTC` writer - DABORTC"]
pub type DABORTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSYD0ENDC` reader - BUSYD0ENDC"]
pub type BUSYD0ENDC_R = crate::BitReader;
#[doc = "Field `BUSYD0ENDC` writer - BUSYD0ENDC"]
pub type BUSYD0ENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITC` reader - SDIOITC"]
pub type SDIOITC_R = crate::BitReader;
#[doc = "Field `SDIOITC` writer - SDIOITC"]
pub type SDIOITC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAILC` reader - ACKFAILC"]
pub type ACKFAILC_R = crate::BitReader;
#[doc = "Field `ACKFAILC` writer - ACKFAILC"]
pub type ACKFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKTIMEOUTC` reader - ACKTIMEOUTC"]
pub type ACKTIMEOUTC_R = crate::BitReader;
#[doc = "Field `ACKTIMEOUTC` writer - ACKTIMEOUTC"]
pub type ACKTIMEOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSWENDC` reader - VSWENDC"]
pub type VSWENDC_R = crate::BitReader;
#[doc = "Field `VSWENDC` writer - VSWENDC"]
pub type VSWENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSTOPC` reader - CKSTOPC"]
pub type CKSTOPC_R = crate::BitReader;
#[doc = "Field `CKSTOPC` writer - CKSTOPC"]
pub type CKSTOPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMATEC` reader - IDMATEC"]
pub type IDMATEC_R = crate::BitReader;
#[doc = "Field `IDMATEC` writer - IDMATEC"]
pub type IDMATEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMABTCC` reader - IDMABTCC"]
pub type IDMABTCC_R = crate::BitReader;
#[doc = "Field `IDMABTCC` writer - IDMABTCC"]
pub type IDMABTCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCRCFAILC"]
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CCRCFAILC_R {
        CCRCFAILC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAILC"]
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DCRCFAILC_R {
        DCRCFAILC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUTC"]
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CTIMEOUTC_R {
        CTIMEOUTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUTC"]
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DTIMEOUTC_R {
        DTIMEOUTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERRC"]
    #[inline(always)]
    pub fn txunderrc(&self) -> TXUNDERRC_R {
        TXUNDERRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERRC"]
    #[inline(always)]
    pub fn rxoverrc(&self) -> RXOVERRC_R {
        RXOVERRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDRENDC"]
    #[inline(always)]
    pub fn cmdrendc(&self) -> CMDRENDC_R {
        CMDRENDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENTC"]
    #[inline(always)]
    pub fn cmdsentc(&self) -> CMDSENTC_R {
        CMDSENTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAENDC"]
    #[inline(always)]
    pub fn dataendc(&self) -> DATAENDC_R {
        DATAENDC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DHOLDC"]
    #[inline(always)]
    pub fn dholdc(&self) -> DHOLDC_R {
        DHOLDC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DABORTC"]
    #[inline(always)]
    pub fn dabortc(&self) -> DABORTC_R {
        DABORTC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 21 - BUSYD0ENDC"]
    #[inline(always)]
    pub fn busyd0endc(&self) -> BUSYD0ENDC_R {
        BUSYD0ENDC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ACKFAILC"]
    #[inline(always)]
    pub fn ackfailc(&self) -> ACKFAILC_R {
        ACKFAILC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ACKTIMEOUTC"]
    #[inline(always)]
    pub fn acktimeoutc(&self) -> ACKTIMEOUTC_R {
        ACKTIMEOUTC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VSWENDC"]
    #[inline(always)]
    pub fn vswendc(&self) -> VSWENDC_R {
        VSWENDC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CKSTOPC"]
    #[inline(always)]
    pub fn ckstopc(&self) -> CKSTOPC_R {
        CKSTOPC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IDMATEC"]
    #[inline(always)]
    pub fn idmatec(&self) -> IDMATEC_R {
        IDMATEC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - IDMABTCC"]
    #[inline(always)]
    pub fn idmabtcc(&self) -> IDMABTCC_R {
        IDMABTCC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAILC"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W<SDMMC_ICRrs> {
        CCRCFAILC_W::new(self, 0)
    }
    #[doc = "Bit 1 - DCRCFAILC"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W<SDMMC_ICRrs> {
        DCRCFAILC_W::new(self, 1)
    }
    #[doc = "Bit 2 - CTIMEOUTC"]
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W<SDMMC_ICRrs> {
        CTIMEOUTC_W::new(self, 2)
    }
    #[doc = "Bit 3 - DTIMEOUTC"]
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W<SDMMC_ICRrs> {
        DTIMEOUTC_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXUNDERRC"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W<SDMMC_ICRrs> {
        TXUNDERRC_W::new(self, 4)
    }
    #[doc = "Bit 5 - RXOVERRC"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W<SDMMC_ICRrs> {
        RXOVERRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - CMDRENDC"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W<SDMMC_ICRrs> {
        CMDRENDC_W::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSENTC"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W<SDMMC_ICRrs> {
        CMDSENTC_W::new(self, 7)
    }
    #[doc = "Bit 8 - DATAENDC"]
    #[inline(always)]
    #[must_use]
    pub fn dataendc(&mut self) -> DATAENDC_W<SDMMC_ICRrs> {
        DATAENDC_W::new(self, 8)
    }
    #[doc = "Bit 9 - DHOLDC"]
    #[inline(always)]
    #[must_use]
    pub fn dholdc(&mut self) -> DHOLDC_W<SDMMC_ICRrs> {
        DHOLDC_W::new(self, 9)
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    #[must_use]
    pub fn dbckendc(&mut self) -> DBCKENDC_W<SDMMC_ICRrs> {
        DBCKENDC_W::new(self, 10)
    }
    #[doc = "Bit 11 - DABORTC"]
    #[inline(always)]
    #[must_use]
    pub fn dabortc(&mut self) -> DABORTC_W<SDMMC_ICRrs> {
        DABORTC_W::new(self, 11)
    }
    #[doc = "Bit 21 - BUSYD0ENDC"]
    #[inline(always)]
    #[must_use]
    pub fn busyd0endc(&mut self) -> BUSYD0ENDC_W<SDMMC_ICRrs> {
        BUSYD0ENDC_W::new(self, 21)
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    #[must_use]
    pub fn sdioitc(&mut self) -> SDIOITC_W<SDMMC_ICRrs> {
        SDIOITC_W::new(self, 22)
    }
    #[doc = "Bit 23 - ACKFAILC"]
    #[inline(always)]
    #[must_use]
    pub fn ackfailc(&mut self) -> ACKFAILC_W<SDMMC_ICRrs> {
        ACKFAILC_W::new(self, 23)
    }
    #[doc = "Bit 24 - ACKTIMEOUTC"]
    #[inline(always)]
    #[must_use]
    pub fn acktimeoutc(&mut self) -> ACKTIMEOUTC_W<SDMMC_ICRrs> {
        ACKTIMEOUTC_W::new(self, 24)
    }
    #[doc = "Bit 25 - VSWENDC"]
    #[inline(always)]
    #[must_use]
    pub fn vswendc(&mut self) -> VSWENDC_W<SDMMC_ICRrs> {
        VSWENDC_W::new(self, 25)
    }
    #[doc = "Bit 26 - CKSTOPC"]
    #[inline(always)]
    #[must_use]
    pub fn ckstopc(&mut self) -> CKSTOPC_W<SDMMC_ICRrs> {
        CKSTOPC_W::new(self, 26)
    }
    #[doc = "Bit 27 - IDMATEC"]
    #[inline(always)]
    #[must_use]
    pub fn idmatec(&mut self) -> IDMATEC_W<SDMMC_ICRrs> {
        IDMATEC_W::new(self, 27)
    }
    #[doc = "Bit 28 - IDMABTCC"]
    #[inline(always)]
    #[must_use]
    pub fn idmabtcc(&mut self) -> IDMABTCC_W<SDMMC_ICRrs> {
        IDMABTCC_W::new(self, 28)
    }
}
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_ICRrs;
impl crate::RegisterSpec for SDMMC_ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_icr::R`](R) reader structure"]
impl crate::Readable for SDMMC_ICRrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_icr::W`](W) writer structure"]
impl crate::Writable for SDMMC_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_ICR to value 0"]
impl crate::Resettable for SDMMC_ICRrs {
    const RESET_VALUE: u32 = 0;
}
