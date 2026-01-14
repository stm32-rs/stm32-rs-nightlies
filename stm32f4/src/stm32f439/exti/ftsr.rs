///Register `FTSR` reader
pub type R = crate::R<FTSRrs>;
///Register `FTSR` writer
pub type W = crate::W<FTSRrs>;
///Field `TR0` reader - Falling trigger event configuration of line 0
pub type TR0_R = crate::BitReader;
///Field `TR0` writer - Falling trigger event configuration of line 0
pub type TR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR1` reader - Falling trigger event configuration of line 1
pub type TR1_R = crate::BitReader;
///Field `TR1` writer - Falling trigger event configuration of line 1
pub type TR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR2` reader - Falling trigger event configuration of line 2
pub type TR2_R = crate::BitReader;
///Field `TR2` writer - Falling trigger event configuration of line 2
pub type TR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR3` reader - Falling trigger event configuration of line 3
pub type TR3_R = crate::BitReader;
///Field `TR3` writer - Falling trigger event configuration of line 3
pub type TR3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR4` reader - Falling trigger event configuration of line 4
pub type TR4_R = crate::BitReader;
///Field `TR4` writer - Falling trigger event configuration of line 4
pub type TR4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR5` reader - Falling trigger event configuration of line 5
pub type TR5_R = crate::BitReader;
///Field `TR5` writer - Falling trigger event configuration of line 5
pub type TR5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR6` reader - Falling trigger event configuration of line 6
pub type TR6_R = crate::BitReader;
///Field `TR6` writer - Falling trigger event configuration of line 6
pub type TR6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR7` reader - Falling trigger event configuration of line 7
pub type TR7_R = crate::BitReader;
///Field `TR7` writer - Falling trigger event configuration of line 7
pub type TR7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR8` reader - Falling trigger event configuration of line 8
pub type TR8_R = crate::BitReader;
///Field `TR8` writer - Falling trigger event configuration of line 8
pub type TR8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR9` reader - Falling trigger event configuration of line 9
pub type TR9_R = crate::BitReader;
///Field `TR9` writer - Falling trigger event configuration of line 9
pub type TR9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR10` reader - Falling trigger event configuration of line 10
pub type TR10_R = crate::BitReader;
///Field `TR10` writer - Falling trigger event configuration of line 10
pub type TR10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR11` reader - Falling trigger event configuration of line 11
pub type TR11_R = crate::BitReader;
///Field `TR11` writer - Falling trigger event configuration of line 11
pub type TR11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR12` reader - Falling trigger event configuration of line 12
pub type TR12_R = crate::BitReader;
///Field `TR12` writer - Falling trigger event configuration of line 12
pub type TR12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR13` reader - Falling trigger event configuration of line 13
pub type TR13_R = crate::BitReader;
///Field `TR13` writer - Falling trigger event configuration of line 13
pub type TR13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR14` reader - Falling trigger event configuration of line 14
pub type TR14_R = crate::BitReader;
///Field `TR14` writer - Falling trigger event configuration of line 14
pub type TR14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR15` reader - Falling trigger event configuration of line 15
pub type TR15_R = crate::BitReader;
///Field `TR15` writer - Falling trigger event configuration of line 15
pub type TR15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR16` reader - Falling trigger event configuration of line 16
pub type TR16_R = crate::BitReader;
///Field `TR16` writer - Falling trigger event configuration of line 16
pub type TR16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR17` reader - Falling trigger event configuration of line 17
pub type TR17_R = crate::BitReader;
///Field `TR17` writer - Falling trigger event configuration of line 17
pub type TR17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR18` reader - Falling trigger event configuration of line 18
pub type TR18_R = crate::BitReader;
///Field `TR18` writer - Falling trigger event configuration of line 18
pub type TR18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR19` reader - Falling trigger event configuration of line 19
pub type TR19_R = crate::BitReader;
///Field `TR19` writer - Falling trigger event configuration of line 19
pub type TR19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR20` reader - Falling trigger event configuration of line 20
pub type TR20_R = crate::BitReader;
///Field `TR20` writer - Falling trigger event configuration of line 20
pub type TR20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR21` reader - Falling trigger event configuration of line 21
pub type TR21_R = crate::BitReader;
///Field `TR21` writer - Falling trigger event configuration of line 21
pub type TR21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TR22` reader - Falling trigger event configuration of line 22
pub type TR22_R = crate::BitReader;
///Field `TR22` writer - Falling trigger event configuration of line 22
pub type TR22_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Falling trigger event configuration of line 0
    #[inline(always)]
    pub fn tr0(&self) -> TR0_R {
        TR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Falling trigger event configuration of line 1
    #[inline(always)]
    pub fn tr1(&self) -> TR1_R {
        TR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Falling trigger event configuration of line 2
    #[inline(always)]
    pub fn tr2(&self) -> TR2_R {
        TR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Falling trigger event configuration of line 3
    #[inline(always)]
    pub fn tr3(&self) -> TR3_R {
        TR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Falling trigger event configuration of line 4
    #[inline(always)]
    pub fn tr4(&self) -> TR4_R {
        TR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Falling trigger event configuration of line 5
    #[inline(always)]
    pub fn tr5(&self) -> TR5_R {
        TR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Falling trigger event configuration of line 6
    #[inline(always)]
    pub fn tr6(&self) -> TR6_R {
        TR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Falling trigger event configuration of line 7
    #[inline(always)]
    pub fn tr7(&self) -> TR7_R {
        TR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Falling trigger event configuration of line 8
    #[inline(always)]
    pub fn tr8(&self) -> TR8_R {
        TR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Falling trigger event configuration of line 9
    #[inline(always)]
    pub fn tr9(&self) -> TR9_R {
        TR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Falling trigger event configuration of line 10
    #[inline(always)]
    pub fn tr10(&self) -> TR10_R {
        TR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Falling trigger event configuration of line 11
    #[inline(always)]
    pub fn tr11(&self) -> TR11_R {
        TR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Falling trigger event configuration of line 12
    #[inline(always)]
    pub fn tr12(&self) -> TR12_R {
        TR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Falling trigger event configuration of line 13
    #[inline(always)]
    pub fn tr13(&self) -> TR13_R {
        TR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Falling trigger event configuration of line 14
    #[inline(always)]
    pub fn tr14(&self) -> TR14_R {
        TR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Falling trigger event configuration of line 15
    #[inline(always)]
    pub fn tr15(&self) -> TR15_R {
        TR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Falling trigger event configuration of line 16
    #[inline(always)]
    pub fn tr16(&self) -> TR16_R {
        TR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Falling trigger event configuration of line 17
    #[inline(always)]
    pub fn tr17(&self) -> TR17_R {
        TR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Falling trigger event configuration of line 18
    #[inline(always)]
    pub fn tr18(&self) -> TR18_R {
        TR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Falling trigger event configuration of line 19
    #[inline(always)]
    pub fn tr19(&self) -> TR19_R {
        TR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Falling trigger event configuration of line 20
    #[inline(always)]
    pub fn tr20(&self) -> TR20_R {
        TR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Falling trigger event configuration of line 21
    #[inline(always)]
    pub fn tr21(&self) -> TR21_R {
        TR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Falling trigger event configuration of line 22
    #[inline(always)]
    pub fn tr22(&self) -> TR22_R {
        TR22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR")
            .field("tr0", &self.tr0())
            .field("tr1", &self.tr1())
            .field("tr2", &self.tr2())
            .field("tr3", &self.tr3())
            .field("tr4", &self.tr4())
            .field("tr5", &self.tr5())
            .field("tr6", &self.tr6())
            .field("tr7", &self.tr7())
            .field("tr8", &self.tr8())
            .field("tr9", &self.tr9())
            .field("tr10", &self.tr10())
            .field("tr11", &self.tr11())
            .field("tr12", &self.tr12())
            .field("tr13", &self.tr13())
            .field("tr14", &self.tr14())
            .field("tr15", &self.tr15())
            .field("tr16", &self.tr16())
            .field("tr17", &self.tr17())
            .field("tr18", &self.tr18())
            .field("tr19", &self.tr19())
            .field("tr20", &self.tr20())
            .field("tr21", &self.tr21())
            .field("tr22", &self.tr22())
            .finish()
    }
}
impl W {
    ///Bit 0 - Falling trigger event configuration of line 0
    #[inline(always)]
    pub fn tr0(&mut self) -> TR0_W<'_, FTSRrs> {
        TR0_W::new(self, 0)
    }
    ///Bit 1 - Falling trigger event configuration of line 1
    #[inline(always)]
    pub fn tr1(&mut self) -> TR1_W<'_, FTSRrs> {
        TR1_W::new(self, 1)
    }
    ///Bit 2 - Falling trigger event configuration of line 2
    #[inline(always)]
    pub fn tr2(&mut self) -> TR2_W<'_, FTSRrs> {
        TR2_W::new(self, 2)
    }
    ///Bit 3 - Falling trigger event configuration of line 3
    #[inline(always)]
    pub fn tr3(&mut self) -> TR3_W<'_, FTSRrs> {
        TR3_W::new(self, 3)
    }
    ///Bit 4 - Falling trigger event configuration of line 4
    #[inline(always)]
    pub fn tr4(&mut self) -> TR4_W<'_, FTSRrs> {
        TR4_W::new(self, 4)
    }
    ///Bit 5 - Falling trigger event configuration of line 5
    #[inline(always)]
    pub fn tr5(&mut self) -> TR5_W<'_, FTSRrs> {
        TR5_W::new(self, 5)
    }
    ///Bit 6 - Falling trigger event configuration of line 6
    #[inline(always)]
    pub fn tr6(&mut self) -> TR6_W<'_, FTSRrs> {
        TR6_W::new(self, 6)
    }
    ///Bit 7 - Falling trigger event configuration of line 7
    #[inline(always)]
    pub fn tr7(&mut self) -> TR7_W<'_, FTSRrs> {
        TR7_W::new(self, 7)
    }
    ///Bit 8 - Falling trigger event configuration of line 8
    #[inline(always)]
    pub fn tr8(&mut self) -> TR8_W<'_, FTSRrs> {
        TR8_W::new(self, 8)
    }
    ///Bit 9 - Falling trigger event configuration of line 9
    #[inline(always)]
    pub fn tr9(&mut self) -> TR9_W<'_, FTSRrs> {
        TR9_W::new(self, 9)
    }
    ///Bit 10 - Falling trigger event configuration of line 10
    #[inline(always)]
    pub fn tr10(&mut self) -> TR10_W<'_, FTSRrs> {
        TR10_W::new(self, 10)
    }
    ///Bit 11 - Falling trigger event configuration of line 11
    #[inline(always)]
    pub fn tr11(&mut self) -> TR11_W<'_, FTSRrs> {
        TR11_W::new(self, 11)
    }
    ///Bit 12 - Falling trigger event configuration of line 12
    #[inline(always)]
    pub fn tr12(&mut self) -> TR12_W<'_, FTSRrs> {
        TR12_W::new(self, 12)
    }
    ///Bit 13 - Falling trigger event configuration of line 13
    #[inline(always)]
    pub fn tr13(&mut self) -> TR13_W<'_, FTSRrs> {
        TR13_W::new(self, 13)
    }
    ///Bit 14 - Falling trigger event configuration of line 14
    #[inline(always)]
    pub fn tr14(&mut self) -> TR14_W<'_, FTSRrs> {
        TR14_W::new(self, 14)
    }
    ///Bit 15 - Falling trigger event configuration of line 15
    #[inline(always)]
    pub fn tr15(&mut self) -> TR15_W<'_, FTSRrs> {
        TR15_W::new(self, 15)
    }
    ///Bit 16 - Falling trigger event configuration of line 16
    #[inline(always)]
    pub fn tr16(&mut self) -> TR16_W<'_, FTSRrs> {
        TR16_W::new(self, 16)
    }
    ///Bit 17 - Falling trigger event configuration of line 17
    #[inline(always)]
    pub fn tr17(&mut self) -> TR17_W<'_, FTSRrs> {
        TR17_W::new(self, 17)
    }
    ///Bit 18 - Falling trigger event configuration of line 18
    #[inline(always)]
    pub fn tr18(&mut self) -> TR18_W<'_, FTSRrs> {
        TR18_W::new(self, 18)
    }
    ///Bit 19 - Falling trigger event configuration of line 19
    #[inline(always)]
    pub fn tr19(&mut self) -> TR19_W<'_, FTSRrs> {
        TR19_W::new(self, 19)
    }
    ///Bit 20 - Falling trigger event configuration of line 20
    #[inline(always)]
    pub fn tr20(&mut self) -> TR20_W<'_, FTSRrs> {
        TR20_W::new(self, 20)
    }
    ///Bit 21 - Falling trigger event configuration of line 21
    #[inline(always)]
    pub fn tr21(&mut self) -> TR21_W<'_, FTSRrs> {
        TR21_W::new(self, 21)
    }
    ///Bit 22 - Falling trigger event configuration of line 22
    #[inline(always)]
    pub fn tr22(&mut self) -> TR22_W<'_, FTSRrs> {
        TR22_W::new(self, 22)
    }
}
/**Falling Trigger selection register (EXTI_FTSR)

You can [`read`](crate::Reg::read) this register and get [`ftsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#EXTI:FTSR)*/
pub struct FTSRrs;
impl crate::RegisterSpec for FTSRrs {
    type Ux = u32;
}
///`read()` method returns [`ftsr::R`](R) reader structure
impl crate::Readable for FTSRrs {}
///`write(|w| ..)` method takes [`ftsr::W`](W) writer structure
impl crate::Writable for FTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTSR to value 0
impl crate::Resettable for FTSRrs {}
