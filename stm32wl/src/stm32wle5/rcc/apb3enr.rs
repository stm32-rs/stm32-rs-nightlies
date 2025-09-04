///Register `APB3ENR` reader
pub type R = crate::R<APB3ENRrs>;
///Register `APB3ENR` writer
pub type W = crate::W<APB3ENRrs>;
/**sub-GHz radio SPI clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUBGHZSPIEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<SUBGHZSPIEN> for bool {
    #[inline(always)]
    fn from(variant: SUBGHZSPIEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SUBGHZSPIEN` reader - sub-GHz radio SPI clock enable
pub type SUBGHZSPIEN_R = crate::BitReader<SUBGHZSPIEN>;
impl SUBGHZSPIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUBGHZSPIEN {
        match self.bits {
            false => SUBGHZSPIEN::Disabled,
            true => SUBGHZSPIEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUBGHZSPIEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUBGHZSPIEN::Enabled
    }
}
///Field `SUBGHZSPIEN` writer - sub-GHz radio SPI clock enable
pub type SUBGHZSPIEN_W<'a, REG> = crate::BitWriter<'a, REG, SUBGHZSPIEN>;
impl<'a, REG> SUBGHZSPIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHZSPIEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SUBGHZSPIEN::Enabled)
    }
}
impl R {
    ///Bit 0 - sub-GHz radio SPI clock enable
    #[inline(always)]
    pub fn subghzspien(&self) -> SUBGHZSPIEN_R {
        SUBGHZSPIEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3ENR")
            .field("subghzspien", &self.subghzspien())
            .finish()
    }
}
impl W {
    ///Bit 0 - sub-GHz radio SPI clock enable
    #[inline(always)]
    pub fn subghzspien(&mut self) -> SUBGHZSPIEN_W<APB3ENRrs> {
        SUBGHZSPIEN_W::new(self, 0)
    }
}
/**APB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#RCC:APB3ENR)*/
pub struct APB3ENRrs;
impl crate::RegisterSpec for APB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3enr::R`](R) reader structure
impl crate::Readable for APB3ENRrs {}
///`write(|w| ..)` method takes [`apb3enr::W`](W) writer structure
impl crate::Writable for APB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3ENR to value 0
impl crate::Resettable for APB3ENRrs {}
