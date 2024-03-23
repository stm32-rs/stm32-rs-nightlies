#[doc = "Register `APB3ENR` reader"]
pub type R = crate::R<APB3ENRrs>;
#[doc = "Register `APB3ENR` writer"]
pub type W = crate::W<APB3ENRrs>;
#[doc = "sub-GHz radio SPI clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUBGHZSPIEN {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<SUBGHZSPIEN> for bool {
    #[inline(always)]
    fn from(variant: SUBGHZSPIEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUBGHZSPIEN` reader - sub-GHz radio SPI clock enable"]
pub type SUBGHZSPIEN_R = crate::BitReader<SUBGHZSPIEN>;
impl SUBGHZSPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUBGHZSPIEN {
        match self.bits {
            false => SUBGHZSPIEN::Disabled,
            true => SUBGHZSPIEN::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUBGHZSPIEN::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUBGHZSPIEN::Enabled
    }
}
#[doc = "Field `SUBGHZSPIEN` writer - sub-GHz radio SPI clock enable"]
pub type SUBGHZSPIEN_W<'a, REG> = crate::BitWriter<'a, REG, SUBGHZSPIEN>;
impl<'a, REG> SUBGHZSPIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHZSPIEN::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHZSPIEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - sub-GHz radio SPI clock enable"]
    #[inline(always)]
    pub fn subghzspien(&self) -> SUBGHZSPIEN_R {
        SUBGHZSPIEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sub-GHz radio SPI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn subghzspien(&mut self) -> SUBGHZSPIEN_W<APB3ENRrs> {
        SUBGHZSPIEN_W::new(self, 0)
    }
}
#[doc = "APB3 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3ENRrs;
impl crate::RegisterSpec for APB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3enr::R`](R) reader structure"]
impl crate::Readable for APB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb3enr::W`](W) writer structure"]
impl crate::Writable for APB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3ENR to value 0"]
impl crate::Resettable for APB3ENRrs {
    const RESET_VALUE: u32 = 0;
}
