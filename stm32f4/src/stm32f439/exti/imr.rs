///Register `IMR` reader
pub type R = crate::R<IMRrs>;
///Register `IMR` writer
pub type W = crate::W<IMRrs>;
///Field `MR0` reader - Interrupt Mask on line 0
pub type MR0_R = crate::BitReader;
///Field `MR0` writer - Interrupt Mask on line 0
pub type MR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR1` reader - Interrupt Mask on line 1
pub type MR1_R = crate::BitReader;
///Field `MR1` writer - Interrupt Mask on line 1
pub type MR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR2` reader - Interrupt Mask on line 2
pub type MR2_R = crate::BitReader;
///Field `MR2` writer - Interrupt Mask on line 2
pub type MR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR3` reader - Interrupt Mask on line 3
pub type MR3_R = crate::BitReader;
///Field `MR3` writer - Interrupt Mask on line 3
pub type MR3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR4` reader - Interrupt Mask on line 4
pub type MR4_R = crate::BitReader;
///Field `MR4` writer - Interrupt Mask on line 4
pub type MR4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR5` reader - Interrupt Mask on line 5
pub type MR5_R = crate::BitReader;
///Field `MR5` writer - Interrupt Mask on line 5
pub type MR5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR6` reader - Interrupt Mask on line 6
pub type MR6_R = crate::BitReader;
///Field `MR6` writer - Interrupt Mask on line 6
pub type MR6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR7` reader - Interrupt Mask on line 7
pub type MR7_R = crate::BitReader;
///Field `MR7` writer - Interrupt Mask on line 7
pub type MR7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR8` reader - Interrupt Mask on line 8
pub type MR8_R = crate::BitReader;
///Field `MR8` writer - Interrupt Mask on line 8
pub type MR8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR9` reader - Interrupt Mask on line 9
pub type MR9_R = crate::BitReader;
///Field `MR9` writer - Interrupt Mask on line 9
pub type MR9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR10` reader - Interrupt Mask on line 10
pub type MR10_R = crate::BitReader;
///Field `MR10` writer - Interrupt Mask on line 10
pub type MR10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR11` reader - Interrupt Mask on line 11
pub type MR11_R = crate::BitReader;
///Field `MR11` writer - Interrupt Mask on line 11
pub type MR11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR12` reader - Interrupt Mask on line 12
pub type MR12_R = crate::BitReader;
///Field `MR12` writer - Interrupt Mask on line 12
pub type MR12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR13` reader - Interrupt Mask on line 13
pub type MR13_R = crate::BitReader;
///Field `MR13` writer - Interrupt Mask on line 13
pub type MR13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR14` reader - Interrupt Mask on line 14
pub type MR14_R = crate::BitReader;
///Field `MR14` writer - Interrupt Mask on line 14
pub type MR14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR15` reader - Interrupt Mask on line 15
pub type MR15_R = crate::BitReader;
///Field `MR15` writer - Interrupt Mask on line 15
pub type MR15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR16` reader - Interrupt Mask on line 16
pub type MR16_R = crate::BitReader;
///Field `MR16` writer - Interrupt Mask on line 16
pub type MR16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR17` reader - Interrupt Mask on line 17
pub type MR17_R = crate::BitReader;
///Field `MR17` writer - Interrupt Mask on line 17
pub type MR17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR18` reader - Interrupt Mask on line 18
pub type MR18_R = crate::BitReader;
///Field `MR18` writer - Interrupt Mask on line 18
pub type MR18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR19` reader - Interrupt Mask on line 19
pub type MR19_R = crate::BitReader;
///Field `MR19` writer - Interrupt Mask on line 19
pub type MR19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR20` reader - Interrupt Mask on line 20
pub type MR20_R = crate::BitReader;
///Field `MR20` writer - Interrupt Mask on line 20
pub type MR20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR21` reader - Interrupt Mask on line 21
pub type MR21_R = crate::BitReader;
///Field `MR21` writer - Interrupt Mask on line 21
pub type MR21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR22` reader - Interrupt Mask on line 22
pub type MR22_R = crate::BitReader;
///Field `MR22` writer - Interrupt Mask on line 22
pub type MR22_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Interrupt Mask on line 0
    #[inline(always)]
    pub fn mr0(&self) -> MR0_R {
        MR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt Mask on line 1
    #[inline(always)]
    pub fn mr1(&self) -> MR1_R {
        MR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt Mask on line 2
    #[inline(always)]
    pub fn mr2(&self) -> MR2_R {
        MR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt Mask on line 3
    #[inline(always)]
    pub fn mr3(&self) -> MR3_R {
        MR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt Mask on line 4
    #[inline(always)]
    pub fn mr4(&self) -> MR4_R {
        MR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt Mask on line 5
    #[inline(always)]
    pub fn mr5(&self) -> MR5_R {
        MR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt Mask on line 6
    #[inline(always)]
    pub fn mr6(&self) -> MR6_R {
        MR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt Mask on line 7
    #[inline(always)]
    pub fn mr7(&self) -> MR7_R {
        MR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt Mask on line 8
    #[inline(always)]
    pub fn mr8(&self) -> MR8_R {
        MR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt Mask on line 9
    #[inline(always)]
    pub fn mr9(&self) -> MR9_R {
        MR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt Mask on line 10
    #[inline(always)]
    pub fn mr10(&self) -> MR10_R {
        MR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt Mask on line 11
    #[inline(always)]
    pub fn mr11(&self) -> MR11_R {
        MR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt Mask on line 12
    #[inline(always)]
    pub fn mr12(&self) -> MR12_R {
        MR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt Mask on line 13
    #[inline(always)]
    pub fn mr13(&self) -> MR13_R {
        MR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt Mask on line 14
    #[inline(always)]
    pub fn mr14(&self) -> MR14_R {
        MR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt Mask on line 15
    #[inline(always)]
    pub fn mr15(&self) -> MR15_R {
        MR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Interrupt Mask on line 16
    #[inline(always)]
    pub fn mr16(&self) -> MR16_R {
        MR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Interrupt Mask on line 17
    #[inline(always)]
    pub fn mr17(&self) -> MR17_R {
        MR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Interrupt Mask on line 18
    #[inline(always)]
    pub fn mr18(&self) -> MR18_R {
        MR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Interrupt Mask on line 19
    #[inline(always)]
    pub fn mr19(&self) -> MR19_R {
        MR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Interrupt Mask on line 20
    #[inline(always)]
    pub fn mr20(&self) -> MR20_R {
        MR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Interrupt Mask on line 21
    #[inline(always)]
    pub fn mr21(&self) -> MR21_R {
        MR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Interrupt Mask on line 22
    #[inline(always)]
    pub fn mr22(&self) -> MR22_R {
        MR22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR")
            .field("mr0", &self.mr0())
            .field("mr1", &self.mr1())
            .field("mr2", &self.mr2())
            .field("mr3", &self.mr3())
            .field("mr4", &self.mr4())
            .field("mr5", &self.mr5())
            .field("mr6", &self.mr6())
            .field("mr7", &self.mr7())
            .field("mr8", &self.mr8())
            .field("mr9", &self.mr9())
            .field("mr10", &self.mr10())
            .field("mr11", &self.mr11())
            .field("mr12", &self.mr12())
            .field("mr13", &self.mr13())
            .field("mr14", &self.mr14())
            .field("mr15", &self.mr15())
            .field("mr16", &self.mr16())
            .field("mr17", &self.mr17())
            .field("mr18", &self.mr18())
            .field("mr19", &self.mr19())
            .field("mr20", &self.mr20())
            .field("mr21", &self.mr21())
            .field("mr22", &self.mr22())
            .finish()
    }
}
impl W {
    ///Bit 0 - Interrupt Mask on line 0
    #[inline(always)]
    pub fn mr0(&mut self) -> MR0_W<'_, IMRrs> {
        MR0_W::new(self, 0)
    }
    ///Bit 1 - Interrupt Mask on line 1
    #[inline(always)]
    pub fn mr1(&mut self) -> MR1_W<'_, IMRrs> {
        MR1_W::new(self, 1)
    }
    ///Bit 2 - Interrupt Mask on line 2
    #[inline(always)]
    pub fn mr2(&mut self) -> MR2_W<'_, IMRrs> {
        MR2_W::new(self, 2)
    }
    ///Bit 3 - Interrupt Mask on line 3
    #[inline(always)]
    pub fn mr3(&mut self) -> MR3_W<'_, IMRrs> {
        MR3_W::new(self, 3)
    }
    ///Bit 4 - Interrupt Mask on line 4
    #[inline(always)]
    pub fn mr4(&mut self) -> MR4_W<'_, IMRrs> {
        MR4_W::new(self, 4)
    }
    ///Bit 5 - Interrupt Mask on line 5
    #[inline(always)]
    pub fn mr5(&mut self) -> MR5_W<'_, IMRrs> {
        MR5_W::new(self, 5)
    }
    ///Bit 6 - Interrupt Mask on line 6
    #[inline(always)]
    pub fn mr6(&mut self) -> MR6_W<'_, IMRrs> {
        MR6_W::new(self, 6)
    }
    ///Bit 7 - Interrupt Mask on line 7
    #[inline(always)]
    pub fn mr7(&mut self) -> MR7_W<'_, IMRrs> {
        MR7_W::new(self, 7)
    }
    ///Bit 8 - Interrupt Mask on line 8
    #[inline(always)]
    pub fn mr8(&mut self) -> MR8_W<'_, IMRrs> {
        MR8_W::new(self, 8)
    }
    ///Bit 9 - Interrupt Mask on line 9
    #[inline(always)]
    pub fn mr9(&mut self) -> MR9_W<'_, IMRrs> {
        MR9_W::new(self, 9)
    }
    ///Bit 10 - Interrupt Mask on line 10
    #[inline(always)]
    pub fn mr10(&mut self) -> MR10_W<'_, IMRrs> {
        MR10_W::new(self, 10)
    }
    ///Bit 11 - Interrupt Mask on line 11
    #[inline(always)]
    pub fn mr11(&mut self) -> MR11_W<'_, IMRrs> {
        MR11_W::new(self, 11)
    }
    ///Bit 12 - Interrupt Mask on line 12
    #[inline(always)]
    pub fn mr12(&mut self) -> MR12_W<'_, IMRrs> {
        MR12_W::new(self, 12)
    }
    ///Bit 13 - Interrupt Mask on line 13
    #[inline(always)]
    pub fn mr13(&mut self) -> MR13_W<'_, IMRrs> {
        MR13_W::new(self, 13)
    }
    ///Bit 14 - Interrupt Mask on line 14
    #[inline(always)]
    pub fn mr14(&mut self) -> MR14_W<'_, IMRrs> {
        MR14_W::new(self, 14)
    }
    ///Bit 15 - Interrupt Mask on line 15
    #[inline(always)]
    pub fn mr15(&mut self) -> MR15_W<'_, IMRrs> {
        MR15_W::new(self, 15)
    }
    ///Bit 16 - Interrupt Mask on line 16
    #[inline(always)]
    pub fn mr16(&mut self) -> MR16_W<'_, IMRrs> {
        MR16_W::new(self, 16)
    }
    ///Bit 17 - Interrupt Mask on line 17
    #[inline(always)]
    pub fn mr17(&mut self) -> MR17_W<'_, IMRrs> {
        MR17_W::new(self, 17)
    }
    ///Bit 18 - Interrupt Mask on line 18
    #[inline(always)]
    pub fn mr18(&mut self) -> MR18_W<'_, IMRrs> {
        MR18_W::new(self, 18)
    }
    ///Bit 19 - Interrupt Mask on line 19
    #[inline(always)]
    pub fn mr19(&mut self) -> MR19_W<'_, IMRrs> {
        MR19_W::new(self, 19)
    }
    ///Bit 20 - Interrupt Mask on line 20
    #[inline(always)]
    pub fn mr20(&mut self) -> MR20_W<'_, IMRrs> {
        MR20_W::new(self, 20)
    }
    ///Bit 21 - Interrupt Mask on line 21
    #[inline(always)]
    pub fn mr21(&mut self) -> MR21_W<'_, IMRrs> {
        MR21_W::new(self, 21)
    }
    ///Bit 22 - Interrupt Mask on line 22
    #[inline(always)]
    pub fn mr22(&mut self) -> MR22_W<'_, IMRrs> {
        MR22_W::new(self, 22)
    }
}
/**Interrupt mask register (EXTI_IMR)

You can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#EXTI:IMR)*/
pub struct IMRrs;
impl crate::RegisterSpec for IMRrs {
    type Ux = u32;
}
///`read()` method returns [`imr::R`](R) reader structure
impl crate::Readable for IMRrs {}
///`write(|w| ..)` method takes [`imr::W`](W) writer structure
impl crate::Writable for IMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMR to value 0
impl crate::Resettable for IMRrs {}
