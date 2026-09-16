///Register `PUCRE` reader
pub type R = crate::R<PUCRErs>;
///Register `PUCRE` writer
pub type W = crate::W<PUCRErs>;
///Field `PU3` reader - Port E pull-up bit 3 When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
pub type PU3_R = crate::BitReader;
///Field `PU3` writer - Port E pull-up bit 3 When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU7` reader - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
pub type PU7_R = crate::BitReader;
///Field `PU7` writer - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
pub type PU7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU8` reader - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
pub type PU8_R = crate::BitReader;
///Field `PU8` writer - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
pub type PU8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU9` reader - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
pub type PU9_R = crate::BitReader;
///Field `PU9` writer - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
pub type PU9_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Port E pull-up bit 3 When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRE")
            .field("pu3", &self.pu3())
            .field("pu7", &self.pu7())
            .field("pu8", &self.pu8())
            .field("pu9", &self.pu9())
            .finish()
    }
}
impl W {
    ///Bit 3 - Port E pull-up bit 3 When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W<'_, PUCRErs> {
        PU3_W::new(self, 3)
    }
    ///Bit 7 - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
    #[inline(always)]
    pub fn pu7(&mut self) -> PU7_W<'_, PUCRErs> {
        PU7_W::new(self, 7)
    }
    ///Bit 8 - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
    #[inline(always)]
    pub fn pu8(&mut self) -> PU8_W<'_, PUCRErs> {
        PU8_W::new(self, 8)
    }
    ///Bit 9 - Port E pull-up bit y When set, this bit activates the pull-up on PE\[y\] when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority.
    #[inline(always)]
    pub fn pu9(&mut self) -> PU9_W<'_, PUCRErs> {
        PU9_W::new(self, 9)
    }
}
/**Power Port E pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#PWR:PUCRE)*/
pub struct PUCRErs;
impl crate::RegisterSpec for PUCRErs {
    type Ux = u32;
}
///`read()` method returns [`pucre::R`](R) reader structure
impl crate::Readable for PUCRErs {}
///`write(|w| ..)` method takes [`pucre::W`](W) writer structure
impl crate::Writable for PUCRErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRE to value 0
impl crate::Resettable for PUCRErs {}
