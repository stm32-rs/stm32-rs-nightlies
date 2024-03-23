#[doc = "Register `PRIVBB1R` reader"]
pub type R = crate::R<PRIVBB1Rrs>;
#[doc = "Register `PRIVBB1R` writer"]
pub type W = crate::W<PRIVBB1Rrs>;
#[doc = "Field `PRIVBB1` reader - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)"]
pub type PRIVBB1_R = crate::FieldReader;
#[doc = "Field `PRIVBB1` writer - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)"]
pub type PRIVBB1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)"]
    #[inline(always)]
    pub fn privbb1(&self) -> PRIVBB1_R {
        PRIVBB1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn privbb1(&mut self) -> PRIVBB1_W<PRIVBB1Rrs> {
        PRIVBB1_W::new(self, 0)
    }
}
#[doc = "FLASH privilege register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privbb1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privbb1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVBB1Rrs;
impl crate::RegisterSpec for PRIVBB1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privbb1r::R`](R) reader structure"]
impl crate::Readable for PRIVBB1Rrs {}
#[doc = "`write(|w| ..)` method takes [`privbb1r::W`](W) writer structure"]
impl crate::Writable for PRIVBB1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVBB1R to value 0"]
impl crate::Resettable for PRIVBB1Rrs {
    const RESET_VALUE: u32 = 0;
}
