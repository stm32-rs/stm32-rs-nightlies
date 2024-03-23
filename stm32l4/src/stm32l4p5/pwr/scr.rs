#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Clear wakeup flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF1 {
    #[doc = "1: Setting this bit clears the WUFx flag in the PWR_SR1 register"]
    Clear = 1,
}
impl From<CWUF1> for bool {
    #[inline(always)]
    fn from(variant: CWUF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF1` writer - Clear wakeup flag 1"]
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG, CWUF1>;
impl<'a, REG> CWUF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the WUFx flag in the PWR_SR1 register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF1::Clear)
    }
}
#[doc = "Field `CWUF2` writer - Clear wakeup flag 2"]
pub use CWUF1_W as CWUF2_W;
#[doc = "Field `CWUF3` writer - Clear wakeup flag 3"]
pub use CWUF1_W as CWUF3_W;
#[doc = "Field `CWUF4` writer - Clear wakeup flag 4"]
pub use CWUF1_W as CWUF4_W;
#[doc = "Field `CWUF5` writer - Clear wakeup flag 5"]
pub use CWUF1_W as CWUF5_W;
#[doc = "Clear standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSBF {
    #[doc = "1: Setting this bit clears the SBF flag in the PWR_SR1 register"]
    Clear = 1,
}
impl From<CSBF> for bool {
    #[inline(always)]
    fn from(variant: CSBF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSBF` writer - Clear standby flag"]
pub type CSBF_W<'a, REG> = crate::BitWriter<'a, REG, CSBF>;
impl<'a, REG> CSBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the SBF flag in the PWR_SR1 register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSBF::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Clear wakeup flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<SCRrs> {
        CWUF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear wakeup flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<SCRrs> {
        CWUF2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<SCRrs> {
        CWUF3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear wakeup flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> CWUF4_W<SCRrs> {
        CWUF4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear wakeup flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf5(&mut self) -> CWUF5_W<SCRrs> {
        CWUF5_W::new(self, 4)
    }
    #[doc = "Bit 8 - Clear standby flag"]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<SCRrs> {
        CSBF_W::new(self, 8)
    }
}
#[doc = "Power status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
