///Register `AHB2RSTR` reader
pub type R = crate::R<AHB2RSTRrs>;
///Register `AHB2RSTR` writer
pub type W = crate::W<AHB2RSTRrs>;
/**AES module reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<AESRST> for bool {
    #[inline(always)]
    fn from(variant: AESRST) -> Self {
        variant as u8 != 0
    }
}
///Field `AESRST` reader - AES module reset
pub type AESRST_R = crate::BitReader<AESRST>;
impl AESRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<AESRST> {
        match self.bits {
            true => Some(AESRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == AESRST::Reset
    }
}
///Field `AESRST` writer - AES module reset
pub type AESRST_W<'a, REG> = crate::BitWriter<'a, REG, AESRST>;
impl<'a, REG> AESRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(AESRST::Reset)
    }
}
///Field `RNGRST` reader - Random number generator module reset
pub use AESRST_R as RNGRST_R;
///Field `OTGFSRST` reader - USB OTG FS module reset
pub use AESRST_R as OTGFSRST_R;
///Field `RNGRST` writer - Random number generator module reset
pub use AESRST_W as RNGRST_W;
///Field `OTGFSRST` writer - USB OTG FS module reset
pub use AESRST_W as OTGFSRST_W;
impl R {
    ///Bit 4 - AES module reset
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Random number generator module reset
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USB OTG FS module reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR")
            .field("aesrst", &self.aesrst())
            .field("otgfsrst", &self.otgfsrst())
            .field("rngrst", &self.rngrst())
            .finish()
    }
}
impl W {
    ///Bit 4 - AES module reset
    #[inline(always)]
    pub fn aesrst(&mut self) -> AESRST_W<'_, AHB2RSTRrs> {
        AESRST_W::new(self, 4)
    }
    ///Bit 6 - Random number generator module reset
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<'_, AHB2RSTRrs> {
        RNGRST_W::new(self, 6)
    }
    ///Bit 7 - USB OTG FS module reset
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<'_, AHB2RSTRrs> {
        OTGFSRST_W::new(self, 7)
    }
}
/**AHB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#RCC:AHB2RSTR)*/
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2rstr::R`](R) reader structure
impl crate::Readable for AHB2RSTRrs {}
///`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTRrs {}
