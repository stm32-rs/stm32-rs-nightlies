///Register `AHB4ENR` reader
pub type R = crate::R<AHB4ENRrs>;
///Register `AHB4ENR` writer
pub type W = crate::W<AHB4ENRrs>;
/**GPIOA peripheral clock enable Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOAEN` reader - GPIOA peripheral clock enable Set and reset by software.
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
///Field `GPIOAEN` writer - GPIOA peripheral clock enable Set and reset by software.
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
///Field `GPIOBEN` reader - GPIOB peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - GPIOC peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - GPIOD peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - GPIOE peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - GPIOF peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOGEN` reader - GPIOG peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOGEN_R;
///Field `GPIOHEN` reader - GPIOH peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOHEN_R;
///Field `GPIOIEN` reader - GPIOI peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOIEN_R;
///Field `GPIOJEN` reader - GPIOJ peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOJEN_R;
///Field `GPIOKEN` reader - GPIOK peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOKEN_R;
///Field `BDMA2EN` reader - SmartRun domain DMA and DMAMUX clock enable Set and reset by software.
pub use GPIOAEN_R as BDMA2EN_R;
///Field `BKPRAMEN` reader - Backup RAM clock enable Set and reset by software.
pub use GPIOAEN_R as BKPRAMEN_R;
///Field `SRDSRAMEN` reader - SmartRun domain SRAM clock enable Set and reset by software.
pub use GPIOAEN_R as SRDSRAMEN_R;
///Field `GPIOBEN` writer - GPIOB peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - GPIOC peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - GPIOD peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - GPIOE peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - GPIOF peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOFEN_W;
///Field `GPIOGEN` writer - GPIOG peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOGEN_W;
///Field `GPIOHEN` writer - GPIOH peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOHEN_W;
///Field `GPIOIEN` writer - GPIOI peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOIEN_W;
///Field `GPIOJEN` writer - GPIOJ peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOJEN_W;
///Field `GPIOKEN` writer - GPIOK peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOKEN_W;
///Field `BDMA2EN` writer - SmartRun domain DMA and DMAMUX clock enable Set and reset by software.
pub use GPIOAEN_W as BDMA2EN_W;
///Field `BKPRAMEN` writer - Backup RAM clock enable Set and reset by software.
pub use GPIOAEN_W as BKPRAMEN_W;
///Field `SRDSRAMEN` writer - SmartRun domain SRAM clock enable Set and reset by software.
pub use GPIOAEN_W as SRDSRAMEN_W;
impl R {
    ///Bit 0 - GPIOA peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOE peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOI peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIOJ peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPIOK peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioken(&self) -> GPIOKEN_R {
        GPIOKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 21 - SmartRun domain DMA and DMAMUX clock enable Set and reset by software.
    #[inline(always)]
    pub fn bdma2en(&self) -> BDMA2EN_R {
        BDMA2EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 28 - Backup RAM clock enable Set and reset by software.
    #[inline(always)]
    pub fn bkpramen(&self) -> BKPRAMEN_R {
        BKPRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SmartRun domain SRAM clock enable Set and reset by software.
    #[inline(always)]
    pub fn srdsramen(&self) -> SRDSRAMEN_R {
        SRDSRAMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4ENR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .field("gpiogen", &self.gpiogen())
            .field("gpiohen", &self.gpiohen())
            .field("gpioien", &self.gpioien())
            .field("gpiojen", &self.gpiojen())
            .field("gpioken", &self.gpioken())
            .field("bdma2en", &self.bdma2en())
            .field("bkpramen", &self.bkpramen())
            .field("srdsramen", &self.srdsramen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOA peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, AHB4ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - GPIOB peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, AHB4ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - GPIOC peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, AHB4ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - GPIOD peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, AHB4ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - GPIOE peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, AHB4ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - GPIOF peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, AHB4ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    ///Bit 6 - GPIOG peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<'_, AHB4ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    ///Bit 7 - GPIOH peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<'_, AHB4ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 8 - GPIOI peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioien(&mut self) -> GPIOIEN_W<'_, AHB4ENRrs> {
        GPIOIEN_W::new(self, 8)
    }
    ///Bit 9 - GPIOJ peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiojen(&mut self) -> GPIOJEN_W<'_, AHB4ENRrs> {
        GPIOJEN_W::new(self, 9)
    }
    ///Bit 10 - GPIOK peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioken(&mut self) -> GPIOKEN_W<'_, AHB4ENRrs> {
        GPIOKEN_W::new(self, 10)
    }
    ///Bit 21 - SmartRun domain DMA and DMAMUX clock enable Set and reset by software.
    #[inline(always)]
    pub fn bdma2en(&mut self) -> BDMA2EN_W<'_, AHB4ENRrs> {
        BDMA2EN_W::new(self, 21)
    }
    ///Bit 28 - Backup RAM clock enable Set and reset by software.
    #[inline(always)]
    pub fn bkpramen(&mut self) -> BKPRAMEN_W<'_, AHB4ENRrs> {
        BKPRAMEN_W::new(self, 28)
    }
    ///Bit 29 - SmartRun domain SRAM clock enable Set and reset by software.
    #[inline(always)]
    pub fn srdsramen(&mut self) -> SRDSRAMEN_W<'_, AHB4ENRrs> {
        SRDSRAMEN_W::new(self, 29)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#RCC:AHB4ENR)*/
pub struct AHB4ENRrs;
impl crate::RegisterSpec for AHB4ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4enr::R`](R) reader structure
impl crate::Readable for AHB4ENRrs {}
///`write(|w| ..)` method takes [`ahb4enr::W`](W) writer structure
impl crate::Writable for AHB4ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4ENR to value 0
impl crate::Resettable for AHB4ENRrs {}
