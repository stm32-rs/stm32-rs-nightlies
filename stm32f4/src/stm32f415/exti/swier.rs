///Register `SWIER` reader
pub type R = crate::R<SWIERrs>;
///Register `SWIER` writer
pub type W = crate::W<SWIERrs>;
///Field `SWIER0` reader - Software Interrupt on line 0
pub type SWIER0_R = crate::BitReader;
///Field `SWIER0` writer - Software Interrupt on line 0
pub type SWIER0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER1` reader - Software Interrupt on line 1
pub type SWIER1_R = crate::BitReader;
///Field `SWIER1` writer - Software Interrupt on line 1
pub type SWIER1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER2` reader - Software Interrupt on line 2
pub type SWIER2_R = crate::BitReader;
///Field `SWIER2` writer - Software Interrupt on line 2
pub type SWIER2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER3` reader - Software Interrupt on line 3
pub type SWIER3_R = crate::BitReader;
///Field `SWIER3` writer - Software Interrupt on line 3
pub type SWIER3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER4` reader - Software Interrupt on line 4
pub type SWIER4_R = crate::BitReader;
///Field `SWIER4` writer - Software Interrupt on line 4
pub type SWIER4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER5` reader - Software Interrupt on line 5
pub type SWIER5_R = crate::BitReader;
///Field `SWIER5` writer - Software Interrupt on line 5
pub type SWIER5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER6` reader - Software Interrupt on line 6
pub type SWIER6_R = crate::BitReader;
///Field `SWIER6` writer - Software Interrupt on line 6
pub type SWIER6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER7` reader - Software Interrupt on line 7
pub type SWIER7_R = crate::BitReader;
///Field `SWIER7` writer - Software Interrupt on line 7
pub type SWIER7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER8` reader - Software Interrupt on line 8
pub type SWIER8_R = crate::BitReader;
///Field `SWIER8` writer - Software Interrupt on line 8
pub type SWIER8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER9` reader - Software Interrupt on line 9
pub type SWIER9_R = crate::BitReader;
///Field `SWIER9` writer - Software Interrupt on line 9
pub type SWIER9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER10` reader - Software Interrupt on line 10
pub type SWIER10_R = crate::BitReader;
///Field `SWIER10` writer - Software Interrupt on line 10
pub type SWIER10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER11` reader - Software Interrupt on line 11
pub type SWIER11_R = crate::BitReader;
///Field `SWIER11` writer - Software Interrupt on line 11
pub type SWIER11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER12` reader - Software Interrupt on line 12
pub type SWIER12_R = crate::BitReader;
///Field `SWIER12` writer - Software Interrupt on line 12
pub type SWIER12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER13` reader - Software Interrupt on line 13
pub type SWIER13_R = crate::BitReader;
///Field `SWIER13` writer - Software Interrupt on line 13
pub type SWIER13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER14` reader - Software Interrupt on line 14
pub type SWIER14_R = crate::BitReader;
///Field `SWIER14` writer - Software Interrupt on line 14
pub type SWIER14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER15` reader - Software Interrupt on line 15
pub type SWIER15_R = crate::BitReader;
///Field `SWIER15` writer - Software Interrupt on line 15
pub type SWIER15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER16` reader - Software Interrupt on line 16
pub type SWIER16_R = crate::BitReader;
///Field `SWIER16` writer - Software Interrupt on line 16
pub type SWIER16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER17` reader - Software Interrupt on line 17
pub type SWIER17_R = crate::BitReader;
///Field `SWIER17` writer - Software Interrupt on line 17
pub type SWIER17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER18` reader - Software Interrupt on line 18
pub type SWIER18_R = crate::BitReader;
///Field `SWIER18` writer - Software Interrupt on line 18
pub type SWIER18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER19` reader - Software Interrupt on line 19
pub type SWIER19_R = crate::BitReader;
///Field `SWIER19` writer - Software Interrupt on line 19
pub type SWIER19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER20` reader - Software Interrupt on line 20
pub type SWIER20_R = crate::BitReader;
///Field `SWIER20` writer - Software Interrupt on line 20
pub type SWIER20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER21` reader - Software Interrupt on line 21
pub type SWIER21_R = crate::BitReader;
///Field `SWIER21` writer - Software Interrupt on line 21
pub type SWIER21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWIER22` reader - Software Interrupt on line 22
pub type SWIER22_R = crate::BitReader;
///Field `SWIER22` writer - Software Interrupt on line 22
pub type SWIER22_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Software Interrupt on line 0
    #[inline(always)]
    pub fn swier0(&self) -> SWIER0_R {
        SWIER0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Software Interrupt on line 1
    #[inline(always)]
    pub fn swier1(&self) -> SWIER1_R {
        SWIER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Software Interrupt on line 2
    #[inline(always)]
    pub fn swier2(&self) -> SWIER2_R {
        SWIER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Software Interrupt on line 3
    #[inline(always)]
    pub fn swier3(&self) -> SWIER3_R {
        SWIER3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Software Interrupt on line 4
    #[inline(always)]
    pub fn swier4(&self) -> SWIER4_R {
        SWIER4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Software Interrupt on line 5
    #[inline(always)]
    pub fn swier5(&self) -> SWIER5_R {
        SWIER5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Software Interrupt on line 6
    #[inline(always)]
    pub fn swier6(&self) -> SWIER6_R {
        SWIER6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Software Interrupt on line 7
    #[inline(always)]
    pub fn swier7(&self) -> SWIER7_R {
        SWIER7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Software Interrupt on line 8
    #[inline(always)]
    pub fn swier8(&self) -> SWIER8_R {
        SWIER8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software Interrupt on line 9
    #[inline(always)]
    pub fn swier9(&self) -> SWIER9_R {
        SWIER9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Software Interrupt on line 10
    #[inline(always)]
    pub fn swier10(&self) -> SWIER10_R {
        SWIER10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Software Interrupt on line 11
    #[inline(always)]
    pub fn swier11(&self) -> SWIER11_R {
        SWIER11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Software Interrupt on line 12
    #[inline(always)]
    pub fn swier12(&self) -> SWIER12_R {
        SWIER12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Software Interrupt on line 13
    #[inline(always)]
    pub fn swier13(&self) -> SWIER13_R {
        SWIER13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Software Interrupt on line 14
    #[inline(always)]
    pub fn swier14(&self) -> SWIER14_R {
        SWIER14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Software Interrupt on line 15
    #[inline(always)]
    pub fn swier15(&self) -> SWIER15_R {
        SWIER15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Software Interrupt on line 16
    #[inline(always)]
    pub fn swier16(&self) -> SWIER16_R {
        SWIER16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Software Interrupt on line 17
    #[inline(always)]
    pub fn swier17(&self) -> SWIER17_R {
        SWIER17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Software Interrupt on line 18
    #[inline(always)]
    pub fn swier18(&self) -> SWIER18_R {
        SWIER18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Software Interrupt on line 19
    #[inline(always)]
    pub fn swier19(&self) -> SWIER19_R {
        SWIER19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Software Interrupt on line 20
    #[inline(always)]
    pub fn swier20(&self) -> SWIER20_R {
        SWIER20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Software Interrupt on line 21
    #[inline(always)]
    pub fn swier21(&self) -> SWIER21_R {
        SWIER21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Software Interrupt on line 22
    #[inline(always)]
    pub fn swier22(&self) -> SWIER22_R {
        SWIER22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER")
            .field("swier0", &self.swier0())
            .field("swier1", &self.swier1())
            .field("swier2", &self.swier2())
            .field("swier3", &self.swier3())
            .field("swier4", &self.swier4())
            .field("swier5", &self.swier5())
            .field("swier6", &self.swier6())
            .field("swier7", &self.swier7())
            .field("swier8", &self.swier8())
            .field("swier9", &self.swier9())
            .field("swier10", &self.swier10())
            .field("swier11", &self.swier11())
            .field("swier12", &self.swier12())
            .field("swier13", &self.swier13())
            .field("swier14", &self.swier14())
            .field("swier15", &self.swier15())
            .field("swier16", &self.swier16())
            .field("swier17", &self.swier17())
            .field("swier18", &self.swier18())
            .field("swier19", &self.swier19())
            .field("swier20", &self.swier20())
            .field("swier21", &self.swier21())
            .field("swier22", &self.swier22())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software Interrupt on line 0
    #[inline(always)]
    pub fn swier0(&mut self) -> SWIER0_W<'_, SWIERrs> {
        SWIER0_W::new(self, 0)
    }
    ///Bit 1 - Software Interrupt on line 1
    #[inline(always)]
    pub fn swier1(&mut self) -> SWIER1_W<'_, SWIERrs> {
        SWIER1_W::new(self, 1)
    }
    ///Bit 2 - Software Interrupt on line 2
    #[inline(always)]
    pub fn swier2(&mut self) -> SWIER2_W<'_, SWIERrs> {
        SWIER2_W::new(self, 2)
    }
    ///Bit 3 - Software Interrupt on line 3
    #[inline(always)]
    pub fn swier3(&mut self) -> SWIER3_W<'_, SWIERrs> {
        SWIER3_W::new(self, 3)
    }
    ///Bit 4 - Software Interrupt on line 4
    #[inline(always)]
    pub fn swier4(&mut self) -> SWIER4_W<'_, SWIERrs> {
        SWIER4_W::new(self, 4)
    }
    ///Bit 5 - Software Interrupt on line 5
    #[inline(always)]
    pub fn swier5(&mut self) -> SWIER5_W<'_, SWIERrs> {
        SWIER5_W::new(self, 5)
    }
    ///Bit 6 - Software Interrupt on line 6
    #[inline(always)]
    pub fn swier6(&mut self) -> SWIER6_W<'_, SWIERrs> {
        SWIER6_W::new(self, 6)
    }
    ///Bit 7 - Software Interrupt on line 7
    #[inline(always)]
    pub fn swier7(&mut self) -> SWIER7_W<'_, SWIERrs> {
        SWIER7_W::new(self, 7)
    }
    ///Bit 8 - Software Interrupt on line 8
    #[inline(always)]
    pub fn swier8(&mut self) -> SWIER8_W<'_, SWIERrs> {
        SWIER8_W::new(self, 8)
    }
    ///Bit 9 - Software Interrupt on line 9
    #[inline(always)]
    pub fn swier9(&mut self) -> SWIER9_W<'_, SWIERrs> {
        SWIER9_W::new(self, 9)
    }
    ///Bit 10 - Software Interrupt on line 10
    #[inline(always)]
    pub fn swier10(&mut self) -> SWIER10_W<'_, SWIERrs> {
        SWIER10_W::new(self, 10)
    }
    ///Bit 11 - Software Interrupt on line 11
    #[inline(always)]
    pub fn swier11(&mut self) -> SWIER11_W<'_, SWIERrs> {
        SWIER11_W::new(self, 11)
    }
    ///Bit 12 - Software Interrupt on line 12
    #[inline(always)]
    pub fn swier12(&mut self) -> SWIER12_W<'_, SWIERrs> {
        SWIER12_W::new(self, 12)
    }
    ///Bit 13 - Software Interrupt on line 13
    #[inline(always)]
    pub fn swier13(&mut self) -> SWIER13_W<'_, SWIERrs> {
        SWIER13_W::new(self, 13)
    }
    ///Bit 14 - Software Interrupt on line 14
    #[inline(always)]
    pub fn swier14(&mut self) -> SWIER14_W<'_, SWIERrs> {
        SWIER14_W::new(self, 14)
    }
    ///Bit 15 - Software Interrupt on line 15
    #[inline(always)]
    pub fn swier15(&mut self) -> SWIER15_W<'_, SWIERrs> {
        SWIER15_W::new(self, 15)
    }
    ///Bit 16 - Software Interrupt on line 16
    #[inline(always)]
    pub fn swier16(&mut self) -> SWIER16_W<'_, SWIERrs> {
        SWIER16_W::new(self, 16)
    }
    ///Bit 17 - Software Interrupt on line 17
    #[inline(always)]
    pub fn swier17(&mut self) -> SWIER17_W<'_, SWIERrs> {
        SWIER17_W::new(self, 17)
    }
    ///Bit 18 - Software Interrupt on line 18
    #[inline(always)]
    pub fn swier18(&mut self) -> SWIER18_W<'_, SWIERrs> {
        SWIER18_W::new(self, 18)
    }
    ///Bit 19 - Software Interrupt on line 19
    #[inline(always)]
    pub fn swier19(&mut self) -> SWIER19_W<'_, SWIERrs> {
        SWIER19_W::new(self, 19)
    }
    ///Bit 20 - Software Interrupt on line 20
    #[inline(always)]
    pub fn swier20(&mut self) -> SWIER20_W<'_, SWIERrs> {
        SWIER20_W::new(self, 20)
    }
    ///Bit 21 - Software Interrupt on line 21
    #[inline(always)]
    pub fn swier21(&mut self) -> SWIER21_W<'_, SWIERrs> {
        SWIER21_W::new(self, 21)
    }
    ///Bit 22 - Software Interrupt on line 22
    #[inline(always)]
    pub fn swier22(&mut self) -> SWIER22_W<'_, SWIERrs> {
        SWIER22_W::new(self, 22)
    }
}
/**Software interrupt event register (EXTI_SWIER)

You can [`read`](crate::Reg::read) this register and get [`swier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#EXTI:SWIER)*/
pub struct SWIERrs;
impl crate::RegisterSpec for SWIERrs {
    type Ux = u32;
}
///`read()` method returns [`swier::R`](R) reader structure
impl crate::Readable for SWIERrs {}
///`write(|w| ..)` method takes [`swier::W`](W) writer structure
impl crate::Writable for SWIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER to value 0
impl crate::Resettable for SWIERrs {}
