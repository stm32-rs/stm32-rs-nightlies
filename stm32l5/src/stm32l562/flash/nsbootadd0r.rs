///Register `NSBOOTADD0R` writer
pub type W = crate::W<NSBOOTADD0Rrs>;
///Field `NSBOOTADD0` writer - NSBOOTADD0
pub type NSBOOTADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl core::fmt::Debug for crate::generic::Reg<NSBOOTADD0Rrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 7:31 - NSBOOTADD0
    #[inline(always)]
    pub fn nsbootadd0(&mut self) -> NSBOOTADD0_W<'_, NSBOOTADD0Rrs> {
        NSBOOTADD0_W::new(self, 7)
    }
}
/**Flash non-secure boot address 0 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootadd0r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#FLASH:NSBOOTADD0R)*/
pub struct NSBOOTADD0Rrs;
impl crate::RegisterSpec for NSBOOTADD0Rrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`nsbootadd0r::W`](W) writer structure
impl crate::Writable for NSBOOTADD0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSBOOTADD0R to value 0x0f
impl crate::Resettable for NSBOOTADD0Rrs {
    const RESET_VALUE: u32 = 0x0f;
}
