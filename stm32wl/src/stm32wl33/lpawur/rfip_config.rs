///Register `RFIP_CONFIG` reader
pub type R = crate::R<RFIP_CONFIGrs>;
///Register `RFIP_CONFIG` writer
pub type W = crate::W<RFIP_CONFIGrs>;
///Field `LPAWUR_ENABLE` reader - Enable (start) or Disable (stop) the LPAWUR feature (0: disabled by default)
pub type LPAWUR_ENABLE_R = crate::BitReader;
///Field `LPAWUR_ENABLE` writer - Enable (start) or Disable (stop) the LPAWUR feature (0: disabled by default)
pub type LPAWUR_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAKEUP_LEVEL` reader - - 00: the bit Sync has been detected
pub type WAKEUP_LEVEL_R = crate::FieldReader;
///Field `WAKEUP_LEVEL` writer - - 00: the bit Sync has been detected
pub type WAKEUP_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Enable (start) or Disable (stop) the LPAWUR feature (0: disabled by default)
    #[inline(always)]
    pub fn lpawur_enable(&self) -> LPAWUR_ENABLE_R {
        LPAWUR_ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - - 00: the bit Sync has been detected
    #[inline(always)]
    pub fn wakeup_level(&self) -> WAKEUP_LEVEL_R {
        WAKEUP_LEVEL_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFIP_CONFIG")
            .field("lpawur_enable", &self.lpawur_enable())
            .field("wakeup_level", &self.wakeup_level())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable (start) or Disable (stop) the LPAWUR feature (0: disabled by default)
    #[inline(always)]
    pub fn lpawur_enable(&mut self) -> LPAWUR_ENABLE_W<'_, RFIP_CONFIGrs> {
        LPAWUR_ENABLE_W::new(self, 0)
    }
    ///Bits 1:2 - - 00: the bit Sync has been detected
    #[inline(always)]
    pub fn wakeup_level(&mut self) -> WAKEUP_LEVEL_W<'_, RFIP_CONFIGrs> {
        WAKEUP_LEVEL_W::new(self, 1)
    }
}
/**RFIP_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`rfip_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfip_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:RFIP_CONFIG)*/
pub struct RFIP_CONFIGrs;
impl crate::RegisterSpec for RFIP_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`rfip_config::R`](R) reader structure
impl crate::Readable for RFIP_CONFIGrs {}
///`write(|w| ..)` method takes [`rfip_config::W`](W) writer structure
impl crate::Writable for RFIP_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFIP_CONFIG to value 0x06
impl crate::Resettable for RFIP_CONFIGrs {
    const RESET_VALUE: u32 = 0x06;
}
