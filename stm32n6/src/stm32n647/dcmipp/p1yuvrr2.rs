///Register `P1YUVRR2` reader
pub type R = crate::R<P1YUVRR2rs>;
///Register `P1YUVRR2` writer
pub type W = crate::W<P1YUVRR2rs>;
///Field `RB` reader - Coefficient row 1 column 3 of the matrix
pub type RB_R = crate::FieldReader<u16>;
///Field `RB` writer - Coefficient row 1 column 3 of the matrix
pub type RB_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `RA` reader - Coefficient row 1 of the added column (signed integer value)
pub type RA_R = crate::FieldReader<u16>;
///Field `RA` writer - Coefficient row 1 of the added column (signed integer value)
pub type RA_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:10 - Coefficient row 1 column 3 of the matrix
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:25 - Coefficient row 1 of the added column (signed integer value)
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1YUVRR2")
            .field("rb", &self.rb())
            .field("ra", &self.ra())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Coefficient row 1 column 3 of the matrix
    #[inline(always)]
    pub fn rb(&mut self) -> RB_W<'_, P1YUVRR2rs> {
        RB_W::new(self, 0)
    }
    ///Bits 16:25 - Coefficient row 1 of the added column (signed integer value)
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<'_, P1YUVRR2rs> {
        RA_W::new(self, 16)
    }
}
/**DCMIPP Pipe1 YUVConv red coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1yuvrr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1yuvrr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1YUVRR2)*/
pub struct P1YUVRR2rs;
impl crate::RegisterSpec for P1YUVRR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1yuvrr2::R`](R) reader structure
impl crate::Readable for P1YUVRR2rs {}
///`write(|w| ..)` method takes [`p1yuvrr2::W`](W) writer structure
impl crate::Writable for P1YUVRR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1YUVRR2 to value 0
impl crate::Resettable for P1YUVRR2rs {}
