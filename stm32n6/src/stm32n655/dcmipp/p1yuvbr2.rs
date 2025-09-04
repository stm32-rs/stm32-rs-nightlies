///Register `P1YUVBR2` reader
pub type R = crate::R<P1YUVBR2rs>;
///Register `P1YUVBR2` writer
pub type W = crate::W<P1YUVBR2rs>;
///Field `BB` reader - Coefficient row 3 column 3 of the matrix
pub type BB_R = crate::FieldReader<u16>;
///Field `BB` writer - Coefficient row 3 column 3 of the matrix
pub type BB_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `BA` reader - Coefficient row 3 of the added column (signed integer value)
pub type BA_R = crate::FieldReader<u16>;
///Field `BA` writer - Coefficient row 3 of the added column (signed integer value)
pub type BA_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:10 - Coefficient row 3 column 3 of the matrix
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:25 - Coefficient row 3 of the added column (signed integer value)
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1YUVBR2")
            .field("bb", &self.bb())
            .field("ba", &self.ba())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Coefficient row 3 column 3 of the matrix
    #[inline(always)]
    pub fn bb(&mut self) -> BB_W<P1YUVBR2rs> {
        BB_W::new(self, 0)
    }
    ///Bits 16:25 - Coefficient row 3 of the added column (signed integer value)
    #[inline(always)]
    pub fn ba(&mut self) -> BA_W<P1YUVBR2rs> {
        BA_W::new(self, 16)
    }
}
/**DCMIPP Pipe1 YUV blue coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1yuvbr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1yuvbr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1YUVBR2)*/
pub struct P1YUVBR2rs;
impl crate::RegisterSpec for P1YUVBR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1yuvbr2::R`](R) reader structure
impl crate::Readable for P1YUVBR2rs {}
///`write(|w| ..)` method takes [`p1yuvbr2::W`](W) writer structure
impl crate::Writable for P1YUVBR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1YUVBR2 to value 0
impl crate::Resettable for P1YUVBR2rs {}
