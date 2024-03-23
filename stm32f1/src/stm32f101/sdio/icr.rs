#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
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
#[doc = "Field `STBITERRC` reader - STBITERRC"]
pub type STBITERRC_R = crate::BitReader;
#[doc = "Field `STBITERRC` writer - STBITERRC"]
pub type STBITERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCKENDC` reader - DBCKENDC"]
pub type DBCKENDC_R = crate::BitReader;
#[doc = "Field `DBCKENDC` writer - DBCKENDC"]
pub type DBCKENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITC` reader - SDIOITC"]
pub type SDIOITC_R = crate::BitReader;
#[doc = "Field `SDIOITC` writer - SDIOITC"]
pub type SDIOITC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEATAENDC` reader - CEATAENDC"]
pub type CEATAENDC_R = crate::BitReader;
#[doc = "Field `CEATAENDC` writer - CEATAENDC"]
pub type CEATAENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 9 - STBITERRC"]
    #[inline(always)]
    pub fn stbiterrc(&self) -> STBITERRC_R {
        STBITERRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CEATAENDC"]
    #[inline(always)]
    pub fn ceataendc(&self) -> CEATAENDC_R {
        CEATAENDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAILC"]
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W<ICRrs> {
        CCRCFAILC_W::new(self, 0)
    }
    #[doc = "Bit 1 - DCRCFAILC"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W<ICRrs> {
        DCRCFAILC_W::new(self, 1)
    }
    #[doc = "Bit 2 - CTIMEOUTC"]
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W<ICRrs> {
        CTIMEOUTC_W::new(self, 2)
    }
    #[doc = "Bit 3 - DTIMEOUTC"]
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W<ICRrs> {
        DTIMEOUTC_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXUNDERRC"]
    #[inline(always)]
    #[must_use]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W<ICRrs> {
        TXUNDERRC_W::new(self, 4)
    }
    #[doc = "Bit 5 - RXOVERRC"]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W<ICRrs> {
        RXOVERRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - CMDRENDC"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W<ICRrs> {
        CMDRENDC_W::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSENTC"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W<ICRrs> {
        CMDSENTC_W::new(self, 7)
    }
    #[doc = "Bit 8 - DATAENDC"]
    #[inline(always)]
    #[must_use]
    pub fn dataendc(&mut self) -> DATAENDC_W<ICRrs> {
        DATAENDC_W::new(self, 8)
    }
    #[doc = "Bit 9 - STBITERRC"]
    #[inline(always)]
    #[must_use]
    pub fn stbiterrc(&mut self) -> STBITERRC_W<ICRrs> {
        STBITERRC_W::new(self, 9)
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    #[must_use]
    pub fn dbckendc(&mut self) -> DBCKENDC_W<ICRrs> {
        DBCKENDC_W::new(self, 10)
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    #[must_use]
    pub fn sdioitc(&mut self) -> SDIOITC_W<ICRrs> {
        SDIOITC_W::new(self, 22)
    }
    #[doc = "Bit 23 - CEATAENDC"]
    #[inline(always)]
    #[must_use]
    pub fn ceataendc(&mut self) -> CEATAENDC_W<ICRrs> {
        CEATAENDC_W::new(self, 23)
    }
}
#[doc = "SDIO interrupt clear register (SDIO_ICR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
