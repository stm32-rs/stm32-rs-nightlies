///Register `GICV_DIR` writer
pub type W = crate::W<GICV_DIRrs>;
///Field `INTERRUPT_ID` writer - INTERRUPT_ID
pub type INTERRUPT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CPUID` writer - CPUID
pub type CPUID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GICV_DIRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:9 - INTERRUPT_ID
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<GICV_DIRrs> {
        INTERRUPT_ID_W::new(self, 0)
    }
    ///Bit 10 - CPUID
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<GICV_DIRrs> {
        CPUID_W::new(self, 10)
    }
}
/**GICV VM deactivate interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_dir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICV:GICV_DIR)*/
pub struct GICV_DIRrs;
impl crate::RegisterSpec for GICV_DIRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gicv_dir::W`](W) writer structure
impl crate::Writable for GICV_DIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICV_DIR to value 0
impl crate::Resettable for GICV_DIRrs {
    const RESET_VALUE: u32 = 0;
}
