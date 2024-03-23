#[doc = "Register `FLASH_PRIVCFGR` reader"]
pub type R = crate::R<FLASH_PRIVCFGRrs>;
#[doc = "Register `FLASH_PRIVCFGR` writer"]
pub type W = crate::W<FLASH_PRIVCFGRrs>;
#[doc = "Field `SPRIV` reader - Privileged protection for secure registers"]
pub type SPRIV_R = crate::BitReader;
#[doc = "Field `SPRIV` writer - Privileged protection for secure registers"]
pub type SPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSPRIV` reader - Privileged protection for non-secure registers"]
pub type NSPRIV_R = crate::BitReader;
#[doc = "Field `NSPRIV` writer - Privileged protection for non-secure registers"]
pub type NSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privileged protection for secure registers"]
    #[inline(always)]
    pub fn spriv(&self) -> SPRIV_R {
        SPRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Privileged protection for non-secure registers"]
    #[inline(always)]
    pub fn nspriv(&self) -> NSPRIV_R {
        NSPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged protection for secure registers"]
    #[inline(always)]
    #[must_use]
    pub fn spriv(&mut self) -> SPRIV_W<FLASH_PRIVCFGRrs> {
        SPRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Privileged protection for non-secure registers"]
    #[inline(always)]
    #[must_use]
    pub fn nspriv(&mut self) -> NSPRIV_W<FLASH_PRIVCFGRrs> {
        NSPRIV_W::new(self, 1)
    }
}
#[doc = "FLASH privilege configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_privcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_privcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_PRIVCFGRrs;
impl crate::RegisterSpec for FLASH_PRIVCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_privcfgr::R`](R) reader structure"]
impl crate::Readable for FLASH_PRIVCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`flash_privcfgr::W`](W) writer structure"]
impl crate::Writable for FLASH_PRIVCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_PRIVCFGR to value 0"]
impl crate::Resettable for FLASH_PRIVCFGRrs {
    const RESET_VALUE: u32 = 0;
}
