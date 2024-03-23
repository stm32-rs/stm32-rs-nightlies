#[doc = "Register `FLASH_PRIVCFGR` reader"]
pub type R = crate::R<FLASH_PRIVCFGRrs>;
#[doc = "Register `FLASH_PRIVCFGR` writer"]
pub type W = crate::W<FLASH_PRIVCFGRrs>;
#[doc = "Field `SPRIV` reader - Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored."]
pub type SPRIV_R = crate::BitReader;
#[doc = "Field `SPRIV` writer - Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored."]
pub type SPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSPRIV` reader - Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored."]
pub type NSPRIV_R = crate::BitReader;
#[doc = "Field `NSPRIV` writer - Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored."]
pub type NSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored."]
    #[inline(always)]
    pub fn spriv(&self) -> SPRIV_R {
        SPRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored."]
    #[inline(always)]
    pub fn nspriv(&self) -> NSPRIV_R {
        NSPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn spriv(&mut self) -> SPRIV_W<FLASH_PRIVCFGRrs> {
        SPRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored."]
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
