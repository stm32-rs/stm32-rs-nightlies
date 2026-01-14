///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `LPMS` reader - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep.
pub type LPMS_R = crate::BitReader;
///Field `LPMS` writer - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep.
pub type LPMS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENSDNBOR` reader - Enable BOR reset supervising during SHUTDOWN mode.
pub type ENSDNBOR_R = crate::BitReader;
///Field `ENSDNBOR` writer - Enable BOR reset supervising during SHUTDOWN mode.
pub type ENSDNBOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APC` reader - APC Apply Pull-up and pull-down configuration from CPU
pub type APC_R = crate::BitReader;
///Field `APC` writer - APC Apply Pull-up and pull-down configuration from CPU
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep.
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable BOR reset supervising during SHUTDOWN mode.
    #[inline(always)]
    pub fn ensdnbor(&self) -> ENSDNBOR_R {
        ENSDNBOR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - APC Apply Pull-up and pull-down configuration from CPU
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lpms", &self.lpms())
            .field("ensdnbor", &self.ensdnbor())
            .field("apc", &self.apc())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPMS Low Power Mode Selection Selection of the low power mode entered when CPU enters DEEP SLEEP mode and BLE is rdy2sleep.
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<'_, CR1rs> {
        LPMS_W::new(self, 0)
    }
    ///Bit 1 - Enable BOR reset supervising during SHUTDOWN mode.
    #[inline(always)]
    pub fn ensdnbor(&mut self) -> ENSDNBOR_W<'_, CR1rs> {
        ENSDNBOR_W::new(self, 1)
    }
    ///Bit 4 - APC Apply Pull-up and pull-down configuration from CPU
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W<'_, CR1rs> {
        APC_W::new(self, 4)
    }
}
/**CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#PWRC:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0x0114
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x0114;
}
