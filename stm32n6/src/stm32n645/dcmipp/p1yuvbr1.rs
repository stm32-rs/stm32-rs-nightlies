///Register `P1YUVBR1` reader
pub type R = crate::R<P1YUVBR1rs>;
///Register `P1YUVBR1` writer
pub type W = crate::W<P1YUVBR1rs>;
///Field `BR` reader - Coefficient row 3 column 1 of the matrix
pub type BR_R = crate::FieldReader<u16>;
///Field `BR` writer - Coefficient row 3 column 1 of the matrix
pub type BR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `BG` reader - Coefficient row 3 column 2 of the matrix
pub type BG_R = crate::FieldReader<u16>;
///Field `BG` writer - Coefficient row 3 column 2 of the matrix
pub type BG_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - Coefficient row 3 column 1 of the matrix
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Coefficient row 3 column 2 of the matrix
    #[inline(always)]
    pub fn bg(&self) -> BG_R {
        BG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1YUVBR1")
            .field("br", &self.br())
            .field("bg", &self.bg())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Coefficient row 3 column 1 of the matrix
    #[inline(always)]
    pub fn br(&mut self) -> BR_W<'_, P1YUVBR1rs> {
        BR_W::new(self, 0)
    }
    ///Bits 16:26 - Coefficient row 3 column 2 of the matrix
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<'_, P1YUVBR1rs> {
        BG_W::new(self, 16)
    }
}
/**DCMIPP Pipe1 YUVConv blue coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1yuvbr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1yuvbr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1YUVBR1)*/
pub struct P1YUVBR1rs;
impl crate::RegisterSpec for P1YUVBR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1yuvbr1::R`](R) reader structure
impl crate::Readable for P1YUVBR1rs {}
///`write(|w| ..)` method takes [`p1yuvbr1::W`](W) writer structure
impl crate::Writable for P1YUVBR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1YUVBR1 to value 0
impl crate::Resettable for P1YUVBR1rs {}
