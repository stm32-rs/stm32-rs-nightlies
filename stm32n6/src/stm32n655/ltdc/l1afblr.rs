///Register `L1AFBLR` reader
pub type R = crate::R<L1AFBLRrs>;
///Register `L1AFBLR` writer
pub type W = crate::W<L1AFBLRrs>;
///Field `AFBLL` reader - auxiliary frame buffer line length
pub type AFBLL_R = crate::FieldReader<u16>;
///Field `AFBLL` writer - auxiliary frame buffer line length
pub type AFBLL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `AFBP` reader - auxiliary frame buffer pitch in bytes
pub type AFBP_R = crate::FieldReader<u16>;
///Field `AFBP` writer - auxiliary frame buffer pitch in bytes
pub type AFBP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - auxiliary frame buffer line length
    #[inline(always)]
    pub fn afbll(&self) -> AFBLL_R {
        AFBLL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - auxiliary frame buffer pitch in bytes
    #[inline(always)]
    pub fn afbp(&self) -> AFBP_R {
        AFBP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1AFBLR")
            .field("afbll", &self.afbll())
            .field("afbp", &self.afbp())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - auxiliary frame buffer line length
    #[inline(always)]
    pub fn afbll(&mut self) -> AFBLL_W<'_, L1AFBLRrs> {
        AFBLL_W::new(self, 0)
    }
    ///Bits 16:31 - auxiliary frame buffer pitch in bytes
    #[inline(always)]
    pub fn afbp(&mut self) -> AFBP_W<'_, L1AFBLRrs> {
        AFBP_W::new(self, 16)
    }
}
/**LTDC layer1 auxiliary frame buffer length register

You can [`read`](crate::Reg::read) this register and get [`l1afblr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1afblr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#LTDC:L1AFBLR)*/
pub struct L1AFBLRrs;
impl crate::RegisterSpec for L1AFBLRrs {
    type Ux = u32;
}
///`read()` method returns [`l1afblr::R`](R) reader structure
impl crate::Readable for L1AFBLRrs {}
///`write(|w| ..)` method takes [`l1afblr::W`](W) writer structure
impl crate::Writable for L1AFBLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1AFBLR to value 0
impl crate::Resettable for L1AFBLRrs {}
