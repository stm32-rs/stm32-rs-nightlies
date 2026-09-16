///Register `PRCR` reader
pub type R = crate::R<PRCRrs>;
///Register `PRCR` writer
pub type W = crate::W<PRCRrs>;
///Field `PEN` reader - When set to 0, this bit places the digital section of the D-PHY in the reset state.
pub type PEN_R = crate::BitReader;
///Field `PEN` writer - When set to 0, this bit places the digital section of the D-PHY in the reset state.
pub type PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - When set to 0, this bit places the digital section of the D-PHY in the reset state.
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRCR").field("pen", &self.pen()).finish()
    }
}
impl W {
    ///Bit 1 - When set to 0, this bit places the digital section of the D-PHY in the reset state.
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W<'_, PRCRrs> {
        PEN_W::new(self, 1)
    }
}
/**CSI PHY reset control register

You can [`read`](crate::Reg::read) this register and get [`prcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:PRCR)*/
pub struct PRCRrs;
impl crate::RegisterSpec for PRCRrs {
    type Ux = u32;
}
///`read()` method returns [`prcr::R`](R) reader structure
impl crate::Readable for PRCRrs {}
///`write(|w| ..)` method takes [`prcr::W`](W) writer structure
impl crate::Writable for PRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRCR to value 0
impl crate::Resettable for PRCRrs {}
