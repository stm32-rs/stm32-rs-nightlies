///Register `RFSHCTL0` reader
pub type R = crate::R<RFSHCTL0rs>;
///Register `RFSHCTL0` writer
pub type W = crate::W<RFSHCTL0rs>;
///Field `PER_BANK_REFRESH` reader - PER_BANK_REFRESH
pub type PER_BANK_REFRESH_R = crate::BitReader;
///Field `PER_BANK_REFRESH` writer - PER_BANK_REFRESH
pub type PER_BANK_REFRESH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REFRESH_BURST` reader - REFRESH_BURST
pub type REFRESH_BURST_R = crate::FieldReader;
///Field `REFRESH_BURST` writer - REFRESH_BURST
pub type REFRESH_BURST_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `REFRESH_TO_X32` reader - REFRESH_TO_X32
pub type REFRESH_TO_X32_R = crate::FieldReader;
///Field `REFRESH_TO_X32` writer - REFRESH_TO_X32
pub type REFRESH_TO_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `REFRESH_MARGIN` reader - REFRESH_MARGIN
pub type REFRESH_MARGIN_R = crate::FieldReader;
///Field `REFRESH_MARGIN` writer - REFRESH_MARGIN
pub type REFRESH_MARGIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 2 - PER_BANK_REFRESH
    #[inline(always)]
    pub fn per_bank_refresh(&self) -> PER_BANK_REFRESH_R {
        PER_BANK_REFRESH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:8 - REFRESH_BURST
    #[inline(always)]
    pub fn refresh_burst(&self) -> REFRESH_BURST_R {
        REFRESH_BURST_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    ///Bits 12:16 - REFRESH_TO_X32
    #[inline(always)]
    pub fn refresh_to_x32(&self) -> REFRESH_TO_X32_R {
        REFRESH_TO_X32_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 20:23 - REFRESH_MARGIN
    #[inline(always)]
    pub fn refresh_margin(&self) -> REFRESH_MARGIN_R {
        REFRESH_MARGIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFSHCTL0")
            .field("per_bank_refresh", &self.per_bank_refresh())
            .field("refresh_burst", &self.refresh_burst())
            .field("refresh_to_x32", &self.refresh_to_x32())
            .field("refresh_margin", &self.refresh_margin())
            .finish()
    }
}
impl W {
    ///Bit 2 - PER_BANK_REFRESH
    #[inline(always)]
    pub fn per_bank_refresh(&mut self) -> PER_BANK_REFRESH_W<'_, RFSHCTL0rs> {
        PER_BANK_REFRESH_W::new(self, 2)
    }
    ///Bits 4:8 - REFRESH_BURST
    #[inline(always)]
    pub fn refresh_burst(&mut self) -> REFRESH_BURST_W<'_, RFSHCTL0rs> {
        REFRESH_BURST_W::new(self, 4)
    }
    ///Bits 12:16 - REFRESH_TO_X32
    #[inline(always)]
    pub fn refresh_to_x32(&mut self) -> REFRESH_TO_X32_W<'_, RFSHCTL0rs> {
        REFRESH_TO_X32_W::new(self, 12)
    }
    ///Bits 20:23 - REFRESH_MARGIN
    #[inline(always)]
    pub fn refresh_margin(&mut self) -> REFRESH_MARGIN_W<'_, RFSHCTL0rs> {
        REFRESH_MARGIN_W::new(self, 20)
    }
}
/**DDRCTRL refresh control register 0

You can [`read`](crate::Reg::read) this register and get [`rfshctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfshctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:RFSHCTL0)*/
pub struct RFSHCTL0rs;
impl crate::RegisterSpec for RFSHCTL0rs {
    type Ux = u32;
}
///`read()` method returns [`rfshctl0::R`](R) reader structure
impl crate::Readable for RFSHCTL0rs {}
///`write(|w| ..)` method takes [`rfshctl0::W`](W) writer structure
impl crate::Writable for RFSHCTL0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFSHCTL0 to value 0x0021_0000
impl crate::Resettable for RFSHCTL0rs {
    const RESET_VALUE: u32 = 0x0021_0000;
}
