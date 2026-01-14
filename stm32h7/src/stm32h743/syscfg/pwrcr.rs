///Register `PWRCR` reader
pub type R = crate::R<PWRCRrs>;
///Register `PWRCR` writer
pub type W = crate::W<PWRCRrs>;
///Field `ODEN` reader - Overdrive enable
pub type ODEN_R = crate::BitReader;
///Field `ODEN` writer - Overdrive enable
pub type ODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Overdrive enable
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRCR").field("oden", &self.oden()).finish()
    }
}
impl W {
    ///Bit 0 - Overdrive enable
    #[inline(always)]
    pub fn oden(&mut self) -> ODEN_W<'_, PWRCRrs> {
        ODEN_W::new(self, 0)
    }
}
/**SYSCFG power control register

You can [`read`](crate::Reg::read) this register and get [`pwrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#SYSCFG:PWRCR)*/
pub struct PWRCRrs;
impl crate::RegisterSpec for PWRCRrs {
    type Ux = u32;
}
///`read()` method returns [`pwrcr::R`](R) reader structure
impl crate::Readable for PWRCRrs {}
///`write(|w| ..)` method takes [`pwrcr::W`](W) writer structure
impl crate::Writable for PWRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWRCR to value 0
impl crate::Resettable for PWRCRrs {}
