///Register `P1CCCGR1` reader
pub type R = crate::R<P1CCCGR1rs>;
///Field `GR` reader - Current coefficient row 2 column 1 of the matrix
pub type GR_R = crate::FieldReader<u16>;
///Field `GG` reader - Current coefficient row 2 column 2 of the matrix
pub type GG_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:10 - Current coefficient row 2 column 1 of the matrix
    #[inline(always)]
    pub fn gr(&self) -> GR_R {
        GR_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Current coefficient row 2 column 2 of the matrix
    #[inline(always)]
    pub fn gg(&self) -> GG_R {
        GG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCCGR1")
            .field("gr", &self.gr())
            .field("gg", &self.gg())
            .finish()
    }
}
/**DCMIPP Pipe1 current ColorConv green coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1cccgr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CCCGR1)*/
pub struct P1CCCGR1rs;
impl crate::RegisterSpec for P1CCCGR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1cccgr1::R`](R) reader structure
impl crate::Readable for P1CCCGR1rs {}
///`reset()` method sets P1CCCGR1 to value 0
impl crate::Resettable for P1CCCGR1rs {}
