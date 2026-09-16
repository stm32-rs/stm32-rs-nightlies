///Register `L2CFBLR` reader
pub type R = crate::R<L2CFBLRrs>;
///Register `L2CFBLR` writer
pub type W = crate::W<L2CFBLRrs>;
///Field `CFBLL` reader - Color Frame Buffer Line Length
pub type CFBLL_R = crate::FieldReader<u16>;
///Field `CFBLL` writer - Color Frame Buffer Line Length
pub type CFBLL_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `CFBP` reader - Color Frame Buffer Pitch in bytes
pub type CFBP_R = crate::FieldReader<u16>;
///Field `CFBP` writer - Color Frame Buffer Pitch in bytes
pub type CFBP_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - Color Frame Buffer Line Length
    #[inline(always)]
    pub fn cfbll(&self) -> CFBLL_R {
        CFBLL_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - Color Frame Buffer Pitch in bytes
    #[inline(always)]
    pub fn cfbp(&self) -> CFBP_R {
        CFBP_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2CFBLR")
            .field("cfbp", &self.cfbp())
            .field("cfbll", &self.cfbll())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - Color Frame Buffer Line Length
    #[inline(always)]
    pub fn cfbll(&mut self) -> CFBLL_W<'_, L2CFBLRrs> {
        CFBLL_W::new(self, 0)
    }
    ///Bits 16:28 - Color Frame Buffer Pitch in bytes
    #[inline(always)]
    pub fn cfbp(&mut self) -> CFBP_W<'_, L2CFBLRrs> {
        CFBP_W::new(self, 16)
    }
}
/**Layerx Color Frame Buffer Length Register

You can [`read`](crate::Reg::read) this register and get [`l2cfblr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfblr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#LTDC:L2CFBLR)*/
pub struct L2CFBLRrs;
impl crate::RegisterSpec for L2CFBLRrs {
    type Ux = u32;
}
///`read()` method returns [`l2cfblr::R`](R) reader structure
impl crate::Readable for L2CFBLRrs {}
///`write(|w| ..)` method takes [`l2cfblr::W`](W) writer structure
impl crate::Writable for L2CFBLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2CFBLR to value 0
impl crate::Resettable for L2CFBLRrs {}
