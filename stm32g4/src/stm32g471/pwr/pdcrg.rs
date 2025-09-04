///Register `PDCRG` reader
pub type R = crate::R<PDCRGrs>;
///Register `PDCRG` writer
pub type W = crate::W<PDCRGrs>;
///Field `PD0` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD0_R = crate::BitReader;
///Field `PD0` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD1` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD1_R = crate::BitReader;
///Field `PD1` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD2` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD2_R = crate::BitReader;
///Field `PD2` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD3` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD3_R = crate::BitReader;
///Field `PD3` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD4` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD4_R = crate::BitReader;
///Field `PD4` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD5` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD5_R = crate::BitReader;
///Field `PD5` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD6` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD6_R = crate::BitReader;
///Field `PD6` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD7` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD7_R = crate::BitReader;
///Field `PD7` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD8` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD8_R = crate::BitReader;
///Field `PD8` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD9` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD9_R = crate::BitReader;
///Field `PD9` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD10` reader - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD10_R = crate::BitReader;
///Field `PD10` writer - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
pub type PD10_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRG")
            .field("pd0", &self.pd0())
            .field("pd1", &self.pd1())
            .field("pd2", &self.pd2())
            .field("pd3", &self.pd3())
            .field("pd4", &self.pd4())
            .field("pd5", &self.pd5())
            .field("pd6", &self.pd6())
            .field("pd7", &self.pd7())
            .field("pd8", &self.pd8())
            .field("pd9", &self.pd9())
            .field("pd10", &self.pd10())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W<PDCRGrs> {
        PD0_W::new(self, 0)
    }
    ///Bit 1 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W<PDCRGrs> {
        PD1_W::new(self, 1)
    }
    ///Bit 2 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W<PDCRGrs> {
        PD2_W::new(self, 2)
    }
    ///Bit 3 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<PDCRGrs> {
        PD3_W::new(self, 3)
    }
    ///Bit 4 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W<PDCRGrs> {
        PD4_W::new(self, 4)
    }
    ///Bit 5 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W<PDCRGrs> {
        PD5_W::new(self, 5)
    }
    ///Bit 6 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W<PDCRGrs> {
        PD6_W::new(self, 6)
    }
    ///Bit 7 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W<PDCRGrs> {
        PD7_W::new(self, 7)
    }
    ///Bit 8 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W<PDCRGrs> {
        PD8_W::new(self, 8)
    }
    ///Bit 9 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W<PDCRGrs> {
        PD9_W::new(self, 9)
    }
    ///Bit 10 - Port G pull-down bit y (y=0..10) When set, this bit activates the pull-down on PG\[y\] when APC bit is set in PWR_CR3 register.
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W<PDCRGrs> {
        PD10_W::new(self, 10)
    }
}
/**Power Port G pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G471.html#PWR:PDCRG)*/
pub struct PDCRGrs;
impl crate::RegisterSpec for PDCRGrs {
    type Ux = u32;
}
///`read()` method returns [`pdcrg::R`](R) reader structure
impl crate::Readable for PDCRGrs {}
///`write(|w| ..)` method takes [`pdcrg::W`](W) writer structure
impl crate::Writable for PDCRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRG to value 0
impl crate::Resettable for PDCRGrs {}
