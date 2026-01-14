///Register `AHB1ENR` reader
pub type R = crate::R<AHB1ENRrs>;
///Register `AHB1ENR` writer
pub type W = crate::W<AHB1ENRrs>;
/**GPDMA1 clock enable Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPDMA1EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<GPDMA1EN> for bool {
    #[inline(always)]
    fn from(variant: GPDMA1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPDMA1EN` reader - GPDMA1 clock enable Set and reset by software.
pub type GPDMA1EN_R = crate::BitReader<GPDMA1EN>;
impl GPDMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPDMA1EN {
        match self.bits {
            false => GPDMA1EN::Disabled,
            true => GPDMA1EN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPDMA1EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPDMA1EN::Enabled
    }
}
///Field `GPDMA1EN` writer - GPDMA1 clock enable Set and reset by software.
pub type GPDMA1EN_W<'a, REG> = crate::BitWriter<'a, REG, GPDMA1EN>;
impl<'a, REG> GPDMA1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPDMA1EN::Enabled)
    }
}
///Field `GPDMA2EN` reader - GPDMA2 clock enable Set and reset by software.
pub use GPDMA1EN_R as GPDMA2EN_R;
///Field `FLITFEN` reader - Flash interface clock enable Set and reset by software.
pub use GPDMA1EN_R as FLITFEN_R;
///Field `CRCEN` reader - CRC clock enable Set and reset by software.
pub use GPDMA1EN_R as CRCEN_R;
///Field `RAMCFGEN` reader - RAMCFG clock enable Set and reset by software.
pub use GPDMA1EN_R as RAMCFGEN_R;
///Field `GTZC1EN` reader - GTZC1 clock enable
pub use GPDMA1EN_R as GTZC1EN_R;
///Field `BKPRAMEN` reader - BKPRAM clock enable Set and reset by software
pub use GPDMA1EN_R as BKPRAMEN_R;
///Field `SRAM1EN` reader - SRAM1 clock enable Set and reset by software.
pub use GPDMA1EN_R as SRAM1EN_R;
///Field `GPDMA2EN` writer - GPDMA2 clock enable Set and reset by software.
pub use GPDMA1EN_W as GPDMA2EN_W;
///Field `FLITFEN` writer - Flash interface clock enable Set and reset by software.
pub use GPDMA1EN_W as FLITFEN_W;
///Field `CRCEN` writer - CRC clock enable Set and reset by software.
pub use GPDMA1EN_W as CRCEN_W;
///Field `RAMCFGEN` writer - RAMCFG clock enable Set and reset by software.
pub use GPDMA1EN_W as RAMCFGEN_W;
///Field `GTZC1EN` writer - GTZC1 clock enable
pub use GPDMA1EN_W as GTZC1EN_W;
///Field `BKPRAMEN` writer - BKPRAM clock enable Set and reset by software
pub use GPDMA1EN_W as BKPRAMEN_W;
///Field `SRAM1EN` writer - SRAM1 clock enable Set and reset by software.
pub use GPDMA1EN_W as SRAM1EN_W;
impl R {
    ///Bit 0 - GPDMA1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpdma1en(&self) -> GPDMA1EN_R {
        GPDMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPDMA2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpdma2en(&self) -> GPDMA2EN_R {
        GPDMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Flash interface clock enable Set and reset by software.
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable Set and reset by software.
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 17 - RAMCFG clock enable Set and reset by software.
    #[inline(always)]
    pub fn ramcfgen(&self) -> RAMCFGEN_R {
        RAMCFGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - GTZC1 clock enable
    #[inline(always)]
    pub fn gtzc1en(&self) -> GTZC1EN_R {
        GTZC1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - BKPRAM clock enable Set and reset by software
    #[inline(always)]
    pub fn bkpramen(&self) -> BKPRAMEN_R {
        BKPRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 31 - SRAM1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1ENR")
            .field("gpdma1en", &self.gpdma1en())
            .field("gpdma2en", &self.gpdma2en())
            .field("flitfen", &self.flitfen())
            .field("crcen", &self.crcen())
            .field("ramcfgen", &self.ramcfgen())
            .field("bkpramen", &self.bkpramen())
            .field("sram1en", &self.sram1en())
            .field("gtzc1en", &self.gtzc1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPDMA1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpdma1en(&mut self) -> GPDMA1EN_W<'_, AHB1ENRrs> {
        GPDMA1EN_W::new(self, 0)
    }
    ///Bit 1 - GPDMA2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpdma2en(&mut self) -> GPDMA2EN_W<'_, AHB1ENRrs> {
        GPDMA2EN_W::new(self, 1)
    }
    ///Bit 8 - Flash interface clock enable Set and reset by software.
    #[inline(always)]
    pub fn flitfen(&mut self) -> FLITFEN_W<'_, AHB1ENRrs> {
        FLITFEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clock enable Set and reset by software.
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 17 - RAMCFG clock enable Set and reset by software.
    #[inline(always)]
    pub fn ramcfgen(&mut self) -> RAMCFGEN_W<'_, AHB1ENRrs> {
        RAMCFGEN_W::new(self, 17)
    }
    ///Bit 24 - GTZC1 clock enable
    #[inline(always)]
    pub fn gtzc1en(&mut self) -> GTZC1EN_W<'_, AHB1ENRrs> {
        GTZC1EN_W::new(self, 24)
    }
    ///Bit 28 - BKPRAM clock enable Set and reset by software
    #[inline(always)]
    pub fn bkpramen(&mut self) -> BKPRAMEN_W<'_, AHB1ENRrs> {
        BKPRAMEN_W::new(self, 28)
    }
    ///Bit 31 - SRAM1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sram1en(&mut self) -> SRAM1EN_W<'_, AHB1ENRrs> {
        SRAM1EN_W::new(self, 31)
    }
}
/**RCC AHB1 peripherals clock register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:AHB1ENR)*/
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1enr::R`](R) reader structure
impl crate::Readable for AHB1ENRrs {}
///`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1ENR to value 0x9000_0100
impl crate::Resettable for AHB1ENRrs {
    const RESET_VALUE: u32 = 0x9000_0100;
}
