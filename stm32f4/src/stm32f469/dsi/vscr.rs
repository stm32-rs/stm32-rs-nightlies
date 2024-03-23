#[doc = "Register `VSCR` reader"]
pub type R = crate::R<VSCRrs>;
#[doc = "Register `VSCR` writer"]
pub type W = crate::W<VSCRrs>;
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UR` reader - Update Register"]
pub type UR_R = crate::BitReader;
#[doc = "Field `UR` writer - Update Register"]
pub type UR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Update Register"]
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<VSCRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 8 - Update Register"]
    #[inline(always)]
    #[must_use]
    pub fn ur(&mut self) -> UR_W<VSCRrs> {
        UR_W::new(self, 8)
    }
}
#[doc = "DSI Host Video Shadow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VSCRrs;
impl crate::RegisterSpec for VSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vscr::R`](R) reader structure"]
impl crate::Readable for VSCRrs {}
#[doc = "`write(|w| ..)` method takes [`vscr::W`](W) writer structure"]
impl crate::Writable for VSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VSCR to value 0"]
impl crate::Resettable for VSCRrs {
    const RESET_VALUE: u32 = 0;
}
