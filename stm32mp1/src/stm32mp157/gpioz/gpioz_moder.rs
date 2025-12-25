///Register `GPIOZ_MODER` reader
pub type R = crate::R<GPIOZ_MODERrs>;
///Register `GPIOZ_MODER` writer
pub type W = crate::W<GPIOZ_MODERrs>;
///Field `MODER0` reader - MODER0
pub type MODER0_R = crate::FieldReader;
///Field `MODER0` writer - MODER0
pub type MODER0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER1` reader - MODER1
pub type MODER1_R = crate::FieldReader;
///Field `MODER1` writer - MODER1
pub type MODER1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER2` reader - MODER2
pub type MODER2_R = crate::FieldReader;
///Field `MODER2` writer - MODER2
pub type MODER2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER3` reader - MODER3
pub type MODER3_R = crate::FieldReader;
///Field `MODER3` writer - MODER3
pub type MODER3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER4` reader - MODER4
pub type MODER4_R = crate::FieldReader;
///Field `MODER4` writer - MODER4
pub type MODER4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER5` reader - MODER5
pub type MODER5_R = crate::FieldReader;
///Field `MODER5` writer - MODER5
pub type MODER5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER6` reader - MODER6
pub type MODER6_R = crate::FieldReader;
///Field `MODER6` writer - MODER6
pub type MODER6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER7` reader - MODER7
pub type MODER7_R = crate::FieldReader;
///Field `MODER7` writer - MODER7
pub type MODER7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER8` reader - MODER8
pub type MODER8_R = crate::FieldReader;
///Field `MODER8` writer - MODER8
pub type MODER8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER9` reader - MODER9
pub type MODER9_R = crate::FieldReader;
///Field `MODER9` writer - MODER9
pub type MODER9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER10` reader - MODER10
pub type MODER10_R = crate::FieldReader;
///Field `MODER10` writer - MODER10
pub type MODER10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER11` reader - MODER11
pub type MODER11_R = crate::FieldReader;
///Field `MODER11` writer - MODER11
pub type MODER11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER12` reader - MODER12
pub type MODER12_R = crate::FieldReader;
///Field `MODER12` writer - MODER12
pub type MODER12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER13` reader - MODER13
pub type MODER13_R = crate::FieldReader;
///Field `MODER13` writer - MODER13
pub type MODER13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER14` reader - MODER14
pub type MODER14_R = crate::FieldReader;
///Field `MODER14` writer - MODER14
pub type MODER14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODER15` reader - MODER15
pub type MODER15_R = crate::FieldReader;
///Field `MODER15` writer - MODER15
pub type MODER15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - MODER0
    #[inline(always)]
    pub fn moder0(&self) -> MODER0_R {
        MODER0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - MODER1
    #[inline(always)]
    pub fn moder1(&self) -> MODER1_R {
        MODER1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - MODER2
    #[inline(always)]
    pub fn moder2(&self) -> MODER2_R {
        MODER2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - MODER3
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - MODER4
    #[inline(always)]
    pub fn moder4(&self) -> MODER4_R {
        MODER4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - MODER5
    #[inline(always)]
    pub fn moder5(&self) -> MODER5_R {
        MODER5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - MODER6
    #[inline(always)]
    pub fn moder6(&self) -> MODER6_R {
        MODER6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - MODER7
    #[inline(always)]
    pub fn moder7(&self) -> MODER7_R {
        MODER7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - MODER8
    #[inline(always)]
    pub fn moder8(&self) -> MODER8_R {
        MODER8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - MODER9
    #[inline(always)]
    pub fn moder9(&self) -> MODER9_R {
        MODER9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - MODER10
    #[inline(always)]
    pub fn moder10(&self) -> MODER10_R {
        MODER10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - MODER11
    #[inline(always)]
    pub fn moder11(&self) -> MODER11_R {
        MODER11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - MODER12
    #[inline(always)]
    pub fn moder12(&self) -> MODER12_R {
        MODER12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - MODER13
    #[inline(always)]
    pub fn moder13(&self) -> MODER13_R {
        MODER13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - MODER14
    #[inline(always)]
    pub fn moder14(&self) -> MODER14_R {
        MODER14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - MODER15
    #[inline(always)]
    pub fn moder15(&self) -> MODER15_R {
        MODER15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOZ_MODER")
            .field("moder0", &self.moder0())
            .field("moder1", &self.moder1())
            .field("moder2", &self.moder2())
            .field("moder3", &self.moder3())
            .field("moder4", &self.moder4())
            .field("moder5", &self.moder5())
            .field("moder6", &self.moder6())
            .field("moder7", &self.moder7())
            .field("moder8", &self.moder8())
            .field("moder9", &self.moder9())
            .field("moder10", &self.moder10())
            .field("moder11", &self.moder11())
            .field("moder12", &self.moder12())
            .field("moder13", &self.moder13())
            .field("moder14", &self.moder14())
            .field("moder15", &self.moder15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - MODER0
    #[inline(always)]
    pub fn moder0(&mut self) -> MODER0_W<'_, GPIOZ_MODERrs> {
        MODER0_W::new(self, 0)
    }
    ///Bits 2:3 - MODER1
    #[inline(always)]
    pub fn moder1(&mut self) -> MODER1_W<'_, GPIOZ_MODERrs> {
        MODER1_W::new(self, 2)
    }
    ///Bits 4:5 - MODER2
    #[inline(always)]
    pub fn moder2(&mut self) -> MODER2_W<'_, GPIOZ_MODERrs> {
        MODER2_W::new(self, 4)
    }
    ///Bits 6:7 - MODER3
    #[inline(always)]
    pub fn moder3(&mut self) -> MODER3_W<'_, GPIOZ_MODERrs> {
        MODER3_W::new(self, 6)
    }
    ///Bits 8:9 - MODER4
    #[inline(always)]
    pub fn moder4(&mut self) -> MODER4_W<'_, GPIOZ_MODERrs> {
        MODER4_W::new(self, 8)
    }
    ///Bits 10:11 - MODER5
    #[inline(always)]
    pub fn moder5(&mut self) -> MODER5_W<'_, GPIOZ_MODERrs> {
        MODER5_W::new(self, 10)
    }
    ///Bits 12:13 - MODER6
    #[inline(always)]
    pub fn moder6(&mut self) -> MODER6_W<'_, GPIOZ_MODERrs> {
        MODER6_W::new(self, 12)
    }
    ///Bits 14:15 - MODER7
    #[inline(always)]
    pub fn moder7(&mut self) -> MODER7_W<'_, GPIOZ_MODERrs> {
        MODER7_W::new(self, 14)
    }
    ///Bits 16:17 - MODER8
    #[inline(always)]
    pub fn moder8(&mut self) -> MODER8_W<'_, GPIOZ_MODERrs> {
        MODER8_W::new(self, 16)
    }
    ///Bits 18:19 - MODER9
    #[inline(always)]
    pub fn moder9(&mut self) -> MODER9_W<'_, GPIOZ_MODERrs> {
        MODER9_W::new(self, 18)
    }
    ///Bits 20:21 - MODER10
    #[inline(always)]
    pub fn moder10(&mut self) -> MODER10_W<'_, GPIOZ_MODERrs> {
        MODER10_W::new(self, 20)
    }
    ///Bits 22:23 - MODER11
    #[inline(always)]
    pub fn moder11(&mut self) -> MODER11_W<'_, GPIOZ_MODERrs> {
        MODER11_W::new(self, 22)
    }
    ///Bits 24:25 - MODER12
    #[inline(always)]
    pub fn moder12(&mut self) -> MODER12_W<'_, GPIOZ_MODERrs> {
        MODER12_W::new(self, 24)
    }
    ///Bits 26:27 - MODER13
    #[inline(always)]
    pub fn moder13(&mut self) -> MODER13_W<'_, GPIOZ_MODERrs> {
        MODER13_W::new(self, 26)
    }
    ///Bits 28:29 - MODER14
    #[inline(always)]
    pub fn moder14(&mut self) -> MODER14_W<'_, GPIOZ_MODERrs> {
        MODER14_W::new(self, 28)
    }
    ///Bits 30:31 - MODER15
    #[inline(always)]
    pub fn moder15(&mut self) -> MODER15_W<'_, GPIOZ_MODERrs> {
        MODER15_W::new(self, 30)
    }
}
/**GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`gpioz_moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioz_moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOZ:GPIOZ_MODER)*/
pub struct GPIOZ_MODERrs;
impl crate::RegisterSpec for GPIOZ_MODERrs {
    type Ux = u32;
}
///`read()` method returns [`gpioz_moder::R`](R) reader structure
impl crate::Readable for GPIOZ_MODERrs {}
///`write(|w| ..)` method takes [`gpioz_moder::W`](W) writer structure
impl crate::Writable for GPIOZ_MODERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOZ_MODER to value 0xffff_ffff
impl crate::Resettable for GPIOZ_MODERrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
