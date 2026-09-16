///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `B0OIE` reader - Buffer 0 overflow interrupt enable
pub type B0OIE_R = crate::BitReader;
///Field `B0OIE` writer - Buffer 0 overflow interrupt enable
pub type B0OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B1OIE` reader - Buffer 1 overflow interrupt enable
pub type B1OIE_R = crate::BitReader;
///Field `B1OIE` writer - Buffer 1 overflow interrupt enable
pub type B1OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2OIE` reader - Buffer 2 overflow interrupt enable
pub type B2OIE_R = crate::BitReader;
///Field `B2OIE` writer - Buffer 2 overflow interrupt enable
pub type B2OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B3OIE` reader - Buffer 3 overflow interrupt enable
pub type B3OIE_R = crate::BitReader;
///Field `B3OIE` writer - Buffer 3 overflow interrupt enable
pub type B3OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AMEIE` reader - AHB master error interrupt enable
pub type AMEIE_R = crate::BitReader;
///Field `AMEIE` writer - AHB master error interrupt enable
pub type AMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BM192` reader - 192 Block mode
pub type BM192_R = crate::BitReader;
///Field `BM192` writer - 192 Block mode
pub type BM192_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Buffer 0 overflow interrupt enable
    #[inline(always)]
    pub fn b0oie(&self) -> B0OIE_R {
        B0OIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Buffer 1 overflow interrupt enable
    #[inline(always)]
    pub fn b1oie(&self) -> B1OIE_R {
        B1OIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Buffer 2 overflow interrupt enable
    #[inline(always)]
    pub fn b2oie(&self) -> B2OIE_R {
        B2OIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Buffer 3 overflow interrupt enable
    #[inline(always)]
    pub fn b3oie(&self) -> B3OIE_R {
        B3OIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AHB master error interrupt enable
    #[inline(always)]
    pub fn ameie(&self) -> AMEIE_R {
        AMEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - 192 Block mode
    #[inline(always)]
    pub fn bm192(&self) -> BM192_R {
        BM192_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("b0oie", &self.b0oie())
            .field("b1oie", &self.b1oie())
            .field("b2oie", &self.b2oie())
            .field("b3oie", &self.b3oie())
            .field("ameie", &self.ameie())
            .field("bm192", &self.bm192())
            .finish()
    }
}
impl W {
    ///Bit 0 - Buffer 0 overflow interrupt enable
    #[inline(always)]
    pub fn b0oie(&mut self) -> B0OIE_W<'_, CRrs> {
        B0OIE_W::new(self, 0)
    }
    ///Bit 1 - Buffer 1 overflow interrupt enable
    #[inline(always)]
    pub fn b1oie(&mut self) -> B1OIE_W<'_, CRrs> {
        B1OIE_W::new(self, 1)
    }
    ///Bit 2 - Buffer 2 overflow interrupt enable
    #[inline(always)]
    pub fn b2oie(&mut self) -> B2OIE_W<'_, CRrs> {
        B2OIE_W::new(self, 2)
    }
    ///Bit 3 - Buffer 3 overflow interrupt enable
    #[inline(always)]
    pub fn b3oie(&mut self) -> B3OIE_W<'_, CRrs> {
        B3OIE_W::new(self, 3)
    }
    ///Bit 4 - AHB master error interrupt enable
    #[inline(always)]
    pub fn ameie(&mut self) -> AMEIE_W<'_, CRrs> {
        AMEIE_W::new(self, 4)
    }
    ///Bit 6 - 192 Block mode
    #[inline(always)]
    pub fn bm192(&mut self) -> BM192_W<'_, CRrs> {
        BM192_W::new(self, 6)
    }
}
/**Graphic MMU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#GFXMMU:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
