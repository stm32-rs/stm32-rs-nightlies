///Register `APB2ENR` reader
pub type R = crate::R<APB2ENRrs>;
///Register `APB2ENR` writer
pub type W = crate::W<APB2ENRrs>;
/**System configuration controller clock enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<SYSCFGEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGEN` reader - System configuration controller clock enable bit
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN>;
impl SYSCFGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGEN {
        match self.bits {
            false => SYSCFGEN::Disabled,
            true => SYSCFGEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGEN::Enabled
    }
}
///Field `SYSCFGEN` writer - System configuration controller clock enable bit
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGEN>;
impl<'a, REG> SYSCFGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Enabled)
    }
}
///Field `TIM21EN` reader - TIM21 timer clock enable bit
pub use SYSCFGEN_R as TIM21EN_R;
///Field `TIM22EN` reader - TIM22 timer clock enable bit
pub use SYSCFGEN_R as TIM22EN_R;
///Field `FWEN` reader - Firewall clock enable bit
pub use SYSCFGEN_R as FWEN_R;
///Field `ADCEN` reader - ADC clock enable bit
pub use SYSCFGEN_R as ADCEN_R;
///Field `SPI1EN` reader - SPI1 clock enable bit
pub use SYSCFGEN_R as SPI1EN_R;
///Field `USART1EN` reader - USART1 clock enable bit
pub use SYSCFGEN_R as USART1EN_R;
///Field `DBGEN` reader - DBG clock enable bit
pub use SYSCFGEN_R as DBGEN_R;
///Field `TIM21EN` writer - TIM21 timer clock enable bit
pub use SYSCFGEN_W as TIM21EN_W;
///Field `TIM22EN` writer - TIM22 timer clock enable bit
pub use SYSCFGEN_W as TIM22EN_W;
///Field `FWEN` writer - Firewall clock enable bit
pub use SYSCFGEN_W as FWEN_W;
///Field `ADCEN` writer - ADC clock enable bit
pub use SYSCFGEN_W as ADCEN_W;
///Field `SPI1EN` writer - SPI1 clock enable bit
pub use SYSCFGEN_W as SPI1EN_W;
///Field `USART1EN` writer - USART1 clock enable bit
pub use SYSCFGEN_W as USART1EN_W;
///Field `DBGEN` writer - DBG clock enable bit
pub use SYSCFGEN_W as DBGEN_W;
impl R {
    ///Bit 0 - System configuration controller clock enable bit
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - TIM21 timer clock enable bit
    #[inline(always)]
    pub fn tim21en(&self) -> TIM21EN_R {
        TIM21EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - TIM22 timer clock enable bit
    #[inline(always)]
    pub fn tim22en(&self) -> TIM22EN_R {
        TIM22EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Firewall clock enable bit
    #[inline(always)]
    pub fn fwen(&self) -> FWEN_R {
        FWEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - ADC clock enable bit
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable bit
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable bit
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 22 - DBG clock enable bit
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2ENR")
            .field("syscfgen", &self.syscfgen())
            .field("dbgen", &self.dbgen())
            .field("usart1en", &self.usart1en())
            .field("spi1en", &self.spi1en())
            .field("adcen", &self.adcen())
            .field("fwen", &self.fwen())
            .field("tim22en", &self.tim22en())
            .field("tim21en", &self.tim21en())
            .finish()
    }
}
impl W {
    ///Bit 0 - System configuration controller clock enable bit
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, APB2ENRrs> {
        SYSCFGEN_W::new(self, 0)
    }
    ///Bit 2 - TIM21 timer clock enable bit
    #[inline(always)]
    pub fn tim21en(&mut self) -> TIM21EN_W<'_, APB2ENRrs> {
        TIM21EN_W::new(self, 2)
    }
    ///Bit 5 - TIM22 timer clock enable bit
    #[inline(always)]
    pub fn tim22en(&mut self) -> TIM22EN_W<'_, APB2ENRrs> {
        TIM22EN_W::new(self, 5)
    }
    ///Bit 7 - Firewall clock enable bit
    #[inline(always)]
    pub fn fwen(&mut self) -> FWEN_W<'_, APB2ENRrs> {
        FWEN_W::new(self, 7)
    }
    ///Bit 9 - ADC clock enable bit
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<'_, APB2ENRrs> {
        ADCEN_W::new(self, 9)
    }
    ///Bit 12 - SPI1 clock enable bit
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<'_, APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable bit
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 22 - DBG clock enable bit
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W<'_, APB2ENRrs> {
        DBGEN_W::new(self, 22)
    }
}
/**APB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#RCC:APB2ENR)*/
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2enr::R`](R) reader structure
impl crate::Readable for APB2ENRrs {}
///`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2ENR to value 0
impl crate::Resettable for APB2ENRrs {}
