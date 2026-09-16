///Register `AHB2LPENR` reader
pub type R = crate::R<AHB2LPENRrs>;
///Register `AHB2LPENR` writer
pub type W = crate::W<AHB2LPENRrs>;
/**PSSI peripheral clock enable in low-power mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSSILPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<PSSILPEN> for bool {
    #[inline(always)]
    fn from(variant: PSSILPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PSSILPEN` reader - PSSI peripheral clock enable in low-power mode
pub type PSSILPEN_R = crate::BitReader<PSSILPEN>;
impl PSSILPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSSILPEN {
        match self.bits {
            false => PSSILPEN::Disabled,
            true => PSSILPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PSSILPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PSSILPEN::Enabled
    }
}
///Field `PSSILPEN` writer - PSSI peripheral clock enable in low-power mode
pub type PSSILPEN_W<'a, REG> = crate::BitWriter<'a, REG, PSSILPEN>;
impl<'a, REG> PSSILPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PSSILPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PSSILPEN::Enabled)
    }
}
///Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 delay clock enable in low-power mode
pub use PSSILPEN_R as SDMMC2LPEN_R;
///Field `CORDICLPEN` reader - CORDIC clock enable in low-power mode
pub use PSSILPEN_R as CORDICLPEN_R;
///Field `SRAM1LPEN` reader - SRAM1 clock enable in low-power mode
pub use PSSILPEN_R as SRAM1LPEN_R;
///Field `SRAM2LPEN` reader - SRAM2 clock enable in low-power mode
pub use PSSILPEN_R as SRAM2LPEN_R;
///Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 delay clock enable in low-power mode
pub use PSSILPEN_W as SDMMC2LPEN_W;
///Field `CORDICLPEN` writer - CORDIC clock enable in low-power mode
pub use PSSILPEN_W as CORDICLPEN_W;
///Field `SRAM1LPEN` writer - SRAM1 clock enable in low-power mode
pub use PSSILPEN_W as SRAM1LPEN_W;
///Field `SRAM2LPEN` writer - SRAM2 clock enable in low-power mode
pub use PSSILPEN_W as SRAM2LPEN_W;
impl R {
    ///Bit 1 - PSSI peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn pssilpen(&self) -> PSSILPEN_R {
        PSSILPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable in low-power mode
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - CORDIC clock enable in low-power mode
    #[inline(always)]
    pub fn cordiclpen(&self) -> CORDICLPEN_R {
        CORDICLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 29 - SRAM1 clock enable in low-power mode
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SRAM2 clock enable in low-power mode
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2LPENR")
            .field("pssilpen", &self.pssilpen())
            .field("sdmmc2lpen", &self.sdmmc2lpen())
            .field("cordiclpen", &self.cordiclpen())
            .field("sram1lpen", &self.sram1lpen())
            .field("sram2lpen", &self.sram2lpen())
            .finish()
    }
}
impl W {
    ///Bit 1 - PSSI peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn pssilpen(&mut self) -> PSSILPEN_W<'_, AHB2LPENRrs> {
        PSSILPEN_W::new(self, 1)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable in low-power mode
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<'_, AHB2LPENRrs> {
        SDMMC2LPEN_W::new(self, 9)
    }
    ///Bit 14 - CORDIC clock enable in low-power mode
    #[inline(always)]
    pub fn cordiclpen(&mut self) -> CORDICLPEN_W<'_, AHB2LPENRrs> {
        CORDICLPEN_W::new(self, 14)
    }
    ///Bit 29 - SRAM1 clock enable in low-power mode
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<'_, AHB2LPENRrs> {
        SRAM1LPEN_W::new(self, 29)
    }
    ///Bit 30 - SRAM2 clock enable in low-power mode
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<'_, AHB2LPENRrs> {
        SRAM2LPEN_W::new(self, 30)
    }
}
/**RCC AHB2 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:AHB2LPENR)*/
pub struct AHB2LPENRrs;
impl crate::RegisterSpec for AHB2LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2lpenr::R`](R) reader structure
impl crate::Readable for AHB2LPENRrs {}
///`write(|w| ..)` method takes [`ahb2lpenr::W`](W) writer structure
impl crate::Writable for AHB2LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2LPENR to value 0x6000_4202
impl crate::Resettable for AHB2LPENRrs {
    const RESET_VALUE: u32 = 0x6000_4202;
}
