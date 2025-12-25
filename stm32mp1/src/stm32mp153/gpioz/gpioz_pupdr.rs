///Register `GPIOZ_PUPDR` reader
pub type R = crate::R<GPIOZ_PUPDRrs>;
///Register `GPIOZ_PUPDR` writer
pub type W = crate::W<GPIOZ_PUPDRrs>;
///Field `PUPDR0` reader - PUPDR0
pub type PUPDR0_R = crate::FieldReader;
///Field `PUPDR0` writer - PUPDR0
pub type PUPDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR1` reader - PUPDR1
pub type PUPDR1_R = crate::FieldReader;
///Field `PUPDR1` writer - PUPDR1
pub type PUPDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR2` reader - PUPDR2
pub type PUPDR2_R = crate::FieldReader;
///Field `PUPDR2` writer - PUPDR2
pub type PUPDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR3` reader - PUPDR3
pub type PUPDR3_R = crate::FieldReader;
///Field `PUPDR3` writer - PUPDR3
pub type PUPDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR4` reader - PUPDR4
pub type PUPDR4_R = crate::FieldReader;
///Field `PUPDR4` writer - PUPDR4
pub type PUPDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR5` reader - PUPDR5
pub type PUPDR5_R = crate::FieldReader;
///Field `PUPDR5` writer - PUPDR5
pub type PUPDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR6` reader - PUPDR6
pub type PUPDR6_R = crate::FieldReader;
///Field `PUPDR6` writer - PUPDR6
pub type PUPDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR7` reader - PUPDR7
pub type PUPDR7_R = crate::FieldReader;
///Field `PUPDR7` writer - PUPDR7
pub type PUPDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR8` reader - PUPDR8
pub type PUPDR8_R = crate::FieldReader;
///Field `PUPDR8` writer - PUPDR8
pub type PUPDR8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR9` reader - PUPDR9
pub type PUPDR9_R = crate::FieldReader;
///Field `PUPDR9` writer - PUPDR9
pub type PUPDR9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR10` reader - PUPDR10
pub type PUPDR10_R = crate::FieldReader;
///Field `PUPDR10` writer - PUPDR10
pub type PUPDR10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR11` reader - PUPDR11
pub type PUPDR11_R = crate::FieldReader;
///Field `PUPDR11` writer - PUPDR11
pub type PUPDR11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR12` reader - PUPDR12
pub type PUPDR12_R = crate::FieldReader;
///Field `PUPDR12` writer - PUPDR12
pub type PUPDR12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR13` reader - PUPDR13
pub type PUPDR13_R = crate::FieldReader;
///Field `PUPDR13` writer - PUPDR13
pub type PUPDR13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR14` reader - PUPDR14
pub type PUPDR14_R = crate::FieldReader;
///Field `PUPDR14` writer - PUPDR14
pub type PUPDR14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPDR15` reader - PUPDR15
pub type PUPDR15_R = crate::FieldReader;
///Field `PUPDR15` writer - PUPDR15
pub type PUPDR15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - PUPDR0
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR0_R {
        PUPDR0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PUPDR1
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR1_R {
        PUPDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - PUPDR2
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR2_R {
        PUPDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - PUPDR3
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - PUPDR4
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR4_R {
        PUPDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - PUPDR5
    #[inline(always)]
    pub fn pupdr5(&self) -> PUPDR5_R {
        PUPDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - PUPDR6
    #[inline(always)]
    pub fn pupdr6(&self) -> PUPDR6_R {
        PUPDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - PUPDR7
    #[inline(always)]
    pub fn pupdr7(&self) -> PUPDR7_R {
        PUPDR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - PUPDR8
    #[inline(always)]
    pub fn pupdr8(&self) -> PUPDR8_R {
        PUPDR8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - PUPDR9
    #[inline(always)]
    pub fn pupdr9(&self) -> PUPDR9_R {
        PUPDR9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - PUPDR10
    #[inline(always)]
    pub fn pupdr10(&self) -> PUPDR10_R {
        PUPDR10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - PUPDR11
    #[inline(always)]
    pub fn pupdr11(&self) -> PUPDR11_R {
        PUPDR11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - PUPDR12
    #[inline(always)]
    pub fn pupdr12(&self) -> PUPDR12_R {
        PUPDR12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - PUPDR13
    #[inline(always)]
    pub fn pupdr13(&self) -> PUPDR13_R {
        PUPDR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - PUPDR14
    #[inline(always)]
    pub fn pupdr14(&self) -> PUPDR14_R {
        PUPDR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - PUPDR15
    #[inline(always)]
    pub fn pupdr15(&self) -> PUPDR15_R {
        PUPDR15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOZ_PUPDR")
            .field("pupdr0", &self.pupdr0())
            .field("pupdr1", &self.pupdr1())
            .field("pupdr2", &self.pupdr2())
            .field("pupdr3", &self.pupdr3())
            .field("pupdr4", &self.pupdr4())
            .field("pupdr5", &self.pupdr5())
            .field("pupdr6", &self.pupdr6())
            .field("pupdr7", &self.pupdr7())
            .field("pupdr8", &self.pupdr8())
            .field("pupdr9", &self.pupdr9())
            .field("pupdr10", &self.pupdr10())
            .field("pupdr11", &self.pupdr11())
            .field("pupdr12", &self.pupdr12())
            .field("pupdr13", &self.pupdr13())
            .field("pupdr14", &self.pupdr14())
            .field("pupdr15", &self.pupdr15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PUPDR0
    #[inline(always)]
    pub fn pupdr0(&mut self) -> PUPDR0_W<'_, GPIOZ_PUPDRrs> {
        PUPDR0_W::new(self, 0)
    }
    ///Bits 2:3 - PUPDR1
    #[inline(always)]
    pub fn pupdr1(&mut self) -> PUPDR1_W<'_, GPIOZ_PUPDRrs> {
        PUPDR1_W::new(self, 2)
    }
    ///Bits 4:5 - PUPDR2
    #[inline(always)]
    pub fn pupdr2(&mut self) -> PUPDR2_W<'_, GPIOZ_PUPDRrs> {
        PUPDR2_W::new(self, 4)
    }
    ///Bits 6:7 - PUPDR3
    #[inline(always)]
    pub fn pupdr3(&mut self) -> PUPDR3_W<'_, GPIOZ_PUPDRrs> {
        PUPDR3_W::new(self, 6)
    }
    ///Bits 8:9 - PUPDR4
    #[inline(always)]
    pub fn pupdr4(&mut self) -> PUPDR4_W<'_, GPIOZ_PUPDRrs> {
        PUPDR4_W::new(self, 8)
    }
    ///Bits 10:11 - PUPDR5
    #[inline(always)]
    pub fn pupdr5(&mut self) -> PUPDR5_W<'_, GPIOZ_PUPDRrs> {
        PUPDR5_W::new(self, 10)
    }
    ///Bits 12:13 - PUPDR6
    #[inline(always)]
    pub fn pupdr6(&mut self) -> PUPDR6_W<'_, GPIOZ_PUPDRrs> {
        PUPDR6_W::new(self, 12)
    }
    ///Bits 14:15 - PUPDR7
    #[inline(always)]
    pub fn pupdr7(&mut self) -> PUPDR7_W<'_, GPIOZ_PUPDRrs> {
        PUPDR7_W::new(self, 14)
    }
    ///Bits 16:17 - PUPDR8
    #[inline(always)]
    pub fn pupdr8(&mut self) -> PUPDR8_W<'_, GPIOZ_PUPDRrs> {
        PUPDR8_W::new(self, 16)
    }
    ///Bits 18:19 - PUPDR9
    #[inline(always)]
    pub fn pupdr9(&mut self) -> PUPDR9_W<'_, GPIOZ_PUPDRrs> {
        PUPDR9_W::new(self, 18)
    }
    ///Bits 20:21 - PUPDR10
    #[inline(always)]
    pub fn pupdr10(&mut self) -> PUPDR10_W<'_, GPIOZ_PUPDRrs> {
        PUPDR10_W::new(self, 20)
    }
    ///Bits 22:23 - PUPDR11
    #[inline(always)]
    pub fn pupdr11(&mut self) -> PUPDR11_W<'_, GPIOZ_PUPDRrs> {
        PUPDR11_W::new(self, 22)
    }
    ///Bits 24:25 - PUPDR12
    #[inline(always)]
    pub fn pupdr12(&mut self) -> PUPDR12_W<'_, GPIOZ_PUPDRrs> {
        PUPDR12_W::new(self, 24)
    }
    ///Bits 26:27 - PUPDR13
    #[inline(always)]
    pub fn pupdr13(&mut self) -> PUPDR13_W<'_, GPIOZ_PUPDRrs> {
        PUPDR13_W::new(self, 26)
    }
    ///Bits 28:29 - PUPDR14
    #[inline(always)]
    pub fn pupdr14(&mut self) -> PUPDR14_W<'_, GPIOZ_PUPDRrs> {
        PUPDR14_W::new(self, 28)
    }
    ///Bits 30:31 - PUPDR15
    #[inline(always)]
    pub fn pupdr15(&mut self) -> PUPDR15_W<'_, GPIOZ_PUPDRrs> {
        PUPDR15_W::new(self, 30)
    }
}
/**GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`gpioz_pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioz_pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOZ:GPIOZ_PUPDR)*/
pub struct GPIOZ_PUPDRrs;
impl crate::RegisterSpec for GPIOZ_PUPDRrs {
    type Ux = u32;
}
///`read()` method returns [`gpioz_pupdr::R`](R) reader structure
impl crate::Readable for GPIOZ_PUPDRrs {}
///`write(|w| ..)` method takes [`gpioz_pupdr::W`](W) writer structure
impl crate::Writable for GPIOZ_PUPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOZ_PUPDR to value 0
impl crate::Resettable for GPIOZ_PUPDRrs {}
