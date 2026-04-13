///Register `ICR2` writer
pub type W = crate::W<ICR2rs>;
///Field `CLIF` writer - clears the Line interrupt flag
pub type CLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFUWIF` writer - clears the FIFO underrun warning interrupt flag
pub type CFUWIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTERRIF` writer - clears the Transfer Error interrupt flag
pub type CTERRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRRIF` writer - clears register reload interrupt flag
pub type CRRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFUIF` writer - clears the FIFO underrun interrupt flag
pub type CFUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRCIF` writer - clears the CRC error interrupt flag
pub type CCRCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clears the Line interrupt flag
    #[inline(always)]
    pub fn clif(&mut self) -> CLIF_W<'_, ICR2rs> {
        CLIF_W::new(self, 0)
    }
    ///Bit 1 - clears the FIFO underrun warning interrupt flag
    #[inline(always)]
    pub fn cfuwif(&mut self) -> CFUWIF_W<'_, ICR2rs> {
        CFUWIF_W::new(self, 1)
    }
    ///Bit 2 - clears the Transfer Error interrupt flag
    #[inline(always)]
    pub fn cterrif(&mut self) -> CTERRIF_W<'_, ICR2rs> {
        CTERRIF_W::new(self, 2)
    }
    ///Bit 3 - clears register reload interrupt flag
    #[inline(always)]
    pub fn crrif(&mut self) -> CRRIF_W<'_, ICR2rs> {
        CRRIF_W::new(self, 3)
    }
    ///Bit 6 - clears the FIFO underrun interrupt flag
    #[inline(always)]
    pub fn cfuif(&mut self) -> CFUIF_W<'_, ICR2rs> {
        CFUIF_W::new(self, 6)
    }
    ///Bit 7 - clears the CRC error interrupt flag
    #[inline(always)]
    pub fn ccrcif(&mut self) -> CCRCIF_W<'_, ICR2rs> {
        CCRCIF_W::new(self, 7)
    }
}
/**LTDC interrupt clear register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#LTDC:ICR2)*/
pub struct ICR2rs;
impl crate::RegisterSpec for ICR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr2::W`](W) writer structure
impl crate::Writable for ICR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR2 to value 0
impl crate::Resettable for ICR2rs {}
