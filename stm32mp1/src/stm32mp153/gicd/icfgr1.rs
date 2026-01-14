///Register `ICFGR1` reader
pub type R = crate::R<ICFGR1rs>;
///Register `ICFGR1` writer
pub type W = crate::W<ICFGR1rs>;
///Field `INT_CONFIG0` reader - INT_CONFIG0
pub type INT_CONFIG0_R = crate::FieldReader;
///Field `INT_CONFIG0` writer - INT_CONFIG0
pub type INT_CONFIG0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG1` reader - INT_CONFIG1
pub type INT_CONFIG1_R = crate::FieldReader;
///Field `INT_CONFIG1` writer - INT_CONFIG1
pub type INT_CONFIG1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG2` reader - INT_CONFIG2
pub type INT_CONFIG2_R = crate::FieldReader;
///Field `INT_CONFIG2` writer - INT_CONFIG2
pub type INT_CONFIG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG3` reader - INT_CONFIG3
pub type INT_CONFIG3_R = crate::FieldReader;
///Field `INT_CONFIG3` writer - INT_CONFIG3
pub type INT_CONFIG3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG4` reader - INT_CONFIG4
pub type INT_CONFIG4_R = crate::FieldReader;
///Field `INT_CONFIG4` writer - INT_CONFIG4
pub type INT_CONFIG4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG5` reader - INT_CONFIG5
pub type INT_CONFIG5_R = crate::FieldReader;
///Field `INT_CONFIG5` writer - INT_CONFIG5
pub type INT_CONFIG5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG6` reader - INT_CONFIG6
pub type INT_CONFIG6_R = crate::FieldReader;
///Field `INT_CONFIG6` writer - INT_CONFIG6
pub type INT_CONFIG6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG7` reader - INT_CONFIG7
pub type INT_CONFIG7_R = crate::FieldReader;
///Field `INT_CONFIG7` writer - INT_CONFIG7
pub type INT_CONFIG7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG8` reader - INT_CONFIG8
pub type INT_CONFIG8_R = crate::FieldReader;
///Field `INT_CONFIG8` writer - INT_CONFIG8
pub type INT_CONFIG8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG9` reader - INT_CONFIG9
pub type INT_CONFIG9_R = crate::FieldReader;
///Field `INT_CONFIG9` writer - INT_CONFIG9
pub type INT_CONFIG9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG10` reader - INT_CONFIG10
pub type INT_CONFIG10_R = crate::FieldReader;
///Field `INT_CONFIG10` writer - INT_CONFIG10
pub type INT_CONFIG10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG11` reader - INT_CONFIG11
pub type INT_CONFIG11_R = crate::FieldReader;
///Field `INT_CONFIG11` writer - INT_CONFIG11
pub type INT_CONFIG11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG12` reader - INT_CONFIG12
pub type INT_CONFIG12_R = crate::FieldReader;
///Field `INT_CONFIG12` writer - INT_CONFIG12
pub type INT_CONFIG12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG13` reader - INT_CONFIG13
pub type INT_CONFIG13_R = crate::FieldReader;
///Field `INT_CONFIG13` writer - INT_CONFIG13
pub type INT_CONFIG13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG14` reader - INT_CONFIG14
pub type INT_CONFIG14_R = crate::FieldReader;
///Field `INT_CONFIG14` writer - INT_CONFIG14
pub type INT_CONFIG14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INT_CONFIG15` reader - INT_CONFIG15
pub type INT_CONFIG15_R = crate::FieldReader;
///Field `INT_CONFIG15` writer - INT_CONFIG15
pub type INT_CONFIG15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - INT_CONFIG0
    #[inline(always)]
    pub fn int_config0(&self) -> INT_CONFIG0_R {
        INT_CONFIG0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - INT_CONFIG1
    #[inline(always)]
    pub fn int_config1(&self) -> INT_CONFIG1_R {
        INT_CONFIG1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - INT_CONFIG2
    #[inline(always)]
    pub fn int_config2(&self) -> INT_CONFIG2_R {
        INT_CONFIG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - INT_CONFIG3
    #[inline(always)]
    pub fn int_config3(&self) -> INT_CONFIG3_R {
        INT_CONFIG3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - INT_CONFIG4
    #[inline(always)]
    pub fn int_config4(&self) -> INT_CONFIG4_R {
        INT_CONFIG4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - INT_CONFIG5
    #[inline(always)]
    pub fn int_config5(&self) -> INT_CONFIG5_R {
        INT_CONFIG5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - INT_CONFIG6
    #[inline(always)]
    pub fn int_config6(&self) -> INT_CONFIG6_R {
        INT_CONFIG6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - INT_CONFIG7
    #[inline(always)]
    pub fn int_config7(&self) -> INT_CONFIG7_R {
        INT_CONFIG7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - INT_CONFIG8
    #[inline(always)]
    pub fn int_config8(&self) -> INT_CONFIG8_R {
        INT_CONFIG8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - INT_CONFIG9
    #[inline(always)]
    pub fn int_config9(&self) -> INT_CONFIG9_R {
        INT_CONFIG9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - INT_CONFIG10
    #[inline(always)]
    pub fn int_config10(&self) -> INT_CONFIG10_R {
        INT_CONFIG10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - INT_CONFIG11
    #[inline(always)]
    pub fn int_config11(&self) -> INT_CONFIG11_R {
        INT_CONFIG11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - INT_CONFIG12
    #[inline(always)]
    pub fn int_config12(&self) -> INT_CONFIG12_R {
        INT_CONFIG12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - INT_CONFIG13
    #[inline(always)]
    pub fn int_config13(&self) -> INT_CONFIG13_R {
        INT_CONFIG13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - INT_CONFIG14
    #[inline(always)]
    pub fn int_config14(&self) -> INT_CONFIG14_R {
        INT_CONFIG14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - INT_CONFIG15
    #[inline(always)]
    pub fn int_config15(&self) -> INT_CONFIG15_R {
        INT_CONFIG15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICFGR1")
            .field("int_config0", &self.int_config0())
            .field("int_config1", &self.int_config1())
            .field("int_config2", &self.int_config2())
            .field("int_config3", &self.int_config3())
            .field("int_config4", &self.int_config4())
            .field("int_config5", &self.int_config5())
            .field("int_config6", &self.int_config6())
            .field("int_config7", &self.int_config7())
            .field("int_config8", &self.int_config8())
            .field("int_config9", &self.int_config9())
            .field("int_config10", &self.int_config10())
            .field("int_config11", &self.int_config11())
            .field("int_config12", &self.int_config12())
            .field("int_config13", &self.int_config13())
            .field("int_config14", &self.int_config14())
            .field("int_config15", &self.int_config15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - INT_CONFIG0
    #[inline(always)]
    pub fn int_config0(&mut self) -> INT_CONFIG0_W<'_, ICFGR1rs> {
        INT_CONFIG0_W::new(self, 0)
    }
    ///Bits 2:3 - INT_CONFIG1
    #[inline(always)]
    pub fn int_config1(&mut self) -> INT_CONFIG1_W<'_, ICFGR1rs> {
        INT_CONFIG1_W::new(self, 2)
    }
    ///Bits 4:5 - INT_CONFIG2
    #[inline(always)]
    pub fn int_config2(&mut self) -> INT_CONFIG2_W<'_, ICFGR1rs> {
        INT_CONFIG2_W::new(self, 4)
    }
    ///Bits 6:7 - INT_CONFIG3
    #[inline(always)]
    pub fn int_config3(&mut self) -> INT_CONFIG3_W<'_, ICFGR1rs> {
        INT_CONFIG3_W::new(self, 6)
    }
    ///Bits 8:9 - INT_CONFIG4
    #[inline(always)]
    pub fn int_config4(&mut self) -> INT_CONFIG4_W<'_, ICFGR1rs> {
        INT_CONFIG4_W::new(self, 8)
    }
    ///Bits 10:11 - INT_CONFIG5
    #[inline(always)]
    pub fn int_config5(&mut self) -> INT_CONFIG5_W<'_, ICFGR1rs> {
        INT_CONFIG5_W::new(self, 10)
    }
    ///Bits 12:13 - INT_CONFIG6
    #[inline(always)]
    pub fn int_config6(&mut self) -> INT_CONFIG6_W<'_, ICFGR1rs> {
        INT_CONFIG6_W::new(self, 12)
    }
    ///Bits 14:15 - INT_CONFIG7
    #[inline(always)]
    pub fn int_config7(&mut self) -> INT_CONFIG7_W<'_, ICFGR1rs> {
        INT_CONFIG7_W::new(self, 14)
    }
    ///Bits 16:17 - INT_CONFIG8
    #[inline(always)]
    pub fn int_config8(&mut self) -> INT_CONFIG8_W<'_, ICFGR1rs> {
        INT_CONFIG8_W::new(self, 16)
    }
    ///Bits 18:19 - INT_CONFIG9
    #[inline(always)]
    pub fn int_config9(&mut self) -> INT_CONFIG9_W<'_, ICFGR1rs> {
        INT_CONFIG9_W::new(self, 18)
    }
    ///Bits 20:21 - INT_CONFIG10
    #[inline(always)]
    pub fn int_config10(&mut self) -> INT_CONFIG10_W<'_, ICFGR1rs> {
        INT_CONFIG10_W::new(self, 20)
    }
    ///Bits 22:23 - INT_CONFIG11
    #[inline(always)]
    pub fn int_config11(&mut self) -> INT_CONFIG11_W<'_, ICFGR1rs> {
        INT_CONFIG11_W::new(self, 22)
    }
    ///Bits 24:25 - INT_CONFIG12
    #[inline(always)]
    pub fn int_config12(&mut self) -> INT_CONFIG12_W<'_, ICFGR1rs> {
        INT_CONFIG12_W::new(self, 24)
    }
    ///Bits 26:27 - INT_CONFIG13
    #[inline(always)]
    pub fn int_config13(&mut self) -> INT_CONFIG13_W<'_, ICFGR1rs> {
        INT_CONFIG13_W::new(self, 26)
    }
    ///Bits 28:29 - INT_CONFIG14
    #[inline(always)]
    pub fn int_config14(&mut self) -> INT_CONFIG14_W<'_, ICFGR1rs> {
        INT_CONFIG14_W::new(self, 28)
    }
    ///Bits 30:31 - INT_CONFIG15
    #[inline(always)]
    pub fn int_config15(&mut self) -> INT_CONFIG15_W<'_, ICFGR1rs> {
        INT_CONFIG15_W::new(self, 30)
    }
}
/**GICD interrupt configuration register

You can [`read`](crate::Reg::read) this register and get [`icfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ICFGR1)*/
pub struct ICFGR1rs;
impl crate::RegisterSpec for ICFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`icfgr1::R`](R) reader structure
impl crate::Readable for ICFGR1rs {}
///`write(|w| ..)` method takes [`icfgr1::W`](W) writer structure
impl crate::Writable for ICFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICFGR1 to value 0x5554_0000
impl crate::Resettable for ICFGR1rs {
    const RESET_VALUE: u32 = 0x5554_0000;
}
