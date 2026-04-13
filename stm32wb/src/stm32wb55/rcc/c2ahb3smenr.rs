///Register `C2AHB3SMENR` reader
pub type R = crate::R<C2AHB3SMENRrs>;
///Register `C2AHB3SMENR` writer
pub type W = crate::W<C2AHB3SMENRrs>;
/**PKA accelerator clocks enable during CPU2 sleep modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKASMEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<PKASMEN> for bool {
    #[inline(always)]
    fn from(variant: PKASMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PKASMEN` reader - PKA accelerator clocks enable during CPU2 sleep modes
pub type PKASMEN_R = crate::BitReader<PKASMEN>;
impl PKASMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PKASMEN {
        match self.bits {
            false => PKASMEN::Disabled,
            true => PKASMEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PKASMEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PKASMEN::Enabled
    }
}
///Field `PKASMEN` writer - PKA accelerator clocks enable during CPU2 sleep modes
pub type PKASMEN_W<'a, REG> = crate::BitWriter<'a, REG, PKASMEN>;
impl<'a, REG> PKASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKASMEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKASMEN::Enabled)
    }
}
///Field `AES2SMEN` reader - AES2 accelerator clocks enable during CPU2 sleep modes
pub use PKASMEN_R as AES2SMEN_R;
///Field `RNGSMEN` reader - True RNG clocks enable during CPU2 sleep modes
pub use PKASMEN_R as RNGSMEN_R;
///Field `SRAM2SMEN` reader - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes
pub use PKASMEN_R as SRAM2SMEN_R;
///Field `FLASHSMEN` reader - Flash interface clocks enable during CPU2 sleep modes
pub use PKASMEN_R as FLASHSMEN_R;
///Field `AES2SMEN` writer - AES2 accelerator clocks enable during CPU2 sleep modes
pub use PKASMEN_W as AES2SMEN_W;
///Field `RNGSMEN` writer - True RNG clocks enable during CPU2 sleep modes
pub use PKASMEN_W as RNGSMEN_W;
///Field `SRAM2SMEN` writer - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes
pub use PKASMEN_W as SRAM2SMEN_W;
///Field `FLASHSMEN` writer - Flash interface clocks enable during CPU2 sleep modes
pub use PKASMEN_W as FLASHSMEN_W;
impl R {
    ///Bit 16 - PKA accelerator clocks enable during CPU2 sleep modes
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AES2 accelerator clocks enable during CPU2 sleep modes
    #[inline(always)]
    pub fn aes2smen(&self) -> AES2SMEN_R {
        AES2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - True RNG clocks enable during CPU2 sleep modes
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Flash interface clocks enable during CPU2 sleep modes
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2AHB3SMENR")
            .field("pkasmen", &self.pkasmen())
            .field("flashsmen", &self.flashsmen())
            .field("sram2smen", &self.sram2smen())
            .field("rngsmen", &self.rngsmen())
            .field("aes2smen", &self.aes2smen())
            .finish()
    }
}
impl W {
    ///Bit 16 - PKA accelerator clocks enable during CPU2 sleep modes
    #[inline(always)]
    pub fn pkasmen(&mut self) -> PKASMEN_W<'_, C2AHB3SMENRrs> {
        PKASMEN_W::new(self, 16)
    }
    ///Bit 17 - AES2 accelerator clocks enable during CPU2 sleep modes
    #[inline(always)]
    pub fn aes2smen(&mut self) -> AES2SMEN_W<'_, C2AHB3SMENRrs> {
        AES2SMEN_W::new(self, 17)
    }
    ///Bit 18 - True RNG clocks enable during CPU2 sleep modes
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<'_, C2AHB3SMENRrs> {
        RNGSMEN_W::new(self, 18)
    }
    ///Bit 24 - SRAM2a and SRAM2b memory interface clocks enable during CPU2 sleep modes
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<'_, C2AHB3SMENRrs> {
        SRAM2SMEN_W::new(self, 24)
    }
    ///Bit 25 - Flash interface clocks enable during CPU2 sleep modes
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<'_, C2AHB3SMENRrs> {
        FLASHSMEN_W::new(self, 25)
    }
}
/**CPU2 AHB3 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`c2ahb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:C2AHB3SMENR)*/
pub struct C2AHB3SMENRrs;
impl crate::RegisterSpec for C2AHB3SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2ahb3smenr::R`](R) reader structure
impl crate::Readable for C2AHB3SMENRrs {}
///`write(|w| ..)` method takes [`c2ahb3smenr::W`](W) writer structure
impl crate::Writable for C2AHB3SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2AHB3SMENR to value 0x0307_0000
impl crate::Resettable for C2AHB3SMENRrs {
    const RESET_VALUE: u32 = 0x0307_0000;
}
