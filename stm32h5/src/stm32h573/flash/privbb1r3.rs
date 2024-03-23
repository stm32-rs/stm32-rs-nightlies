#[doc = "Register `PRIVBB1R3` reader"]
pub type R = crate::R<PRIVBB1R3rs>;
#[doc = "Register `PRIVBB1R3` writer"]
pub type W = crate::W<PRIVBB1R3rs>;
#[doc = "Field `PRIVBB1` reader - Privileged/unprivileged 8-Kbyte flash Bank 1 sector attribute"]
pub type PRIVBB1_R = crate::FieldReader<u32>;
#[doc = "Field `PRIVBB1` writer - Privileged/unprivileged 8-Kbyte flash Bank 1 sector attribute"]
pub type PRIVBB1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Privileged/unprivileged 8-Kbyte flash Bank 1 sector attribute"]
    #[inline(always)]
    pub fn privbb1(&self) -> PRIVBB1_R {
        PRIVBB1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Privileged/unprivileged 8-Kbyte flash Bank 1 sector attribute"]
    #[inline(always)]
    #[must_use]
    pub fn privbb1(&mut self) -> PRIVBB1_W<PRIVBB1R3rs> {
        PRIVBB1_W::new(self, 0)
    }
}
#[doc = "FLASH privilege block based register for Bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privbb1r3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privbb1r3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVBB1R3rs;
impl crate::RegisterSpec for PRIVBB1R3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privbb1r3::R`](R) reader structure"]
impl crate::Readable for PRIVBB1R3rs {}
#[doc = "`write(|w| ..)` method takes [`privbb1r3::W`](W) writer structure"]
impl crate::Writable for PRIVBB1R3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVBB1R3 to value 0"]
impl crate::Resettable for PRIVBB1R3rs {
    const RESET_VALUE: u32 = 0;
}
