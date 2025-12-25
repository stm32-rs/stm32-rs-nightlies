///Register `DIR` writer
pub type W = crate::W<DIRrs>;
///Field `INTERRUPT_ID` writer - INTERRUPT_ID
pub type INTERRUPT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CPUID` writer - CPUID
pub type CPUID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DIRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:9 - INTERRUPT_ID
    #[inline(always)]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<'_, DIRrs> {
        INTERRUPT_ID_W::new(self, 0)
    }
    ///Bit 10 - CPUID
    #[inline(always)]
    pub fn cpuid(&mut self) -> CPUID_W<'_, DIRrs> {
        CPUID_W::new(self, 10)
    }
}
/**GICC deactivate interrupt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICC:DIR)*/
pub struct DIRrs;
impl crate::RegisterSpec for DIRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dir::W`](W) writer structure
impl crate::Writable for DIRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIR to value 0
impl crate::Resettable for DIRrs {}
