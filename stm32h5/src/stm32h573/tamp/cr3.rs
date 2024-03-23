#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "Field `ITAMP1NOER` reader - Internal Tamper 1 no erase"]
pub type ITAMP1NOER_R = crate::BitReader;
#[doc = "Field `ITAMP1NOER` writer - Internal Tamper 1 no erase"]
pub type ITAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP2NOER` reader - Internal Tamper 2 no erase"]
pub type ITAMP2NOER_R = crate::BitReader;
#[doc = "Field `ITAMP2NOER` writer - Internal Tamper 2 no erase"]
pub type ITAMP2NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP3NOER` reader - Internal Tamper 3 no erase"]
pub type ITAMP3NOER_R = crate::BitReader;
#[doc = "Field `ITAMP3NOER` writer - Internal Tamper 3 no erase"]
pub type ITAMP3NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP4NOER` reader - Internal Tamper 4 no erase"]
pub type ITAMP4NOER_R = crate::BitReader;
#[doc = "Field `ITAMP4NOER` writer - Internal Tamper 4 no erase"]
pub type ITAMP4NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP5NOER` reader - Internal Tamper 5 no erase"]
pub type ITAMP5NOER_R = crate::BitReader;
#[doc = "Field `ITAMP5NOER` writer - Internal Tamper 5 no erase"]
pub type ITAMP5NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP6NOER` reader - Internal Tamper 6 no erase"]
pub type ITAMP6NOER_R = crate::BitReader;
#[doc = "Field `ITAMP6NOER` writer - Internal Tamper 6 no erase"]
pub type ITAMP6NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP7NOER` reader - Internal Tamper 7 no erase"]
pub type ITAMP7NOER_R = crate::BitReader;
#[doc = "Field `ITAMP7NOER` writer - Internal Tamper 7 no erase"]
pub type ITAMP7NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP8NOER` reader - Internal Tamper 8 no erase"]
pub type ITAMP8NOER_R = crate::BitReader;
#[doc = "Field `ITAMP8NOER` writer - Internal Tamper 8 no erase"]
pub type ITAMP8NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP9NOER` reader - Internal Tamper 9 no erase"]
pub type ITAMP9NOER_R = crate::BitReader;
#[doc = "Field `ITAMP9NOER` writer - Internal Tamper 9 no erase"]
pub type ITAMP9NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP11NOER` reader - Internal Tamper 11 no erase"]
pub type ITAMP11NOER_R = crate::BitReader;
#[doc = "Field `ITAMP11NOER` writer - Internal Tamper 11 no erase"]
pub type ITAMP11NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP12NOER` reader - Internal Tamper 12 no erase"]
pub type ITAMP12NOER_R = crate::BitReader;
#[doc = "Field `ITAMP12NOER` writer - Internal Tamper 12 no erase"]
pub type ITAMP12NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP13NOER` reader - Internal Tamper 13 no erase"]
pub type ITAMP13NOER_R = crate::BitReader;
#[doc = "Field `ITAMP13NOER` writer - Internal Tamper 13 no erase"]
pub type ITAMP13NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP15NOER` reader - Internal Tamper 15 no erase"]
pub type ITAMP15NOER_R = crate::BitReader;
#[doc = "Field `ITAMP15NOER` writer - Internal Tamper 15 no erase"]
pub type ITAMP15NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal Tamper 1 no erase"]
    #[inline(always)]
    pub fn itamp1noer(&self) -> ITAMP1NOER_R {
        ITAMP1NOER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Tamper 2 no erase"]
    #[inline(always)]
    pub fn itamp2noer(&self) -> ITAMP2NOER_R {
        ITAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal Tamper 3 no erase"]
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Internal Tamper 4 no erase"]
    #[inline(always)]
    pub fn itamp4noer(&self) -> ITAMP4NOER_R {
        ITAMP4NOER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Internal Tamper 5 no erase"]
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal Tamper 6 no erase"]
    #[inline(always)]
    pub fn itamp6noer(&self) -> ITAMP6NOER_R {
        ITAMP6NOER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Internal Tamper 7 no erase"]
    #[inline(always)]
    pub fn itamp7noer(&self) -> ITAMP7NOER_R {
        ITAMP7NOER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Internal Tamper 8 no erase"]
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal Tamper 9 no erase"]
    #[inline(always)]
    pub fn itamp9noer(&self) -> ITAMP9NOER_R {
        ITAMP9NOER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Internal Tamper 11 no erase"]
    #[inline(always)]
    pub fn itamp11noer(&self) -> ITAMP11NOER_R {
        ITAMP11NOER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Internal Tamper 12 no erase"]
    #[inline(always)]
    pub fn itamp12noer(&self) -> ITAMP12NOER_R {
        ITAMP12NOER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Internal Tamper 13 no erase"]
    #[inline(always)]
    pub fn itamp13noer(&self) -> ITAMP13NOER_R {
        ITAMP13NOER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Internal Tamper 15 no erase"]
    #[inline(always)]
    pub fn itamp15noer(&self) -> ITAMP15NOER_R {
        ITAMP15NOER_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Tamper 1 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp1noer(&mut self) -> ITAMP1NOER_W<CR3rs> {
        ITAMP1NOER_W::new(self, 0)
    }
    #[doc = "Bit 1 - Internal Tamper 2 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp2noer(&mut self) -> ITAMP2NOER_W<CR3rs> {
        ITAMP2NOER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Internal Tamper 3 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W<CR3rs> {
        ITAMP3NOER_W::new(self, 2)
    }
    #[doc = "Bit 3 - Internal Tamper 4 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp4noer(&mut self) -> ITAMP4NOER_W<CR3rs> {
        ITAMP4NOER_W::new(self, 3)
    }
    #[doc = "Bit 4 - Internal Tamper 5 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W<CR3rs> {
        ITAMP5NOER_W::new(self, 4)
    }
    #[doc = "Bit 5 - Internal Tamper 6 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6noer(&mut self) -> ITAMP6NOER_W<CR3rs> {
        ITAMP6NOER_W::new(self, 5)
    }
    #[doc = "Bit 6 - Internal Tamper 7 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp7noer(&mut self) -> ITAMP7NOER_W<CR3rs> {
        ITAMP7NOER_W::new(self, 6)
    }
    #[doc = "Bit 7 - Internal Tamper 8 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W<CR3rs> {
        ITAMP8NOER_W::new(self, 7)
    }
    #[doc = "Bit 8 - Internal Tamper 9 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp9noer(&mut self) -> ITAMP9NOER_W<CR3rs> {
        ITAMP9NOER_W::new(self, 8)
    }
    #[doc = "Bit 10 - Internal Tamper 11 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp11noer(&mut self) -> ITAMP11NOER_W<CR3rs> {
        ITAMP11NOER_W::new(self, 10)
    }
    #[doc = "Bit 11 - Internal Tamper 12 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp12noer(&mut self) -> ITAMP12NOER_W<CR3rs> {
        ITAMP12NOER_W::new(self, 11)
    }
    #[doc = "Bit 12 - Internal Tamper 13 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp13noer(&mut self) -> ITAMP13NOER_W<CR3rs> {
        ITAMP13NOER_W::new(self, 12)
    }
    #[doc = "Bit 14 - Internal Tamper 15 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn itamp15noer(&mut self) -> ITAMP15NOER_W<CR3rs> {
        ITAMP15NOER_W::new(self, 14)
    }
}
#[doc = "TAMP control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3rs {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0;
}
