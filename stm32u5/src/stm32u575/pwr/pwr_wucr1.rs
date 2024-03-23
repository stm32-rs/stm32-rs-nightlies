#[doc = "Register `PWR_WUCR1` reader"]
pub type R = crate::R<PWR_WUCR1rs>;
#[doc = "Register `PWR_WUCR1` writer"]
pub type W = crate::W<PWR_WUCR1rs>;
#[doc = "Field `WUPEN1` reader - Wakeup pin WKUP1 enable"]
pub type WUPEN1_R = crate::BitReader;
#[doc = "Field `WUPEN1` writer - Wakeup pin WKUP1 enable"]
pub type WUPEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN2` reader - Wakeup pin WKUP2 enable"]
pub type WUPEN2_R = crate::BitReader;
#[doc = "Field `WUPEN2` writer - Wakeup pin WKUP2 enable"]
pub type WUPEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN3` reader - Wakeup pin WKUP3 enable"]
pub type WUPEN3_R = crate::BitReader;
#[doc = "Field `WUPEN3` writer - Wakeup pin WKUP3 enable"]
pub type WUPEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN4` reader - Wakeup pin WKUP4 enable"]
pub type WUPEN4_R = crate::BitReader;
#[doc = "Field `WUPEN4` writer - Wakeup pin WKUP4 enable"]
pub type WUPEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN5` reader - Wakeup pin WKUP5 enable"]
pub type WUPEN5_R = crate::BitReader;
#[doc = "Field `WUPEN5` writer - Wakeup pin WKUP5 enable"]
pub type WUPEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN6` reader - Wakeup pin WKUP6 enable"]
pub type WUPEN6_R = crate::BitReader;
#[doc = "Field `WUPEN6` writer - Wakeup pin WKUP6 enable"]
pub type WUPEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN7` reader - Wakeup pin WKUP7 enable"]
pub type WUPEN7_R = crate::BitReader;
#[doc = "Field `WUPEN7` writer - Wakeup pin WKUP7 enable"]
pub type WUPEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUPEN8` reader - Wakeup pin WKUP8 enable"]
pub type WUPEN8_R = crate::BitReader;
#[doc = "Field `WUPEN8` writer - Wakeup pin WKUP8 enable"]
pub type WUPEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup pin WKUP1 enable"]
    #[inline(always)]
    pub fn wupen1(&self) -> WUPEN1_R {
        WUPEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 enable"]
    #[inline(always)]
    pub fn wupen2(&self) -> WUPEN2_R {
        WUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 enable"]
    #[inline(always)]
    pub fn wupen3(&self) -> WUPEN3_R {
        WUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 enable"]
    #[inline(always)]
    pub fn wupen4(&self) -> WUPEN4_R {
        WUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 enable"]
    #[inline(always)]
    pub fn wupen5(&self) -> WUPEN5_R {
        WUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup pin WKUP6 enable"]
    #[inline(always)]
    pub fn wupen6(&self) -> WUPEN6_R {
        WUPEN6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup pin WKUP7 enable"]
    #[inline(always)]
    pub fn wupen7(&self) -> WUPEN7_R {
        WUPEN7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup pin WKUP8 enable"]
    #[inline(always)]
    pub fn wupen8(&self) -> WUPEN8_R {
        WUPEN8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup pin WKUP1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen1(&mut self) -> WUPEN1_W<PWR_WUCR1rs> {
        WUPEN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen2(&mut self) -> WUPEN2_W<PWR_WUCR1rs> {
        WUPEN2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen3(&mut self) -> WUPEN3_W<PWR_WUCR1rs> {
        WUPEN3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Wakeup pin WKUP4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen4(&mut self) -> WUPEN4_W<PWR_WUCR1rs> {
        WUPEN4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen5(&mut self) -> WUPEN5_W<PWR_WUCR1rs> {
        WUPEN5_W::new(self, 4)
    }
    #[doc = "Bit 5 - Wakeup pin WKUP6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen6(&mut self) -> WUPEN6_W<PWR_WUCR1rs> {
        WUPEN6_W::new(self, 5)
    }
    #[doc = "Bit 6 - Wakeup pin WKUP7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen7(&mut self) -> WUPEN7_W<PWR_WUCR1rs> {
        WUPEN7_W::new(self, 6)
    }
    #[doc = "Bit 7 - Wakeup pin WKUP8 enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen8(&mut self) -> WUPEN8_W<PWR_WUCR1rs> {
        WUPEN8_W::new(self, 7)
    }
}
#[doc = "PWR wakeup control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_wucr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_wucr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_WUCR1rs;
impl crate::RegisterSpec for PWR_WUCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_wucr1::R`](R) reader structure"]
impl crate::Readable for PWR_WUCR1rs {}
#[doc = "`write(|w| ..)` method takes [`pwr_wucr1::W`](W) writer structure"]
impl crate::Writable for PWR_WUCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_WUCR1 to value 0"]
impl crate::Resettable for PWR_WUCR1rs {
    const RESET_VALUE: u32 = 0;
}
