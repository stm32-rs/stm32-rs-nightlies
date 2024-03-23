#[doc = "Register `FPCCR` reader"]
pub type R = crate::R<FPCCRrs>;
#[doc = "Register `FPCCR` writer"]
pub type W = crate::W<FPCCRrs>;
#[doc = "Field `LSPACT` reader - LSPACT"]
pub type LSPACT_R = crate::BitReader;
#[doc = "Field `LSPACT` writer - LSPACT"]
pub type LSPACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USER` reader - USER"]
pub type USER_R = crate::BitReader;
#[doc = "Field `USER` writer - USER"]
pub type USER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THREAD` reader - THREAD"]
pub type THREAD_R = crate::BitReader;
#[doc = "Field `THREAD` writer - THREAD"]
pub type THREAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRDY` reader - HFRDY"]
pub type HFRDY_R = crate::BitReader;
#[doc = "Field `HFRDY` writer - HFRDY"]
pub type HFRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRDY` reader - MMRDY"]
pub type MMRDY_R = crate::BitReader;
#[doc = "Field `MMRDY` writer - MMRDY"]
pub type MMRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFRDY` reader - BFRDY"]
pub type BFRDY_R = crate::BitReader;
#[doc = "Field `BFRDY` writer - BFRDY"]
pub type BFRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONRDY` reader - MONRDY"]
pub type MONRDY_R = crate::BitReader;
#[doc = "Field `MONRDY` writer - MONRDY"]
pub type MONRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSPEN` reader - LSPEN"]
pub type LSPEN_R = crate::BitReader;
#[doc = "Field `LSPEN` writer - LSPEN"]
pub type LSPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASPEN` reader - ASPEN"]
pub type ASPEN_R = crate::BitReader;
#[doc = "Field `ASPEN` writer - ASPEN"]
pub type ASPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSPACT"]
    #[inline(always)]
    pub fn lspact(&self) -> LSPACT_R {
        LSPACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USER"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - THREAD"]
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HFRDY"]
    #[inline(always)]
    pub fn hfrdy(&self) -> HFRDY_R {
        HFRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMRDY"]
    #[inline(always)]
    pub fn mmrdy(&self) -> MMRDY_R {
        MMRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BFRDY"]
    #[inline(always)]
    pub fn bfrdy(&self) -> BFRDY_R {
        BFRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - MONRDY"]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 30 - LSPEN"]
    #[inline(always)]
    pub fn lspen(&self) -> LSPEN_R {
        LSPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ASPEN"]
    #[inline(always)]
    pub fn aspen(&self) -> ASPEN_R {
        ASPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSPACT"]
    #[inline(always)]
    #[must_use]
    pub fn lspact(&mut self) -> LSPACT_W<FPCCRrs> {
        LSPACT_W::new(self, 0)
    }
    #[doc = "Bit 1 - USER"]
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> USER_W<FPCCRrs> {
        USER_W::new(self, 1)
    }
    #[doc = "Bit 3 - THREAD"]
    #[inline(always)]
    #[must_use]
    pub fn thread(&mut self) -> THREAD_W<FPCCRrs> {
        THREAD_W::new(self, 3)
    }
    #[doc = "Bit 4 - HFRDY"]
    #[inline(always)]
    #[must_use]
    pub fn hfrdy(&mut self) -> HFRDY_W<FPCCRrs> {
        HFRDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - MMRDY"]
    #[inline(always)]
    #[must_use]
    pub fn mmrdy(&mut self) -> MMRDY_W<FPCCRrs> {
        MMRDY_W::new(self, 5)
    }
    #[doc = "Bit 6 - BFRDY"]
    #[inline(always)]
    #[must_use]
    pub fn bfrdy(&mut self) -> BFRDY_W<FPCCRrs> {
        BFRDY_W::new(self, 6)
    }
    #[doc = "Bit 8 - MONRDY"]
    #[inline(always)]
    #[must_use]
    pub fn monrdy(&mut self) -> MONRDY_W<FPCCRrs> {
        MONRDY_W::new(self, 8)
    }
    #[doc = "Bit 30 - LSPEN"]
    #[inline(always)]
    #[must_use]
    pub fn lspen(&mut self) -> LSPEN_W<FPCCRrs> {
        LSPEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - ASPEN"]
    #[inline(always)]
    #[must_use]
    pub fn aspen(&mut self) -> ASPEN_W<FPCCRrs> {
        ASPEN_W::new(self, 31)
    }
}
#[doc = "Floating-point context control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPCCRrs;
impl crate::RegisterSpec for FPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpccr::R`](R) reader structure"]
impl crate::Readable for FPCCRrs {}
#[doc = "`write(|w| ..)` method takes [`fpccr::W`](W) writer structure"]
impl crate::Writable for FPCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPCCR to value 0"]
impl crate::Resettable for FPCCRrs {
    const RESET_VALUE: u32 = 0;
}
