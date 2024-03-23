#[doc = "Register `CSLCKR` reader"]
pub type R = crate::R<CSLCKRrs>;
#[doc = "Register `CSLCKR` writer"]
pub type W = crate::W<CSLCKRrs>;
#[doc = "Field `LOCKSVTAIRCR` reader - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
pub type LOCKSVTAIRCR_R = crate::BitReader;
#[doc = "Field `LOCKSVTAIRCR` writer - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
pub type LOCKSVTAIRCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKSMPU` reader - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
pub type LOCKSMPU_R = crate::BitReader;
#[doc = "Field `LOCKSMPU` writer - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
pub type LOCKSMPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKSAU` reader - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
pub type LOCKSAU_R = crate::BitReader;
#[doc = "Field `LOCKSAU` writer - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
pub type LOCKSAU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
    #[inline(always)]
    #[must_use]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W<CSLCKRrs> {
        LOCKSVTAIRCR_W::new(self, 0)
    }
    #[doc = "Bit 1 - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
    #[inline(always)]
    #[must_use]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W<CSLCKRrs> {
        LOCKSMPU_W::new(self, 1)
    }
    #[doc = "Bit 2 - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
    #[inline(always)]
    #[must_use]
    pub fn locksau(&mut self) -> LOCKSAU_W<CSLCKRrs> {
        LOCKSAU_W::new(self, 2)
    }
}
#[doc = "SBS CPU secure lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cslckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cslckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSLCKRrs;
impl crate::RegisterSpec for CSLCKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cslckr::R`](R) reader structure"]
impl crate::Readable for CSLCKRrs {}
#[doc = "`write(|w| ..)` method takes [`cslckr::W`](W) writer structure"]
impl crate::Writable for CSLCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSLCKR to value 0"]
impl crate::Resettable for CSLCKRrs {
    const RESET_VALUE: u32 = 0;
}
