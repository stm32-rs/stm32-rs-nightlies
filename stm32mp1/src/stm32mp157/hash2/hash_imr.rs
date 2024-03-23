#[doc = "Register `HASH_IMR` reader"]
pub type R = crate::R<HASH_IMRrs>;
#[doc = "Register `HASH_IMR` writer"]
pub type W = crate::W<HASH_IMRrs>;
#[doc = "Field `DINIE` reader - DINIE"]
pub type DINIE_R = crate::BitReader;
#[doc = "Field `DINIE` writer - DINIE"]
pub type DINIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCIE` reader - DCIE"]
pub type DCIE_R = crate::BitReader;
#[doc = "Field `DCIE` writer - DCIE"]
pub type DCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DINIE"]
    #[inline(always)]
    pub fn dinie(&self) -> DINIE_R {
        DINIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCIE"]
    #[inline(always)]
    pub fn dcie(&self) -> DCIE_R {
        DCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DINIE"]
    #[inline(always)]
    #[must_use]
    pub fn dinie(&mut self) -> DINIE_W<HASH_IMRrs> {
        DINIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - DCIE"]
    #[inline(always)]
    #[must_use]
    pub fn dcie(&mut self) -> DCIE_W<HASH_IMRrs> {
        DCIE_W::new(self, 1)
    }
}
#[doc = "HASH interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_IMRrs;
impl crate::RegisterSpec for HASH_IMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_imr::R`](R) reader structure"]
impl crate::Readable for HASH_IMRrs {}
#[doc = "`write(|w| ..)` method takes [`hash_imr::W`](W) writer structure"]
impl crate::Writable for HASH_IMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_IMR to value 0"]
impl crate::Resettable for HASH_IMRrs {
    const RESET_VALUE: u32 = 0;
}
