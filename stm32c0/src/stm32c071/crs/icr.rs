///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `SYNCOKC` reader - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register.
pub type SYNCOKC_R = crate::BitReader;
///Field `SYNCOKC` writer - SYNC event OK clear flag Writing 1 to this bit clears the SYNCOKF flag in the CRS_ISR register.
pub type SYNCOKC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCWARNC` reader - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register.
pub type SYNCWARNC_R = crate::BitReader;
///Field `SYNCWARNC` writer - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register.
pub type SYNCWARNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRC` reader - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register.
pub type ERRC_R = crate::BitReader;
///Field `ERRC` writer - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register.
pub type ERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESYNCC` reader - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register.
pub type ESYNCC_R = crate::BitReader;
///Field `ESYNCC` writer - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register.
pub type ESYNCC_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn syncokc(&mut self) -> SYNCOKC_W<ICRrs> {
        SYNCOKC_W::new(self, 0)
    }
    ///Bit 1 - SYNC warning clear flag Writing 1 to this bit clears the SYNCWARNF flag in the CRS_ISR register.
    #[inline(always)]
    pub fn syncwarnc(&mut self) -> SYNCWARNC_W<ICRrs> {
        SYNCWARNC_W::new(self, 1)
    }
    ///Bit 2 - Error clear flag Writing 1 to this bit clears TRIMOVF, SYNCMISS and SYNCERR bits and consequently also the ERRF flag in the CRS_ISR register.
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W<ICRrs> {
        ERRC_W::new(self, 2)
    }
    ///Bit 3 - Expected SYNC clear flag Writing 1 to this bit clears the ESYNCF flag in the CRS_ISR register.
    #[inline(always)]
    pub fn esyncc(&mut self) -> ESYNCC_W<ICRrs> {
        ESYNCC_W::new(self, 3)
    }
}
/**CRS interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#CRS:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
