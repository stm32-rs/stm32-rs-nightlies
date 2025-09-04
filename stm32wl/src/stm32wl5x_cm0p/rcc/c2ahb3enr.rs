///Register `C2AHB3ENR` reader
pub type R = crate::R<C2AHB3ENRrs>;
///Register `C2AHB3ENR` writer
pub type W = crate::W<C2AHB3ENRrs>;
/**CPU2 PKA accelerator clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKAEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<PKAEN> for bool {
    #[inline(always)]
    fn from(variant: PKAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PKAEN` reader - CPU2 PKA accelerator clock enable
pub type PKAEN_R = crate::BitReader<PKAEN>;
impl PKAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PKAEN {
        match self.bits {
            false => PKAEN::Disabled,
            true => PKAEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PKAEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PKAEN::Enabled
    }
}
///Field `PKAEN` writer - CPU2 PKA accelerator clock enable
pub type PKAEN_W<'a, REG> = crate::BitWriter<'a, REG, PKAEN>;
impl<'a, REG> PKAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKAEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PKAEN::Enabled)
    }
}
///Field `AESEN` reader - CPU2 AES accelerator clock enable
pub use PKAEN_R as AESEN_R;
///Field `RNGEN` reader - CPU2 True RNG clocks enable
pub use PKAEN_R as RNGEN_R;
///Field `HSEMEN` reader - CPU2 HSEM clock enable
pub use PKAEN_R as HSEMEN_R;
///Field `IPCCEN` reader - CPU2 IPCC interface clock enable
pub use PKAEN_R as IPCCEN_R;
///Field `FLASHEN` reader - CPU2 Flash interface clock enable
pub use PKAEN_R as FLASHEN_R;
///Field `AESEN` writer - CPU2 AES accelerator clock enable
pub use PKAEN_W as AESEN_W;
///Field `RNGEN` writer - CPU2 True RNG clocks enable
pub use PKAEN_W as RNGEN_W;
///Field `HSEMEN` writer - CPU2 HSEM clock enable
pub use PKAEN_W as HSEMEN_W;
///Field `IPCCEN` writer - CPU2 IPCC interface clock enable
pub use PKAEN_W as IPCCEN_W;
///Field `FLASHEN` writer - CPU2 Flash interface clock enable
pub use PKAEN_W as FLASHEN_W;
impl R {
    ///Bit 16 - CPU2 PKA accelerator clock enable
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CPU2 AES accelerator clock enable
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CPU2 True RNG clocks enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CPU2 HSEM clock enable
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CPU2 IPCC interface clock enable
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 25 - CPU2 Flash interface clock enable
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2AHB3ENR")
            .field("pkaen", &self.pkaen())
            .field("flashen", &self.flashen())
            .field("ipccen", &self.ipccen())
            .field("hsemen", &self.hsemen())
            .field("rngen", &self.rngen())
            .field("aesen", &self.aesen())
            .finish()
    }
}
impl W {
    ///Bit 16 - CPU2 PKA accelerator clock enable
    #[inline(always)]
    pub fn pkaen(&mut self) -> PKAEN_W<C2AHB3ENRrs> {
        PKAEN_W::new(self, 16)
    }
    ///Bit 17 - CPU2 AES accelerator clock enable
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W<C2AHB3ENRrs> {
        AESEN_W::new(self, 17)
    }
    ///Bit 18 - CPU2 True RNG clocks enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<C2AHB3ENRrs> {
        RNGEN_W::new(self, 18)
    }
    ///Bit 19 - CPU2 HSEM clock enable
    #[inline(always)]
    pub fn hsemen(&mut self) -> HSEMEN_W<C2AHB3ENRrs> {
        HSEMEN_W::new(self, 19)
    }
    ///Bit 20 - CPU2 IPCC interface clock enable
    #[inline(always)]
    pub fn ipccen(&mut self) -> IPCCEN_W<C2AHB3ENRrs> {
        IPCCEN_W::new(self, 20)
    }
    ///Bit 25 - CPU2 Flash interface clock enable
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W<C2AHB3ENRrs> {
        FLASHEN_W::new(self, 25)
    }
}
/**CPU2 AHB3 peripheral clock enable register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2AHB3ENR)*/
pub struct C2AHB3ENRrs;
impl crate::RegisterSpec for C2AHB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2ahb3enr::R`](R) reader structure
impl crate::Readable for C2AHB3ENRrs {}
///`write(|w| ..)` method takes [`c2ahb3enr::W`](W) writer structure
impl crate::Writable for C2AHB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2AHB3ENR to value 0x0208_0000
impl crate::Resettable for C2AHB3ENRrs {
    const RESET_VALUE: u32 = 0x0208_0000;
}
