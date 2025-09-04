///Register `AHB3RSTR` reader
pub type R = crate::R<AHB3RSTRrs>;
///Register `AHB3RSTR` writer
pub type W = crate::W<AHB3RSTRrs>;
/**random number generator block reset Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<RNGRST> for bool {
    #[inline(always)]
    fn from(variant: RNGRST) -> Self {
        variant as u8 != 0
    }
}
///Field `RNGRST` reader - random number generator block reset Set and reset by software.
pub type RNGRST_R = crate::BitReader<RNGRST>;
impl RNGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RNGRST> {
        match self.bits {
            true => Some(RNGRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RNGRST::Reset
    }
}
///Field `RNGRST` writer - random number generator block reset Set and reset by software.
pub type RNGRST_W<'a, REG> = crate::BitWriter<'a, REG, RNGRST>;
impl<'a, REG> RNGRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RNGRST::Reset)
    }
}
///Field `HASHRST` reader - HASH block reset Set and reset by software.
pub use RNGRST_R as HASHRST_R;
///Field `CRYPRST` reader - CRYP block reset Set and reset by software.
pub use RNGRST_R as CRYPRST_R;
///Field `SAESRST` reader - SAES block reset Set and reset by software.
pub use RNGRST_R as SAESRST_R;
///Field `PKARST` reader - PKA block reset Set and reset by software.
pub use RNGRST_R as PKARST_R;
///Field `HASHRST` writer - HASH block reset Set and reset by software.
pub use RNGRST_W as HASHRST_W;
///Field `CRYPRST` writer - CRYP block reset Set and reset by software.
pub use RNGRST_W as CRYPRST_W;
///Field `SAESRST` writer - SAES block reset Set and reset by software.
pub use RNGRST_W as SAESRST_W;
///Field `PKARST` writer - PKA block reset Set and reset by software.
pub use RNGRST_W as PKARST_W;
impl R {
    ///Bit 0 - random number generator block reset Set and reset by software.
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HASH block reset Set and reset by software.
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CRYP block reset Set and reset by software.
    #[inline(always)]
    pub fn cryprst(&self) -> CRYPRST_R {
        CRYPRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - SAES block reset Set and reset by software.
    #[inline(always)]
    pub fn saesrst(&self) -> SAESRST_R {
        SAESRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - PKA block reset Set and reset by software.
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3RSTR")
            .field("rngrst", &self.rngrst())
            .field("hashrst", &self.hashrst())
            .field("cryprst", &self.cryprst())
            .field("saesrst", &self.saesrst())
            .field("pkarst", &self.pkarst())
            .finish()
    }
}
impl W {
    ///Bit 0 - random number generator block reset Set and reset by software.
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<AHB3RSTRrs> {
        RNGRST_W::new(self, 0)
    }
    ///Bit 1 - HASH block reset Set and reset by software.
    #[inline(always)]
    pub fn hashrst(&mut self) -> HASHRST_W<AHB3RSTRrs> {
        HASHRST_W::new(self, 1)
    }
    ///Bit 2 - CRYP block reset Set and reset by software.
    #[inline(always)]
    pub fn cryprst(&mut self) -> CRYPRST_W<AHB3RSTRrs> {
        CRYPRST_W::new(self, 2)
    }
    ///Bit 4 - SAES block reset Set and reset by software.
    #[inline(always)]
    pub fn saesrst(&mut self) -> SAESRST_W<AHB3RSTRrs> {
        SAESRST_W::new(self, 4)
    }
    ///Bit 6 - PKA block reset Set and reset by software.
    #[inline(always)]
    pub fn pkarst(&mut self) -> PKARST_W<AHB3RSTRrs> {
        PKARST_W::new(self, 6)
    }
}
/**RCC AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:AHB3RSTR)*/
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
