///Register `SHIFTR` writer
pub type W = crate::W<SHIFTRrs>;
///Field `SUBFS` writer - Subtract a fraction of a second
pub type SUBFS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `ADD1S` writer - Add one second
pub type ADD1S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SHIFTRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:14 - Subtract a fraction of a second
    #[inline(always)]
    pub fn subfs(&mut self) -> SUBFS_W<'_, SHIFTRrs> {
        SUBFS_W::new(self, 0)
    }
    ///Bit 31 - Add one second
    #[inline(always)]
    pub fn add1s(&mut self) -> ADD1S_W<'_, SHIFTRrs> {
        ADD1S_W::new(self, 31)
    }
}
/**shift control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#RTC:SHIFTR)*/
pub struct SHIFTRrs;
impl crate::RegisterSpec for SHIFTRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`shiftr::W`](W) writer structure
impl crate::Writable for SHIFTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SHIFTR to value 0
impl crate::Resettable for SHIFTRrs {}
