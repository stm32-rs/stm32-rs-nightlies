///Register `AHB3ENR` reader
pub type R = crate::R<AHB3ENRrs>;
///Register `AHB3ENR` writer
pub type W = crate::W<AHB3ENRrs>;
/**Flexible static memory controller clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<FMCEN> for bool {
    #[inline(always)]
    fn from(variant: FMCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FMCEN` reader - Flexible static memory controller clock enable
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
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMCEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMCEN::Enabled
    }
}
///Field `FMCEN` writer - Flexible static memory controller clock enable
pub type FMCEN_W<'a, REG> = crate::BitWriter<'a, REG, FMCEN>;
impl<'a, REG> FMCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMCEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMCEN::Enabled)
    }
}
///Field `QSPIEN` reader - QUADSPI memory interface clock enable
pub use FMCEN_R as QSPIEN_R;
///Field `QSPIEN` writer - QUADSPI memory interface clock enable
pub use FMCEN_W as QSPIEN_W;
impl R {
    ///Bit 0 - Flexible static memory controller clock enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - QUADSPI memory interface clock enable
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3ENR")
            .field("fmcen", &self.fmcen())
            .field("qspien", &self.qspien())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flexible static memory controller clock enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<'_, AHB3ENRrs> {
        FMCEN_W::new(self, 0)
    }
    ///Bit 8 - QUADSPI memory interface clock enable
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W<'_, AHB3ENRrs> {
        QSPIEN_W::new(self, 8)
    }
}
/**AHB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#RCC:AHB3ENR)*/
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
