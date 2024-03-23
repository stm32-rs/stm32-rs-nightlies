#[doc = "Register `IWDG_RLR` reader"]
pub type R = crate::R<IWDG_RLRrs>;
#[doc = "Register `IWDG_RLR` writer"]
pub type W = crate::W<IWDG_RLRrs>;
#[doc = "Field `RL` reader - RL"]
pub type RL_R = crate::FieldReader<u16>;
#[doc = "Field `RL` writer - RL"]
pub type RL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RL"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RL"]
    #[inline(always)]
    #[must_use]
    pub fn rl(&mut self) -> RL_W<IWDG_RLRrs> {
        RL_W::new(self, 0)
    }
}
#[doc = "Reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_rlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_rlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWDG_RLRrs;
impl crate::RegisterSpec for IWDG_RLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_rlr::R`](R) reader structure"]
impl crate::Readable for IWDG_RLRrs {}
#[doc = "`write(|w| ..)` method takes [`iwdg_rlr::W`](W) writer structure"]
impl crate::Writable for IWDG_RLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWDG_RLR to value 0x0fff"]
impl crate::Resettable for IWDG_RLRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
