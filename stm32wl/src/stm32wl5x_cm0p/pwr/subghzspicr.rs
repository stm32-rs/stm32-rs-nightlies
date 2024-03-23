#[doc = "Register `SUBGHZSPICR` reader"]
pub type R = crate::R<SUBGHZSPICRrs>;
#[doc = "Register `SUBGHZSPICR` writer"]
pub type W = crate::W<SUBGHZSPICRrs>;
#[doc = "sub-GHz SPI NSS control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSS {
    #[doc = "0: Sub-GHz SPI NSS signal at level low"]
    Low = 0,
    #[doc = "1: Sub-GHz SPI NSS signal is at level high"]
    High = 1,
}
impl From<NSS> for bool {
    #[inline(always)]
    fn from(variant: NSS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSS` reader - sub-GHz SPI NSS control"]
pub type NSS_R = crate::BitReader<NSS>;
impl NSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NSS {
        match self.bits {
            false => NSS::Low,
            true => NSS::High,
        }
    }
    #[doc = "Sub-GHz SPI NSS signal at level low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == NSS::Low
    }
    #[doc = "Sub-GHz SPI NSS signal is at level high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == NSS::High
    }
}
#[doc = "Field `NSS` writer - sub-GHz SPI NSS control"]
pub type NSS_W<'a, REG> = crate::BitWriter<'a, REG, NSS>;
impl<'a, REG> NSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sub-GHz SPI NSS signal at level low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(NSS::Low)
    }
    #[doc = "Sub-GHz SPI NSS signal is at level high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(NSS::High)
    }
}
impl R {
    #[doc = "Bit 15 - sub-GHz SPI NSS control"]
    #[inline(always)]
    pub fn nss(&self) -> NSS_R {
        NSS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - sub-GHz SPI NSS control"]
    #[inline(always)]
    #[must_use]
    pub fn nss(&mut self) -> NSS_W<SUBGHZSPICRrs> {
        NSS_W::new(self, 15)
    }
}
#[doc = "Power SPI3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subghzspicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subghzspicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUBGHZSPICRrs;
impl crate::RegisterSpec for SUBGHZSPICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subghzspicr::R`](R) reader structure"]
impl crate::Readable for SUBGHZSPICRrs {}
#[doc = "`write(|w| ..)` method takes [`subghzspicr::W`](W) writer structure"]
impl crate::Writable for SUBGHZSPICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUBGHZSPICR to value 0x8000"]
impl crate::Resettable for SUBGHZSPICRrs {
    const RESET_VALUE: u32 = 0x8000;
}
