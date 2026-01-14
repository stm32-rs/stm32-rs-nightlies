///Register `WUCR3` reader
pub type R = crate::R<WUCR3rs>;
///Register `WUCR3` writer
pub type W = crate::W<WUCR3rs>;
///Field `WUSEL1` reader - Wakeup and interrupt pin WKUP1 selection This field must be configured when WUPEN1 = 0. Access can be secured by PWR WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL1_R = crate::FieldReader;
///Field `WUSEL1` writer - Wakeup and interrupt pin WKUP1 selection This field must be configured when WUPEN1 = 0. Access can be secured by PWR WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL2` reader - Wakeup and interrupt pin WKUP2 selection This field must be configured when WUPEN2 = 0. Access can be secured by PWR WUP2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL2_R = crate::FieldReader;
///Field `WUSEL2` writer - Wakeup and interrupt pin WKUP2 selection This field must be configured when WUPEN2 = 0. Access can be secured by PWR WUP2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL3` reader - Wakeup and interrupt pin WKUP3 selection This field must be configured when WUPEN3 = 0. Access can be secured by PWR WUP3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL3_R = crate::FieldReader;
///Field `WUSEL3` writer - Wakeup and interrupt pin WKUP3 selection This field must be configured when WUPEN3 = 0. Access can be secured by PWR WUP3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL4` reader - Wakeup and interrupt pin WKUP4 selection This field must be configured when WUPEN4 = 0. Access can be secured by PWR WUP4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL4_R = crate::FieldReader;
///Field `WUSEL4` writer - Wakeup and interrupt pin WKUP4 selection This field must be configured when WUPEN4 = 0. Access can be secured by PWR WUP4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL5` reader - Wakeup and interrupt pin WKUP5 selection This field must be configured when WUPEN5 = 0. Access can be secured by PWR WUP5SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL5_R = crate::FieldReader;
///Field `WUSEL5` writer - Wakeup and interrupt pin WKUP5 selection This field must be configured when WUPEN5 = 0. Access can be secured by PWR WUP5SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL6` reader - Wakeup and interrupt pin WKUP6 selection This field must be configured when WUPEN6 = 0. Access can be secured by PWR WUP6SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL6_R = crate::FieldReader;
///Field `WUSEL6` writer - Wakeup and interrupt pin WKUP6 selection This field must be configured when WUPEN6 = 0. Access can be secured by PWR WUP6SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL7` reader - Wakeup and interrupt pin WKUP7 selection This field must be configured when WUPEN7 = 0. Access can be secured by PWR WUP7SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL7_R = crate::FieldReader;
///Field `WUSEL7` writer - Wakeup and interrupt pin WKUP7 selection This field must be configured when WUPEN7 = 0. Access can be secured by PWR WUP7SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUSEL8` reader - Wakeup and interrupt pin WKUP8 selection This field must be configured when WUPEN8 = 0. Access can be secured by PWR WUP8SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL8_R = crate::FieldReader;
///Field `WUSEL8` writer - Wakeup and interrupt pin WKUP8 selection This field must be configured when WUPEN8 = 0. Access can be secured by PWR WUP8SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Wakeup and interrupt pin WKUP1 selection This field must be configured when WUPEN1 = 0. Access can be secured by PWR WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel1(&self) -> WUSEL1_R {
        WUSEL1_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Wakeup and interrupt pin WKUP2 selection This field must be configured when WUPEN2 = 0. Access can be secured by PWR WUP2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel2(&self) -> WUSEL2_R {
        WUSEL2_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Wakeup and interrupt pin WKUP3 selection This field must be configured when WUPEN3 = 0. Access can be secured by PWR WUP3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel3(&self) -> WUSEL3_R {
        WUSEL3_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Wakeup and interrupt pin WKUP4 selection This field must be configured when WUPEN4 = 0. Access can be secured by PWR WUP4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel4(&self) -> WUSEL4_R {
        WUSEL4_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Wakeup and interrupt pin WKUP5 selection This field must be configured when WUPEN5 = 0. Access can be secured by PWR WUP5SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel5(&self) -> WUSEL5_R {
        WUSEL5_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Wakeup and interrupt pin WKUP6 selection This field must be configured when WUPEN6 = 0. Access can be secured by PWR WUP6SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel6(&self) -> WUSEL6_R {
        WUSEL6_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Wakeup and interrupt pin WKUP7 selection This field must be configured when WUPEN7 = 0. Access can be secured by PWR WUP7SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel7(&self) -> WUSEL7_R {
        WUSEL7_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Wakeup and interrupt pin WKUP8 selection This field must be configured when WUPEN8 = 0. Access can be secured by PWR WUP8SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel8(&self) -> WUSEL8_R {
        WUSEL8_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUCR3")
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
    ///Bits 0:1 - Wakeup and interrupt pin WKUP1 selection This field must be configured when WUPEN1 = 0. Access can be secured by PWR WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel1(&mut self) -> WUSEL1_W<'_, WUCR3rs> {
        WUSEL1_W::new(self, 0)
    }
    ///Bits 2:3 - Wakeup and interrupt pin WKUP2 selection This field must be configured when WUPEN2 = 0. Access can be secured by PWR WUP2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel2(&mut self) -> WUSEL2_W<'_, WUCR3rs> {
        WUSEL2_W::new(self, 2)
    }
    ///Bits 4:5 - Wakeup and interrupt pin WKUP3 selection This field must be configured when WUPEN3 = 0. Access can be secured by PWR WUP3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel3(&mut self) -> WUSEL3_W<'_, WUCR3rs> {
        WUSEL3_W::new(self, 4)
    }
    ///Bits 6:7 - Wakeup and interrupt pin WKUP4 selection This field must be configured when WUPEN4 = 0. Access can be secured by PWR WUP4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel4(&mut self) -> WUSEL4_W<'_, WUCR3rs> {
        WUSEL4_W::new(self, 6)
    }
    ///Bits 8:9 - Wakeup and interrupt pin WKUP5 selection This field must be configured when WUPEN5 = 0. Access can be secured by PWR WUP5SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel5(&mut self) -> WUSEL5_W<'_, WUCR3rs> {
        WUSEL5_W::new(self, 8)
    }
    ///Bits 10:11 - Wakeup and interrupt pin WKUP6 selection This field must be configured when WUPEN6 = 0. Access can be secured by PWR WUP6SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel6(&mut self) -> WUSEL6_W<'_, WUCR3rs> {
        WUSEL6_W::new(self, 10)
    }
    ///Bits 12:13 - Wakeup and interrupt pin WKUP7 selection This field must be configured when WUPEN7 = 0. Access can be secured by PWR WUP7SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel7(&mut self) -> WUSEL7_W<'_, WUCR3rs> {
        WUSEL7_W::new(self, 12)
    }
    ///Bits 14:15 - Wakeup and interrupt pin WKUP8 selection This field must be configured when WUPEN8 = 0. Access can be secured by PWR WUP8SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wusel8(&mut self) -> WUSEL8_W<'_, WUCR3rs> {
        WUSEL8_W::new(self, 14)
    }
}
/**PWR wakeup control register 3

You can [`read`](crate::Reg::read) this register and get [`wucr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#PWR:WUCR3)*/
pub struct WUCR3rs;
impl crate::RegisterSpec for WUCR3rs {
    type Ux = u32;
}
///`read()` method returns [`wucr3::R`](R) reader structure
impl crate::Readable for WUCR3rs {}
///`write(|w| ..)` method takes [`wucr3::W`](W) writer structure
impl crate::Writable for WUCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUCR3 to value 0
impl crate::Resettable for WUCR3rs {}
