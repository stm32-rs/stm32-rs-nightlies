#[doc = "Register `LPTIM_IER` reader"]
pub type R = crate::R<LPTIM_IERrs>;
#[doc = "Register `LPTIM_IER` writer"]
pub type W = crate::W<LPTIM_IERrs>;
#[doc = "Field `CMPMIE` reader - Compare match Interrupt Enable"]
pub type CMPMIE_R = crate::BitReader;
#[doc = "Field `CMPMIE` writer - Compare match Interrupt Enable"]
pub type CMPMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMIE` reader - Autoreload match Interrupt Enable"]
pub type ARRMIE_R = crate::BitReader;
#[doc = "Field `ARRMIE` writer - Autoreload match Interrupt Enable"]
pub type ARRMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_R = crate::BitReader;
#[doc = "Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable"]
pub type EXTTRIGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOKIE` reader - Compare register update OK Interrupt Enable"]
pub type CMPOKIE_R = crate::BitReader;
#[doc = "Field `CMPOKIE` writer - Compare register update OK Interrupt Enable"]
pub type CMPOKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_R = crate::BitReader;
#[doc = "Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable"]
pub type ARROKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPIE` reader - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPIE_R = crate::BitReader;
#[doc = "Field `UPIE` writer - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNIE` reader - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNIE_R = crate::BitReader;
#[doc = "Field `DOWNIE` writer - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWNIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare match Interrupt Enable"]
    #[inline(always)]
    pub fn cmpmie(&self) -> CMPMIE_R {
        CMPMIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn cmpokie(&self) -> CMPOKIE_R {
        CMPOKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmie(&mut self) -> CMPMIE_W<LPTIM_IERrs> {
        CMPMIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrmie(&mut self) -> ARRMIE_W<LPTIM_IERrs> {
        ARRMIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<LPTIM_IERrs> {
        EXTTRIGIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register update OK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpokie(&mut self) -> CMPOKIE_W<LPTIM_IERrs> {
        CMPOKIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arrokie(&mut self) -> ARROKIE_W<LPTIM_IERrs> {
        ARROKIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<LPTIM_IERrs> {
        UPIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn downie(&mut self) -> DOWNIE_W<LPTIM_IERrs> {
        DOWNIE_W::new(self, 6)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptim_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPTIM_IERrs;
impl crate::RegisterSpec for LPTIM_IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim_ier::R`](R) reader structure"]
impl crate::Readable for LPTIM_IERrs {}
#[doc = "`write(|w| ..)` method takes [`lptim_ier::W`](W) writer structure"]
impl crate::Writable for LPTIM_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPTIM_IER to value 0"]
impl crate::Resettable for LPTIM_IERrs {
    const RESET_VALUE: u32 = 0;
}
