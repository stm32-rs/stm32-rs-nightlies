///Register `AHBENR` reader
pub type R = crate::R<AHBENRrs>;
///Register `AHBENR` writer
pub type W = crate::W<AHBENRrs>;
/**DMA clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - DMA clock enable
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
///Field `DMAEN` writer - DMA clock enable
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
///Field `FLASHEN` reader - Flash memory interface clock enable
pub use DMAEN_R as FLASHEN_R;
///Field `CRCEN` reader - CRC clock enable
pub use DMAEN_R as CRCEN_R;
///Field `AESEN` reader - AES hardware accelerator
pub use DMAEN_R as AESEN_R;
///Field `RNGEN` reader - Random number generator clock enable
pub use DMAEN_R as RNGEN_R;
///Field `FLASHEN` writer - Flash memory interface clock enable
pub use DMAEN_W as FLASHEN_W;
///Field `CRCEN` writer - CRC clock enable
pub use DMAEN_W as CRCEN_W;
///Field `AESEN` writer - AES hardware accelerator
pub use DMAEN_W as AESEN_W;
///Field `RNGEN` writer - Random number generator clock enable
pub use DMAEN_W as RNGEN_W;
impl R {
    ///Bit 0 - DMA clock enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - AES hardware accelerator
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Random number generator clock enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBENR")
            .field("dmaen", &self.dmaen())
            .field("flashen", &self.flashen())
            .field("crcen", &self.crcen())
            .field("aesen", &self.aesen())
            .field("rngen", &self.rngen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA clock enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, AHBENRrs> {
        DMAEN_W::new(self, 0)
    }
    ///Bit 8 - Flash memory interface clock enable
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W<'_, AHBENRrs> {
        FLASHEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHBENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 16 - AES hardware accelerator
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W<'_, AHBENRrs> {
        AESEN_W::new(self, 16)
    }
    ///Bit 18 - Random number generator clock enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHBENRrs> {
        RNGEN_W::new(self, 18)
    }
}
/**AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#RCC:AHBENR)*/
pub struct AHBENRrs;
impl crate::RegisterSpec for AHBENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbenr::R`](R) reader structure
impl crate::Readable for AHBENRrs {}
///`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure
impl crate::Writable for AHBENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBENR to value 0
impl crate::Resettable for AHBENRrs {}
