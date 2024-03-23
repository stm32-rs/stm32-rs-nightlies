#[doc = "Register `ICR_output` writer"]
pub type W = crate::W<ICR_OUTPUTrs>;
#[doc = "Field `CC1IF` writer - Capture/compare 1 clear flag"]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMCF` writer - Autoreload match Clear Flag"]
pub type ARRMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge Clear Flag"]
pub type EXTTRIGCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1OKCF` writer - Compare register 1 update OK Clear Flag"]
pub type CMP1OKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKCF` writer - Autoreload register update OK Clear Flag"]
pub type ARROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPCF` writer - Direction change to UP Clear Flag"]
pub type UPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNCF` writer - Direction change to down Clear Flag"]
pub type DOWNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UECF` writer - Update event clear flag"]
pub type UECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPOKCF` writer - Repetition register update OK clear flag"]
pub type REPOKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2CF` writer - Capture/compare 2 clear flag"]
pub type CC2CF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2OKCF` writer - Compare register 2 update OK clear flag"]
pub type CMP2OKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIEROKCF` writer - Interrupt enable register update OK clear flag"]
pub type DIEROKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Capture/compare 1 clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<ICR_OUTPUTrs> {
        CC1IF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn arrmcf(&mut self) -> ARRMCF_W<ICR_OUTPUTrs> {
        ARRMCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<ICR_OUTPUTrs> {
        EXTTRIGCF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register 1 update OK Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1okcf(&mut self) -> CMP1OKCF_W<ICR_OUTPUTrs> {
        CMP1OKCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn arrokcf(&mut self) -> ARROKCF_W<ICR_OUTPUTrs> {
        ARROKCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn upcf(&mut self) -> UPCF_W<ICR_OUTPUTrs> {
        UPCF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down Clear Flag"]
    #[inline(always)]
    #[must_use]
    pub fn downcf(&mut self) -> DOWNCF_W<ICR_OUTPUTrs> {
        DOWNCF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Update event clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn uecf(&mut self) -> UECF_W<ICR_OUTPUTrs> {
        UECF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Repetition register update OK clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn repokcf(&mut self) -> REPOKCF_W<ICR_OUTPUTrs> {
        REPOKCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 2 clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2cf(&mut self) -> CC2CF_W<ICR_OUTPUTrs> {
        CC2CF_W::new(self, 9)
    }
    #[doc = "Bit 19 - Compare register 2 update OK clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2okcf(&mut self) -> CMP2OKCF_W<ICR_OUTPUTrs> {
        CMP2OKCF_W::new(self, 19)
    }
    #[doc = "Bit 24 - Interrupt enable register update OK clear flag"]
    #[inline(always)]
    #[must_use]
    pub fn dierokcf(&mut self) -> DIEROKCF_W<ICR_OUTPUTrs> {
        DIEROKCF_W::new(self, 24)
    }
}
#[doc = "Interrupt Clear Register (output mode)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr_output::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_OUTPUTrs;
impl crate::RegisterSpec for ICR_OUTPUTrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr_output::W`](W) writer structure"]
impl crate::Writable for ICR_OUTPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR_output to value 0"]
impl crate::Resettable for ICR_OUTPUTrs {
    const RESET_VALUE: u32 = 0;
}
