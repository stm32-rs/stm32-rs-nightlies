#[doc = "Register `SCR` reader"]
pub type R = crate::R<SCRrs>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Field `CTAMP1F` reader - CTAMP1F"]
pub type CTAMP1F_R = crate::BitReader;
#[doc = "Field `CTAMP1F` writer - CTAMP1F"]
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP2F` reader - CTAMP2F"]
pub type CTAMP2F_R = crate::BitReader;
#[doc = "Field `CTAMP2F` writer - CTAMP2F"]
pub type CTAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP3F` reader - CTAMP3F"]
pub type CTAMP3F_R = crate::BitReader;
#[doc = "Field `CTAMP3F` writer - CTAMP3F"]
pub type CTAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP3F` reader - CITAMP3F"]
pub type CITAMP3F_R = crate::BitReader;
#[doc = "Field `CITAMP3F` writer - CITAMP3F"]
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP4F` reader - CITAMP4F"]
pub type CITAMP4F_R = crate::BitReader;
#[doc = "Field `CITAMP4F` writer - CITAMP4F"]
pub type CITAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP5F` reader - CITAMP5F"]
pub type CITAMP5F_R = crate::BitReader;
#[doc = "Field `CITAMP5F` writer - CITAMP5F"]
pub type CITAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP6F` reader - CITAMP6F"]
pub type CITAMP6F_R = crate::BitReader;
#[doc = "Field `CITAMP6F` writer - CITAMP6F"]
pub type CITAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    pub fn ctamp1f(&self) -> CTAMP1F_R {
        CTAMP1F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    pub fn ctamp2f(&self) -> CTAMP2F_R {
        CTAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    pub fn ctamp3f(&self) -> CTAMP3F_R {
        CTAMP3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    pub fn citamp3f(&self) -> CITAMP3F_R {
        CITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CITAMP4F"]
    #[inline(always)]
    pub fn citamp4f(&self) -> CITAMP4F_R {
        CITAMP4F_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    pub fn citamp5f(&self) -> CITAMP5F_R {
        CITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CITAMP6F"]
    #[inline(always)]
    pub fn citamp6f(&self) -> CITAMP6F_R {
        CITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
}
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
}
#[doc = "TAMP status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for SCRrs {}
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
