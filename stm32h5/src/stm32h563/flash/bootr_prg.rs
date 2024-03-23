#[doc = "Register `BOOTR_PRG` reader"]
pub type R = crate::R<BOOTR_PRGrs>;
#[doc = "Register `BOOTR_PRG` writer"]
pub type W = crate::W<BOOTR_PRGrs>;
#[doc = "Field `SECBOOT_LOCK` reader - A field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting."]
pub type SECBOOT_LOCK_R = crate::FieldReader;
#[doc = "Field `SECBOOT_LOCK` writer - A field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting."]
pub type SECBOOT_LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SECBOOTADD` reader - Secure unique boot entry address. These bits allow configuring the secure UBE address."]
pub type SECBOOTADD_R = crate::FieldReader<u32>;
#[doc = "Field `SECBOOTADD` writer - Secure unique boot entry address. These bits allow configuring the secure UBE address."]
pub type SECBOOTADD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - A field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting."]
    #[inline(always)]
    pub fn secboot_lock(&self) -> SECBOOT_LOCK_R {
        SECBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Secure unique boot entry address. These bits allow configuring the secure UBE address."]
    #[inline(always)]
    pub fn secbootadd(&self) -> SECBOOTADD_R {
        SECBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - A field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting."]
    #[inline(always)]
    #[must_use]
    pub fn secboot_lock(&mut self) -> SECBOOT_LOCK_W<BOOTR_PRGrs> {
        SECBOOT_LOCK_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Secure unique boot entry address. These bits allow configuring the secure UBE address."]
    #[inline(always)]
    #[must_use]
    pub fn secbootadd(&mut self) -> SECBOOTADD_W<BOOTR_PRGrs> {
        SECBOOTADD_W::new(self, 8)
    }
}
#[doc = "FLASH secure boot register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bootr_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bootr_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTR_PRGrs;
impl crate::RegisterSpec for BOOTR_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootr_prg::R`](R) reader structure"]
impl crate::Readable for BOOTR_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`bootr_prg::W`](W) writer structure"]
impl crate::Writable for BOOTR_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTR_PRG to value 0"]
impl crate::Resettable for BOOTR_PRGrs {
    const RESET_VALUE: u32 = 0;
}
