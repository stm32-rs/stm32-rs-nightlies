#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Field `CTAMP1F` writer - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register."]
pub type CTAMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMP2F` writer - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register."]
pub type CTAMP2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP3F` writer - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
pub type CITAMP3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP4F` writer - Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
pub type CITAMP4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP5F` writer - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
pub type CITAMP5F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CITAMP6F` writer - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
pub type CITAMP6F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear TAMP1 detection flag Writing 1 in this bit clears the TAMP1F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<SCRrs> {
        CTAMP1F_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear TAMP2 detection flag Writing 1 in this bit clears the TAMP2F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<SCRrs> {
        CTAMP2F_W::new(self, 1)
    }
    #[doc = "Bit 18 - Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<SCRrs> {
        CITAMP3F_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp4f(&mut self) -> CITAMP4F_W<SCRrs> {
        CITAMP4F_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<SCRrs> {
        CITAMP5F_W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn citamp6f(&mut self) -> CITAMP6F_W<SCRrs> {
        CITAMP6F_W::new(self, 21)
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
