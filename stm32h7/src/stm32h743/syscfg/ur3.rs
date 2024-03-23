#[doc = "Register `UR3` reader"]
pub type R = crate::R<UR3rs>;
#[doc = "Register `UR3` writer"]
pub type W = crate::W<UR3rs>;
#[doc = "Field `BOOT_ADD1` reader - Boot Address 1"]
pub type BOOT_ADD1_R = crate::FieldReader<u16>;
#[doc = "Field `BOOT_ADD1` writer - Boot Address 1"]
pub type BOOT_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Boot Address 1"]
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Boot Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn boot_add1(&mut self) -> BOOT_ADD1_W<UR3rs> {
        BOOT_ADD1_W::new(self, 16)
    }
}
#[doc = "SYSCFG user register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR3rs;
impl crate::RegisterSpec for UR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur3::R`](R) reader structure"]
impl crate::Readable for UR3rs {}
#[doc = "`write(|w| ..)` method takes [`ur3::W`](W) writer structure"]
impl crate::Writable for UR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UR3 to value 0"]
impl crate::Resettable for UR3rs {
    const RESET_VALUE: u32 = 0;
}
