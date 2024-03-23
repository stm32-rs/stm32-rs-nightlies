#[doc = "Register `GICV_DIR` writer"]
pub type W = crate::W<GICV_DIRrs>;
#[doc = "Field `INTERRUPT_ID` writer - INTERRUPT_ID"]
pub type INTERRUPT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CPUID` writer - CPUID"]
pub type CPUID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:9 - INTERRUPT_ID"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<GICV_DIRrs> {
        INTERRUPT_ID_W::new(self, 0)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<GICV_DIRrs> {
        CPUID_W::new(self, 10)
    }
}
#[doc = "GICV VM deactivate interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicv_dir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICV_DIRrs;
impl crate::RegisterSpec for GICV_DIRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gicv_dir::W`](W) writer structure"]
impl crate::Writable for GICV_DIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICV_DIR to value 0"]
impl crate::Resettable for GICV_DIRrs {
    const RESET_VALUE: u32 = 0;
}
