///Register `LCKR` reader
pub type R = crate::R<LCKRrs>;
///Register `LCKR` writer
pub type W = crate::W<LCKRrs>;
///Field `LCK0` reader - Port x lock bit y (y= 0..15)
pub type LCK0_R = crate::BitReader;
///Field `LCK0` writer - Port x lock bit y (y= 0..15)
pub type LCK0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK1` reader - Port x lock bit y (y= 0..15)
pub type LCK1_R = crate::BitReader;
///Field `LCK1` writer - Port x lock bit y (y= 0..15)
pub type LCK1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK2` reader - Port x lock bit y (y= 0..15)
pub type LCK2_R = crate::BitReader;
///Field `LCK2` writer - Port x lock bit y (y= 0..15)
pub type LCK2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK3` reader - Port x lock bit y (y= 0..15)
pub type LCK3_R = crate::BitReader;
///Field `LCK3` writer - Port x lock bit y (y= 0..15)
pub type LCK3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK4` reader - Port x lock bit y (y= 0..15)
pub type LCK4_R = crate::BitReader;
///Field `LCK4` writer - Port x lock bit y (y= 0..15)
pub type LCK4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK5` reader - Port x lock bit y (y= 0..15)
pub type LCK5_R = crate::BitReader;
///Field `LCK5` writer - Port x lock bit y (y= 0..15)
pub type LCK5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK6` reader - Port x lock bit y (y= 0..15)
pub type LCK6_R = crate::BitReader;
///Field `LCK6` writer - Port x lock bit y (y= 0..15)
pub type LCK6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK7` reader - Port x lock bit y (y= 0..15)
pub type LCK7_R = crate::BitReader;
///Field `LCK7` writer - Port x lock bit y (y= 0..15)
pub type LCK7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK8` reader - Port x lock bit y (y= 0..15)
pub type LCK8_R = crate::BitReader;
///Field `LCK8` writer - Port x lock bit y (y= 0..15)
pub type LCK8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK9` reader - Port x lock bit y (y= 0..15)
pub type LCK9_R = crate::BitReader;
///Field `LCK9` writer - Port x lock bit y (y= 0..15)
pub type LCK9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK10` reader - Port x lock bit y (y= 0..15)
pub type LCK10_R = crate::BitReader;
///Field `LCK10` writer - Port x lock bit y (y= 0..15)
pub type LCK10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK11` reader - Port x lock bit y (y= 0..15)
pub type LCK11_R = crate::BitReader;
///Field `LCK11` writer - Port x lock bit y (y= 0..15)
pub type LCK11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK12` reader - Port x lock bit y (y= 0..15)
pub type LCK12_R = crate::BitReader;
///Field `LCK12` writer - Port x lock bit y (y= 0..15)
pub type LCK12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK13` reader - Port x lock bit y (y= 0..15)
pub type LCK13_R = crate::BitReader;
///Field `LCK13` writer - Port x lock bit y (y= 0..15)
pub type LCK13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK14` reader - Port x lock bit y (y= 0..15)
pub type LCK14_R = crate::BitReader;
///Field `LCK14` writer - Port x lock bit y (y= 0..15)
pub type LCK14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCK15` reader - Port x lock bit y (y= 0..15)
pub type LCK15_R = crate::BitReader;
///Field `LCK15` writer - Port x lock bit y (y= 0..15)
pub type LCK15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCKK` reader - Port x lock bit y (y= 0..15)
pub type LCKK_R = crate::BitReader;
///Field `LCKK` writer - Port x lock bit y (y= 0..15)
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck8(&self) -> LCK8_R {
        LCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck9(&self) -> LCK9_R {
        LCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck10(&self) -> LCK10_R {
        LCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck11(&self) -> LCK11_R {
        LCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck12(&self) -> LCK12_R {
        LCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCKR")
            .field("lckk", &self.lckk())
            .field("lck15", &self.lck15())
            .field("lck14", &self.lck14())
            .field("lck13", &self.lck13())
            .field("lck12", &self.lck12())
            .field("lck11", &self.lck11())
            .field("lck10", &self.lck10())
            .field("lck9", &self.lck9())
            .field("lck8", &self.lck8())
            .field("lck7", &self.lck7())
            .field("lck6", &self.lck6())
            .field("lck5", &self.lck5())
            .field("lck4", &self.lck4())
            .field("lck3", &self.lck3())
            .field("lck2", &self.lck2())
            .field("lck1", &self.lck1())
            .field("lck0", &self.lck0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck0(&mut self) -> LCK0_W<'_, LCKRrs> {
        LCK0_W::new(self, 0)
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck1(&mut self) -> LCK1_W<'_, LCKRrs> {
        LCK1_W::new(self, 1)
    }
    ///Bit 2 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck2(&mut self) -> LCK2_W<'_, LCKRrs> {
        LCK2_W::new(self, 2)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK3_W<'_, LCKRrs> {
        LCK3_W::new(self, 3)
    }
    ///Bit 4 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck4(&mut self) -> LCK4_W<'_, LCKRrs> {
        LCK4_W::new(self, 4)
    }
    ///Bit 5 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck5(&mut self) -> LCK5_W<'_, LCKRrs> {
        LCK5_W::new(self, 5)
    }
    ///Bit 6 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck6(&mut self) -> LCK6_W<'_, LCKRrs> {
        LCK6_W::new(self, 6)
    }
    ///Bit 7 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck7(&mut self) -> LCK7_W<'_, LCKRrs> {
        LCK7_W::new(self, 7)
    }
    ///Bit 8 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck8(&mut self) -> LCK8_W<'_, LCKRrs> {
        LCK8_W::new(self, 8)
    }
    ///Bit 9 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck9(&mut self) -> LCK9_W<'_, LCKRrs> {
        LCK9_W::new(self, 9)
    }
    ///Bit 10 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck10(&mut self) -> LCK10_W<'_, LCKRrs> {
        LCK10_W::new(self, 10)
    }
    ///Bit 11 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck11(&mut self) -> LCK11_W<'_, LCKRrs> {
        LCK11_W::new(self, 11)
    }
    ///Bit 12 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck12(&mut self) -> LCK12_W<'_, LCKRrs> {
        LCK12_W::new(self, 12)
    }
    ///Bit 13 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck13(&mut self) -> LCK13_W<'_, LCKRrs> {
        LCK13_W::new(self, 13)
    }
    ///Bit 14 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck14(&mut self) -> LCK14_W<'_, LCKRrs> {
        LCK14_W::new(self, 14)
    }
    ///Bit 15 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck15(&mut self) -> LCK15_W<'_, LCKRrs> {
        LCK15_W::new(self, 15)
    }
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W<'_, LCKRrs> {
        LCKK_W::new(self, 16)
    }
}
/**GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`lckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#GPIOA:LCKR)*/
pub struct LCKRrs;
impl crate::RegisterSpec for LCKRrs {
    type Ux = u32;
}
///`read()` method returns [`lckr::R`](R) reader structure
impl crate::Readable for LCKRrs {}
///`write(|w| ..)` method takes [`lckr::W`](W) writer structure
impl crate::Writable for LCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCKR to value 0
impl crate::Resettable for LCKRrs {}
