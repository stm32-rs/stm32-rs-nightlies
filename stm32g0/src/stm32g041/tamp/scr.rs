#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Field `CTAMP1F` writer - CTAMP1F"]
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP2F` writer - CTAMP2F"]
pub type CTAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP1F` writer - CITAMP1F"]
pub type CITAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP3F` writer - CITAMP3F"]
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP4F` writer - CITAMP4F"]
pub type CITAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP5F` writer - CITAMP5F"]
pub type CITAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP6F` writer - CITAMP6F"]
pub type CITAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP7F` writer - CITAMP7F"]
pub type CITAMP7F_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 16 - CITAMP1F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp1f(&mut self) -> CITAMP1F_W<SCRrs> {
        CITAMP1F_W::new(self, 16)
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<SCRrs> {
        CITAMP3F_W::new(self, 18)
    }
    #[doc = "Bit 19 - CITAMP4F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp4f(&mut self) -> CITAMP4F_W<SCRrs> {
        CITAMP4F_W::new(self, 19)
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<SCRrs> {
        CITAMP5F_W::new(self, 20)
    }
    #[doc = "Bit 21 - CITAMP6F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp6f(&mut self) -> CITAMP6F_W<SCRrs> {
        CITAMP6F_W::new(self, 21)
    }
    #[doc = "Bit 22 - CITAMP7F"]
    #[inline(always)]
    #[must_use]
    pub fn citamp7f(&mut self) -> CITAMP7F_W<SCRrs> {
        CITAMP7F_W::new(self, 22)
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
