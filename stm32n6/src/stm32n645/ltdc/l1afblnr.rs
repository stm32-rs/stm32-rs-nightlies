///Register `L1AFBLNR` reader
pub type R = crate::R<L1AFBLNRrs>;
///Register `L1AFBLNR` writer
pub type W = crate::W<L1AFBLNRrs>;
///Field `AFBLNBR` reader - auxiliary frame buffer line number
pub type AFBLNBR_R = crate::FieldReader<u16>;
///Field `AFBLNBR` writer - auxiliary frame buffer line number
pub type AFBLNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - auxiliary frame buffer line number
    #[inline(always)]
    pub fn afblnbr(&self) -> AFBLNBR_R {
        AFBLNBR_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1AFBLNR")
            .field("afblnbr", &self.afblnbr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - auxiliary frame buffer line number
    #[inline(always)]
    pub fn afblnbr(&mut self) -> AFBLNBR_W<'_, L1AFBLNRrs> {
        AFBLNBR_W::new(self, 0)
    }
}
/**LTDC layer1 auxiliary frame buffer line number register

You can [`read`](crate::Reg::read) this register and get [`l1afblnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1afblnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1AFBLNR)*/
pub struct L1AFBLNRrs;
impl crate::RegisterSpec for L1AFBLNRrs {
    type Ux = u32;
}
///`read()` method returns [`l1afblnr::R`](R) reader structure
impl crate::Readable for L1AFBLNRrs {}
///`write(|w| ..)` method takes [`l1afblnr::W`](W) writer structure
impl crate::Writable for L1AFBLNRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1AFBLNR to value 0
impl crate::Resettable for L1AFBLNRrs {}
