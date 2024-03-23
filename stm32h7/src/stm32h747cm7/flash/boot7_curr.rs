#[doc = "Register `BOOT7_CURR` reader"]
pub type R = crate::R<BOOT7_CURRrs>;
#[doc = "Register `BOOT7_CURR` writer"]
pub type W = crate::W<BOOT7_CURRrs>;
#[doc = "Field `BOOT_CM7_ADD0` reader - Arm Cortex-M7 boot address 0"]
pub type BOOT_CM7_ADD0_R = crate::FieldReader<u16>;
#[doc = "Field `BOOT_CM7_ADD0` writer - Arm Cortex-M7 boot address 0"]
pub type BOOT_CM7_ADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BOOT_CM7_ADD1` reader - Arm Cortex-M7 boot address 1"]
pub type BOOT_CM7_ADD1_R = crate::FieldReader<u16>;
#[doc = "Field `BOOT_CM7_ADD1` writer - Arm Cortex-M7 boot address 1"]
pub type BOOT_CM7_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Arm Cortex-M7 boot address 0"]
    #[inline(always)]
    pub fn boot_cm7_add0(&self) -> BOOT_CM7_ADD0_R {
        BOOT_CM7_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Arm Cortex-M7 boot address 1"]
    #[inline(always)]
    pub fn boot_cm7_add1(&self) -> BOOT_CM7_ADD1_R {
        BOOT_CM7_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Arm Cortex-M7 boot address 0"]
    #[inline(always)]
    #[must_use]
    pub fn boot_cm7_add0(&mut self) -> BOOT_CM7_ADD0_W<BOOT7_CURRrs> {
        BOOT_CM7_ADD0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Arm Cortex-M7 boot address 1"]
    #[inline(always)]
    #[must_use]
    pub fn boot_cm7_add1(&mut self) -> BOOT_CM7_ADD1_W<BOOT7_CURRrs> {
        BOOT_CM7_ADD1_W::new(self, 16)
    }
}
#[doc = "FLASH register boot address for Arm Cortex-M7 core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot7_curr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot7_curr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT7_CURRrs;
impl crate::RegisterSpec for BOOT7_CURRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot7_curr::R`](R) reader structure"]
impl crate::Readable for BOOT7_CURRrs {}
#[doc = "`write(|w| ..)` method takes [`boot7_curr::W`](W) writer structure"]
impl crate::Writable for BOOT7_CURRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT7_CURR to value 0"]
impl crate::Resettable for BOOT7_CURRrs {
    const RESET_VALUE: u32 = 0;
}
