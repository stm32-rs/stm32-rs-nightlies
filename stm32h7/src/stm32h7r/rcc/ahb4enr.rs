///Register `AHB4ENR` reader
pub type R = crate::R<AHB4ENRrs>;
///Register `AHB4ENR` writer
pub type W = crate::W<AHB4ENRrs>;
/**GPIOA peripheral clock enable

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
///Field `GPIOAEN` reader - GPIOA peripheral clock enable
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
///Field `GPIOAEN` writer - GPIOA peripheral clock enable
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
///Field `GPIOBEN` reader - GPIOB peripheral clock enable
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - GPIOC peripheral clock enable
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - GPIOD peripheral clock enable
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - GPIOE peripheral clock enable
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - GPIOF peripheral clock enable
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOGEN` reader - GPIOG peripheral clock enable
pub use GPIOAEN_R as GPIOGEN_R;
///Field `GPIOHEN` reader - GPIOH peripheral clock enable
pub use GPIOAEN_R as GPIOHEN_R;
///Field `GPIOMEN` reader - GPIOM peripheral clock enable
pub use GPIOAEN_R as GPIOMEN_R;
///Field `GPIONEN` reader - GPION peripheral clock enable
pub use GPIOAEN_R as GPIONEN_R;
///Field `GPIOOEN` reader - GPIOO peripheral clock enable
pub use GPIOAEN_R as GPIOOEN_R;
///Field `GPIOPEN` reader - GPIOP peripheral clock enable
pub use GPIOAEN_R as GPIOPEN_R;
///Field `CRCEN` reader - CRC clock enable
pub use GPIOAEN_R as CRCEN_R;
///Field `BKPRAMEN` reader - Backup RAM clock enable
pub use GPIOAEN_R as BKPRAMEN_R;
///Field `GPIOBEN` writer - GPIOB peripheral clock enable
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - GPIOC peripheral clock enable
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - GPIOD peripheral clock enable
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - GPIOE peripheral clock enable
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - GPIOF peripheral clock enable
pub use GPIOAEN_W as GPIOFEN_W;
///Field `GPIOGEN` writer - GPIOG peripheral clock enable
pub use GPIOAEN_W as GPIOGEN_W;
///Field `GPIOHEN` writer - GPIOH peripheral clock enable
pub use GPIOAEN_W as GPIOHEN_W;
///Field `GPIOMEN` writer - GPIOM peripheral clock enable
pub use GPIOAEN_W as GPIOMEN_W;
///Field `GPIONEN` writer - GPION peripheral clock enable
pub use GPIOAEN_W as GPIONEN_W;
///Field `GPIOOEN` writer - GPIOO peripheral clock enable
pub use GPIOAEN_W as GPIOOEN_W;
///Field `GPIOPEN` writer - GPIOP peripheral clock enable
pub use GPIOAEN_W as GPIOPEN_W;
///Field `CRCEN` writer - CRC clock enable
pub use GPIOAEN_W as CRCEN_W;
///Field `BKPRAMEN` writer - Backup RAM clock enable
pub use GPIOAEN_W as BKPRAMEN_W;
impl R {
    ///Bit 0 - GPIOA peripheral clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB peripheral clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC peripheral clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD peripheral clock enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOE peripheral clock enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF peripheral clock enable
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG peripheral clock enable
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH peripheral clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - GPIOM peripheral clock enable
    #[inline(always)]
    pub fn gpiomen(&self) -> GPIOMEN_R {
        GPIOMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GPION peripheral clock enable
    #[inline(always)]
    pub fn gpionen(&self) -> GPIONEN_R {
        GPIONEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GPIOO peripheral clock enable
    #[inline(always)]
    pub fn gpiooen(&self) -> GPIOOEN_R {
        GPIOOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GPIOP peripheral clock enable
    #[inline(always)]
    pub fn gpiopen(&self) -> GPIOPEN_R {
        GPIOPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 19 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 28 - Backup RAM clock enable
    #[inline(always)]
    pub fn bkpramen(&self) -> BKPRAMEN_R {
        BKPRAMEN_R::new(((self.bits >> 28) & 1) != 0)
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
            .field("gpiomen", &self.gpiomen())
            .field("gpionen", &self.gpionen())
            .field("gpiooen", &self.gpiooen())
            .field("gpiopen", &self.gpiopen())
            .field("crcen", &self.crcen())
            .field("bkpramen", &self.bkpramen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOA peripheral clock enable
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, AHB4ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - GPIOB peripheral clock enable
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, AHB4ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - GPIOC peripheral clock enable
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, AHB4ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - GPIOD peripheral clock enable
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, AHB4ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - GPIOE peripheral clock enable
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, AHB4ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - GPIOF peripheral clock enable
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, AHB4ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    ///Bit 6 - GPIOG peripheral clock enable
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<'_, AHB4ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    ///Bit 7 - GPIOH peripheral clock enable
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<'_, AHB4ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 12 - GPIOM peripheral clock enable
    #[inline(always)]
    pub fn gpiomen(&mut self) -> GPIOMEN_W<'_, AHB4ENRrs> {
        GPIOMEN_W::new(self, 12)
    }
    ///Bit 13 - GPION peripheral clock enable
    #[inline(always)]
    pub fn gpionen(&mut self) -> GPIONEN_W<'_, AHB4ENRrs> {
        GPIONEN_W::new(self, 13)
    }
    ///Bit 14 - GPIOO peripheral clock enable
    #[inline(always)]
    pub fn gpiooen(&mut self) -> GPIOOEN_W<'_, AHB4ENRrs> {
        GPIOOEN_W::new(self, 14)
    }
    ///Bit 15 - GPIOP peripheral clock enable
    #[inline(always)]
    pub fn gpiopen(&mut self) -> GPIOPEN_W<'_, AHB4ENRrs> {
        GPIOPEN_W::new(self, 15)
    }
    ///Bit 19 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHB4ENRrs> {
        CRCEN_W::new(self, 19)
    }
    ///Bit 28 - Backup RAM clock enable
    #[inline(always)]
    pub fn bkpramen(&mut self) -> BKPRAMEN_W<'_, AHB4ENRrs> {
        BKPRAMEN_W::new(self, 28)
    }
}
/**RCC AHB4 clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:AHB4ENR)*/
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
