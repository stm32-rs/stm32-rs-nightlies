///Register `L1CLUTWR` writer
pub type W = crate::W<L1CLUTWRrs>;
///Field `BLUE` writer - Blue value
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `GREEN` writer - Green value
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `RED` writer - Red value
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `CLUTADD` writer - CLUT Address
pub type CLUTADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<L1CLUTWRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Blue value
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<L1CLUTWRrs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 8:15 - Green value
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<L1CLUTWRrs> {
        GREEN_W::new(self, 8)
    }
    ///Bits 16:23 - Red value
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<L1CLUTWRrs> {
        RED_W::new(self, 16)
    }
    ///Bits 24:31 - CLUT Address
    #[inline(always)]
    pub fn clutadd(&mut self) -> CLUTADD_W<L1CLUTWRrs> {
        CLUTADD_W::new(self, 24)
    }
}
/**LTDC Layerx CLUT Write Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1clutwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#LTDC:L1CLUTWR)*/
pub struct L1CLUTWRrs;
impl crate::RegisterSpec for L1CLUTWRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`l1clutwr::W`](W) writer structure
impl crate::Writable for L1CLUTWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1CLUTWR to value 0
impl crate::Resettable for L1CLUTWRrs {
    const RESET_VALUE: u32 = 0;
}
