#[doc = "Register `APB1HFZR` reader"]
pub type R = crate::R<APB1HFZRrs>;
#[doc = "Register `APB1HFZR` writer"]
pub type W = crate::W<APB1HFZRrs>;
#[doc = "Field `DBG_LPTIM2_STOP` reader - LPTIM2 stop in debug"]
pub type DBG_LPTIM2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM2_STOP` writer - LPTIM2 stop in debug"]
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - LPTIM2 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<APB1HFZRrs> {
        DBG_LPTIM2_STOP_W::new(self, 5)
    }
}
#[doc = "DBGMCU APB1H peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hfzr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hfzr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1HFZRrs;
impl crate::RegisterSpec for APB1HFZRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1hfzr::R`](R) reader structure"]
impl crate::Readable for APB1HFZRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1hfzr::W`](W) writer structure"]
impl crate::Writable for APB1HFZRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1HFZR to value 0"]
impl crate::Resettable for APB1HFZRrs {
    const RESET_VALUE: u32 = 0;
}
