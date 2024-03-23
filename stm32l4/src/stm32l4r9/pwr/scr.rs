#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCRrs>;
#[doc = "Field `WUF1` writer - Clear wakeup flag 1"]
pub type WUF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUF2` writer - Clear wakeup flag 2"]
pub type WUF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUF3` writer - Clear wakeup flag 3"]
pub type WUF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUF4` writer - Clear wakeup flag 4"]
pub type WUF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUF5` writer - Clear wakeup flag 5"]
pub type WUF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBF` writer - Clear standby flag"]
pub type SBF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear wakeup flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn wuf1(&mut self) -> WUF1_W<SCRrs> {
        WUF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear wakeup flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn wuf2(&mut self) -> WUF2_W<SCRrs> {
        WUF2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wakeup flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn wuf3(&mut self) -> WUF3_W<SCRrs> {
        WUF3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear wakeup flag 4"]
    #[inline(always)]
    #[must_use]
    pub fn wuf4(&mut self) -> WUF4_W<SCRrs> {
        WUF4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear wakeup flag 5"]
    #[inline(always)]
    #[must_use]
    pub fn wuf5(&mut self) -> WUF5_W<SCRrs> {
        WUF5_W::new(self, 4)
    }
    #[doc = "Bit 8 - Clear standby flag"]
    #[inline(always)]
    #[must_use]
    pub fn sbf(&mut self) -> SBF_W<SCRrs> {
        SBF_W::new(self, 8)
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
