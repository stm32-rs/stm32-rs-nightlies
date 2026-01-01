///Register `FS1R` reader
pub type R = crate::R<FS1Rrs>;
///Register `FS1R` writer
pub type W = crate::W<FS1Rrs>;
///Field `FSC0` reader - Filter scale configuration
pub type FSC0_R = crate::BitReader;
///Field `FSC0` writer - Filter scale configuration
pub type FSC0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC1` reader - Filter scale configuration
pub type FSC1_R = crate::BitReader;
///Field `FSC1` writer - Filter scale configuration
pub type FSC1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC2` reader - Filter scale configuration
pub type FSC2_R = crate::BitReader;
///Field `FSC2` writer - Filter scale configuration
pub type FSC2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC3` reader - Filter scale configuration
pub type FSC3_R = crate::BitReader;
///Field `FSC3` writer - Filter scale configuration
pub type FSC3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC4` reader - Filter scale configuration
pub type FSC4_R = crate::BitReader;
///Field `FSC4` writer - Filter scale configuration
pub type FSC4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC5` reader - Filter scale configuration
pub type FSC5_R = crate::BitReader;
///Field `FSC5` writer - Filter scale configuration
pub type FSC5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC6` reader - Filter scale configuration
pub type FSC6_R = crate::BitReader;
///Field `FSC6` writer - Filter scale configuration
pub type FSC6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC7` reader - Filter scale configuration
pub type FSC7_R = crate::BitReader;
///Field `FSC7` writer - Filter scale configuration
pub type FSC7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC8` reader - Filter scale configuration
pub type FSC8_R = crate::BitReader;
///Field `FSC8` writer - Filter scale configuration
pub type FSC8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC9` reader - Filter scale configuration
pub type FSC9_R = crate::BitReader;
///Field `FSC9` writer - Filter scale configuration
pub type FSC9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC10` reader - Filter scale configuration
pub type FSC10_R = crate::BitReader;
///Field `FSC10` writer - Filter scale configuration
pub type FSC10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC11` reader - Filter scale configuration
pub type FSC11_R = crate::BitReader;
///Field `FSC11` writer - Filter scale configuration
pub type FSC11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC12` reader - Filter scale configuration
pub type FSC12_R = crate::BitReader;
///Field `FSC12` writer - Filter scale configuration
pub type FSC12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC13` reader - Filter scale configuration
pub type FSC13_R = crate::BitReader;
///Field `FSC13` writer - Filter scale configuration
pub type FSC13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC14` reader - Filter scale configuration
pub type FSC14_R = crate::BitReader;
///Field `FSC14` writer - Filter scale configuration
pub type FSC14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC15` reader - Filter scale configuration
pub type FSC15_R = crate::BitReader;
///Field `FSC15` writer - Filter scale configuration
pub type FSC15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC16` reader - Filter scale configuration
pub type FSC16_R = crate::BitReader;
///Field `FSC16` writer - Filter scale configuration
pub type FSC16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC17` reader - Filter scale configuration
pub type FSC17_R = crate::BitReader;
///Field `FSC17` writer - Filter scale configuration
pub type FSC17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC18` reader - Filter scale configuration
pub type FSC18_R = crate::BitReader;
///Field `FSC18` writer - Filter scale configuration
pub type FSC18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC19` reader - Filter scale configuration
pub type FSC19_R = crate::BitReader;
///Field `FSC19` writer - Filter scale configuration
pub type FSC19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC20` reader - Filter scale configuration
pub type FSC20_R = crate::BitReader;
///Field `FSC20` writer - Filter scale configuration
pub type FSC20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC21` reader - Filter scale configuration
pub type FSC21_R = crate::BitReader;
///Field `FSC21` writer - Filter scale configuration
pub type FSC21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC22` reader - Filter scale configuration
pub type FSC22_R = crate::BitReader;
///Field `FSC22` writer - Filter scale configuration
pub type FSC22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC23` reader - Filter scale configuration
pub type FSC23_R = crate::BitReader;
///Field `FSC23` writer - Filter scale configuration
pub type FSC23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC24` reader - Filter scale configuration
pub type FSC24_R = crate::BitReader;
///Field `FSC24` writer - Filter scale configuration
pub type FSC24_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC25` reader - Filter scale configuration
pub type FSC25_R = crate::BitReader;
///Field `FSC25` writer - Filter scale configuration
pub type FSC25_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC26` reader - Filter scale configuration
pub type FSC26_R = crate::BitReader;
///Field `FSC26` writer - Filter scale configuration
pub type FSC26_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSC27` reader - Filter scale configuration
pub type FSC27_R = crate::BitReader;
///Field `FSC27` writer - Filter scale configuration
pub type FSC27_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Filter scale configuration
    #[inline(always)]
    pub fn fsc0(&self) -> FSC0_R {
        FSC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Filter scale configuration
    #[inline(always)]
    pub fn fsc1(&self) -> FSC1_R {
        FSC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Filter scale configuration
    #[inline(always)]
    pub fn fsc2(&self) -> FSC2_R {
        FSC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Filter scale configuration
    #[inline(always)]
    pub fn fsc3(&self) -> FSC3_R {
        FSC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Filter scale configuration
    #[inline(always)]
    pub fn fsc4(&self) -> FSC4_R {
        FSC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Filter scale configuration
    #[inline(always)]
    pub fn fsc5(&self) -> FSC5_R {
        FSC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Filter scale configuration
    #[inline(always)]
    pub fn fsc6(&self) -> FSC6_R {
        FSC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Filter scale configuration
    #[inline(always)]
    pub fn fsc7(&self) -> FSC7_R {
        FSC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Filter scale configuration
    #[inline(always)]
    pub fn fsc8(&self) -> FSC8_R {
        FSC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Filter scale configuration
    #[inline(always)]
    pub fn fsc9(&self) -> FSC9_R {
        FSC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Filter scale configuration
    #[inline(always)]
    pub fn fsc10(&self) -> FSC10_R {
        FSC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Filter scale configuration
    #[inline(always)]
    pub fn fsc11(&self) -> FSC11_R {
        FSC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Filter scale configuration
    #[inline(always)]
    pub fn fsc12(&self) -> FSC12_R {
        FSC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Filter scale configuration
    #[inline(always)]
    pub fn fsc13(&self) -> FSC13_R {
        FSC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Filter scale configuration
    #[inline(always)]
    pub fn fsc14(&self) -> FSC14_R {
        FSC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Filter scale configuration
    #[inline(always)]
    pub fn fsc15(&self) -> FSC15_R {
        FSC15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Filter scale configuration
    #[inline(always)]
    pub fn fsc16(&self) -> FSC16_R {
        FSC16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Filter scale configuration
    #[inline(always)]
    pub fn fsc17(&self) -> FSC17_R {
        FSC17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Filter scale configuration
    #[inline(always)]
    pub fn fsc18(&self) -> FSC18_R {
        FSC18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Filter scale configuration
    #[inline(always)]
    pub fn fsc19(&self) -> FSC19_R {
        FSC19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Filter scale configuration
    #[inline(always)]
    pub fn fsc20(&self) -> FSC20_R {
        FSC20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Filter scale configuration
    #[inline(always)]
    pub fn fsc21(&self) -> FSC21_R {
        FSC21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Filter scale configuration
    #[inline(always)]
    pub fn fsc22(&self) -> FSC22_R {
        FSC22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Filter scale configuration
    #[inline(always)]
    pub fn fsc23(&self) -> FSC23_R {
        FSC23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Filter scale configuration
    #[inline(always)]
    pub fn fsc24(&self) -> FSC24_R {
        FSC24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Filter scale configuration
    #[inline(always)]
    pub fn fsc25(&self) -> FSC25_R {
        FSC25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Filter scale configuration
    #[inline(always)]
    pub fn fsc26(&self) -> FSC26_R {
        FSC26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Filter scale configuration
    #[inline(always)]
    pub fn fsc27(&self) -> FSC27_R {
        FSC27_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS1R")
            .field("fsc0", &self.fsc0())
            .field("fsc1", &self.fsc1())
            .field("fsc2", &self.fsc2())
            .field("fsc3", &self.fsc3())
            .field("fsc4", &self.fsc4())
            .field("fsc5", &self.fsc5())
            .field("fsc6", &self.fsc6())
            .field("fsc7", &self.fsc7())
            .field("fsc8", &self.fsc8())
            .field("fsc9", &self.fsc9())
            .field("fsc10", &self.fsc10())
            .field("fsc11", &self.fsc11())
            .field("fsc12", &self.fsc12())
            .field("fsc13", &self.fsc13())
            .field("fsc14", &self.fsc14())
            .field("fsc15", &self.fsc15())
            .field("fsc16", &self.fsc16())
            .field("fsc17", &self.fsc17())
            .field("fsc18", &self.fsc18())
            .field("fsc19", &self.fsc19())
            .field("fsc20", &self.fsc20())
            .field("fsc21", &self.fsc21())
            .field("fsc22", &self.fsc22())
            .field("fsc23", &self.fsc23())
            .field("fsc24", &self.fsc24())
            .field("fsc25", &self.fsc25())
            .field("fsc26", &self.fsc26())
            .field("fsc27", &self.fsc27())
            .finish()
    }
}
impl W {
    ///Bit 0 - Filter scale configuration
    #[inline(always)]
    pub fn fsc0(&mut self) -> FSC0_W<'_, FS1Rrs> {
        FSC0_W::new(self, 0)
    }
    ///Bit 1 - Filter scale configuration
    #[inline(always)]
    pub fn fsc1(&mut self) -> FSC1_W<'_, FS1Rrs> {
        FSC1_W::new(self, 1)
    }
    ///Bit 2 - Filter scale configuration
    #[inline(always)]
    pub fn fsc2(&mut self) -> FSC2_W<'_, FS1Rrs> {
        FSC2_W::new(self, 2)
    }
    ///Bit 3 - Filter scale configuration
    #[inline(always)]
    pub fn fsc3(&mut self) -> FSC3_W<'_, FS1Rrs> {
        FSC3_W::new(self, 3)
    }
    ///Bit 4 - Filter scale configuration
    #[inline(always)]
    pub fn fsc4(&mut self) -> FSC4_W<'_, FS1Rrs> {
        FSC4_W::new(self, 4)
    }
    ///Bit 5 - Filter scale configuration
    #[inline(always)]
    pub fn fsc5(&mut self) -> FSC5_W<'_, FS1Rrs> {
        FSC5_W::new(self, 5)
    }
    ///Bit 6 - Filter scale configuration
    #[inline(always)]
    pub fn fsc6(&mut self) -> FSC6_W<'_, FS1Rrs> {
        FSC6_W::new(self, 6)
    }
    ///Bit 7 - Filter scale configuration
    #[inline(always)]
    pub fn fsc7(&mut self) -> FSC7_W<'_, FS1Rrs> {
        FSC7_W::new(self, 7)
    }
    ///Bit 8 - Filter scale configuration
    #[inline(always)]
    pub fn fsc8(&mut self) -> FSC8_W<'_, FS1Rrs> {
        FSC8_W::new(self, 8)
    }
    ///Bit 9 - Filter scale configuration
    #[inline(always)]
    pub fn fsc9(&mut self) -> FSC9_W<'_, FS1Rrs> {
        FSC9_W::new(self, 9)
    }
    ///Bit 10 - Filter scale configuration
    #[inline(always)]
    pub fn fsc10(&mut self) -> FSC10_W<'_, FS1Rrs> {
        FSC10_W::new(self, 10)
    }
    ///Bit 11 - Filter scale configuration
    #[inline(always)]
    pub fn fsc11(&mut self) -> FSC11_W<'_, FS1Rrs> {
        FSC11_W::new(self, 11)
    }
    ///Bit 12 - Filter scale configuration
    #[inline(always)]
    pub fn fsc12(&mut self) -> FSC12_W<'_, FS1Rrs> {
        FSC12_W::new(self, 12)
    }
    ///Bit 13 - Filter scale configuration
    #[inline(always)]
    pub fn fsc13(&mut self) -> FSC13_W<'_, FS1Rrs> {
        FSC13_W::new(self, 13)
    }
    ///Bit 14 - Filter scale configuration
    #[inline(always)]
    pub fn fsc14(&mut self) -> FSC14_W<'_, FS1Rrs> {
        FSC14_W::new(self, 14)
    }
    ///Bit 15 - Filter scale configuration
    #[inline(always)]
    pub fn fsc15(&mut self) -> FSC15_W<'_, FS1Rrs> {
        FSC15_W::new(self, 15)
    }
    ///Bit 16 - Filter scale configuration
    #[inline(always)]
    pub fn fsc16(&mut self) -> FSC16_W<'_, FS1Rrs> {
        FSC16_W::new(self, 16)
    }
    ///Bit 17 - Filter scale configuration
    #[inline(always)]
    pub fn fsc17(&mut self) -> FSC17_W<'_, FS1Rrs> {
        FSC17_W::new(self, 17)
    }
    ///Bit 18 - Filter scale configuration
    #[inline(always)]
    pub fn fsc18(&mut self) -> FSC18_W<'_, FS1Rrs> {
        FSC18_W::new(self, 18)
    }
    ///Bit 19 - Filter scale configuration
    #[inline(always)]
    pub fn fsc19(&mut self) -> FSC19_W<'_, FS1Rrs> {
        FSC19_W::new(self, 19)
    }
    ///Bit 20 - Filter scale configuration
    #[inline(always)]
    pub fn fsc20(&mut self) -> FSC20_W<'_, FS1Rrs> {
        FSC20_W::new(self, 20)
    }
    ///Bit 21 - Filter scale configuration
    #[inline(always)]
    pub fn fsc21(&mut self) -> FSC21_W<'_, FS1Rrs> {
        FSC21_W::new(self, 21)
    }
    ///Bit 22 - Filter scale configuration
    #[inline(always)]
    pub fn fsc22(&mut self) -> FSC22_W<'_, FS1Rrs> {
        FSC22_W::new(self, 22)
    }
    ///Bit 23 - Filter scale configuration
    #[inline(always)]
    pub fn fsc23(&mut self) -> FSC23_W<'_, FS1Rrs> {
        FSC23_W::new(self, 23)
    }
    ///Bit 24 - Filter scale configuration
    #[inline(always)]
    pub fn fsc24(&mut self) -> FSC24_W<'_, FS1Rrs> {
        FSC24_W::new(self, 24)
    }
    ///Bit 25 - Filter scale configuration
    #[inline(always)]
    pub fn fsc25(&mut self) -> FSC25_W<'_, FS1Rrs> {
        FSC25_W::new(self, 25)
    }
    ///Bit 26 - Filter scale configuration
    #[inline(always)]
    pub fn fsc26(&mut self) -> FSC26_W<'_, FS1Rrs> {
        FSC26_W::new(self, 26)
    }
    ///Bit 27 - Filter scale configuration
    #[inline(always)]
    pub fn fsc27(&mut self) -> FSC27_W<'_, FS1Rrs> {
        FSC27_W::new(self, 27)
    }
}
/**filter scale register

You can [`read`](crate::Reg::read) this register and get [`fs1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:FS1R)*/
pub struct FS1Rrs;
impl crate::RegisterSpec for FS1Rrs {
    type Ux = u32;
}
///`read()` method returns [`fs1r::R`](R) reader structure
impl crate::Readable for FS1Rrs {}
///`write(|w| ..)` method takes [`fs1r::W`](W) writer structure
impl crate::Writable for FS1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS1R to value 0
impl crate::Resettable for FS1Rrs {}
