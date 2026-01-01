///Register `AHBRSTR` reader
pub type R = crate::R<AHBRSTRrs>;
///Register `AHBRSTR` writer
pub type W = crate::W<AHBRSTRrs>;
/**DMA1 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMARST {
    ///1: Reset peripheral
    Reset = 1,
}
impl From<DMARST> for bool {
    #[inline(always)]
    fn from(variant: DMARST) -> Self {
        variant as u8 != 0
    }
}
///Field `DMARST` reader - DMA1 reset
pub type DMARST_R = crate::BitReader<DMARST>;
impl DMARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMARST> {
        match self.bits {
            true => Some(DMARST::Reset),
            _ => None,
        }
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMARST::Reset
    }
}
///Field `DMARST` writer - DMA1 reset
pub type DMARST_W<'a, REG> = crate::BitWriter<'a, REG, DMARST>;
impl<'a, REG> DMARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DMARST::Reset)
    }
}
///Field `FLASHRST` reader - FLITF reset
pub use DMARST_R as FLASHRST_R;
///Field `CRCRST` reader - CRC reset
pub use DMARST_R as CRCRST_R;
///Field `FLASHRST` writer - FLITF reset
pub use DMARST_W as FLASHRST_W;
///Field `CRCRST` writer - CRC reset
pub use DMARST_W as CRCRST_W;
impl R {
    ///Bit 0 - DMA1 reset
    #[inline(always)]
    pub fn dmarst(&self) -> DMARST_R {
        DMARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - FLITF reset
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRSTR")
            .field("dmarst", &self.dmarst())
            .field("flashrst", &self.flashrst())
            .field("crcrst", &self.crcrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 reset
    #[inline(always)]
    pub fn dmarst(&mut self) -> DMARST_W<'_, AHBRSTRrs> {
        DMARST_W::new(self, 0)
    }
    ///Bit 8 - FLITF reset
    #[inline(always)]
    pub fn flashrst(&mut self) -> FLASHRST_W<'_, AHBRSTRrs> {
        FLASHRST_W::new(self, 8)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHBRSTRrs> {
        CRCRST_W::new(self, 12)
    }
}
/**AHB peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#RCC:AHBRSTR)*/
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
