///Register `P1GMCR` reader
pub type R = crate::R<P1GMCRrs>;
///Register `P1GMCR` writer
pub type W = crate::W<P1GMCRrs>;
///Field `ENABLE` reader - None
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - None
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1GMCR")
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P1GMCRrs> {
        ENABLE_W::new(self, 0)
    }
}
/**DCMIPP Pipex gamma configuration register

You can [`read`](crate::Reg::read) this register and get [`p1gmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1gmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1GMCR)*/
pub struct P1GMCRrs;
impl crate::RegisterSpec for P1GMCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1gmcr::R`](R) reader structure
impl crate::Readable for P1GMCRrs {}
///`write(|w| ..)` method takes [`p1gmcr::W`](W) writer structure
impl crate::Writable for P1GMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1GMCR to value 0
impl crate::Resettable for P1GMCRrs {}
