#[doc = "Register `BOOT4_PRGR_` reader"]
pub type R = crate::R<BOOT4_PRGR_rs>;
#[doc = "Register `BOOT4_PRGR_` writer"]
pub type W = crate::W<BOOT4_PRGR_rs>;
#[doc = "Field `BOOT_CM4_ADD0` reader - Arm Cortex-M4 boot address 0 configuration"]
pub type BOOT_CM4_ADD0_R = crate::FieldReader<u16>;
#[doc = "Field `BOOT_CM4_ADD0` writer - Arm Cortex-M4 boot address 0 configuration"]
pub type BOOT_CM4_ADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BOOT_CM4_ADD1` reader - Arm Cortex-M4 boot address 1 configuration"]
pub type BOOT_CM4_ADD1_R = crate::FieldReader<u16>;
#[doc = "Field `BOOT_CM4_ADD1` writer - Arm Cortex-M4 boot address 1 configuration"]
pub type BOOT_CM4_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Arm Cortex-M4 boot address 0 configuration"]
    #[inline(always)]
    pub fn boot_cm4_add0(&self) -> BOOT_CM4_ADD0_R {
        BOOT_CM4_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Arm Cortex-M4 boot address 1 configuration"]
    #[inline(always)]
    pub fn boot_cm4_add1(&self) -> BOOT_CM4_ADD1_R {
        BOOT_CM4_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Arm Cortex-M4 boot address 0 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn boot_cm4_add0(&mut self) -> BOOT_CM4_ADD0_W<BOOT4_PRGR_rs> {
        BOOT_CM4_ADD0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Arm Cortex-M4 boot address 1 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn boot_cm4_add1(&mut self) -> BOOT_CM4_ADD1_W<BOOT4_PRGR_rs> {
        BOOT_CM4_ADD1_W::new(self, 16)
    }
}
#[doc = "FLASH register boot address for Arm Cortex-M4 core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot4_prgr_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot4_prgr_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT4_PRGR_rs;
impl crate::RegisterSpec for BOOT4_PRGR_rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot4_prgr_::R`](R) reader structure"]
impl crate::Readable for BOOT4_PRGR_rs {}
#[doc = "`write(|w| ..)` method takes [`boot4_prgr_::W`](W) writer structure"]
impl crate::Writable for BOOT4_PRGR_rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT4_PRGR_ to value 0"]
impl crate::Resettable for BOOT4_PRGR_rs {
    const RESET_VALUE: u32 = 0;
}
