///Register `AHBRSTR` reader
pub type R = crate::R<AHBRSTRrs>;
///Register `AHBRSTR` writer
pub type W = crate::W<AHBRSTRrs>;
/**DMA reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMARSTW {
    ///1: Reset the module
    Reset = 1,
}
impl From<DMARSTW> for bool {
    #[inline(always)]
    fn from(variant: DMARSTW) -> Self {
        variant as u8 != 0
    }
}
///Field `DMARST` reader - DMA reset
pub type DMARST_R = crate::BitReader<DMARSTW>;
impl DMARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMARSTW> {
        match self.bits {
            true => Some(DMARSTW::Reset),
            _ => None,
        }
    }
    ///Reset the module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMARSTW::Reset
    }
}
///Field `DMARST` writer - DMA reset
pub type DMARST_W<'a, REG> = crate::BitWriter<'a, REG, DMARSTW>;
impl<'a, REG> DMARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMARSTW::Reset)
    }
}
///Field `MIFRST` reader - Memory interface reset
pub use DMARST_R as MIFRST_R;
///Field `CRCRST` reader - Test integration module reset
pub use DMARST_R as CRCRST_R;
///Field `CRYPRST` reader - Crypto module reset
pub use DMARST_R as CRYPRST_R;
///Field `MIFRST` writer - Memory interface reset
pub use DMARST_W as MIFRST_W;
///Field `CRCRST` writer - Test integration module reset
pub use DMARST_W as CRCRST_W;
///Field `CRYPRST` writer - Crypto module reset
pub use DMARST_W as CRYPRST_W;
impl R {
    ///Bit 0 - DMA reset
    #[inline(always)]
    pub fn dmarst(&self) -> DMARST_R {
        DMARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Memory interface reset
    #[inline(always)]
    pub fn mifrst(&self) -> MIFRST_R {
        MIFRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Test integration module reset
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 24 - Crypto module reset
    #[inline(always)]
    pub fn cryprst(&self) -> CRYPRST_R {
        CRYPRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRSTR")
            .field("dmarst", &self.dmarst())
            .field("cryprst", &self.cryprst())
            .field("crcrst", &self.crcrst())
            .field("mifrst", &self.mifrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA reset
    #[inline(always)]
    pub fn dmarst(&mut self) -> DMARST_W<'_, AHBRSTRrs> {
        DMARST_W::new(self, 0)
    }
    ///Bit 8 - Memory interface reset
    #[inline(always)]
    pub fn mifrst(&mut self) -> MIFRST_W<'_, AHBRSTRrs> {
        MIFRST_W::new(self, 8)
    }
    ///Bit 12 - Test integration module reset
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHBRSTRrs> {
        CRCRST_W::new(self, 12)
    }
    ///Bit 24 - Crypto module reset
    #[inline(always)]
    pub fn cryprst(&mut self) -> CRYPRST_W<'_, AHBRSTRrs> {
        CRYPRST_W::new(self, 24)
    }
}
/**AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#RCC:AHBRSTR)*/
pub struct AHBRSTRrs;
impl crate::RegisterSpec for AHBRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbrstr::R`](R) reader structure
impl crate::Readable for AHBRSTRrs {}
///`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure
impl crate::Writable for AHBRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBRSTR to value 0
impl crate::Resettable for AHBRSTRrs {}
