#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `CCRCFAILC` reader - CCRCFAIL flag clear bit"]
pub type CCRCFAILC_R = crate::BitReader;
#[doc = "Field `CCRCFAILC` writer - CCRCFAIL flag clear bit"]
pub type CCRCFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCFAILC` reader - DCRCFAIL flag clear bit"]
pub type DCRCFAILC_R = crate::BitReader;
#[doc = "Field `DCRCFAILC` writer - DCRCFAIL flag clear bit"]
pub type DCRCFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIMEOUTC` reader - CTIMEOUT flag clear bit"]
pub type CTIMEOUTC_R = crate::BitReader;
#[doc = "Field `CTIMEOUTC` writer - CTIMEOUT flag clear bit"]
pub type CTIMEOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIMEOUTC` reader - DTIMEOUT flag clear bit"]
pub type DTIMEOUTC_R = crate::BitReader;
#[doc = "Field `DTIMEOUTC` writer - DTIMEOUT flag clear bit"]
pub type DTIMEOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRC` reader - TXUNDERR flag clear bit"]
pub type TXUNDERRC_R = crate::BitReader;
#[doc = "Field `TXUNDERRC` writer - TXUNDERR flag clear bit"]
pub type TXUNDERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRC` reader - RXOVERR flag clear bit"]
pub type RXOVERRC_R = crate::BitReader;
#[doc = "Field `RXOVERRC` writer - RXOVERR flag clear bit"]
pub type RXOVERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRENDC` reader - CMDREND flag clear bit"]
pub type CMDRENDC_R = crate::BitReader;
#[doc = "Field `CMDRENDC` writer - CMDREND flag clear bit"]
pub type CMDRENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSENTC` reader - CMDSENT flag clear bit"]
pub type CMDSENTC_R = crate::BitReader;
#[doc = "Field `CMDSENTC` writer - CMDSENT flag clear bit"]
pub type CMDSENTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDC` reader - DATAEND flag clear bit"]
pub type DATAENDC_R = crate::BitReader;
#[doc = "Field `DATAENDC` writer - DATAEND flag clear bit"]
pub type DATAENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHOLDC` reader - DHOLD flag clear bit"]
pub type DHOLDC_R = crate::BitReader;
#[doc = "Field `DHOLDC` writer - DHOLD flag clear bit"]
pub type DHOLDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCKENDC` reader - DBCKEND flag clear bit"]
pub type DBCKENDC_R = crate::BitReader;
#[doc = "Field `DBCKENDC` writer - DBCKEND flag clear bit"]
pub type DBCKENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DABORTC` reader - DABORT flag clear bit"]
pub type DABORTC_R = crate::BitReader;
#[doc = "Field `DABORTC` writer - DABORT flag clear bit"]
pub type DABORTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSYD0ENDC` reader - BUSYD0END flag clear bit"]
pub type BUSYD0ENDC_R = crate::BitReader;
#[doc = "Field `BUSYD0ENDC` writer - BUSYD0END flag clear bit"]
pub type BUSYD0ENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITC` reader - SDIOIT flag clear bit"]
pub type SDIOITC_R = crate::BitReader;
#[doc = "Field `SDIOITC` writer - SDIOIT flag clear bit"]
pub type SDIOITC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAILC` reader - ACKFAIL flag clear bit"]
pub type ACKFAILC_R = crate::BitReader;
#[doc = "Field `ACKFAILC` writer - ACKFAIL flag clear bit"]
pub type ACKFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKTIMEOUTC` reader - ACKTIMEOUT flag clear bit"]
pub type ACKTIMEOUTC_R = crate::BitReader;
#[doc = "Field `ACKTIMEOUTC` writer - ACKTIMEOUT flag clear bit"]
pub type ACKTIMEOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSWENDC` reader - VSWEND flag clear bit"]
pub type VSWENDC_R = crate::BitReader;
#[doc = "Field `VSWENDC` writer - VSWEND flag clear bit"]
pub type VSWENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSTOPC` reader - CKSTOP flag clear bit"]
pub type CKSTOPC_R = crate::BitReader;
#[doc = "Field `CKSTOPC` writer - CKSTOP flag clear bit"]
pub type CKSTOPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMATEC` reader - IDMA transfer error clear bit"]
pub type IDMATEC_R = crate::BitReader;
#[doc = "Field `IDMATEC` writer - IDMA transfer error clear bit"]
pub type IDMATEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMABTCC` reader - IDMA buffer transfer complete clear bit"]
pub type IDMABTCC_R = crate::BitReader;
#[doc = "Field `IDMABTCC` writer - IDMA buffer transfer complete clear bit"]
pub type IDMABTCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCRCFAIL flag clear bit"]
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CCRCFAILC_R {
        CCRCFAILC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAIL flag clear bit"]
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DCRCFAILC_R {
        DCRCFAILC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUT flag clear bit"]
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CTIMEOUTC_R {
        CTIMEOUTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUT flag clear bit"]
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DTIMEOUTC_R {
        DTIMEOUTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERR flag clear bit"]
    #[inline(always)]
    pub fn txunderrc(&self) -> TXUNDERRC_R {
        TXUNDERRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERR flag clear bit"]
    #[inline(always)]
    pub fn rxoverrc(&self) -> RXOVERRC_R {
        RXOVERRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDREND flag clear bit"]
    #[inline(always)]
    pub fn cmdrendc(&self) -> CMDRENDC_R {
        CMDRENDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENT flag clear bit"]
    #[inline(always)]
    pub fn cmdsentc(&self) -> CMDSENTC_R {
        CMDSENTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAEND flag clear bit"]
    #[inline(always)]
    pub fn dataendc(&self) -> DATAENDC_R {
        DATAENDC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DHOLD flag clear bit"]
    #[inline(always)]
    pub fn dholdc(&self) -> DHOLDC_R {
        DHOLDC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKEND flag clear bit"]
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DABORT flag clear bit"]
    #[inline(always)]
    pub fn dabortc(&self) -> DABORTC_R {
        DABORTC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 21 - BUSYD0END flag clear bit"]
    #[inline(always)]
    pub fn busyd0endc(&self) -> BUSYD0ENDC_R {
        BUSYD0ENDC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOIT flag clear bit"]
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ACKFAIL flag clear bit"]
    #[inline(always)]
    pub fn ackfailc(&self) -> ACKFAILC_R {
        ACKFAILC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ACKTIMEOUT flag clear bit"]
    #[inline(always)]
    pub fn acktimeoutc(&self) -> ACKTIMEOUTC_R {
        ACKTIMEOUTC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VSWEND flag clear bit"]
    #[inline(always)]
    pub fn vswendc(&self) -> VSWENDC_R {
        VSWENDC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CKSTOP flag clear bit"]
    #[inline(always)]
    pub fn ckstopc(&self) -> CKSTOPC_R {
        CKSTOPC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IDMA transfer error clear bit"]
    #[inline(always)]
    pub fn idmatec(&self) -> IDMATEC_R {
        IDMATEC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - IDMA buffer transfer complete clear bit"]
    #[inline(always)]
    pub fn idmabtcc(&self) -> IDMABTCC_R {
        IDMABTCC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAIL flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W<ICRrs> {
        CCRCFAILC_W::new(self, 0)
    }
    #[doc = "Bit 1 - DCRCFAIL flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W<ICRrs> {
        DCRCFAILC_W::new(self, 1)
    }
    #[doc = "Bit 2 - CTIMEOUT flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W<ICRrs> {
        CTIMEOUTC_W::new(self, 2)
    }
    #[doc = "Bit 3 - DTIMEOUT flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W<ICRrs> {
        DTIMEOUTC_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXUNDERR flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W<ICRrs> {
        TXUNDERRC_W::new(self, 4)
    }
    #[doc = "Bit 5 - RXOVERR flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W<ICRrs> {
        RXOVERRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - CMDREND flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W<ICRrs> {
        CMDRENDC_W::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSENT flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W<ICRrs> {
        CMDSENTC_W::new(self, 7)
    }
    #[doc = "Bit 8 - DATAEND flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dataendc(&mut self) -> DATAENDC_W<ICRrs> {
        DATAENDC_W::new(self, 8)
    }
    #[doc = "Bit 9 - DHOLD flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dholdc(&mut self) -> DHOLDC_W<ICRrs> {
        DHOLDC_W::new(self, 9)
    }
    #[doc = "Bit 10 - DBCKEND flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dbckendc(&mut self) -> DBCKENDC_W<ICRrs> {
        DBCKENDC_W::new(self, 10)
    }
    #[doc = "Bit 11 - DABORT flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn dabortc(&mut self) -> DABORTC_W<ICRrs> {
        DABORTC_W::new(self, 11)
    }
    #[doc = "Bit 21 - BUSYD0END flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn busyd0endc(&mut self) -> BUSYD0ENDC_W<ICRrs> {
        BUSYD0ENDC_W::new(self, 21)
    }
    #[doc = "Bit 22 - SDIOIT flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn sdioitc(&mut self) -> SDIOITC_W<ICRrs> {
        SDIOITC_W::new(self, 22)
    }
    #[doc = "Bit 23 - ACKFAIL flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ackfailc(&mut self) -> ACKFAILC_W<ICRrs> {
        ACKFAILC_W::new(self, 23)
    }
    #[doc = "Bit 24 - ACKTIMEOUT flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn acktimeoutc(&mut self) -> ACKTIMEOUTC_W<ICRrs> {
        ACKTIMEOUTC_W::new(self, 24)
    }
    #[doc = "Bit 25 - VSWEND flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn vswendc(&mut self) -> VSWENDC_W<ICRrs> {
        VSWENDC_W::new(self, 25)
    }
    #[doc = "Bit 26 - CKSTOP flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn ckstopc(&mut self) -> CKSTOPC_W<ICRrs> {
        CKSTOPC_W::new(self, 26)
    }
    #[doc = "Bit 27 - IDMA transfer error clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn idmatec(&mut self) -> IDMATEC_W<ICRrs> {
        IDMATEC_W::new(self, 27)
    }
    #[doc = "Bit 28 - IDMA buffer transfer complete clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn idmabtcc(&mut self) -> IDMABTCC_W<ICRrs> {
        IDMABTCC_W::new(self, 28)
    }
}
#[doc = "interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICRrs {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
