///Register `AHB2ENR` reader
pub type R = crate::R<AHB2ENRrs>;
///Register `AHB2ENR` writer
pub type W = crate::W<AHB2ENRrs>;
/**PSSI peripheral clocks enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSSIEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<PSSIEN> for bool {
    #[inline(always)]
    fn from(variant: PSSIEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PSSIEN` reader - PSSI peripheral clocks enable
pub type PSSIEN_R = crate::BitReader<PSSIEN>;
impl PSSIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSSIEN {
        match self.bits {
            false => PSSIEN::Disabled,
            true => PSSIEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PSSIEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PSSIEN::Enabled
    }
}
///Field `PSSIEN` writer - PSSI peripheral clocks enable
pub type PSSIEN_W<'a, REG> = crate::BitWriter<'a, REG, PSSIEN>;
impl<'a, REG> PSSIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PSSIEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PSSIEN::Enabled)
    }
}
///Field `SDMMC2EN` reader - SDMMC2 and SDMMC2 delay clock enable
pub use PSSIEN_R as SDMMC2EN_R;
///Field `CORDICEN` reader - CORDIC clock enable
pub use PSSIEN_R as CORDICEN_R;
///Field `SRAM1EN` reader - SRAM1 clock enable
pub use PSSIEN_R as SRAM1EN_R;
///Field `SRAM2EN` reader - SRAM2 clock enable
pub use PSSIEN_R as SRAM2EN_R;
///Field `SDMMC2EN` writer - SDMMC2 and SDMMC2 delay clock enable
pub use PSSIEN_W as SDMMC2EN_W;
///Field `CORDICEN` writer - CORDIC clock enable
pub use PSSIEN_W as CORDICEN_W;
///Field `SRAM1EN` writer - SRAM1 clock enable
pub use PSSIEN_W as SRAM1EN_W;
///Field `SRAM2EN` writer - SRAM2 clock enable
pub use PSSIEN_W as SRAM2EN_W;
impl R {
    ///Bit 1 - PSSI peripheral clocks enable
    #[inline(always)]
    pub fn pssien(&self) -> PSSIEN_R {
        PSSIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - CORDIC clock enable
    #[inline(always)]
    pub fn cordicen(&self) -> CORDICEN_R {
        CORDICEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 29 - SRAM1 clock enable
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SRAM2 clock enable
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR")
            .field("pssien", &self.pssien())
            .field("sdmmc2en", &self.sdmmc2en())
            .field("cordicen", &self.cordicen())
            .field("sram1en", &self.sram1en())
            .field("sram2en", &self.sram2en())
            .finish()
    }
}
impl W {
    ///Bit 1 - PSSI peripheral clocks enable
    #[inline(always)]
    pub fn pssien(&mut self) -> PSSIEN_W<'_, AHB2ENRrs> {
        PSSIEN_W::new(self, 1)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<'_, AHB2ENRrs> {
        SDMMC2EN_W::new(self, 9)
    }
    ///Bit 14 - CORDIC clock enable
    #[inline(always)]
    pub fn cordicen(&mut self) -> CORDICEN_W<'_, AHB2ENRrs> {
        CORDICEN_W::new(self, 14)
    }
    ///Bit 29 - SRAM1 clock enable
    #[inline(always)]
    pub fn sram1en(&mut self) -> SRAM1EN_W<'_, AHB2ENRrs> {
        SRAM1EN_W::new(self, 29)
    }
    ///Bit 30 - SRAM2 clock enable
    #[inline(always)]
    pub fn sram2en(&mut self) -> SRAM2EN_W<'_, AHB2ENRrs> {
        SRAM2EN_W::new(self, 30)
    }
}
/**RCC AHB2 clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB2ENR)*/
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2enr::R`](R) reader structure
impl crate::Readable for AHB2ENRrs {}
///`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENR to value 0
impl crate::Resettable for AHB2ENRrs {}
