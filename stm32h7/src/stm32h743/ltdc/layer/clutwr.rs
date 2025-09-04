///Register `CLUTWR` writer
pub type W = crate::W<CLUTWRrs>;
///Field `BLUE` writer - Blue value
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `GREEN` writer - Green value
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `RED` writer - Red value
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `CLUTADD` writer - CLUT Address
pub type CLUTADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<CLUTWRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Blue value
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<CLUTWRrs> {
        BLUE_W::new(self, 0)
    }
    ///Bits 8:15 - Green value
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<CLUTWRrs> {
        GREEN_W::new(self, 8)
    }
    ///Bits 16:23 - Red value
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<CLUTWRrs> {
        RED_W::new(self, 16)
    }
    ///Bits 24:31 - CLUT Address
    #[inline(always)]
    pub fn clutadd(&mut self) -> CLUTADD_W<CLUTWRrs> {
        CLUTADD_W::new(self, 24)
    }
}
/**Layerx CLUT Write Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clutwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLUTWRrs;
impl crate::RegisterSpec for CLUTWRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`clutwr::W`](W) writer structure
impl crate::Writable for CLUTWRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLUTWR to value 0
impl crate::Resettable for CLUTWRrs {}
