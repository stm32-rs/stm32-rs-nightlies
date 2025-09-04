///Register `GPIOZ_OSPEEDR` reader
pub type R = crate::R<GPIOZ_OSPEEDRrs>;
///Register `GPIOZ_OSPEEDR` writer
pub type W = crate::W<GPIOZ_OSPEEDRrs>;
///Field `OSPEEDR0` reader - OSPEEDR0
pub type OSPEEDR0_R = crate::FieldReader;
///Field `OSPEEDR0` writer - OSPEEDR0
pub type OSPEEDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR1` reader - OSPEEDR1
pub type OSPEEDR1_R = crate::FieldReader;
///Field `OSPEEDR1` writer - OSPEEDR1
pub type OSPEEDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR2` reader - OSPEEDR2
pub type OSPEEDR2_R = crate::FieldReader;
///Field `OSPEEDR2` writer - OSPEEDR2
pub type OSPEEDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR3` reader - OSPEEDR3
pub type OSPEEDR3_R = crate::FieldReader;
///Field `OSPEEDR3` writer - OSPEEDR3
pub type OSPEEDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR4` reader - OSPEEDR4
pub type OSPEEDR4_R = crate::FieldReader;
///Field `OSPEEDR4` writer - OSPEEDR4
pub type OSPEEDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR5` reader - OSPEEDR5
pub type OSPEEDR5_R = crate::FieldReader;
///Field `OSPEEDR5` writer - OSPEEDR5
pub type OSPEEDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR6` reader - OSPEEDR6
pub type OSPEEDR6_R = crate::FieldReader;
///Field `OSPEEDR6` writer - OSPEEDR6
pub type OSPEEDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR7` reader - OSPEEDR7
pub type OSPEEDR7_R = crate::FieldReader;
///Field `OSPEEDR7` writer - OSPEEDR7
pub type OSPEEDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR8` reader - OSPEEDR8
pub type OSPEEDR8_R = crate::FieldReader;
///Field `OSPEEDR8` writer - OSPEEDR8
pub type OSPEEDR8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR9` reader - OSPEEDR9
pub type OSPEEDR9_R = crate::FieldReader;
///Field `OSPEEDR9` writer - OSPEEDR9
pub type OSPEEDR9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR10` reader - OSPEEDR10
pub type OSPEEDR10_R = crate::FieldReader;
///Field `OSPEEDR10` writer - OSPEEDR10
pub type OSPEEDR10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR11` reader - OSPEEDR11
pub type OSPEEDR11_R = crate::FieldReader;
///Field `OSPEEDR11` writer - OSPEEDR11
pub type OSPEEDR11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR12` reader - OSPEEDR12
pub type OSPEEDR12_R = crate::FieldReader;
///Field `OSPEEDR12` writer - OSPEEDR12
pub type OSPEEDR12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR13` reader - OSPEEDR13
pub type OSPEEDR13_R = crate::FieldReader;
///Field `OSPEEDR13` writer - OSPEEDR13
pub type OSPEEDR13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR14` reader - OSPEEDR14
pub type OSPEEDR14_R = crate::FieldReader;
///Field `OSPEEDR14` writer - OSPEEDR14
pub type OSPEEDR14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEEDR15` reader - OSPEEDR15
pub type OSPEEDR15_R = crate::FieldReader;
///Field `OSPEEDR15` writer - OSPEEDR15
pub type OSPEEDR15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - OSPEEDR0
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - OSPEEDR1
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - OSPEEDR2
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR2_R {
        OSPEEDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - OSPEEDR3
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - OSPEEDR4
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR4_R {
        OSPEEDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - OSPEEDR5
    #[inline(always)]
    pub fn ospeedr5(&self) -> OSPEEDR5_R {
        OSPEEDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - OSPEEDR6
    #[inline(always)]
    pub fn ospeedr6(&self) -> OSPEEDR6_R {
        OSPEEDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - OSPEEDR7
    #[inline(always)]
    pub fn ospeedr7(&self) -> OSPEEDR7_R {
        OSPEEDR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - OSPEEDR8
    #[inline(always)]
    pub fn ospeedr8(&self) -> OSPEEDR8_R {
        OSPEEDR8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - OSPEEDR9
    #[inline(always)]
    pub fn ospeedr9(&self) -> OSPEEDR9_R {
        OSPEEDR9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - OSPEEDR10
    #[inline(always)]
    pub fn ospeedr10(&self) -> OSPEEDR10_R {
        OSPEEDR10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - OSPEEDR11
    #[inline(always)]
    pub fn ospeedr11(&self) -> OSPEEDR11_R {
        OSPEEDR11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - OSPEEDR12
    #[inline(always)]
    pub fn ospeedr12(&self) -> OSPEEDR12_R {
        OSPEEDR12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - OSPEEDR13
    #[inline(always)]
    pub fn ospeedr13(&self) -> OSPEEDR13_R {
        OSPEEDR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - OSPEEDR14
    #[inline(always)]
    pub fn ospeedr14(&self) -> OSPEEDR14_R {
        OSPEEDR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - OSPEEDR15
    #[inline(always)]
    pub fn ospeedr15(&self) -> OSPEEDR15_R {
        OSPEEDR15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOZ_OSPEEDR")
            .field("ospeedr0", &self.ospeedr0())
            .field("ospeedr1", &self.ospeedr1())
            .field("ospeedr2", &self.ospeedr2())
            .field("ospeedr3", &self.ospeedr3())
            .field("ospeedr4", &self.ospeedr4())
            .field("ospeedr5", &self.ospeedr5())
            .field("ospeedr6", &self.ospeedr6())
            .field("ospeedr7", &self.ospeedr7())
            .field("ospeedr8", &self.ospeedr8())
            .field("ospeedr9", &self.ospeedr9())
            .field("ospeedr10", &self.ospeedr10())
            .field("ospeedr11", &self.ospeedr11())
            .field("ospeedr12", &self.ospeedr12())
            .field("ospeedr13", &self.ospeedr13())
            .field("ospeedr14", &self.ospeedr14())
            .field("ospeedr15", &self.ospeedr15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - OSPEEDR0
    #[inline(always)]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR0_W::new(self, 0)
    }
    ///Bits 2:3 - OSPEEDR1
    #[inline(always)]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR1_W::new(self, 2)
    }
    ///Bits 4:5 - OSPEEDR2
    #[inline(always)]
    pub fn ospeedr2(&mut self) -> OSPEEDR2_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR2_W::new(self, 4)
    }
    ///Bits 6:7 - OSPEEDR3
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR3_W::new(self, 6)
    }
    ///Bits 8:9 - OSPEEDR4
    #[inline(always)]
    pub fn ospeedr4(&mut self) -> OSPEEDR4_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR4_W::new(self, 8)
    }
    ///Bits 10:11 - OSPEEDR5
    #[inline(always)]
    pub fn ospeedr5(&mut self) -> OSPEEDR5_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR5_W::new(self, 10)
    }
    ///Bits 12:13 - OSPEEDR6
    #[inline(always)]
    pub fn ospeedr6(&mut self) -> OSPEEDR6_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR6_W::new(self, 12)
    }
    ///Bits 14:15 - OSPEEDR7
    #[inline(always)]
    pub fn ospeedr7(&mut self) -> OSPEEDR7_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR7_W::new(self, 14)
    }
    ///Bits 16:17 - OSPEEDR8
    #[inline(always)]
    pub fn ospeedr8(&mut self) -> OSPEEDR8_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR8_W::new(self, 16)
    }
    ///Bits 18:19 - OSPEEDR9
    #[inline(always)]
    pub fn ospeedr9(&mut self) -> OSPEEDR9_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR9_W::new(self, 18)
    }
    ///Bits 20:21 - OSPEEDR10
    #[inline(always)]
    pub fn ospeedr10(&mut self) -> OSPEEDR10_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR10_W::new(self, 20)
    }
    ///Bits 22:23 - OSPEEDR11
    #[inline(always)]
    pub fn ospeedr11(&mut self) -> OSPEEDR11_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR11_W::new(self, 22)
    }
    ///Bits 24:25 - OSPEEDR12
    #[inline(always)]
    pub fn ospeedr12(&mut self) -> OSPEEDR12_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR12_W::new(self, 24)
    }
    ///Bits 26:27 - OSPEEDR13
    #[inline(always)]
    pub fn ospeedr13(&mut self) -> OSPEEDR13_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR13_W::new(self, 26)
    }
    ///Bits 28:29 - OSPEEDR14
    #[inline(always)]
    pub fn ospeedr14(&mut self) -> OSPEEDR14_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR14_W::new(self, 28)
    }
    ///Bits 30:31 - OSPEEDR15
    #[inline(always)]
    pub fn ospeedr15(&mut self) -> OSPEEDR15_W<GPIOZ_OSPEEDRrs> {
        OSPEEDR15_W::new(self, 30)
    }
}
/**GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`gpioz_ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioz_ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOZ:GPIOZ_OSPEEDR)*/
pub struct GPIOZ_OSPEEDRrs;
impl crate::RegisterSpec for GPIOZ_OSPEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`gpioz_ospeedr::R`](R) reader structure
impl crate::Readable for GPIOZ_OSPEEDRrs {}
///`write(|w| ..)` method takes [`gpioz_ospeedr::W`](W) writer structure
impl crate::Writable for GPIOZ_OSPEEDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOZ_OSPEEDR to value 0
impl crate::Resettable for GPIOZ_OSPEEDRrs {}
