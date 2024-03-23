#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Field `CTAMP1F` writer - CTAMP1F"]
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP2F` writer - CTAMP2F"]
pub type CTAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP3F` writer - CTAMP3F"]
pub type CTAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP4F` writer - CTAMP4F"]
pub type CTAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP5F` writer - CTAMP5F"]
pub type CTAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP6F` writer - CTAMP6F"]
pub type CTAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP7F` writer - CTAMP7F"]
pub type CTAMP7F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP8F` writer - CTAMP8F"]
pub type CTAMP8F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP1F` writer - CITAMP1F"]
pub type CITAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP2F` writer - CITAMP2F"]
pub type CITAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP3F` writer - CITAMP3F"]
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP5F` writer - CITAMP5F"]
pub type CITAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP8F` writer - CITAMP8F"]
pub type CITAMP8F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<SCRrs> {
        CTAMP1F_W::new(self, 0)
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<SCRrs> {
        CTAMP2F_W::new(self, 1)
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W<SCRrs> {
        CTAMP3F_W::new(self, 2)
    }
    #[doc = "Bit 3 - CTAMP4F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp4f(&mut self) -> CTAMP4F_W<SCRrs> {
        CTAMP4F_W::new(self, 3)
    }
    #[doc = "Bit 4 - CTAMP5F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp5f(&mut self) -> CTAMP5F_W<SCRrs> {
        CTAMP5F_W::new(self, 4)
    }
    #[doc = "Bit 5 - CTAMP6F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp6f(&mut self) -> CTAMP6F_W<SCRrs> {
        CTAMP6F_W::new(self, 5)
    }
    #[doc = "Bit 6 - CTAMP7F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp7f(&mut self) -> CTAMP7F_W<SCRrs> {
        CTAMP7F_W::new(self, 6)
    }
    #[doc = "Bit 7 - CTAMP8F"]
    #[inline(always)]
    #[must_use]
    pub fn ctamp8f(&mut self) -> CTAMP8F_W<SCRrs> {
        CTAMP8F_W::new(self, 7)
    }
    #[doc = "Bit 16 - CITAMP1F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp1f(&mut self) -> CITAMP1F_W<SCRrs> {
        CITAMP1F_W::new(self, 16)
    }
    #[doc = "Bit 17 - CITAMP2F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp2f(&mut self) -> CITAMP2F_W<SCRrs> {
        CITAMP2F_W::new(self, 17)
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<SCRrs> {
        CITAMP3F_W::new(self, 18)
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<SCRrs> {
        CITAMP5F_W::new(self, 20)
    }
    #[doc = "Bit 23 - CITAMP8F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp8f(&mut self) -> CITAMP8F_W<SCRrs> {
        CITAMP8F_W::new(self, 23)
    }
}
#[doc = "TAMP status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}
