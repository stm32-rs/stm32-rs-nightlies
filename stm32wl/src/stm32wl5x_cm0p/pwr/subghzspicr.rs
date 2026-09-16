///Register `SUBGHZSPICR` reader
pub type R = crate::R<SUBGHZSPICRrs>;
///Register `SUBGHZSPICR` writer
pub type W = crate::W<SUBGHZSPICRrs>;
/**sub-GHz SPI NSS control

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSS {
    ///0: Sub-GHz SPI NSS signal at level low
    Low = 0,
    ///1: Sub-GHz SPI NSS signal is at level high
    High = 1,
}
impl From<NSS> for bool {
    #[inline(always)]
    fn from(variant: NSS) -> Self {
        variant as u8 != 0
    }
}
///Field `NSS` reader - sub-GHz SPI NSS control
pub type NSS_R = crate::BitReader<NSS>;
impl NSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NSS {
        match self.bits {
            false => NSS::Low,
            true => NSS::High,
        }
    }
    ///Sub-GHz SPI NSS signal at level low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == NSS::Low
    }
    ///Sub-GHz SPI NSS signal is at level high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == NSS::High
    }
}
///Field `NSS` writer - sub-GHz SPI NSS control
pub type NSS_W<'a, REG> = crate::BitWriter<'a, REG, NSS>;
impl<'a, REG> NSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sub-GHz SPI NSS signal at level low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(NSS::Low)
    }
    ///Sub-GHz SPI NSS signal is at level high
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(NSS::High)
    }
}
impl R {
    ///Bit 15 - sub-GHz SPI NSS control
    #[inline(always)]
    pub fn nss(&self) -> NSS_R {
        NSS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUBGHZSPICR")
            .field("nss", &self.nss())
            .finish()
    }
}
impl W {
    ///Bit 15 - sub-GHz SPI NSS control
    #[inline(always)]
    pub fn nss(&mut self) -> NSS_W<'_, SUBGHZSPICRrs> {
        NSS_W::new(self, 15)
    }
}
/**Power SPI3 control register

You can [`read`](crate::Reg::read) this register and get [`subghzspicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subghzspicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#PWR:SUBGHZSPICR)*/
pub struct SUBGHZSPICRrs;
impl crate::RegisterSpec for SUBGHZSPICRrs {
    type Ux = u32;
}
///`read()` method returns [`subghzspicr::R`](R) reader structure
impl crate::Readable for SUBGHZSPICRrs {}
///`write(|w| ..)` method takes [`subghzspicr::W`](W) writer structure
impl crate::Writable for SUBGHZSPICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SUBGHZSPICR to value 0x8000
impl crate::Resettable for SUBGHZSPICRrs {
    const RESET_VALUE: u32 = 0x8000;
}
