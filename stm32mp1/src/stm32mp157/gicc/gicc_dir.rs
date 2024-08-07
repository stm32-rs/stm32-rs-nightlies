///Register `GICC_DIR` writer
pub type W = crate::W<GICC_DIRrs>;
///Field `INTERRUPT_ID` writer - INTERRUPT_ID
pub type INTERRUPT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CPUID` writer - CPUID
pub type CPUID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GICC_DIRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:9 - INTERRUPT_ID
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<GICC_DIRrs> {
        INTERRUPT_ID_W::new(self, 0)
    }
    ///Bit 10 - CPUID
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<GICC_DIRrs> {
        CPUID_W::new(self, 10)
    }
}
/**GICC deactivate interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_dir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICC:GICC_DIR)*/
pub struct GICC_DIRrs;
impl crate::RegisterSpec for GICC_DIRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gicc_dir::W`](W) writer structure
impl crate::Writable for GICC_DIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICC_DIR to value 0
impl crate::Resettable for GICC_DIRrs {
    const RESET_VALUE: u32 = 0;
}
