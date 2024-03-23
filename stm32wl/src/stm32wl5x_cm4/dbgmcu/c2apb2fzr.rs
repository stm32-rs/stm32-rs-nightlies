#[doc = "Register `C2APB2FZR` reader"]
pub type R = crate::R<C2APB2FZRrs>;
#[doc = "Register `C2APB2FZR` writer"]
pub type W = crate::W<C2APB2FZRrs>;
#[doc = "Field `DBG_TIM1_STOP` reader - DBG_TIM1_STOP"]
pub type DBG_TIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - DBG_TIM1_STOP"]
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - DBG_TIM16_STOP"]
pub type DBG_TIM16_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - DBG_TIM16_STOP"]
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17_STOP` reader - DBG_TIM17_STOP"]
pub type DBG_TIM17_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM17_STOP` writer - DBG_TIM17_STOP"]
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - DBG_TIM1_STOP"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - DBG_TIM16_STOP"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DBG_TIM17_STOP"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - DBG_TIM1_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<C2APB2FZRrs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    #[doc = "Bit 17 - DBG_TIM16_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<C2APB2FZRrs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - DBG_TIM17_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<C2APB2FZRrs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
}
#[doc = "DBGMCU CPU2 APB2 Peripheral Freeze Register \\[dual core device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2apb2fzr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2apb2fzr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2APB2FZRrs;
impl crate::RegisterSpec for C2APB2FZRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2apb2fzr::R`](R) reader structure"]
impl crate::Readable for C2APB2FZRrs {}
#[doc = "`write(|w| ..)` method takes [`c2apb2fzr::W`](W) writer structure"]
impl crate::Writable for C2APB2FZRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2APB2FZR to value 0"]
impl crate::Resettable for C2APB2FZRrs {
    const RESET_VALUE: u32 = 0;
}
