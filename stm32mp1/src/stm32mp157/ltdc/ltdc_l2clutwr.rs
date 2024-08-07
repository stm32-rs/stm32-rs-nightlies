///Register `LTDC_L2CLUTWR` writer
pub type W = crate::W<LTDC_L2CLUTWRrs>;
///Field `BLUE` writer - BLUE
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GREEN` writer - GREEN
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RED` writer - RED
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLUTADD` writer - CLUTADD
pub type CLUTADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<LTDC_L2CLUTWRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - BLUE
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<LTDC_L2CLUTWRrs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 8:15 - GREEN
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<LTDC_L2CLUTWRrs> {
        GREEN_W::new(self, 8)
    }
    ///Bits 16:23 - RED
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<LTDC_L2CLUTWRrs> {
        RED_W::new(self, 16)
    }
    ///Bits 24:31 - CLUTADD
    #[inline(always)]
    #[must_use]
    pub fn clutadd(&mut self) -> CLUTADD_W<LTDC_L2CLUTWRrs> {
        CLUTADD_W::new(self, 24)
    }
}
/**This register defines the CLUT address and the RGB value.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2clutwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LTDC:LTDC_L2CLUTWR)*/
pub struct LTDC_L2CLUTWRrs;
impl crate::RegisterSpec for LTDC_L2CLUTWRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ltdc_l2clutwr::W`](W) writer structure
impl crate::Writable for LTDC_L2CLUTWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_L2CLUTWR to value 0
impl crate::Resettable for LTDC_L2CLUTWRrs {
    const RESET_VALUE: u32 = 0;
}
