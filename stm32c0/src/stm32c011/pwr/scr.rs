#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Field `CWUF1` writer - Clear wakeup flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register."]
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF2` writer - Clear wakeup flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register."]
pub type CWUF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF3` writer - Clear wakeup flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register."]
pub type CWUF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF4` writer - Clear wakeup flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register."]
pub type CWUF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWUF6` writer - Clear wakeup flag 6 Setting this bit clears the WUF6 flag in the PWR_SR1 register."]
pub type CWUF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSBF` writer - Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear wakeup flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<SCRrs> {
        CWUF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear wakeup flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<SCRrs> {
        CWUF2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<SCRrs> {
        CWUF3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear wakeup flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> CWUF4_W<SCRrs> {
        CWUF4_W::new(self, 3)
    }
    #[doc = "Bit 5 - Clear wakeup flag 6 Setting this bit clears the WUF6 flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf6(&mut self) -> CWUF6_W<SCRrs> {
        CWUF6_W::new(self, 5)
    }
    #[doc = "Bit 8 - Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<SCRrs> {
        CSBF_W::new(self, 8)
    }
}
#[doc = "PWR status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
