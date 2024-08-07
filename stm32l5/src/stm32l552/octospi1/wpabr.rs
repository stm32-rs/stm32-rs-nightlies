///Register `WPABR` reader
pub type R = crate::R<WPABRrs>;
///Register `WPABR` writer
pub type W = crate::W<WPABRrs>;
///Field `LM` reader - Latency mode
pub type LM_R = crate::BitReader;
///Field `LM` writer - Latency mode
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WZL` reader - Write zero latency
pub type WZL_R = crate::BitReader;
///Field `WZL` writer - Write zero latency
pub type WZL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TACC` reader - Access time
pub type TACC_R = crate::FieldReader;
///Field `TACC` writer - Access time
pub type TACC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TRWR` reader - Read write recovery time
pub type TRWR_R = crate::FieldReader;
///Field `TRWR` writer - Read write recovery time
pub type TRWR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Latency mode
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Write zero latency
    #[inline(always)]
    pub fn wzl(&self) -> WZL_R {
        WZL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:15 - Access time
    #[inline(always)]
    pub fn tacc(&self) -> TACC_R {
        TACC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Read write recovery time
    #[inline(always)]
    pub fn trwr(&self) -> TRWR_R {
        TRWR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPABR")
            .field("lm", &self.lm())
            .field("wzl", &self.wzl())
            .field("tacc", &self.tacc())
            .field("trwr", &self.trwr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Latency mode
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<WPABRrs> {
        LM_W::new(self, 0)
    }
    ///Bit 1 - Write zero latency
    #[inline(always)]
    #[must_use]
    pub fn wzl(&mut self) -> WZL_W<WPABRrs> {
        WZL_W::new(self, 1)
    }
    ///Bits 8:15 - Access time
    #[inline(always)]
    #[must_use]
    pub fn tacc(&mut self) -> TACC_W<WPABRrs> {
        TACC_W::new(self, 8)
    }
    ///Bits 16:23 - Read write recovery time
    #[inline(always)]
    #[must_use]
    pub fn trwr(&mut self) -> TRWR_W<WPABRrs> {
        TRWR_W::new(self, 16)
    }
}
/**write alternate bytes register

You can [`read`](crate::Reg::read) this register and get [`wpabr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpabr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#OCTOSPI1:WPABR)*/
pub struct WPABRrs;
impl crate::RegisterSpec for WPABRrs {
    type Ux = u32;
}
///`read()` method returns [`wpabr::R`](R) reader structure
impl crate::Readable for WPABRrs {}
///`write(|w| ..)` method takes [`wpabr::W`](W) writer structure
impl crate::Writable for WPABRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WPABR to value 0
impl crate::Resettable for WPABRrs {
    const RESET_VALUE: u32 = 0;
}
