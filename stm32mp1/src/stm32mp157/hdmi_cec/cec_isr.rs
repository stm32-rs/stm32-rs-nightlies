#[doc = "Register `CEC_ISR` reader"]
pub type R = crate::R<CEC_ISRrs>;
#[doc = "Register `CEC_ISR` writer"]
pub type W = crate::W<CEC_ISRrs>;
#[doc = "Field `RXBR` reader - RXBR"]
pub type RXBR_R = crate::BitReader;
#[doc = "Field `RXBR` writer - RXBR"]
pub type RXBR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEND` reader - RXEND"]
pub type RXEND_R = crate::BitReader;
#[doc = "Field `RXEND` writer - RXEND"]
pub type RXEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVR` reader - RXOVR"]
pub type RXOVR_R = crate::BitReader;
#[doc = "Field `RXOVR` writer - RXOVR"]
pub type RXOVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRE` reader - BRE"]
pub type BRE_R = crate::BitReader;
#[doc = "Field `BRE` writer - BRE"]
pub type BRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBPE` reader - SBPE"]
pub type SBPE_R = crate::BitReader;
#[doc = "Field `SBPE` writer - SBPE"]
pub type SBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBPE` reader - LBPE"]
pub type LBPE_R = crate::BitReader;
#[doc = "Field `LBPE` writer - LBPE"]
pub type LBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXACKE` reader - RXACKE"]
pub type RXACKE_R = crate::BitReader;
#[doc = "Field `RXACKE` writer - RXACKE"]
pub type RXACKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLST` reader - ARBLST"]
pub type ARBLST_R = crate::BitReader;
#[doc = "Field `ARBLST` writer - ARBLST"]
pub type ARBLST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBR` reader - TXBR"]
pub type TXBR_R = crate::BitReader;
#[doc = "Field `TXBR` writer - TXBR"]
pub type TXBR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEND` reader - TXEND"]
pub type TXEND_R = crate::BitReader;
#[doc = "Field `TXEND` writer - TXEND"]
pub type TXEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUDR` reader - TXUDR"]
pub type TXUDR_R = crate::BitReader;
#[doc = "Field `TXUDR` writer - TXUDR"]
pub type TXUDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERR` reader - TXERR"]
pub type TXERR_R = crate::BitReader;
#[doc = "Field `TXERR` writer - TXERR"]
pub type TXERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXACKE` reader - TXACKE"]
pub type TXACKE_R = crate::BitReader;
#[doc = "Field `TXACKE` writer - TXACKE"]
pub type TXACKE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXBR"]
    #[inline(always)]
    pub fn rxbr(&self) -> RXBR_R {
        RXBR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXEND"]
    #[inline(always)]
    pub fn rxend(&self) -> RXEND_R {
        RXEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXOVR"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRE"]
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SBPE"]
    #[inline(always)]
    pub fn sbpe(&self) -> SBPE_R {
        SBPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LBPE"]
    #[inline(always)]
    pub fn lbpe(&self) -> LBPE_R {
        LBPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXACKE"]
    #[inline(always)]
    pub fn rxacke(&self) -> RXACKE_R {
        RXACKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ARBLST"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TXBR"]
    #[inline(always)]
    pub fn txbr(&self) -> TXBR_R {
        TXBR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXEND"]
    #[inline(always)]
    pub fn txend(&self) -> TXEND_R {
        TXEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXUDR"]
    #[inline(always)]
    pub fn txudr(&self) -> TXUDR_R {
        TXUDR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXERR"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXACKE"]
    #[inline(always)]
    pub fn txacke(&self) -> TXACKE_R {
        TXACKE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXBR"]
    #[inline(always)]
    #[must_use]
    pub fn rxbr(&mut self) -> RXBR_W<CEC_ISRrs> {
        RXBR_W::new(self, 0)
    }
    #[doc = "Bit 1 - RXEND"]
    #[inline(always)]
    #[must_use]
    pub fn rxend(&mut self) -> RXEND_W<CEC_ISRrs> {
        RXEND_W::new(self, 1)
    }
    #[doc = "Bit 2 - RXOVR"]
    #[inline(always)]
    #[must_use]
    pub fn rxovr(&mut self) -> RXOVR_W<CEC_ISRrs> {
        RXOVR_W::new(self, 2)
    }
    #[doc = "Bit 3 - BRE"]
    #[inline(always)]
    #[must_use]
    pub fn bre(&mut self) -> BRE_W<CEC_ISRrs> {
        BRE_W::new(self, 3)
    }
    #[doc = "Bit 4 - SBPE"]
    #[inline(always)]
    #[must_use]
    pub fn sbpe(&mut self) -> SBPE_W<CEC_ISRrs> {
        SBPE_W::new(self, 4)
    }
    #[doc = "Bit 5 - LBPE"]
    #[inline(always)]
    #[must_use]
    pub fn lbpe(&mut self) -> LBPE_W<CEC_ISRrs> {
        LBPE_W::new(self, 5)
    }
    #[doc = "Bit 6 - RXACKE"]
    #[inline(always)]
    #[must_use]
    pub fn rxacke(&mut self) -> RXACKE_W<CEC_ISRrs> {
        RXACKE_W::new(self, 6)
    }
    #[doc = "Bit 7 - ARBLST"]
    #[inline(always)]
    #[must_use]
    pub fn arblst(&mut self) -> ARBLST_W<CEC_ISRrs> {
        ARBLST_W::new(self, 7)
    }
    #[doc = "Bit 8 - TXBR"]
    #[inline(always)]
    #[must_use]
    pub fn txbr(&mut self) -> TXBR_W<CEC_ISRrs> {
        TXBR_W::new(self, 8)
    }
    #[doc = "Bit 9 - TXEND"]
    #[inline(always)]
    #[must_use]
    pub fn txend(&mut self) -> TXEND_W<CEC_ISRrs> {
        TXEND_W::new(self, 9)
    }
    #[doc = "Bit 10 - TXUDR"]
    #[inline(always)]
    #[must_use]
    pub fn txudr(&mut self) -> TXUDR_W<CEC_ISRrs> {
        TXUDR_W::new(self, 10)
    }
    #[doc = "Bit 11 - TXERR"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<CEC_ISRrs> {
        TXERR_W::new(self, 11)
    }
    #[doc = "Bit 12 - TXACKE"]
    #[inline(always)]
    #[must_use]
    pub fn txacke(&mut self) -> TXACKE_W<CEC_ISRrs> {
        TXACKE_W::new(self, 12)
    }
}
#[doc = "CEC Interrupt and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CEC_ISRrs;
impl crate::RegisterSpec for CEC_ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cec_isr::R`](R) reader structure"]
impl crate::Readable for CEC_ISRrs {}
#[doc = "`write(|w| ..)` method takes [`cec_isr::W`](W) writer structure"]
impl crate::Writable for CEC_ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CEC_ISR to value 0"]
impl crate::Resettable for CEC_ISRrs {
    const RESET_VALUE: u32 = 0;
}
