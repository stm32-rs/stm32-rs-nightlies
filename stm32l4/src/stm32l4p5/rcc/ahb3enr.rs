///Register `AHB3ENR` reader
pub type R = crate::R<AHB3ENRrs>;
///Register `AHB3ENR` writer
pub type W = crate::W<AHB3ENRrs>;
/**Flexible memory controller clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCEN {
    ///0: FMC clock disabled
    Disabled = 0,
    ///1: FMC clock enabled
    Enabled = 1,
}
impl From<FMCEN> for bool {
    #[inline(always)]
    fn from(variant: FMCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FMCEN` reader - Flexible memory controller clock enable
pub type FMCEN_R = crate::BitReader<FMCEN>;
impl FMCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMCEN {
        match self.bits {
            false => FMCEN::Disabled,
            true => FMCEN::Enabled,
        }
    }
    ///FMC clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMCEN::Disabled
    }
    ///FMC clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMCEN::Enabled
    }
}
///Field `FMCEN` writer - Flexible memory controller clock enable
pub type FMCEN_W<'a, REG> = crate::BitWriter<'a, REG, FMCEN>;
impl<'a, REG> FMCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FMC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMCEN::Disabled)
    }
    ///FMC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMCEN::Enabled)
    }
}
/**OctoSPI1 memory interface clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPI1EN {
    ///0: OctoSPI x clock disabled
    Disabled = 0,
    ///1: OctoSPI x clock enabled
    Enabled = 1,
}
impl From<OSPI1EN> for bool {
    #[inline(always)]
    fn from(variant: OSPI1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `OSPI1EN` reader - OctoSPI1 memory interface clock enable
pub type OSPI1EN_R = crate::BitReader<OSPI1EN>;
impl OSPI1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPI1EN {
        match self.bits {
            false => OSPI1EN::Disabled,
            true => OSPI1EN::Enabled,
        }
    }
    ///OctoSPI x clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPI1EN::Disabled
    }
    ///OctoSPI x clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPI1EN::Enabled
    }
}
///Field `OSPI1EN` writer - OctoSPI1 memory interface clock enable
pub type OSPI1EN_W<'a, REG> = crate::BitWriter<'a, REG, OSPI1EN>;
impl<'a, REG> OSPI1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OctoSPI x clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPI1EN::Disabled)
    }
    ///OctoSPI x clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSPI1EN::Enabled)
    }
}
///Field `OSPI2EN` reader - OSPI2EN memory interface clock enable
pub use OSPI1EN_R as OSPI2EN_R;
///Field `OSPI2EN` writer - OSPI2EN memory interface clock enable
pub use OSPI1EN_W as OSPI2EN_W;
impl R {
    ///Bit 0 - Flexible memory controller clock enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - OctoSPI1 memory interface clock enable
    #[inline(always)]
    pub fn ospi1en(&self) -> OSPI1EN_R {
        OSPI1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - OSPI2EN memory interface clock enable
    #[inline(always)]
    pub fn ospi2en(&self) -> OSPI2EN_R {
        OSPI2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3ENR")
            .field("fmcen", &self.fmcen())
            .field("ospi1en", &self.ospi1en())
            .field("ospi2en", &self.ospi2en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flexible memory controller clock enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<'_, AHB3ENRrs> {
        FMCEN_W::new(self, 0)
    }
    ///Bit 8 - OctoSPI1 memory interface clock enable
    #[inline(always)]
    pub fn ospi1en(&mut self) -> OSPI1EN_W<'_, AHB3ENRrs> {
        OSPI1EN_W::new(self, 8)
    }
    ///Bit 9 - OSPI2EN memory interface clock enable
    #[inline(always)]
    pub fn ospi2en(&mut self) -> OSPI2EN_W<'_, AHB3ENRrs> {
        OSPI2EN_W::new(self, 9)
    }
}
/**AHB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:AHB3ENR)*/
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3enr::R`](R) reader structure
impl crate::Readable for AHB3ENRrs {}
///`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3ENR to value 0
impl crate::Resettable for AHB3ENRrs {}
