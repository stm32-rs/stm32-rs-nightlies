///Register `APB2ENR` reader
pub type R = crate::R<APB2ENRrs>;
///Register `APB2ENR` writer
pub type W = crate::W<APB2ENRrs>;
///Field `SYSCFGEN` reader - SYSCFG clock enable
pub type SYSCFGEN_R = crate::BitReader;
///Field `SYSCFGEN` writer - SYSCFG clock enable
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIREWALLEN` reader - Firewall clock enable
pub type FIREWALLEN_R = crate::BitReader;
///Field `FIREWALLEN` writer - Firewall clock enable
pub type FIREWALLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMCEN` reader - SDMMC clock enable
pub type SDMMCEN_R = crate::BitReader;
///Field `SDMMCEN` writer - SDMMC clock enable
pub type SDMMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1EN` reader - TIM1 timer clock enable
pub type TIM1EN_R = crate::BitReader;
///Field `TIM1EN` writer - TIM1 timer clock enable
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1EN` reader - SPI1 clock enable
pub type SPI1EN_R = crate::BitReader;
///Field `SPI1EN` writer - SPI1 clock enable
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8EN` reader - TIM8 timer clock enable
pub type TIM8EN_R = crate::BitReader;
///Field `TIM8EN` writer - TIM8 timer clock enable
pub type TIM8EN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**USART1clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1EN {
    ///0: USART1 clock disabled
    Disabled = 0,
    ///1: USART1 clock enabled
    Enabled = 1,
}
impl From<USART1EN> for bool {
    #[inline(always)]
    fn from(variant: USART1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `USART1EN` reader - USART1clock enable
pub type USART1EN_R = crate::BitReader<USART1EN>;
impl USART1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USART1EN {
        match self.bits {
            false => USART1EN::Disabled,
            true => USART1EN::Enabled,
        }
    }
    ///USART1 clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART1EN::Disabled
    }
    ///USART1 clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART1EN::Enabled
    }
}
///Field `USART1EN` writer - USART1clock enable
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG, USART1EN>;
impl<'a, REG> USART1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USART1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART1EN::Disabled)
    }
    ///USART1 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(USART1EN::Enabled)
    }
}
///Field `TIM15EN` reader - TIM15 timer clock enable
pub type TIM15EN_R = crate::BitReader;
///Field `TIM15EN` writer - TIM15 timer clock enable
pub type TIM15EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16EN` reader - TIM16 timer clock enable
pub type TIM16EN_R = crate::BitReader;
///Field `TIM16EN` writer - TIM16 timer clock enable
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17EN` reader - TIM17 timer clock enable
pub type TIM17EN_R = crate::BitReader;
///Field `TIM17EN` writer - TIM17 timer clock enable
pub type TIM17EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1EN` reader - SAI1 clock enable
pub type SAI1EN_R = crate::BitReader;
///Field `SAI1EN` writer - SAI1 clock enable
pub type SAI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2EN` reader - SAI2 clock enable
pub type SAI2EN_R = crate::BitReader;
///Field `SAI2EN` writer - SAI2 clock enable
pub type SAI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDMEN` reader - DFSDM timer clock enable
pub type DFSDMEN_R = crate::BitReader;
///Field `DFSDMEN` writer - DFSDM timer clock enable
pub type DFSDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SYSCFG clock enable
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 7 - Firewall clock enable
    #[inline(always)]
    pub fn firewallen(&self) -> FIREWALLEN_R {
        FIREWALLEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - SDMMC clock enable
    #[inline(always)]
    pub fn sdmmcen(&self) -> SDMMCEN_R {
        SDMMCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8 timer clock enable
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 timer clock enable
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - SAI1 clock enable
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 clock enable
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - DFSDM timer clock enable
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2ENR")
            .field("dfsdmen", &self.dfsdmen())
            .field("sai2en", &self.sai2en())
            .field("sai1en", &self.sai1en())
            .field("tim17en", &self.tim17en())
            .field("tim16en", &self.tim16en())
            .field("tim15en", &self.tim15en())
            .field("usart1en", &self.usart1en())
            .field("tim8en", &self.tim8en())
            .field("spi1en", &self.spi1en())
            .field("tim1en", &self.tim1en())
            .field("sdmmcen", &self.sdmmcen())
            .field("firewallen", &self.firewallen())
            .field("syscfgen", &self.syscfgen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG clock enable
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, APB2ENRrs> {
        SYSCFGEN_W::new(self, 0)
    }
    ///Bit 7 - Firewall clock enable
    #[inline(always)]
    pub fn firewallen(&mut self) -> FIREWALLEN_W<'_, APB2ENRrs> {
        FIREWALLEN_W::new(self, 7)
    }
    ///Bit 10 - SDMMC clock enable
    #[inline(always)]
    pub fn sdmmcen(&mut self) -> SDMMCEN_W<'_, APB2ENRrs> {
        SDMMCEN_W::new(self, 10)
    }
    ///Bit 11 - TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<'_, APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<'_, APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 13 - TIM8 timer clock enable
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W<'_, APB2ENRrs> {
        TIM8EN_W::new(self, 13)
    }
    ///Bit 14 - USART1clock enable
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 16 - TIM15 timer clock enable
    #[inline(always)]
    pub fn tim15en(&mut self) -> TIM15EN_W<'_, APB2ENRrs> {
        TIM15EN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<'_, APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W<'_, APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    ///Bit 21 - SAI1 clock enable
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W<'_, APB2ENRrs> {
        SAI1EN_W::new(self, 21)
    }
    ///Bit 22 - SAI2 clock enable
    #[inline(always)]
    pub fn sai2en(&mut self) -> SAI2EN_W<'_, APB2ENRrs> {
        SAI2EN_W::new(self, 22)
    }
    ///Bit 24 - DFSDM timer clock enable
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<'_, APB2ENRrs> {
        DFSDMEN_W::new(self, 24)
    }
}
/**APB2ENR

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#RCC:APB2ENR)*/
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
