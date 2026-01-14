///Register `P1CCGR2` reader
pub type R = crate::R<P1CCGR2rs>;
///Register `P1CCGR2` writer
pub type W = crate::W<P1CCGR2rs>;
///Field `GB` reader - Coefficient row 2 column 3 of the matrix
pub type GB_R = crate::FieldReader<u16>;
///Field `GB` writer - Coefficient row 2 column 3 of the matrix
pub type GB_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `GA` reader - Coefficient row 2 of the added column (signed integer value)
pub type GA_R = crate::FieldReader<u16>;
///Field `GA` writer - Coefficient row 2 of the added column (signed integer value)
pub type GA_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:10 - Coefficient row 2 column 3 of the matrix
    #[inline(always)]
    pub fn gb(&self) -> GB_R {
        GB_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:25 - Coefficient row 2 of the added column (signed integer value)
    #[inline(always)]
    pub fn ga(&self) -> GA_R {
        GA_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCGR2")
            .field("gb", &self.gb())
            .field("ga", &self.ga())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Coefficient row 2 column 3 of the matrix
    #[inline(always)]
    pub fn gb(&mut self) -> GB_W<'_, P1CCGR2rs> {
        GB_W::new(self, 0)
    }
    ///Bits 16:25 - Coefficient row 2 of the added column (signed integer value)
    #[inline(always)]
    pub fn ga(&mut self) -> GA_W<'_, P1CCGR2rs> {
        GA_W::new(self, 16)
    }
}
/**DCMIPP Pipe1 ColorConv green coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1ccgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ccgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1CCGR2)*/
pub struct P1CCGR2rs;
impl crate::RegisterSpec for P1CCGR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1ccgr2::R`](R) reader structure
impl crate::Readable for P1CCGR2rs {}
///`write(|w| ..)` method takes [`p1ccgr2::W`](W) writer structure
impl crate::Writable for P1CCGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1CCGR2 to value 0
impl crate::Resettable for P1CCGR2rs {}
