///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCOKC {
    ///1: Clear flag
    Clear = 1,
}
impl From<SYNCOKC> for bool {
    #[inline(always)]
    fn from(variant: SYNCOKC) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCOKC` reader - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register.
pub type SYNCOKC_R = crate::BitReader<SYNCOKC>;
impl SYNCOKC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCOKC> {
        match self.bits {
            true => Some(SYNCOKC::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SYNCOKC::Clear
    }
}
///Field `SYNCOKC` writer - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register.
pub type SYNCOKC_W<'a, REG> = crate::BitWriter<'a, REG, SYNCOKC>;
impl<'a, REG> SYNCOKC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOKC::Clear)
    }
}
///Field `SYNCWARNC` reader - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register.
pub use SYNCOKC_R as SYNCWARNC_R;
///Field `ERRC` reader - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register.
pub use SYNCOKC_R as ERRC_R;
///Field `ESYNCC` reader - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register.
pub use SYNCOKC_R as ESYNCC_R;
///Field `SYNCWARNC` writer - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register.
pub use SYNCOKC_W as SYNCWARNC_W;
///Field `ERRC` writer - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register.
pub use SYNCOKC_W as ERRC_W;
///Field `ESYNCC` writer - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register.
pub use SYNCOKC_W as ESYNCC_W;
impl R {
    ///Bit 0 - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register.
    #[inline(always)]
    pub fn syncokc(&self) -> SYNCOKC_R {
        SYNCOKC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register.
    #[inline(always)]
    pub fn syncwarnc(&self) -> SYNCWARNC_R {
        SYNCWARNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register.
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register.
    #[inline(always)]
    pub fn esyncc(&self) -> ESYNCC_R {
        ESYNCC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("syncokc", &self.syncokc())
            .field("syncwarnc", &self.syncwarnc())
            .field("errc", &self.errc())
            .field("esyncc", &self.esyncc())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register.
    #[inline(always)]
    pub fn syncokc(&mut self) -> SYNCOKC_W<'_, ICRrs> {
        SYNCOKC_W::new(self, 0)
    }
    ///Bit 1 - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register.
    #[inline(always)]
    pub fn syncwarnc(&mut self) -> SYNCWARNC_W<'_, ICRrs> {
        SYNCWARNC_W::new(self, 1)
    }
    ///Bit 2 - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register.
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W<'_, ICRrs> {
        ERRC_W::new(self, 2)
    }
    ///Bit 3 - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register.
    #[inline(always)]
    pub fn esyncc(&mut self) -> ESYNCC_W<'_, ICRrs> {
        ESYNCC_W::new(self, 3)
    }
}
/**CRS interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#CRS:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
