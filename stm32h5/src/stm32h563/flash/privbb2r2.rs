#[doc = "Register `PRIVBB2R2` reader"]
pub type R = crate::R<PRIVBB2R2rs>;
#[doc = "Register `PRIVBB2R2` writer"]
pub type W = crate::W<PRIVBB2R2rs>;
#[doc = "Field `PRIVBB2` reader - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute"]
pub type PRIVBB2_R = crate::FieldReader<u32>;
#[doc = "Field `PRIVBB2` writer - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute"]
pub type PRIVBB2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute"]
    #[inline(always)]
    pub fn privbb2(&self) -> PRIVBB2_R {
        PRIVBB2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute"]
    #[inline(always)]
    #[must_use]
    pub fn privbb2(&mut self) -> PRIVBB2_W<PRIVBB2R2rs> {
        PRIVBB2_W::new(self, 0)
    }
}
#[doc = "FLASH privilege block-based register for Bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privbb2r2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privbb2r2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVBB2R2rs;
impl crate::RegisterSpec for PRIVBB2R2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privbb2r2::R`](R) reader structure"]
impl crate::Readable for PRIVBB2R2rs {}
#[doc = "`write(|w| ..)` method takes [`privbb2r2::W`](W) writer structure"]
impl crate::Writable for PRIVBB2R2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVBB2R2 to value 0"]
impl crate::Resettable for PRIVBB2R2rs {
    const RESET_VALUE: u32 = 0;
}
