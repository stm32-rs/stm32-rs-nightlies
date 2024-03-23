#[doc = "Register `NSBOOTR_PRG` reader"]
pub type R = crate::R<NSBOOTR_PRGrs>;
#[doc = "Register `NSBOOTR_PRG` writer"]
pub type W = crate::W<NSBOOTR_PRGrs>;
#[doc = "Field `NSBOOT_LOCK` reader - A field locking the values of SWAP_ BANK, and NSBOOTADD settings."]
pub type NSBOOT_LOCK_R = crate::FieldReader;
#[doc = "Field `NSBOOT_LOCK` writer - A field locking the values of SWAP_ BANK, and NSBOOTADD settings."]
pub type NSBOOT_LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NSBOOTADD` reader - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address"]
pub type NSBOOTADD_R = crate::FieldReader<u32>;
#[doc = "Field `NSBOOTADD` writer - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address"]
pub type NSBOOTADD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - A field locking the values of SWAP_ BANK, and NSBOOTADD settings."]
    #[inline(always)]
    pub fn nsboot_lock(&self) -> NSBOOT_LOCK_R {
        NSBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address"]
    #[inline(always)]
    pub fn nsbootadd(&self) -> NSBOOTADD_R {
        NSBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - A field locking the values of SWAP_ BANK, and NSBOOTADD settings."]
    #[inline(always)]
    #[must_use]
    pub fn nsboot_lock(&mut self) -> NSBOOT_LOCK_W<NSBOOTR_PRGrs> {
        NSBOOT_LOCK_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address"]
    #[inline(always)]
    #[must_use]
    pub fn nsbootadd(&mut self) -> NSBOOTADD_W<NSBOOTR_PRGrs> {
        NSBOOTADD_W::new(self, 8)
    }
}
#[doc = "FLASH non-secure boot register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsbootr_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsbootr_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSBOOTR_PRGrs;
impl crate::RegisterSpec for NSBOOTR_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsbootr_prg::R`](R) reader structure"]
impl crate::Readable for NSBOOTR_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`nsbootr_prg::W`](W) writer structure"]
impl crate::Writable for NSBOOTR_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSBOOTR_PRG to value 0"]
impl crate::Resettable for NSBOOTR_PRGrs {
    const RESET_VALUE: u32 = 0;
}
