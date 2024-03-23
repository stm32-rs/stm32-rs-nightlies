#[doc = "Register `SECBB1R3` reader"]
pub type R = crate::R<SECBB1R3rs>;
#[doc = "Register `SECBB1R3` writer"]
pub type W = crate::W<SECBB1R3rs>;
#[doc = "Field `SECBB1` reader - Secure/non-secure 8�Kbytes flash Bank 1 sector attributes"]
pub type SECBB1_R = crate::FieldReader<u32>;
#[doc = "Field `SECBB1` writer - Secure/non-secure 8�Kbytes flash Bank 1 sector attributes"]
pub type SECBB1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure/non-secure 8�Kbytes flash Bank 1 sector attributes"]
    #[inline(always)]
    pub fn secbb1(&self) -> SECBB1_R {
        SECBB1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure/non-secure 8�Kbytes flash Bank 1 sector attributes"]
    #[inline(always)]
    #[must_use]
    pub fn secbb1(&mut self) -> SECBB1_W<SECBB1R3rs> {
        SECBB1_W::new(self, 0)
    }
}
#[doc = "FLASH secure block based register for Bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secbb1r3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secbb1r3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECBB1R3rs;
impl crate::RegisterSpec for SECBB1R3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secbb1r3::R`](R) reader structure"]
impl crate::Readable for SECBB1R3rs {}
#[doc = "`write(|w| ..)` method takes [`secbb1r3::W`](W) writer structure"]
impl crate::Writable for SECBB1R3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECBB1R3 to value 0"]
impl crate::Resettable for SECBB1R3rs {
    const RESET_VALUE: u32 = 0;
}
