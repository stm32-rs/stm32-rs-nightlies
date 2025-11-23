///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `CIREF` writer - Clear Interrupt rising edge flag
pub type CIREF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIHLF` writer - Clear Interrupt high-level flag
pub type CIHLF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIFEF` writer - Clear Interrupt falling edge flag
pub type CIFEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear Interrupt rising edge flag
    #[inline(always)]
    pub fn ciref(&mut self) -> CIREF_W<'_, ICRrs> {
        CIREF_W::new(self, 0)
    }
    ///Bit 1 - Clear Interrupt high-level flag
    #[inline(always)]
    pub fn cihlf(&mut self) -> CIHLF_W<'_, ICRrs> {
        CIHLF_W::new(self, 1)
    }
    ///Bit 2 - Clear Interrupt falling edge flag
    #[inline(always)]
    pub fn cifef(&mut self) -> CIFEF_W<'_, ICRrs> {
        CIFEF_W::new(self, 2)
    }
}
/**FMC NAND controller interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#FMC1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
