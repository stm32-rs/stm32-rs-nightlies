#[doc = "Register `SECR` reader"]
pub type R = crate::R<SECRrs>;
#[doc = "Register `SECR` writer"]
pub type W = crate::W<SECRrs>;
#[doc = "Field `SEC_SIZE` reader - Securable memory area size"]
pub type SEC_SIZE_R = crate::FieldReader;
#[doc = "Field `SEC_SIZE` writer - Securable memory area size"]
pub type SEC_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BOOT_LOCK` reader - used to force boot from user area"]
pub type BOOT_LOCK_R = crate::BitReader;
#[doc = "Field `BOOT_LOCK` writer - used to force boot from user area"]
pub type BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_SIZE2` reader - Securable memory area size"]
pub type SEC_SIZE2_R = crate::FieldReader;
#[doc = "Field `SEC_SIZE2` writer - Securable memory area size"]
pub type SEC_SIZE2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Securable memory area size"]
    #[inline(always)]
    pub fn sec_size(&self) -> SEC_SIZE_R {
        SEC_SIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - used to force boot from user area"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:27 - Securable memory area size"]
    #[inline(always)]
    pub fn sec_size2(&self) -> SEC_SIZE2_R {
        SEC_SIZE2_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Securable memory area size"]
    #[inline(always)]
    #[must_use]
    pub fn sec_size(&mut self) -> SEC_SIZE_W<SECRrs> {
        SEC_SIZE_W::new(self, 0)
    }
    #[doc = "Bit 16 - used to force boot from user area"]
    #[inline(always)]
    #[must_use]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<SECRrs> {
        BOOT_LOCK_W::new(self, 16)
    }
    #[doc = "Bits 20:27 - Securable memory area size"]
    #[inline(always)]
    #[must_use]
    pub fn sec_size2(&mut self) -> SEC_SIZE2_W<SECRrs> {
        SEC_SIZE2_W::new(self, 20)
    }
}
#[doc = "Flash Security register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECRrs;
impl crate::RegisterSpec for SECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secr::R`](R) reader structure"]
impl crate::Readable for SECRrs {}
#[doc = "`write(|w| ..)` method takes [`secr::W`](W) writer structure"]
impl crate::Writable for SECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECR to value 0"]
impl crate::Resettable for SECRrs {
    const RESET_VALUE: u32 = 0;
}
