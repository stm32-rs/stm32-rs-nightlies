///Register `AHB3RSTR` reader
pub type R = crate::R<AHB3RSTRrs>;
///Register `AHB3RSTR` writer
pub type W = crate::W<AHB3RSTRrs>;
/**PKARST

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKARST {
    ///0: No effect
    NoReset = 0,
    ///1: Reset peripheral
    Reset = 1,
}
impl From<PKARST> for bool {
    #[inline(always)]
    fn from(variant: PKARST) -> Self {
        variant as u8 != 0
    }
}
///Field `PKARST` reader - PKARST
pub type PKARST_R = crate::BitReader<PKARST>;
impl PKARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PKARST {
        match self.bits {
            false => PKARST::NoReset,
            true => PKARST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == PKARST::NoReset
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PKARST::Reset
    }
}
///Field `PKARST` writer - PKARST
pub type PKARST_W<'a, REG> = crate::BitWriter<'a, REG, PKARST>;
impl<'a, REG> PKARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(PKARST::NoReset)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PKARST::Reset)
    }
}
///Field `AESRST` reader - AESRST
pub use PKARST_R as AESRST_R;
///Field `RNGRST` reader - RNGRST
pub use PKARST_R as RNGRST_R;
///Field `HSEMRST` reader - HSEMRST
pub use PKARST_R as HSEMRST_R;
///Field `IPCCRST` reader - IPCCRST
pub use PKARST_R as IPCCRST_R;
///Field `FLASHRST` reader - Flash interface reset
pub use PKARST_R as FLASHRST_R;
///Field `AESRST` writer - AESRST
pub use PKARST_W as AESRST_W;
///Field `RNGRST` writer - RNGRST
pub use PKARST_W as RNGRST_W;
///Field `HSEMRST` writer - HSEMRST
pub use PKARST_W as HSEMRST_W;
///Field `IPCCRST` writer - IPCCRST
pub use PKARST_W as IPCCRST_W;
///Field `FLASHRST` writer - Flash interface reset
pub use PKARST_W as FLASHRST_W;
impl R {
    ///Bit 16 - PKARST
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AESRST
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNGRST
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - HSEMRST
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - IPCCRST
    #[inline(always)]
    pub fn ipccrst(&self) -> IPCCRST_R {
        IPCCRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 25 - Flash interface reset
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3RSTR")
            .field("pkarst", &self.pkarst())
            .field("flashrst", &self.flashrst())
            .field("ipccrst", &self.ipccrst())
            .field("hsemrst", &self.hsemrst())
            .field("rngrst", &self.rngrst())
            .field("aesrst", &self.aesrst())
            .finish()
    }
}
impl W {
    ///Bit 16 - PKARST
    #[inline(always)]
    pub fn pkarst(&mut self) -> PKARST_W<'_, AHB3RSTRrs> {
        PKARST_W::new(self, 16)
    }
    ///Bit 17 - AESRST
    #[inline(always)]
    pub fn aesrst(&mut self) -> AESRST_W<'_, AHB3RSTRrs> {
        AESRST_W::new(self, 17)
    }
    ///Bit 18 - RNGRST
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHB3RSTRrs> {
        RNGRST_W::new(self, 18)
    }
    ///Bit 19 - HSEMRST
    #[inline(always)]
    pub fn hsemrst(&mut self) -> HSEMRST_W<'_, AHB3RSTRrs> {
        HSEMRST_W::new(self, 19)
    }
    ///Bit 20 - IPCCRST
    #[inline(always)]
    pub fn ipccrst(&mut self) -> IPCCRST_W<'_, AHB3RSTRrs> {
        IPCCRST_W::new(self, 20)
    }
    ///Bit 25 - Flash interface reset
    #[inline(always)]
    pub fn flashrst(&mut self) -> FLASHRST_W<'_, AHB3RSTRrs> {
        FLASHRST_W::new(self, 25)
    }
}
/**AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:AHB3RSTR)*/
pub struct AHB3RSTRrs;
impl crate::RegisterSpec for AHB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3rstr::R`](R) reader structure
impl crate::Readable for AHB3RSTRrs {}
///`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure
impl crate::Writable for AHB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3RSTR to value 0
impl crate::Resettable for AHB3RSTRrs {}
