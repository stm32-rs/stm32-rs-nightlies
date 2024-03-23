#[doc = "Register `SECBOOTADD0R` reader"]
pub type R = crate::R<SECBOOTADD0Rrs>;
#[doc = "Register `SECBOOTADD0R` writer"]
pub type W = crate::W<SECBOOTADD0Rrs>;
#[doc = "Field `BOOT_LOCK` reader - BOOT_LOCK"]
pub type BOOT_LOCK_R = crate::BitReader;
#[doc = "Field `BOOT_LOCK` writer - BOOT_LOCK"]
pub type BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECBOOTADD0` writer - SECBOOTADD0"]
pub type SECBOOTADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - BOOT_LOCK"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOOT_LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<SECBOOTADD0Rrs> {
        BOOT_LOCK_W::new(self, 0)
    }
    #[doc = "Bits 7:31 - SECBOOTADD0"]
    #[inline(always)]
    #[must_use]
    pub fn secbootadd0(&mut self) -> SECBOOTADD0_W<SECBOOTADD0Rrs> {
        SECBOOTADD0_W::new(self, 7)
    }
}
#[doc = "FFlash secure boot address 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secbootadd0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secbootadd0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECBOOTADD0Rrs;
impl crate::RegisterSpec for SECBOOTADD0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secbootadd0r::R`](R) reader structure"]
impl crate::Readable for SECBOOTADD0Rrs {}
#[doc = "`write(|w| ..)` method takes [`secbootadd0r::W`](W) writer structure"]
impl crate::Writable for SECBOOTADD0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECBOOTADD0R to value 0"]
impl crate::Resettable for SECBOOTADD0Rrs {
    const RESET_VALUE: u32 = 0;
}
