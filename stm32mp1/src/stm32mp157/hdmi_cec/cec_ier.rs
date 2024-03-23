#[doc = "Register `CEC_IER` reader"]
pub type R = crate::R<CEC_IERrs>;
#[doc = "Register `CEC_IER` writer"]
pub type W = crate::W<CEC_IERrs>;
#[doc = "Field `RXBRIE` reader - RXBRIE"]
pub type RXBRIE_R = crate::BitReader;
#[doc = "Field `RXBRIE` writer - RXBRIE"]
pub type RXBRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENDIE` reader - RXENDIE"]
pub type RXENDIE_R = crate::BitReader;
#[doc = "Field `RXENDIE` writer - RXENDIE"]
pub type RXENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVRIE` reader - RXOVRIE"]
pub type RXOVRIE_R = crate::BitReader;
#[doc = "Field `RXOVRIE` writer - RXOVRIE"]
pub type RXOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREIE` reader - BREIE"]
pub type BREIE_R = crate::BitReader;
#[doc = "Field `BREIE` writer - BREIE"]
pub type BREIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBPEIE` reader - SBPEIE"]
pub type SBPEIE_R = crate::BitReader;
#[doc = "Field `SBPEIE` writer - SBPEIE"]
pub type SBPEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBPEIE` reader - LBPEIE"]
pub type LBPEIE_R = crate::BitReader;
#[doc = "Field `LBPEIE` writer - LBPEIE"]
pub type LBPEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXACKIE` reader - RXACKIE"]
pub type RXACKIE_R = crate::BitReader;
#[doc = "Field `RXACKIE` writer - RXACKIE"]
pub type RXACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLSTIE` reader - ARBLSTIE"]
pub type ARBLSTIE_R = crate::BitReader;
#[doc = "Field `ARBLSTIE` writer - ARBLSTIE"]
pub type ARBLSTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBRIE` reader - TXBRIE"]
pub type TXBRIE_R = crate::BitReader;
#[doc = "Field `TXBRIE` writer - TXBRIE"]
pub type TXBRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXENDIE` reader - TXENDIE"]
pub type TXENDIE_R = crate::BitReader;
#[doc = "Field `TXENDIE` writer - TXENDIE"]
pub type TXENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUDRIE` reader - TXUDRIE"]
pub type TXUDRIE_R = crate::BitReader;
#[doc = "Field `TXUDRIE` writer - TXUDRIE"]
pub type TXUDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRIE` reader - TXERRIE"]
pub type TXERRIE_R = crate::BitReader;
#[doc = "Field `TXERRIE` writer - TXERRIE"]
pub type TXERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXACKIE` reader - TXACKIE"]
pub type TXACKIE_R = crate::BitReader;
#[doc = "Field `TXACKIE` writer - TXACKIE"]
pub type TXACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXBRIE"]
    #[inline(always)]
    pub fn rxbrie(&self) -> RXBRIE_R {
        RXBRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXENDIE"]
    #[inline(always)]
    pub fn rxendie(&self) -> RXENDIE_R {
        RXENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXOVRIE"]
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BREIE"]
    #[inline(always)]
    pub fn breie(&self) -> BREIE_R {
        BREIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SBPEIE"]
    #[inline(always)]
    pub fn sbpeie(&self) -> SBPEIE_R {
        SBPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LBPEIE"]
    #[inline(always)]
    pub fn lbpeie(&self) -> LBPEIE_R {
        LBPEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXACKIE"]
    #[inline(always)]
    pub fn rxackie(&self) -> RXACKIE_R {
        RXACKIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ARBLSTIE"]
    #[inline(always)]
    pub fn arblstie(&self) -> ARBLSTIE_R {
        ARBLSTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TXBRIE"]
    #[inline(always)]
    pub fn txbrie(&self) -> TXBRIE_R {
        TXBRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXENDIE"]
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXUDRIE"]
    #[inline(always)]
    pub fn txudrie(&self) -> TXUDRIE_R {
        TXUDRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXERRIE"]
    #[inline(always)]
    pub fn txerrie(&self) -> TXERRIE_R {
        TXERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXACKIE"]
    #[inline(always)]
    pub fn txackie(&self) -> TXACKIE_R {
        TXACKIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXBRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrie(&mut self) -> RXBRIE_W<CEC_IERrs> {
        RXBRIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - RXENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxendie(&mut self) -> RXENDIE_W<CEC_IERrs> {
        RXENDIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - RXOVRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<CEC_IERrs> {
        RXOVRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - BREIE"]
    #[inline(always)]
    #[must_use]
    pub fn breie(&mut self) -> BREIE_W<CEC_IERrs> {
        BREIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - SBPEIE"]
    #[inline(always)]
    #[must_use]
    pub fn sbpeie(&mut self) -> SBPEIE_W<CEC_IERrs> {
        SBPEIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - LBPEIE"]
    #[inline(always)]
    #[must_use]
    pub fn lbpeie(&mut self) -> LBPEIE_W<CEC_IERrs> {
        LBPEIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - RXACKIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxackie(&mut self) -> RXACKIE_W<CEC_IERrs> {
        RXACKIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - ARBLSTIE"]
    #[inline(always)]
    #[must_use]
    pub fn arblstie(&mut self) -> ARBLSTIE_W<CEC_IERrs> {
        ARBLSTIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - TXBRIE"]
    #[inline(always)]
    #[must_use]
    pub fn txbrie(&mut self) -> TXBRIE_W<CEC_IERrs> {
        TXBRIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - TXENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn txendie(&mut self) -> TXENDIE_W<CEC_IERrs> {
        TXENDIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - TXUDRIE"]
    #[inline(always)]
    #[must_use]
    pub fn txudrie(&mut self) -> TXUDRIE_W<CEC_IERrs> {
        TXUDRIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - TXERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn txerrie(&mut self) -> TXERRIE_W<CEC_IERrs> {
        TXERRIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - TXACKIE"]
    #[inline(always)]
    #[must_use]
    pub fn txackie(&mut self) -> TXACKIE_W<CEC_IERrs> {
        TXACKIE_W::new(self, 12)
    }
}
#[doc = "CEC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cec_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CEC_IERrs;
impl crate::RegisterSpec for CEC_IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cec_ier::R`](R) reader structure"]
impl crate::Readable for CEC_IERrs {}
#[doc = "`write(|w| ..)` method takes [`cec_ier::W`](W) writer structure"]
impl crate::Writable for CEC_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CEC_IER to value 0"]
impl crate::Resettable for CEC_IERrs {
    const RESET_VALUE: u32 = 0;
}
