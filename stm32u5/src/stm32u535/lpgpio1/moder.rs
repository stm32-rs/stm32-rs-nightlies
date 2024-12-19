///Register `MODER` reader
pub type R = crate::R<MODERrs>;
///Register `MODER` writer
pub type W = crate::W<MODERrs>;
///Field `MODE0` reader - MODE0
pub type MODE0_R = crate::BitReader;
///Field `MODE0` writer - MODE0
pub type MODE0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE1` reader - MODE1
pub type MODE1_R = crate::BitReader;
///Field `MODE1` writer - MODE1
pub type MODE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE2` reader - MODE2
pub type MODE2_R = crate::BitReader;
///Field `MODE2` writer - MODE2
pub type MODE2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE3` reader - MODE3
pub type MODE3_R = crate::BitReader;
///Field `MODE3` writer - MODE3
pub type MODE3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE4` reader - MODE4
pub type MODE4_R = crate::BitReader;
///Field `MODE4` writer - MODE4
pub type MODE4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE5` reader - MODE5
pub type MODE5_R = crate::BitReader;
///Field `MODE5` writer - MODE5
pub type MODE5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE6` reader - MODE6
pub type MODE6_R = crate::BitReader;
///Field `MODE6` writer - MODE6
pub type MODE6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE7` reader - MODE7
pub type MODE7_R = crate::BitReader;
///Field `MODE7` writer - MODE7
pub type MODE7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE8` reader - MODE8
pub type MODE8_R = crate::BitReader;
///Field `MODE8` writer - MODE8
pub type MODE8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE9` reader - MODE9
pub type MODE9_R = crate::BitReader;
///Field `MODE9` writer - MODE9
pub type MODE9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE10` reader - MODE10
pub type MODE10_R = crate::BitReader;
///Field `MODE10` writer - MODE10
pub type MODE10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE11` reader - MODE11
pub type MODE11_R = crate::BitReader;
///Field `MODE11` writer - MODE11
pub type MODE11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE12` reader - MODE12
pub type MODE12_R = crate::BitReader;
///Field `MODE12` writer - MODE12
pub type MODE12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE13` reader - MODE13
pub type MODE13_R = crate::BitReader;
///Field `MODE13` writer - MODE13
pub type MODE13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE14` reader - MODE14
pub type MODE14_R = crate::BitReader;
///Field `MODE14` writer - MODE14
pub type MODE14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE15` reader - MODE15
pub type MODE15_R = crate::BitReader;
///Field `MODE15` writer - MODE15
pub type MODE15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MODE0
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MODE1
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MODE2
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MODE3
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MODE4
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MODE5
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MODE6
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MODE7
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - MODE8
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MODE9
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - MODE10
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - MODE11
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - MODE12
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - MODE13
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - MODE14
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - MODE15
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODER")
            .field("mode15", &self.mode15())
            .field("mode14", &self.mode14())
            .field("mode13", &self.mode13())
            .field("mode12", &self.mode12())
            .field("mode11", &self.mode11())
            .field("mode10", &self.mode10())
            .field("mode9", &self.mode9())
            .field("mode8", &self.mode8())
            .field("mode7", &self.mode7())
            .field("mode6", &self.mode6())
            .field("mode5", &self.mode5())
            .field("mode4", &self.mode4())
            .field("mode3", &self.mode3())
            .field("mode2", &self.mode2())
            .field("mode1", &self.mode1())
            .field("mode0", &self.mode0())
            .finish()
    }
}
impl W {
    ///Bit 0 - MODE0
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W<MODERrs> {
        MODE0_W::new(self, 0)
    }
    ///Bit 1 - MODE1
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W<MODERrs> {
        MODE1_W::new(self, 1)
    }
    ///Bit 2 - MODE2
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W<MODERrs> {
        MODE2_W::new(self, 2)
    }
    ///Bit 3 - MODE3
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W<MODERrs> {
        MODE3_W::new(self, 3)
    }
    ///Bit 4 - MODE4
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE4_W<MODERrs> {
        MODE4_W::new(self, 4)
    }
    ///Bit 5 - MODE5
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE5_W<MODERrs> {
        MODE5_W::new(self, 5)
    }
    ///Bit 6 - MODE6
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE6_W<MODERrs> {
        MODE6_W::new(self, 6)
    }
    ///Bit 7 - MODE7
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE7_W<MODERrs> {
        MODE7_W::new(self, 7)
    }
    ///Bit 8 - MODE8
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE8_W<MODERrs> {
        MODE8_W::new(self, 8)
    }
    ///Bit 9 - MODE9
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE9_W<MODERrs> {
        MODE9_W::new(self, 9)
    }
    ///Bit 10 - MODE10
    #[inline(always)]
    pub fn mode10(&mut self) -> MODE10_W<MODERrs> {
        MODE10_W::new(self, 10)
    }
    ///Bit 11 - MODE11
    #[inline(always)]
    pub fn mode11(&mut self) -> MODE11_W<MODERrs> {
        MODE11_W::new(self, 11)
    }
    ///Bit 12 - MODE12
    #[inline(always)]
    pub fn mode12(&mut self) -> MODE12_W<MODERrs> {
        MODE12_W::new(self, 12)
    }
    ///Bit 13 - MODE13
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE13_W<MODERrs> {
        MODE13_W::new(self, 13)
    }
    ///Bit 14 - MODE14
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE14_W<MODERrs> {
        MODE14_W::new(self, 14)
    }
    ///Bit 15 - MODE15
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE15_W<MODERrs> {
        MODE15_W::new(self, 15)
    }
}
/**LPGPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#LPGPIO1:MODER)*/
pub struct MODERrs;
impl crate::RegisterSpec for MODERrs {
    type Ux = u32;
}
///`read()` method returns [`moder::R`](R) reader structure
impl crate::Readable for MODERrs {}
///`write(|w| ..)` method takes [`moder::W`](W) writer structure
impl crate::Writable for MODERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MODER to value 0
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0;
}
