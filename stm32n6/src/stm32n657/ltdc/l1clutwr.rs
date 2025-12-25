///Register `L1CLUTWR` writer
pub type W = crate::W<L1CLUTWRrs>;
///Field `BLUE` writer - blue value
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GREEN` writer - green value
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RED` writer - red value
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLUTADD` writer - CLUT address
pub type CLUTADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<L1CLUTWRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - blue value
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<'_, L1CLUTWRrs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 8:15 - green value
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<'_, L1CLUTWRrs> {
        GREEN_W::new(self, 8)
    }
    ///Bits 16:23 - red value
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<'_, L1CLUTWRrs> {
        RED_W::new(self, 16)
    }
    ///Bits 24:31 - CLUT address
    #[inline(always)]
    pub fn clutadd(&mut self) -> CLUTADD_W<'_, L1CLUTWRrs> {
        CLUTADD_W::new(self, 24)
    }
}
/**LTDC layerx CLUT write register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1clutwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#LTDC:L1CLUTWR)*/
pub struct L1CLUTWRrs;
impl crate::RegisterSpec for L1CLUTWRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`l1clutwr::W`](W) writer structure
impl crate::Writable for L1CLUTWRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1CLUTWR to value 0
impl crate::Resettable for L1CLUTWRrs {}
