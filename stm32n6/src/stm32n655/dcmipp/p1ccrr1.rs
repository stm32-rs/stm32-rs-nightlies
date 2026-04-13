///Register `P1CCRR1` reader
pub type R = crate::R<P1CCRR1rs>;
///Register `P1CCRR1` writer
pub type W = crate::W<P1CCRR1rs>;
///Field `RR` reader - Coefficient row 1 column 1 of the matrix
pub type RR_R = crate::FieldReader<u16>;
///Field `RR` writer - Coefficient row 1 column 1 of the matrix
pub type RR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `RG` reader - Coefficient row 1 column 2 of the matrix
pub type RG_R = crate::FieldReader<u16>;
///Field `RG` writer - Coefficient row 1 column 2 of the matrix
pub type RG_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - Coefficient row 1 column 1 of the matrix
    #[inline(always)]
    pub fn rr(&self) -> RR_R {
        RR_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Coefficient row 1 column 2 of the matrix
    #[inline(always)]
    pub fn rg(&self) -> RG_R {
        RG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCRR1")
            .field("rr", &self.rr())
            .field("rg", &self.rg())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Coefficient row 1 column 1 of the matrix
    #[inline(always)]
    pub fn rr(&mut self) -> RR_W<'_, P1CCRR1rs> {
        RR_W::new(self, 0)
    }
    ///Bits 16:26 - Coefficient row 1 column 2 of the matrix
    #[inline(always)]
    pub fn rg(&mut self) -> RG_W<'_, P1CCRR1rs> {
        RG_W::new(self, 16)
    }
}
/**DCMIPP Pipe1 ColorConv red coefficient register 1

You can [`read`](crate::Reg::read) this register and get [`p1ccrr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ccrr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1CCRR1)*/
pub struct P1CCRR1rs;
impl crate::RegisterSpec for P1CCRR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1ccrr1::R`](R) reader structure
impl crate::Readable for P1CCRR1rs {}
///`write(|w| ..)` method takes [`p1ccrr1::W`](W) writer structure
impl crate::Writable for P1CCRR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1CCRR1 to value 0
impl crate::Resettable for P1CCRR1rs {}
