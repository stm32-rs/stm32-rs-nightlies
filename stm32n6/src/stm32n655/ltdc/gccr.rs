///Register `GCCR` writer
pub type W = crate::W<GCCRrs>;
///Field `ADDR` writer - address of the R,G,B table where the COMP component is written
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COMP` writer - color component to be written, in either (or all) the R,G,B tables
pub type COMP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BEN` writer - write trigger to the blue table
pub type BEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GEN` writer - write trigger to the green table
pub type GEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REN` writer - write trigger to the red table
pub type REN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GCCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - address of the R,G,B table where the COMP component is written
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<'_, GCCRrs> {
        ADDR_W::new(self, 0)
    }
    ///Bits 8:15 - color component to be written, in either (or all) the R,G,B tables
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W<'_, GCCRrs> {
        COMP_W::new(self, 8)
    }
    ///Bit 16 - write trigger to the blue table
    #[inline(always)]
    pub fn ben(&mut self) -> BEN_W<'_, GCCRrs> {
        BEN_W::new(self, 16)
    }
    ///Bit 17 - write trigger to the green table
    #[inline(always)]
    pub fn gen_(&mut self) -> GEN_W<'_, GCCRrs> {
        GEN_W::new(self, 17)
    }
    ///Bit 18 - write trigger to the red table
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W<'_, GCCRrs> {
        REN_W::new(self, 18)
    }
}
/**LTDC gamma correction configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#LTDC:GCCR)*/
pub struct GCCRrs;
impl crate::RegisterSpec for GCCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`gccr::W`](W) writer structure
impl crate::Writable for GCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCCR to value 0
impl crate::Resettable for GCCRrs {}
