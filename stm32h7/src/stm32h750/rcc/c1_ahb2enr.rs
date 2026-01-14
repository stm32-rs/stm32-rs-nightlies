///Register `C1_AHB2ENR` reader
pub type R = crate::R<C1_AHB2ENRrs>;
///Register `C1_AHB2ENR` writer
pub type W = crate::W<C1_AHB2ENRrs>;
/**DCMI peripheral clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<DCMIEN> for bool {
    #[inline(always)]
    fn from(variant: DCMIEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DCMIEN` reader - DCMI peripheral clock enable
pub type DCMIEN_R = crate::BitReader<DCMIEN>;
impl DCMIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DCMIEN {
        match self.bits {
            false => DCMIEN::Disabled,
            true => DCMIEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMIEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMIEN::Enabled
    }
}
///Field `DCMIEN` writer - DCMI peripheral clock enable
pub type DCMIEN_W<'a, REG> = crate::BitWriter<'a, REG, DCMIEN>;
impl<'a, REG> DCMIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DCMIEN::Enabled)
    }
}
///Field `CRYPTEN` reader - CRYPT peripheral clock enable
pub use DCMIEN_R as CRYPTEN_R;
///Field `HASHEN` reader - HASH peripheral clock enable
pub use DCMIEN_R as HASHEN_R;
///Field `RNGEN` reader - RNG peripheral clocks enable
pub use DCMIEN_R as RNGEN_R;
///Field `SDMMC2EN` reader - SDMMC2 and SDMMC2 delay clock enable
pub use DCMIEN_R as SDMMC2EN_R;
///Field `SRAM1EN` reader - SRAM1 block enable
pub use DCMIEN_R as SRAM1EN_R;
///Field `SRAM2EN` reader - SRAM2 block enable
pub use DCMIEN_R as SRAM2EN_R;
///Field `SRAM3EN` reader - SRAM3 block enable
pub use DCMIEN_R as SRAM3EN_R;
///Field `CRYPTEN` writer - CRYPT peripheral clock enable
pub use DCMIEN_W as CRYPTEN_W;
///Field `HASHEN` writer - HASH peripheral clock enable
pub use DCMIEN_W as HASHEN_W;
///Field `RNGEN` writer - RNG peripheral clocks enable
pub use DCMIEN_W as RNGEN_W;
///Field `SDMMC2EN` writer - SDMMC2 and SDMMC2 delay clock enable
pub use DCMIEN_W as SDMMC2EN_W;
///Field `SRAM1EN` writer - SRAM1 block enable
pub use DCMIEN_W as SRAM1EN_W;
///Field `SRAM2EN` writer - SRAM2 block enable
pub use DCMIEN_W as SRAM2EN_W;
///Field `SRAM3EN` writer - SRAM3 block enable
pub use DCMIEN_W as SRAM3EN_W;
impl R {
    ///Bit 0 - DCMI peripheral clock enable
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYPT peripheral clock enable
    #[inline(always)]
    pub fn crypten(&self) -> CRYPTEN_R {
        CRYPTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH peripheral clock enable
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG peripheral clocks enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 29 - SRAM1 block enable
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SRAM2 block enable
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM3 block enable
    #[inline(always)]
    pub fn sram3en(&self) -> SRAM3EN_R {
        SRAM3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1_AHB2ENR")
            .field("dcmien", &self.dcmien())
            .field("crypten", &self.crypten())
            .field("hashen", &self.hashen())
            .field("rngen", &self.rngen())
            .field("sdmmc2en", &self.sdmmc2en())
            .field("sram1en", &self.sram1en())
            .field("sram2en", &self.sram2en())
            .field("sram3en", &self.sram3en())
            .finish()
    }
}
impl W {
    ///Bit 0 - DCMI peripheral clock enable
    #[inline(always)]
    pub fn dcmien(&mut self) -> DCMIEN_W<'_, C1_AHB2ENRrs> {
        DCMIEN_W::new(self, 0)
    }
    ///Bit 4 - CRYPT peripheral clock enable
    #[inline(always)]
    pub fn crypten(&mut self) -> CRYPTEN_W<'_, C1_AHB2ENRrs> {
        CRYPTEN_W::new(self, 4)
    }
    ///Bit 5 - HASH peripheral clock enable
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W<'_, C1_AHB2ENRrs> {
        HASHEN_W::new(self, 5)
    }
    ///Bit 6 - RNG peripheral clocks enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, C1_AHB2ENRrs> {
        RNGEN_W::new(self, 6)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<'_, C1_AHB2ENRrs> {
        SDMMC2EN_W::new(self, 9)
    }
    ///Bit 29 - SRAM1 block enable
    #[inline(always)]
    pub fn sram1en(&mut self) -> SRAM1EN_W<'_, C1_AHB2ENRrs> {
        SRAM1EN_W::new(self, 29)
    }
    ///Bit 30 - SRAM2 block enable
    #[inline(always)]
    pub fn sram2en(&mut self) -> SRAM2EN_W<'_, C1_AHB2ENRrs> {
        SRAM2EN_W::new(self, 30)
    }
    ///Bit 31 - SRAM3 block enable
    #[inline(always)]
    pub fn sram3en(&mut self) -> SRAM3EN_W<'_, C1_AHB2ENRrs> {
        SRAM3EN_W::new(self, 31)
    }
}
/**RCC AHB2 Clock Register

You can [`read`](crate::Reg::read) this register and get [`c1_ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RCC:C1_AHB2ENR)*/
pub struct C1_AHB2ENRrs;
impl crate::RegisterSpec for C1_AHB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`c1_ahb2enr::R`](R) reader structure
impl crate::Readable for C1_AHB2ENRrs {}
///`write(|w| ..)` method takes [`c1_ahb2enr::W`](W) writer structure
impl crate::Writable for C1_AHB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1_AHB2ENR to value 0
impl crate::Resettable for C1_AHB2ENRrs {}
