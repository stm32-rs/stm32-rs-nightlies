///Register `PDCRE` reader
pub type R = crate::R<PDCRErs>;
///Register `PDCRE` writer
pub type W = crate::W<PDCRErs>;
///Field `PD3` reader - Port E pull-down bit 3 When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
pub type PD3_R = crate::BitReader;
///Field `PD3` writer - Port E pull-down bit 3 When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD7` reader - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
pub type PD7_R = crate::BitReader;
///Field `PD7` writer - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD8` reader - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
pub type PD8_R = crate::BitReader;
///Field `PD8` writer - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
pub type PD8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD9` reader - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
pub type PD9_R = crate::BitReader;
///Field `PD9` writer - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
pub type PD9_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Port E pull-down bit 3 When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRE")
            .field("pd3", &self.pd3())
            .field("pd7", &self.pd7())
            .field("pd8", &self.pd8())
            .field("pd9", &self.pd9())
            .finish()
    }
}
impl W {
    ///Bit 3 - Port E pull-down bit 3 When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<'_, PDCRErs> {
        PD3_W::new(self, 3)
    }
    ///Bit 7 - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W<'_, PDCRErs> {
        PD7_W::new(self, 7)
    }
    ///Bit 8 - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W<'_, PDCRErs> {
        PD8_W::new(self, 8)
    }
    ///Bit 9 - Port E pull-down bit y When set, this bit activates the pull-down on PE\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W<'_, PDCRErs> {
        PD9_W::new(self, 9)
    }
}
/**Power Port E pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#PWR:PDCRE)*/
pub struct PDCRErs;
impl crate::RegisterSpec for PDCRErs {
    type Ux = u32;
}
///`read()` method returns [`pdcre::R`](R) reader structure
impl crate::Readable for PDCRErs {}
///`write(|w| ..)` method takes [`pdcre::W`](W) writer structure
impl crate::Writable for PDCRErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRE to value 0
impl crate::Resettable for PDCRErs {}
