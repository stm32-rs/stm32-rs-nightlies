#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `TXMODE` reader - TXMODE"]
pub type TXMODE_R = crate::FieldReader;
#[doc = "Field `TXMODE` writer - TXMODE"]
pub type TXMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXSEND` reader - TXSEND"]
pub type TXSEND_R = crate::BitReader;
#[doc = "Field `TXSEND` writer - TXSEND"]
pub type TXSEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXHRST` reader - TXHRST"]
pub type TXHRST_R = crate::BitReader;
#[doc = "Field `TXHRST` writer - TXHRST"]
pub type TXHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMODE` reader - RXMODE"]
pub type RXMODE_R = crate::BitReader;
#[doc = "Field `RXMODE` writer - RXMODE"]
pub type RXMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYRXEN` reader - PHYRXEN"]
pub type PHYRXEN_R = crate::BitReader;
#[doc = "Field `PHYRXEN` writer - PHYRXEN"]
pub type PHYRXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYCCSEL` reader - PHYCCSEL"]
pub type PHYCCSEL_R = crate::BitReader;
#[doc = "Field `PHYCCSEL` writer - PHYCCSEL"]
pub type PHYCCSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANASUBMODE` reader - ANASUBMODE"]
pub type ANASUBMODE_R = crate::FieldReader;
#[doc = "Field `ANASUBMODE` writer - ANASUBMODE"]
pub type ANASUBMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANAMODE` reader - ANAMODE"]
pub type ANAMODE_R = crate::BitReader;
#[doc = "Field `ANAMODE` writer - ANAMODE"]
pub type ANAMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCENABLE` reader - CCENABLE"]
pub type CCENABLE_R = crate::FieldReader;
#[doc = "Field `CCENABLE` writer - CCENABLE"]
pub type CCENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FRSRXEN` reader - FRSRXEN"]
pub type FRSRXEN_R = crate::BitReader;
#[doc = "Field `FRSRXEN` writer - FRSRXEN"]
pub type FRSRXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRSTX` reader - FRSTX"]
pub type FRSTX_R = crate::BitReader;
#[doc = "Field `FRSTX` writer - FRSTX"]
pub type FRSTX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDCH` reader - RDCH"]
pub type RDCH_R = crate::BitReader;
#[doc = "Field `RDCH` writer - RDCH"]
pub type RDCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1TCDIS` reader - CC1TCDIS"]
pub type CC1TCDIS_R = crate::BitReader;
#[doc = "Field `CC1TCDIS` writer - CC1TCDIS"]
pub type CC1TCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2TCDIS` reader - CC2TCDIS"]
pub type CC2TCDIS_R = crate::BitReader;
#[doc = "Field `CC2TCDIS` writer - CC2TCDIS"]
pub type CC2TCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - TXMODE"]
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - TXSEND"]
    #[inline(always)]
    pub fn txsend(&self) -> TXSEND_R {
        TXSEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXHRST"]
    #[inline(always)]
    pub fn txhrst(&self) -> TXHRST_R {
        TXHRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXMODE"]
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PHYRXEN"]
    #[inline(always)]
    pub fn phyrxen(&self) -> PHYRXEN_R {
        PHYRXEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PHYCCSEL"]
    #[inline(always)]
    pub fn phyccsel(&self) -> PHYCCSEL_R {
        PHYCCSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - ANASUBMODE"]
    #[inline(always)]
    pub fn anasubmode(&self) -> ANASUBMODE_R {
        ANASUBMODE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - ANAMODE"]
    #[inline(always)]
    pub fn anamode(&self) -> ANAMODE_R {
        ANAMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - CCENABLE"]
    #[inline(always)]
    pub fn ccenable(&self) -> CCENABLE_R {
        CCENABLE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - FRSRXEN"]
    #[inline(always)]
    pub fn frsrxen(&self) -> FRSRXEN_R {
        FRSRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FRSTX"]
    #[inline(always)]
    pub fn frstx(&self) -> FRSTX_R {
        FRSTX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RDCH"]
    #[inline(always)]
    pub fn rdch(&self) -> RDCH_R {
        RDCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CC1TCDIS"]
    #[inline(always)]
    pub fn cc1tcdis(&self) -> CC1TCDIS_R {
        CC1TCDIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CC2TCDIS"]
    #[inline(always)]
    pub fn cc2tcdis(&self) -> CC2TCDIS_R {
        CC2TCDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - TXMODE"]
    #[inline(always)]
    #[must_use]
    pub fn txmode(&mut self) -> TXMODE_W<CRrs> {
        TXMODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - TXSEND"]
    #[inline(always)]
    #[must_use]
    pub fn txsend(&mut self) -> TXSEND_W<CRrs> {
        TXSEND_W::new(self, 2)
    }
    #[doc = "Bit 3 - TXHRST"]
    #[inline(always)]
    #[must_use]
    pub fn txhrst(&mut self) -> TXHRST_W<CRrs> {
        TXHRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - RXMODE"]
    #[inline(always)]
    #[must_use]
    pub fn rxmode(&mut self) -> RXMODE_W<CRrs> {
        RXMODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - PHYRXEN"]
    #[inline(always)]
    #[must_use]
    pub fn phyrxen(&mut self) -> PHYRXEN_W<CRrs> {
        PHYRXEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - PHYCCSEL"]
    #[inline(always)]
    #[must_use]
    pub fn phyccsel(&mut self) -> PHYCCSEL_W<CRrs> {
        PHYCCSEL_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - ANASUBMODE"]
    #[inline(always)]
    #[must_use]
    pub fn anasubmode(&mut self) -> ANASUBMODE_W<CRrs> {
        ANASUBMODE_W::new(self, 7)
    }
    #[doc = "Bit 9 - ANAMODE"]
    #[inline(always)]
    #[must_use]
    pub fn anamode(&mut self) -> ANAMODE_W<CRrs> {
        ANAMODE_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - CCENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn ccenable(&mut self) -> CCENABLE_W<CRrs> {
        CCENABLE_W::new(self, 10)
    }
    #[doc = "Bit 16 - FRSRXEN"]
    #[inline(always)]
    #[must_use]
    pub fn frsrxen(&mut self) -> FRSRXEN_W<CRrs> {
        FRSRXEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - FRSTX"]
    #[inline(always)]
    #[must_use]
    pub fn frstx(&mut self) -> FRSTX_W<CRrs> {
        FRSTX_W::new(self, 17)
    }
    #[doc = "Bit 18 - RDCH"]
    #[inline(always)]
    #[must_use]
    pub fn rdch(&mut self) -> RDCH_W<CRrs> {
        RDCH_W::new(self, 18)
    }
    #[doc = "Bit 20 - CC1TCDIS"]
    #[inline(always)]
    #[must_use]
    pub fn cc1tcdis(&mut self) -> CC1TCDIS_W<CRrs> {
        CC1TCDIS_W::new(self, 20)
    }
    #[doc = "Bit 21 - CC2TCDIS"]
    #[inline(always)]
    #[must_use]
    pub fn cc2tcdis(&mut self) -> CC2TCDIS_W<CRrs> {
        CC2TCDIS_W::new(self, 21)
    }
}
#[doc = "UCPD configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
