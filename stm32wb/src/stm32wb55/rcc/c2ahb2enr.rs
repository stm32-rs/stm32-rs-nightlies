///Register `C2AHB2ENR` reader
pub type R = crate::R<C2AHB2ENRrs>;
///Register `C2AHB2ENR` writer
pub type W = crate::W<C2AHB2ENRrs>;
/**CPU2 IO port A clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOAEN` reader - CPU2 IO port A clock enable
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
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
///Field `GPIOAEN` writer - CPU2 IO port A clock enable
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
///Field `GPIOBEN` reader - CPU2 IO port B clock enable
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - CPU2 IO port C clock enable
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - CPU2 IO port D clock enable
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - CPU2 IO port E clock enable
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOHEN` reader - CPU2 IO port H clock enable
pub use GPIOAEN_R as GPIOHEN_R;
///Field `ADCEN` reader - CPU2 ADC clock enable
pub use GPIOAEN_R as ADCEN_R;
///Field `AES1EN` reader - CPU2 AES1 accelerator clock enable
pub use GPIOAEN_R as AES1EN_R;
///Field `GPIOBEN` writer - CPU2 IO port B clock enable
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - CPU2 IO port C clock enable
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - CPU2 IO port D clock enable
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - CPU2 IO port E clock enable
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOHEN` writer - CPU2 IO port H clock enable
pub use GPIOAEN_W as GPIOHEN_W;
///Field `ADCEN` writer - CPU2 ADC clock enable
pub use GPIOAEN_W as ADCEN_W;
///Field `AES1EN` writer - CPU2 AES1 accelerator clock enable
pub use GPIOAEN_W as AES1EN_W;
impl R {
    ///Bit 0 - CPU2 IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU2 IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU2 IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU2 IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU2 IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - CPU2 IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - CPU2 ADC clock enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - CPU2 AES1 accelerator clock enable
    #[inline(always)]
    pub fn aes1en(&self) -> AES1EN_R {
        AES1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2AHB2ENR")
            .field("gpioaen", &self.gpioaen())
            .field("aes1en", &self.aes1en())
            .field("adcen", &self.adcen())
            .field("gpiohen", &self.gpiohen())
            .field("gpioeen", &self.gpioeen())
            .field("gpioden", &self.gpioden())
            .field("gpiocen", &self.gpiocen())
            .field("gpioben", &self.gpioben())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU2 IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, C2AHB2ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - CPU2 IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, C2AHB2ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - CPU2 IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, C2AHB2ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - CPU2 IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, C2AHB2ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - CPU2 IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, C2AHB2ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 7 - CPU2 IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<'_, C2AHB2ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    ///Bit 13 - CPU2 ADC clock enable
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<'_, C2AHB2ENRrs> {
        ADCEN_W::new(self, 13)
    }
    ///Bit 16 - CPU2 AES1 accelerator clock enable
    #[inline(always)]
    pub fn aes1en(&mut self) -> AES1EN_W<'_, C2AHB2ENRrs> {
        AES1EN_W::new(self, 16)
    }
}
/**CPU2 AHB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`c2ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:C2AHB2ENR)*/
pub struct C2AHB2ENRrs;
impl crate::RegisterSpec for C2AHB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2ahb2enr::R`](R) reader structure
impl crate::Readable for C2AHB2ENRrs {}
///`write(|w| ..)` method takes [`c2ahb2enr::W`](W) writer structure
impl crate::Writable for C2AHB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2AHB2ENR to value 0
impl crate::Resettable for C2AHB2ENRrs {}
