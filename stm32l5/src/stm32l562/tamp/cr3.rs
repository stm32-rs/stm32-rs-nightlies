#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "Field `ITAMP1NOER` reader - ITAMP1NOER"]
pub type ITAMP1NOER_R = crate::BitReader;
#[doc = "Field `ITAMP1NOER` writer - ITAMP1NOER"]
pub type ITAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP2NOER` reader - ITAMP2NOER"]
pub type ITAMP2NOER_R = crate::BitReader;
#[doc = "Field `ITAMP2NOER` writer - ITAMP2NOER"]
pub type ITAMP2NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP3NOER` reader - ITAMP3NOER"]
pub type ITAMP3NOER_R = crate::BitReader;
#[doc = "Field `ITAMP3NOER` writer - ITAMP3NOER"]
pub type ITAMP3NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP5NOER` reader - ITAMP5NOER"]
pub type ITAMP5NOER_R = crate::BitReader;
#[doc = "Field `ITAMP5NOER` writer - ITAMP5NOER"]
pub type ITAMP5NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP8NOER` reader - ITAMP8NOER"]
pub type ITAMP8NOER_R = crate::BitReader;
#[doc = "Field `ITAMP8NOER` writer - ITAMP8NOER"]
pub type ITAMP8NOER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ITAMP1NOER"]
    #[inline(always)]
    pub fn itamp1noer(&self) -> ITAMP1NOER_R {
        ITAMP1NOER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ITAMP2NOER"]
    #[inline(always)]
    pub fn itamp2noer(&self) -> ITAMP2NOER_R {
        ITAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ITAMP1NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp1noer(&mut self) -> ITAMP1NOER_W<CR3rs> {
        ITAMP1NOER_W::new(self, 0)
    }
    #[doc = "Bit 1 - ITAMP2NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp2noer(&mut self) -> ITAMP2NOER_W<CR3rs> {
        ITAMP2NOER_W::new(self, 1)
    }
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W<CR3rs> {
        ITAMP3NOER_W::new(self, 2)
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W<CR3rs> {
        ITAMP5NOER_W::new(self, 4)
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    #[must_use]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W<CR3rs> {
        ITAMP8NOER_W::new(self, 7)
    }
}
#[doc = "control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
