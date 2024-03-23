#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `CMPMCF` writer - compare match Clear Flag"]
pub type CMPMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMCF` writer - Autoreload match Clear Flag"]
pub type ARRMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge Clear Flag"]
pub type EXTTRIGCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOKCF` writer - Compare register update OK Clear Flag"]
pub type CMPOKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKCF` writer - Autoreload register update OK Clear Flag"]
pub type ARROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPCF` writer - Direction change to UP Clear Flag"]
pub type UPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNCF` writer - Direction change to down Clear Flag"]
pub type DOWNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - compare match Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmcf(&mut self) -> CMPMCF_W<ICRrs> {
        CMPMCF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn arrmcf(&mut self) -> ARRMCF_W<ICRrs> {
        ARRMCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<ICRrs> {
        EXTTRIGCF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register update OK Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W<ICRrs> {
        CMPOKCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn arrokcf(&mut self) -> ARROKCF_W<ICRrs> {
        ARROKCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn upcf(&mut self) -> UPCF_W<ICRrs> {
        UPCF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn downcf(&mut self) -> DOWNCF_W<ICRrs> {
        DOWNCF_W::new(self, 6)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
