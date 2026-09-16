///Register `P1CCCBR2` reader
pub type R = crate::R<P1CCCBR2rs>;
///Field `BB` reader - Current coefficient row 3 column 3 of the matrix
pub type BB_R = crate::FieldReader<u16>;
///Field `BA` reader - Current coefficient row 3 of the added column (signed integer value)
pub type BA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:10 - Current coefficient row 3 column 3 of the matrix
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:25 - Current coefficient row 3 of the added column (signed integer value)
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCCBR2")
            .field("bb", &self.bb())
            .field("ba", &self.ba())
            .finish()
    }
}
/**DCMIPP Pipe1 current ColorConv blue coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1cccbr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1CCCBR2)*/
pub struct P1CCCBR2rs;
impl crate::RegisterSpec for P1CCCBR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1cccbr2::R`](R) reader structure
impl crate::Readable for P1CCCBR2rs {}
///`reset()` method sets P1CCCBR2 to value 0
impl crate::Resettable for P1CCCBR2rs {}
