///Register `L2CLUTWR` writer
pub type W = crate::W<L2CLUTWRrs>;
///Field `BLUE` writer - BLUE
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GREEN` writer - GREEN
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RED` writer - RED
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLUTADD` writer - CLUTADD
pub type CLUTADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<L2CLUTWRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - BLUE
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<'_, L2CLUTWRrs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 8:15 - GREEN
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<'_, L2CLUTWRrs> {
        GREEN_W::new(self, 8)
    }
    ///Bits 16:23 - RED
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<'_, L2CLUTWRrs> {
        RED_W::new(self, 16)
    }
    ///Bits 24:31 - CLUTADD
    #[inline(always)]
    pub fn clutadd(&mut self) -> CLUTADD_W<'_, L2CLUTWRrs> {
        CLUTADD_W::new(self, 24)
    }
}
/**This register defines the CLUT address and the RGB value.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2clutwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2CLUTWR)*/
pub struct L2CLUTWRrs;
impl crate::RegisterSpec for L2CLUTWRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`l2clutwr::W`](W) writer structure
impl crate::Writable for L2CLUTWRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2CLUTWR to value 0
impl crate::Resettable for L2CLUTWRrs {}
