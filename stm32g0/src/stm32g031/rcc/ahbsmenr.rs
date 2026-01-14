///Register `AHBSMENR` reader
pub type R = crate::R<AHBSMENRrs>;
///Register `AHBSMENR` writer
pub type W = crate::W<AHBSMENRrs>;
/**DMA clock enable during Sleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMASMEN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<DMASMEN> for bool {
    #[inline(always)]
    fn from(variant: DMASMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMASMEN` reader - DMA clock enable during Sleep mode
pub type DMASMEN_R = crate::BitReader<DMASMEN>;
impl DMASMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMASMEN {
        match self.bits {
            false => DMASMEN::Disabled,
            true => DMASMEN::Enabled,
        }
    }
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMASMEN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMASMEN::Enabled
    }
}
///Field `DMASMEN` writer - DMA clock enable during Sleep mode
pub type DMASMEN_W<'a, REG> = crate::BitWriter<'a, REG, DMASMEN>;
impl<'a, REG> DMASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMASMEN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMASMEN::Enabled)
    }
}
///Field `FLASHSMEN` reader - Flash memory interface clock enable during Sleep mode
pub use DMASMEN_R as FLASHSMEN_R;
///Field `SRAMSMEN` reader - SRAM clock enable during Sleep mode
pub use DMASMEN_R as SRAMSMEN_R;
///Field `CRCSMEN` reader - CRC clock enable during Sleep mode
pub use DMASMEN_R as CRCSMEN_R;
///Field `FLASHSMEN` writer - Flash memory interface clock enable during Sleep mode
pub use DMASMEN_W as FLASHSMEN_W;
///Field `SRAMSMEN` writer - SRAM clock enable during Sleep mode
pub use DMASMEN_W as SRAMSMEN_W;
///Field `CRCSMEN` writer - CRC clock enable during Sleep mode
pub use DMASMEN_W as CRCSMEN_W;
impl R {
    ///Bit 0 - DMA clock enable during Sleep mode
    #[inline(always)]
    pub fn dmasmen(&self) -> DMASMEN_R {
        DMASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBSMENR")
            .field("dmasmen", &self.dmasmen())
            .field("flashsmen", &self.flashsmen())
            .field("sramsmen", &self.sramsmen())
            .field("crcsmen", &self.crcsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA clock enable during Sleep mode
    #[inline(always)]
    pub fn dmasmen(&mut self) -> DMASMEN_W<'_, AHBSMENRrs> {
        DMASMEN_W::new(self, 0)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<'_, AHBSMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode
    #[inline(always)]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W<'_, AHBSMENRrs> {
        SRAMSMEN_W::new(self, 9)
    }
    ///Bit 12 - CRC clock enable during Sleep mode
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, AHBSMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
}
/**AHB peripheral clock enable in Sleep mode register

You can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#RCC:AHBSMENR)*/
pub struct AHBSMENRrs;
impl crate::RegisterSpec for AHBSMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbsmenr::R`](R) reader structure
impl crate::Readable for AHBSMENRrs {}
///`write(|w| ..)` method takes [`ahbsmenr::W`](W) writer structure
impl crate::Writable for AHBSMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBSMENR to value 0
impl crate::Resettable for AHBSMENRrs {}
