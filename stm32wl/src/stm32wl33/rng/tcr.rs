///Register `TCR` reader
pub type R = crate::R<TCRrs>;
///Register `TCR` writer
pub type W = crate::W<TCRrs>;
///Field `TCR` reader - Test-control register
pub type TCR_R = crate::BitReader;
///Field `TCR` writer - Test-control register
pub type TCR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Test-control register
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR").field("tcr", &self.tcr()).finish()
    }
}
impl W {
    ///Bit 0 - Test-control register
    #[inline(always)]
    pub fn tcr(&mut self) -> TCR_W<'_, TCRrs> {
        TCR_W::new(self, 0)
    }
}
/**RNG_TCR register

You can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:TCR)*/
pub struct TCRrs;
impl crate::RegisterSpec for TCRrs {
    type Ux = u32;
}
///`read()` method returns [`tcr::R`](R) reader structure
impl crate::Readable for TCRrs {}
///`write(|w| ..)` method takes [`tcr::W`](W) writer structure
impl crate::Writable for TCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TCR to value 0
impl crate::Resettable for TCRrs {}
