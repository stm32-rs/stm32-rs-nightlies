///Register `P1CCCBR1` reader
pub type R = crate::R<P1CCCBR1rs>;
///Field `BR` reader - Current coefficient row 3 column 1 of the matrix
pub type BR_R = crate::FieldReader<u16>;
///Field `BG` reader - Current coefficient row 3 column 2 of the matrix
pub type BG_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:10 - Current coefficient row 3 column 1 of the matrix
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Current coefficient row 3 column 2 of the matrix
    #[inline(always)]
    pub fn bg(&self) -> BG_R {
        BG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCCBR1")
            .field("br", &self.br())
            .field("bg", &self.bg())
            .finish()
    }
}
/**DCMIPP Pipex current ColorConv blue coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1cccbr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1CCCBR1)*/
pub struct P1CCCBR1rs;
impl crate::RegisterSpec for P1CCCBR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1cccbr1::R`](R) reader structure
impl crate::Readable for P1CCCBR1rs {}
///`reset()` method sets P1CCCBR1 to value 0
impl crate::Resettable for P1CCCBR1rs {}
