///Register `P1CCGR1` reader
pub type R = crate::R<P1CCGR1rs>;
///Register `P1CCGR1` writer
pub type W = crate::W<P1CCGR1rs>;
///Field `GR` reader - Coefficient row 2 column 1 of the matrix
pub type GR_R = crate::FieldReader<u16>;
///Field `GR` writer - Coefficient row 2 column 1 of the matrix
pub type GR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `GG` reader - Coefficient row 2 column 2 of the matrix
pub type GG_R = crate::FieldReader<u16>;
///Field `GG` writer - Coefficient row 2 column 2 of the matrix
pub type GG_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - Coefficient row 2 column 1 of the matrix
    #[inline(always)]
    pub fn gr(&self) -> GR_R {
        GR_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Coefficient row 2 column 2 of the matrix
    #[inline(always)]
    pub fn gg(&self) -> GG_R {
        GG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCGR1")
            .field("gr", &self.gr())
            .field("gg", &self.gg())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Coefficient row 2 column 1 of the matrix
    #[inline(always)]
    pub fn gr(&mut self) -> GR_W<P1CCGR1rs> {
        GR_W::new(self, 0)
    }
    ///Bits 16:26 - Coefficient row 2 column 2 of the matrix
    #[inline(always)]
    pub fn gg(&mut self) -> GG_W<P1CCGR1rs> {
        GG_W::new(self, 16)
    }
}
/**DCMIPP Pipe1 ColorConv green coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1ccgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ccgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1CCGR1)*/
pub struct P1CCGR1rs;
impl crate::RegisterSpec for P1CCGR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1ccgr1::R`](R) reader structure
impl crate::Readable for P1CCGR1rs {}
///`write(|w| ..)` method takes [`p1ccgr1::W`](W) writer structure
impl crate::Writable for P1CCGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1CCGR1 to value 0
impl crate::Resettable for P1CCGR1rs {}
