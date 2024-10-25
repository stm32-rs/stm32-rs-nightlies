///Register `SHIFTR` writer
pub type W = crate::W<SHIFTRrs>;
///Field `SUBFS` writer - SUBFS
pub type SUBFS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `ADD1S` writer - ADD1S
pub type ADD1S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SHIFTRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:14 - SUBFS
    #[inline(always)]
    #[must_use]
    pub fn subfs(&mut self) -> SUBFS_W<SHIFTRrs> {
        SUBFS_W::new(self, 0)
    }
    ///Bit 31 - ADD1S
    #[inline(always)]
    #[must_use]
    pub fn add1s(&mut self) -> ADD1S_W<SHIFTRrs> {
        ADD1S_W::new(self, 31)
    }
}
/**This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RTC:SHIFTR)*/
pub struct SHIFTRrs;
impl crate::RegisterSpec for SHIFTRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`shiftr::W`](W) writer structure
impl crate::Writable for SHIFTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SHIFTR to value 0
impl crate::Resettable for SHIFTRrs {
    const RESET_VALUE: u32 = 0;
}
