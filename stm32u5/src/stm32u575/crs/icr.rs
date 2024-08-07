///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `SYNCOKC` reader - SYNC event OK clear flag
pub type SYNCOKC_R = crate::BitReader;
///Field `SYNCOKC` writer - SYNC event OK clear flag
pub type SYNCOKC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCWARNC` reader - SYNC warning clear flag
pub type SYNCWARNC_R = crate::BitReader;
///Field `SYNCWARNC` writer - SYNC warning clear flag
pub type SYNCWARNC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRC` reader - Error clear flag
pub type ERRC_R = crate::BitReader;
///Field `ERRC` writer - Error clear flag
pub type ERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESYNCC` reader - Expected SYNC clear flag
pub type ESYNCC_R = crate::BitReader;
///Field `ESYNCC` writer - Expected SYNC clear flag
pub type ESYNCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SYNC event OK clear flag
    #[inline(always)]
    pub fn syncokc(&self) -> SYNCOKC_R {
        SYNCOKC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SYNC warning clear flag
    #[inline(always)]
    pub fn syncwarnc(&self) -> SYNCWARNC_R {
        SYNCWARNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Error clear flag
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Expected SYNC clear flag
    #[inline(always)]
    pub fn esyncc(&self) -> ESYNCC_R {
        ESYNCC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("esyncc", &self.esyncc())
            .field("errc", &self.errc())
            .field("syncwarnc", &self.syncwarnc())
            .field("syncokc", &self.syncokc())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYNC event OK clear flag
    #[inline(always)]
    #[must_use]
    pub fn syncokc(&mut self) -> SYNCOKC_W<ICRrs> {
        SYNCOKC_W::new(self, 0)
    }
    ///Bit 1 - SYNC warning clear flag
    #[inline(always)]
    #[must_use]
    pub fn syncwarnc(&mut self) -> SYNCWARNC_W<ICRrs> {
        SYNCWARNC_W::new(self, 1)
    }
    ///Bit 2 - Error clear flag
    #[inline(always)]
    #[must_use]
    pub fn errc(&mut self) -> ERRC_W<ICRrs> {
        ERRC_W::new(self, 2)
    }
    ///Bit 3 - Expected SYNC clear flag
    #[inline(always)]
    #[must_use]
    pub fn esyncc(&mut self) -> ESYNCC_W<ICRrs> {
        ESYNCC_W::new(self, 3)
    }
}
/**interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#CRS:ICR)*/
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
