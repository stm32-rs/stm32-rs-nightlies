///Register `NSBOOTADD1R` writer
pub type W = crate::W<NSBOOTADD1Rrs>;
///Field `NSBOOTADD1` writer - NSBOOTADD1
pub type NSBOOTADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl core::fmt::Debug for crate::generic::Reg<NSBOOTADD1Rrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 7:31 - NSBOOTADD1
    #[inline(always)]
    pub fn nsbootadd1(&mut self) -> NSBOOTADD1_W<'_, NSBOOTADD1Rrs> {
        NSBOOTADD1_W::new(self, 7)
    }
}
/**Flash non-secure boot address 1 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootadd1r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#FLASH:NSBOOTADD1R)*/
pub struct NSBOOTADD1Rrs;
impl crate::RegisterSpec for NSBOOTADD1Rrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`nsbootadd1r::W`](W) writer structure
impl crate::Writable for NSBOOTADD1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSBOOTADD1R to value 0x0f
impl crate::Resettable for NSBOOTADD1Rrs {
    const RESET_VALUE: u32 = 0x0f;
}
