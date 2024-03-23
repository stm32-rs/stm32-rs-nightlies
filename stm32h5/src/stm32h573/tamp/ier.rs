#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `TAMP1IE` reader - Tamper 1 interrupt enable"]
pub type TAMP1IE_R = crate::BitReader;
#[doc = "Field `TAMP1IE` writer - Tamper 1 interrupt enable"]
pub type TAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2IE` reader - Tamper 2 interrupt enable"]
pub type TAMP2IE_R = crate::BitReader;
#[doc = "Field `TAMP2IE` writer - Tamper 2 interrupt enable"]
pub type TAMP2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3IE` reader - Tamper 3 interrupt enable"]
pub type TAMP3IE_R = crate::BitReader;
#[doc = "Field `TAMP3IE` writer - Tamper 3 interrupt enable"]
pub type TAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP4IE` reader - Tamper 4 interrupt enable"]
pub type TAMP4IE_R = crate::BitReader;
#[doc = "Field `TAMP4IE` writer - Tamper 4 interrupt enable"]
pub type TAMP4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP5IE` reader - Tamper 5 interrupt enable"]
pub type TAMP5IE_R = crate::BitReader;
#[doc = "Field `TAMP5IE` writer - Tamper 5 interrupt enable"]
pub type TAMP5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP6IE` reader - Tamper 6 interrupt enable"]
pub type TAMP6IE_R = crate::BitReader;
#[doc = "Field `TAMP6IE` writer - Tamper 6 interrupt enable"]
pub type TAMP6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP7IE` reader - Tamper 7interrupt enable"]
pub type TAMP7IE_R = crate::BitReader;
#[doc = "Field `TAMP7IE` writer - Tamper 7interrupt enable"]
pub type TAMP7IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP8IE` reader - Tamper 8 interrupt enable"]
pub type TAMP8IE_R = crate::BitReader;
#[doc = "Field `TAMP8IE` writer - Tamper 8 interrupt enable"]
pub type TAMP8IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP1IE` reader - Internal tamper 1 interrupt enable"]
pub type ITAMP1IE_R = crate::BitReader;
#[doc = "Field `ITAMP1IE` writer - Internal tamper 1 interrupt enable"]
pub type ITAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP2IE` reader - Internal tamper 2 interrupt enable"]
pub type ITAMP2IE_R = crate::BitReader;
#[doc = "Field `ITAMP2IE` writer - Internal tamper 2 interrupt enable"]
pub type ITAMP2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP3IE` reader - Internal tamper 3 interrupt enable"]
pub type ITAMP3IE_R = crate::BitReader;
#[doc = "Field `ITAMP3IE` writer - Internal tamper 3 interrupt enable"]
pub type ITAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP4IE` reader - Internal tamper 4 interrupt enable"]
pub type ITAMP4IE_R = crate::BitReader;
#[doc = "Field `ITAMP4IE` writer - Internal tamper 4 interrupt enable"]
pub type ITAMP4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP5IE` reader - Internal tamper 5 interrupt enable"]
pub type ITAMP5IE_R = crate::BitReader;
#[doc = "Field `ITAMP5IE` writer - Internal tamper 5 interrupt enable"]
pub type ITAMP5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP6IE` reader - Internal tamper 6 interrupt enable"]
pub type ITAMP6IE_R = crate::BitReader;
#[doc = "Field `ITAMP6IE` writer - Internal tamper 6 interrupt enable"]
pub type ITAMP6IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP7IE` reader - Internal tamper 7 interrupt enable"]
pub type ITAMP7IE_R = crate::BitReader;
#[doc = "Field `ITAMP7IE` writer - Internal tamper 7 interrupt enable"]
pub type ITAMP7IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP8IE` reader - Internal tamper 8 interrupt enable"]
pub type ITAMP8IE_R = crate::BitReader;
#[doc = "Field `ITAMP8IE` writer - Internal tamper 8 interrupt enable"]
pub type ITAMP8IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP9IE` reader - Internal tamper 9 interrupt enable"]
pub type ITAMP9IE_R = crate::BitReader;
#[doc = "Field `ITAMP9IE` writer - Internal tamper 9 interrupt enable"]
pub type ITAMP9IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP11IE` reader - Internal tamper 11 interrupt enable"]
pub type ITAMP11IE_R = crate::BitReader;
#[doc = "Field `ITAMP11IE` writer - Internal tamper 11 interrupt enable"]
pub type ITAMP11IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP12IE` reader - Internal tamper 12 interrupt enable"]
pub type ITAMP12IE_R = crate::BitReader;
#[doc = "Field `ITAMP12IE` writer - Internal tamper 12 interrupt enable"]
pub type ITAMP12IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP13IE` reader - Internal tamper 13 interrupt enable"]
pub type ITAMP13IE_R = crate::BitReader;
#[doc = "Field `ITAMP13IE` writer - Internal tamper 13 interrupt enable"]
pub type ITAMP13IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP15IE` reader - Internal tamper 15 interrupt enable"]
pub type ITAMP15IE_R = crate::BitReader;
#[doc = "Field `ITAMP15IE` writer - Internal tamper 15 interrupt enable"]
pub type ITAMP15IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 4 interrupt enable"]
    #[inline(always)]
    pub fn tamp4ie(&self) -> TAMP4IE_R {
        TAMP4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper 5 interrupt enable"]
    #[inline(always)]
    pub fn tamp5ie(&self) -> TAMP5IE_R {
        TAMP5IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tamper 6 interrupt enable"]
    #[inline(always)]
    pub fn tamp6ie(&self) -> TAMP6IE_R {
        TAMP6IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tamper 7interrupt enable"]
    #[inline(always)]
    pub fn tamp7ie(&self) -> TAMP7IE_R {
        TAMP7IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper 8 interrupt enable"]
    #[inline(always)]
    pub fn tamp8ie(&self) -> TAMP8IE_R {
        TAMP8IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Internal tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn itamp1ie(&self) -> ITAMP1IE_R {
        ITAMP1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn itamp2ie(&self) -> ITAMP2IE_R {
        ITAMP2IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Internal tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal tamper 4 interrupt enable"]
    #[inline(always)]
    pub fn itamp4ie(&self) -> ITAMP4IE_R {
        ITAMP4IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal tamper 5 interrupt enable"]
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Internal tamper 6 interrupt enable"]
    #[inline(always)]
    pub fn itamp6ie(&self) -> ITAMP6IE_R {
        ITAMP6IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Internal tamper 7 interrupt enable"]
    #[inline(always)]
    pub fn itamp7ie(&self) -> ITAMP7IE_R {
        ITAMP7IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Internal tamper 8 interrupt enable"]
    #[inline(always)]
    pub fn itamp8ie(&self) -> ITAMP8IE_R {
        ITAMP8IE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Internal tamper 9 interrupt enable"]
    #[inline(always)]
    pub fn itamp9ie(&self) -> ITAMP9IE_R {
        ITAMP9IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Internal tamper 11 interrupt enable"]
    #[inline(always)]
    pub fn itamp11ie(&self) -> ITAMP11IE_R {
        ITAMP11IE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Internal tamper 12 interrupt enable"]
    #[inline(always)]
    pub fn itamp12ie(&self) -> ITAMP12IE_R {
        ITAMP12IE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Internal tamper 13 interrupt enable"]
    #[inline(always)]
    pub fn itamp13ie(&self) -> ITAMP13IE_R {
        ITAMP13IE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Internal tamper 15 interrupt enable"]
    #[inline(always)]
    pub fn itamp15ie(&self) -> ITAMP15IE_R {
        ITAMP15IE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<IERrs> {
        TAMP1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<IERrs> {
        TAMP2IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper 3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W<IERrs> {
        TAMP3IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper 4 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp4ie(&mut self) -> TAMP4IE_W<IERrs> {
        TAMP4IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Tamper 5 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp5ie(&mut self) -> TAMP5IE_W<IERrs> {
        TAMP5IE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Tamper 6 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp6ie(&mut self) -> TAMP6IE_W<IERrs> {
        TAMP6IE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Tamper 7interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp7ie(&mut self) -> TAMP7IE_W<IERrs> {
        TAMP7IE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Tamper 8 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp8ie(&mut self) -> TAMP8IE_W<IERrs> {
        TAMP8IE_W::new(self, 7)
    }
    #[doc = "Bit 16 - Internal tamper 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp1ie(&mut self) -> ITAMP1IE_W<IERrs> {
        ITAMP1IE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Internal tamper 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp2ie(&mut self) -> ITAMP2IE_W<IERrs> {
        ITAMP2IE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Internal tamper 3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W<IERrs> {
        ITAMP3IE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Internal tamper 4 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp4ie(&mut self) -> ITAMP4IE_W<IERrs> {
        ITAMP4IE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Internal tamper 5 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W<IERrs> {
        ITAMP5IE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Internal tamper 6 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6ie(&mut self) -> ITAMP6IE_W<IERrs> {
        ITAMP6IE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Internal tamper 7 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp7ie(&mut self) -> ITAMP7IE_W<IERrs> {
        ITAMP7IE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Internal tamper 8 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp8ie(&mut self) -> ITAMP8IE_W<IERrs> {
        ITAMP8IE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Internal tamper 9 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp9ie(&mut self) -> ITAMP9IE_W<IERrs> {
        ITAMP9IE_W::new(self, 24)
    }
    #[doc = "Bit 26 - Internal tamper 11 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp11ie(&mut self) -> ITAMP11IE_W<IERrs> {
        ITAMP11IE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Internal tamper 12 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp12ie(&mut self) -> ITAMP12IE_W<IERrs> {
        ITAMP12IE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Internal tamper 13 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp13ie(&mut self) -> ITAMP13IE_W<IERrs> {
        ITAMP13IE_W::new(self, 28)
    }
    #[doc = "Bit 30 - Internal tamper 15 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp15ie(&mut self) -> ITAMP15IE_W<IERrs> {
        ITAMP15IE_W::new(self, 30)
    }
}
#[doc = "TAMP interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
