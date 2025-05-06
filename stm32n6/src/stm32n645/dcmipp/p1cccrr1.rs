///Register `P1CCCRR1` reader
pub type R = crate::R<P1CCCRR1rs>;
///Field `RR` reader - Current coefficient row 1 column 1 of the matrix
pub type RR_R = crate::FieldReader<u16>;
///Field `RG` reader - Current coefficient row 1 column 2 of the matrix
pub type RG_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:10 - Current coefficient row 1 column 1 of the matrix
    #[inline(always)]
    pub fn rr(&self) -> RR_R {
        RR_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Current coefficient row 1 column 2 of the matrix
    #[inline(always)]
    pub fn rg(&self) -> RG_R {
        RG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCCRR1")
            .field("rr", &self.rr())
            .field("rg", &self.rg())
            .finish()
    }
}
/**DCMIPP Pipe1 current ColorConv red coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1cccrr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1CCCRR1)*/
pub struct P1CCCRR1rs;
impl crate::RegisterSpec for P1CCCRR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1cccrr1::R`](R) reader structure
impl crate::Readable for P1CCCRR1rs {}
///`reset()` method sets P1CCCRR1 to value 0
impl crate::Resettable for P1CCCRR1rs {}
