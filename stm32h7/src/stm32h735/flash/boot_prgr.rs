#[doc = "Register `BOOT_PRGR` reader"]
pub type R = crate::R<BOOT_PRGRrs>;
#[doc = "Register `BOOT_PRGR` writer"]
pub type W = crate::W<BOOT_PRGRrs>;
#[doc = "Field `BOOT_ADD0` reader - Boot address 0"]
pub type BOOT_ADD0_R = crate::FieldReader<u16>;
#[doc = "Field `BOOT_ADD0` writer - Boot address 0"]
pub type BOOT_ADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BOOT_ADD1` reader - Boot address 1"]
pub type BOOT_ADD1_R = crate::FieldReader<u16>;
#[doc = "Field `BOOT_ADD1` writer - Boot address 1"]
pub type BOOT_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Boot address 0"]
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Boot address 1"]
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Boot address 0"]
    #[inline(always)]
    #[must_use]
    pub fn boot_add0(&mut self) -> BOOT_ADD0_W<BOOT_PRGRrs> {
        BOOT_ADD0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Boot address 1"]
    #[inline(always)]
    #[must_use]
    pub fn boot_add1(&mut self) -> BOOT_ADD1_W<BOOT_PRGRrs> {
        BOOT_ADD1_W::new(self, 16)
    }
}
#[doc = "FLASH register with boot address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_prgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_prgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_PRGRrs;
impl crate::RegisterSpec for BOOT_PRGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_prgr::R`](R) reader structure"]
impl crate::Readable for BOOT_PRGRrs {}
#[doc = "`write(|w| ..)` method takes [`boot_prgr::W`](W) writer structure"]
impl crate::Writable for BOOT_PRGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT_PRGR to value 0"]
impl crate::Resettable for BOOT_PRGRrs {
    const RESET_VALUE: u32 = 0;
}
