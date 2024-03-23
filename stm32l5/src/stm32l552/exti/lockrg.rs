#[doc = "Register `LOCKRG` reader"]
pub type R = crate::R<LOCKRGrs>;
#[doc = "Register `LOCKRG` writer"]
pub type W = crate::W<LOCKRGrs>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<LOCKRGrs> {
        LOCK_W::new(self, 0)
    }
}
#[doc = "EXTI lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockrg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockrg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCKRGrs;
impl crate::RegisterSpec for LOCKRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockrg::R`](R) reader structure"]
impl crate::Readable for LOCKRGrs {}
#[doc = "`write(|w| ..)` method takes [`lockrg::W`](W) writer structure"]
impl crate::Writable for LOCKRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCKRG to value 0"]
impl crate::Resettable for LOCKRGrs {
    const RESET_VALUE: u32 = 0;
}
