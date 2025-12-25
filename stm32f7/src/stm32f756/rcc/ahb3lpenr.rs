///Register `AHB3LPENR` reader
pub type R = crate::R<AHB3LPENRrs>;
///Register `AHB3LPENR` writer
pub type W = crate::W<AHB3LPENRrs>;
/**Flexible memory controller module clock enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMCLPEN {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<FMCLPEN> for bool {
    #[inline(always)]
    fn from(variant: FMCLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FMCLPEN` reader - Flexible memory controller module clock enable during Sleep mode
pub type FMCLPEN_R = crate::BitReader<FMCLPEN>;
impl FMCLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMCLPEN {
        match self.bits {
            false => FMCLPEN::DisabledInSleep,
            true => FMCLPEN::EnabledInSleep,
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == FMCLPEN::DisabledInSleep
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == FMCLPEN::EnabledInSleep
    }
}
///Field `FMCLPEN` writer - Flexible memory controller module clock enable during Sleep mode
pub type FMCLPEN_W<'a, REG> = crate::BitWriter<'a, REG, FMCLPEN>;
impl<'a, REG> FMCLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(FMCLPEN::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(FMCLPEN::EnabledInSleep)
    }
}
///Field `QSPILPEN` reader - Quand SPI memory controller clock enable during Sleep mode
pub use FMCLPEN_R as QSPILPEN_R;
///Field `QSPILPEN` writer - Quand SPI memory controller clock enable during Sleep mode
pub use FMCLPEN_W as QSPILPEN_W;
impl R {
    ///Bit 0 - Flexible memory controller module clock enable during Sleep mode
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Quand SPI memory controller clock enable during Sleep mode
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3LPENR")
            .field("fmclpen", &self.fmclpen())
            .field("qspilpen", &self.qspilpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flexible memory controller module clock enable during Sleep mode
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<'_, AHB3LPENRrs> {
        FMCLPEN_W::new(self, 0)
    }
    ///Bit 1 - Quand SPI memory controller clock enable during Sleep mode
    #[inline(always)]
    pub fn qspilpen(&mut self) -> QSPILPEN_W<'_, AHB3LPENRrs> {
        QSPILPEN_W::new(self, 1)
    }
}
/**AHB3 peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`ahb3lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#RCC:AHB3LPENR)*/
pub struct AHB3LPENRrs;
impl crate::RegisterSpec for AHB3LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3lpenr::R`](R) reader structure
impl crate::Readable for AHB3LPENRrs {}
///`write(|w| ..)` method takes [`ahb3lpenr::W`](W) writer structure
impl crate::Writable for AHB3LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3LPENR to value 0x01
impl crate::Resettable for AHB3LPENRrs {
    const RESET_VALUE: u32 = 0x01;
}
