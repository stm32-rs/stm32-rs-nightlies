///Register `PWR_BKP0R` reader
pub type R = crate::R<PWR_BKP0Rrs>;
///Register `PWR_BKP0R` writer
pub type W = crate::W<PWR_BKP0Rrs>;
///Field `BKP` reader - Backup bitfield This bitfield retains information when the device is in Standby.
pub type BKP_R = crate::FieldReader<u16>;
///Field `BKP` writer - Backup bitfield This bitfield retains information when the device is in Standby.
pub type BKP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Backup bitfield This bitfield retains information when the device is in Standby.
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_BKP0R")
            .field("bkp", &self.bkp())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Backup bitfield This bitfield retains information when the device is in Standby.
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<'_, PWR_BKP0Rrs> {
        BKP_W::new(self, 0)
    }
}
/**PWR backup 0 register

You can [`read`](crate::Reg::read) this register and get [`pwr_bkp0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_bkp0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_BKP0R)*/
pub struct PWR_BKP0Rrs;
impl crate::RegisterSpec for PWR_BKP0Rrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_bkp0r::R`](R) reader structure
impl crate::Readable for PWR_BKP0Rrs {}
///`write(|w| ..)` method takes [`pwr_bkp0r::W`](W) writer structure
impl crate::Writable for PWR_BKP0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWR_BKP0R to value 0
impl crate::Resettable for PWR_BKP0Rrs {}
