///Register `P1CCCRR2` reader
pub type R = crate::R<P1CCCRR2rs>;
///Field `RB` reader - Current coefficient row 1 column 3 of the matrix
pub type RB_R = crate::FieldReader<u16>;
///Field `RA` reader - Current coefficient row 1 of the added column (signed integer value)
pub type RA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:10 - Current coefficient row 1 column 3 of the matrix
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:25 - Current coefficient row 1 of the added column (signed integer value)
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCCRR2")
            .field("rb", &self.rb())
            .field("ra", &self.ra())
            .finish()
    }
}
/**DCMIPP Pipe1 current ColorConv red coefficient register 2

You can [`read`](crate::Reg::read) this register and get [`p1cccrr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1CCCRR2)*/
pub struct P1CCCRR2rs;
impl crate::RegisterSpec for P1CCCRR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1cccrr2::R`](R) reader structure
impl crate::Readable for P1CCCRR2rs {}
///`reset()` method sets P1CCCRR2 to value 0
impl crate::Resettable for P1CCCRR2rs {}
