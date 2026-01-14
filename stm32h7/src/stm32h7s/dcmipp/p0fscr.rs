///Register `P0FSCR` reader
pub type R = crate::R<P0FSCRrs>;
///Register `P0FSCR` writer
pub type W = crate::W<P0FSCRrs>;
///Field `PIPEN` reader - Activation of PipeN Note: This bit is not shadowed, differently from all other bits in this register.
pub type PIPEN_R = crate::BitReader;
///Field `PIPEN` writer - Activation of PipeN Note: This bit is not shadowed, differently from all other bits in this register.
pub type PIPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - Activation of PipeN Note: This bit is not shadowed, differently from all other bits in this register.
    #[inline(always)]
    pub fn pipen(&self) -> PIPEN_R {
        PIPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0FSCR")
            .field("pipen", &self.pipen())
            .finish()
    }
}
impl W {
    ///Bit 31 - Activation of PipeN Note: This bit is not shadowed, differently from all other bits in this register.
    #[inline(always)]
    pub fn pipen(&mut self) -> PIPEN_W<'_, P0FSCRrs> {
        PIPEN_W::new(self, 31)
    }
}
/**DCMIPP Pipe0 flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p0fscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0fscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DCMIPP:P0FSCR)*/
pub struct P0FSCRrs;
impl crate::RegisterSpec for P0FSCRrs {
    type Ux = u32;
}
///`read()` method returns [`p0fscr::R`](R) reader structure
impl crate::Readable for P0FSCRrs {}
///`write(|w| ..)` method takes [`p0fscr::W`](W) writer structure
impl crate::Writable for P0FSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P0FSCR to value 0
impl crate::Resettable for P0FSCRrs {}
