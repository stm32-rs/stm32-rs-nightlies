///Register `AHBENR` reader
pub type R = crate::R<AHBENRrs>;
///Register `AHBENR` writer
pub type W = crate::W<AHBENRrs>;
/**DMA clock enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - DMA clock enable bit
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
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
///Field `DMAEN` writer - DMA clock enable bit
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
///Field `MIFEN` reader - NVM interface clock enable bit
pub use DMAEN_R as MIFEN_R;
///Field `CRCEN` reader - CRC clock enable bit
pub use DMAEN_R as CRCEN_R;
///Field `CRYPEN` reader - Crypto clock enable bit
pub use DMAEN_R as CRYPEN_R;
///Field `MIFEN` writer - NVM interface clock enable bit
pub use DMAEN_W as MIFEN_W;
///Field `CRCEN` writer - CRC clock enable bit
pub use DMAEN_W as CRCEN_W;
///Field `CRYPEN` writer - Crypto clock enable bit
pub use DMAEN_W as CRYPEN_W;
impl R {
    ///Bit 0 - DMA clock enable bit
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - NVM interface clock enable bit
    #[inline(always)]
    pub fn mifen(&self) -> MIFEN_R {
        MIFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable bit
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 24 - Crypto clock enable bit
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBENR")
            .field("dmaen", &self.dmaen())
            .field("crypen", &self.crypen())
            .field("crcen", &self.crcen())
            .field("mifen", &self.mifen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA clock enable bit
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, AHBENRrs> {
        DMAEN_W::new(self, 0)
    }
    ///Bit 8 - NVM interface clock enable bit
    #[inline(always)]
    pub fn mifen(&mut self) -> MIFEN_W<'_, AHBENRrs> {
        MIFEN_W::new(self, 8)
    }
    ///Bit 12 - CRC clock enable bit
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHBENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 24 - Crypto clock enable bit
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W<'_, AHBENRrs> {
        CRYPEN_W::new(self, 24)
    }
}
/**AHB peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#RCC:AHBENR)*/
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
///`reset()` method sets AHBENR to value 0x0100
impl crate::Resettable for AHBENRrs {
    const RESET_VALUE: u32 = 0x0100;
}
