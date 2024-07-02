///Register `PWR_WUCR3` reader
pub type R = crate::R<PWR_WUCR3rs>;
///Register `PWR_WUCR3` writer
pub type W = crate::W<PWR_WUCR3rs>;
///Field `WUSEL1` reader - Wakeup pin WKUP1 selection This field must be configured when WUPEN1 = 0.
pub type WUSEL1_R = crate::FieldReader;
///Field `WUSEL1` writer - Wakeup pin WKUP1 selection This field must be configured when WUPEN1 = 0.
pub type WUSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL2` reader - Wakeup pin WKUP2 selection This field must be configured when WUPEN2 = 0.
pub type WUSEL2_R = crate::FieldReader;
///Field `WUSEL2` writer - Wakeup pin WKUP2 selection This field must be configured when WUPEN2 = 0.
pub type WUSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL3` reader - Wakeup pin WKUP3 selection This field must be configured when WUPEN3 = 0.
pub type WUSEL3_R = crate::FieldReader;
///Field `WUSEL3` writer - Wakeup pin WKUP3 selection This field must be configured when WUPEN3 = 0.
pub type WUSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL4` reader - Wakeup pin WKUP4 selection This field must be configured when WUPEN4 = 0.
pub type WUSEL4_R = crate::FieldReader;
///Field `WUSEL4` writer - Wakeup pin WKUP4 selection This field must be configured when WUPEN4 = 0.
pub type WUSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL5` reader - Wakeup pin WKUP5 selection This field must be configured when WUPEN5 = 0.
pub type WUSEL5_R = crate::FieldReader;
///Field `WUSEL5` writer - Wakeup pin WKUP5 selection This field must be configured when WUPEN5 = 0.
pub type WUSEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL6` reader - Wakeup pin WKUP6 selection This field must be configured when WUPEN6 = 0.
pub type WUSEL6_R = crate::FieldReader;
///Field `WUSEL6` writer - Wakeup pin WKUP6 selection This field must be configured when WUPEN6 = 0.
pub type WUSEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL7` reader - Wakeup pin WKUP7 selection This field must be configured when WUPEN7 = 0.
pub type WUSEL7_R = crate::FieldReader;
///Field `WUSEL7` writer - Wakeup pin WKUP7 selection This field must be configured when WUPEN7 = 0.
pub type WUSEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL8` reader - Wakeup pin WKUP8 selection This field must be configured when WUPEN8 = 0.
pub type WUSEL8_R = crate::FieldReader;
///Field `WUSEL8` writer - Wakeup pin WKUP8 selection This field must be configured when WUPEN8 = 0.
pub type WUSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Wakeup pin WKUP1 selection This field must be configured when WUPEN1 = 0.
    #[inline(always)]
    pub fn wusel1(&self) -> WUSEL1_R {
        WUSEL1_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Wakeup pin WKUP2 selection This field must be configured when WUPEN2 = 0.
    #[inline(always)]
    pub fn wusel2(&self) -> WUSEL2_R {
        WUSEL2_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Wakeup pin WKUP3 selection This field must be configured when WUPEN3 = 0.
    #[inline(always)]
    pub fn wusel3(&self) -> WUSEL3_R {
        WUSEL3_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Wakeup pin WKUP4 selection This field must be configured when WUPEN4 = 0.
    #[inline(always)]
    pub fn wusel4(&self) -> WUSEL4_R {
        WUSEL4_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Wakeup pin WKUP5 selection This field must be configured when WUPEN5 = 0.
    #[inline(always)]
    pub fn wusel5(&self) -> WUSEL5_R {
        WUSEL5_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Wakeup pin WKUP6 selection This field must be configured when WUPEN6 = 0.
    #[inline(always)]
    pub fn wusel6(&self) -> WUSEL6_R {
        WUSEL6_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Wakeup pin WKUP7 selection This field must be configured when WUPEN7 = 0.
    #[inline(always)]
    pub fn wusel7(&self) -> WUSEL7_R {
        WUSEL7_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Wakeup pin WKUP8 selection This field must be configured when WUPEN8 = 0.
    #[inline(always)]
    pub fn wusel8(&self) -> WUSEL8_R {
        WUSEL8_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_WUCR3")
            .field("wusel1", &self.wusel1())
            .field("wusel2", &self.wusel2())
            .field("wusel3", &self.wusel3())
            .field("wusel4", &self.wusel4())
            .field("wusel5", &self.wusel5())
            .field("wusel6", &self.wusel6())
            .field("wusel7", &self.wusel7())
            .field("wusel8", &self.wusel8())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Wakeup pin WKUP1 selection This field must be configured when WUPEN1 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel1(&mut self) -> WUSEL1_W<PWR_WUCR3rs> {
        WUSEL1_W::new(self, 0)
    }
    ///Bits 2:3 - Wakeup pin WKUP2 selection This field must be configured when WUPEN2 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel2(&mut self) -> WUSEL2_W<PWR_WUCR3rs> {
        WUSEL2_W::new(self, 2)
    }
    ///Bits 4:5 - Wakeup pin WKUP3 selection This field must be configured when WUPEN3 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel3(&mut self) -> WUSEL3_W<PWR_WUCR3rs> {
        WUSEL3_W::new(self, 4)
    }
    ///Bits 6:7 - Wakeup pin WKUP4 selection This field must be configured when WUPEN4 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel4(&mut self) -> WUSEL4_W<PWR_WUCR3rs> {
        WUSEL4_W::new(self, 6)
    }
    ///Bits 8:9 - Wakeup pin WKUP5 selection This field must be configured when WUPEN5 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel5(&mut self) -> WUSEL5_W<PWR_WUCR3rs> {
        WUSEL5_W::new(self, 8)
    }
    ///Bits 10:11 - Wakeup pin WKUP6 selection This field must be configured when WUPEN6 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel6(&mut self) -> WUSEL6_W<PWR_WUCR3rs> {
        WUSEL6_W::new(self, 10)
    }
    ///Bits 12:13 - Wakeup pin WKUP7 selection This field must be configured when WUPEN7 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel7(&mut self) -> WUSEL7_W<PWR_WUCR3rs> {
        WUSEL7_W::new(self, 12)
    }
    ///Bits 14:15 - Wakeup pin WKUP8 selection This field must be configured when WUPEN8 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel8(&mut self) -> WUSEL8_W<PWR_WUCR3rs> {
        WUSEL8_W::new(self, 14)
    }
}
/**PWR wakeup control register 3

You can [`read`](crate::Reg::read) this register and get [`pwr_wucr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_wucr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#PWR:PWR_WUCR3)*/
pub struct PWR_WUCR3rs;
impl crate::RegisterSpec for PWR_WUCR3rs {
    type Ux = u32;
}
///`read()` method returns [`pwr_wucr3::R`](R) reader structure
impl crate::Readable for PWR_WUCR3rs {}
///`write(|w| ..)` method takes [`pwr_wucr3::W`](W) writer structure
impl crate::Writable for PWR_WUCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_WUCR3 to value 0
impl crate::Resettable for PWR_WUCR3rs {
    const RESET_VALUE: u32 = 0;
}
