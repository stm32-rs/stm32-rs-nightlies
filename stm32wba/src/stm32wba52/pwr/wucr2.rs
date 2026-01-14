///Register `WUCR2` reader
pub type R = crate::R<WUCR2rs>;
///Register `WUCR2` writer
pub type W = crate::W<WUCR2rs>;
///Field `WUPP1` reader - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0. Access can be secured by PWR WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP1_R = crate::BitReader;
///Field `WUPP1` writer - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0. Access can be secured by PWR WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP2` reader - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0. Access can be secured by PWR WUP2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP2_R = crate::BitReader;
///Field `WUPP2` writer - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0. Access can be secured by PWR WUP2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP3` reader - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0. Access can be secured by PWR WUP3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP3_R = crate::BitReader;
///Field `WUPP3` writer - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0. Access can be secured by PWR WUP3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP4` reader - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0. Access can be secured by PWR WUP4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP4_R = crate::BitReader;
///Field `WUPP4` writer - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0. Access can be secured by PWR WUP4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP5` reader - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0. Access can be secured by PWR WUP5SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP5_R = crate::BitReader;
///Field `WUPP5` writer - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0. Access can be secured by PWR WUP5SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP6` reader - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0. Access can be secured by PWR WUP6SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP6_R = crate::BitReader;
///Field `WUPP6` writer - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0. Access can be secured by PWR WUP6SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP7` reader - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0. Access can be secured by PWR WUP7SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP7_R = crate::BitReader;
///Field `WUPP7` writer - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0. Access can be secured by PWR WUP7SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP8` reader - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0. Access can be secured by PWR WUP8SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP8_R = crate::BitReader;
///Field `WUPP8` writer - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0. Access can be secured by PWR WUP8SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type WUPP8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0. Access can be secured by PWR WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0. Access can be secured by PWR WUP2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0. Access can be secured by PWR WUP3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0. Access can be secured by PWR WUP4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0. Access can be secured by PWR WUP5SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0. Access can be secured by PWR WUP6SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp6(&self) -> WUPP6_R {
        WUPP6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0. Access can be secured by PWR WUP7SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp7(&self) -> WUPP7_R {
        WUPP7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0. Access can be secured by PWR WUP8SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp8(&self) -> WUPP8_R {
        WUPP8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUCR2")
            .field("wupp1", &self.wupp1())
            .field("wupp2", &self.wupp2())
            .field("wupp3", &self.wupp3())
            .field("wupp4", &self.wupp4())
            .field("wupp5", &self.wupp5())
            .field("wupp6", &self.wupp6())
            .field("wupp7", &self.wupp7())
            .field("wupp8", &self.wupp8())
            .finish()
    }
}
impl W {
    ///Bit 0 - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0. Access can be secured by PWR WUP1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp1(&mut self) -> WUPP1_W<'_, WUCR2rs> {
        WUPP1_W::new(self, 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0. Access can be secured by PWR WUP2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp2(&mut self) -> WUPP2_W<'_, WUCR2rs> {
        WUPP2_W::new(self, 1)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0. Access can be secured by PWR WUP3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp3(&mut self) -> WUPP3_W<'_, WUCR2rs> {
        WUPP3_W::new(self, 2)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0. Access can be secured by PWR WUP4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp4(&mut self) -> WUPP4_W<'_, WUCR2rs> {
        WUPP4_W::new(self, 3)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0. Access can be secured by PWR WUP5SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp5(&mut self) -> WUPP5_W<'_, WUCR2rs> {
        WUPP5_W::new(self, 4)
    }
    ///Bit 5 - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0. Access can be secured by PWR WUP6SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp6(&mut self) -> WUPP6_W<'_, WUCR2rs> {
        WUPP6_W::new(self, 5)
    }
    ///Bit 6 - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0. Access can be secured by PWR WUP7SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp7(&mut self) -> WUPP7_W<'_, WUCR2rs> {
        WUPP7_W::new(self, 6)
    }
    ///Bit 7 - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0. Access can be secured by PWR WUP8SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn wupp8(&mut self) -> WUPP8_W<'_, WUCR2rs> {
        WUPP8_W::new(self, 7)
    }
}
/**PWR wakeup control register 2

You can [`read`](crate::Reg::read) this register and get [`wucr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wucr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#PWR:WUCR2)*/
pub struct WUCR2rs;
impl crate::RegisterSpec for WUCR2rs {
    type Ux = u32;
}
///`read()` method returns [`wucr2::R`](R) reader structure
impl crate::Readable for WUCR2rs {}
///`write(|w| ..)` method takes [`wucr2::W`](W) writer structure
impl crate::Writable for WUCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUCR2 to value 0
impl crate::Resettable for WUCR2rs {}
