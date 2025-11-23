///Register `SECBOOTADD0R` reader
pub type R = crate::R<SECBOOTADD0Rrs>;
///Register `SECBOOTADD0R` writer
pub type W = crate::W<SECBOOTADD0Rrs>;
///Field `BOOT_LOCK` reader - BOOT_LOCK
pub type BOOT_LOCK_R = crate::BitReader;
///Field `BOOT_LOCK` writer - BOOT_LOCK
pub type BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECBOOTADD0` writer - SECBOOTADD0
pub type SECBOOTADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bit 0 - BOOT_LOCK
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECBOOTADD0R")
            .field("boot_lock", &self.boot_lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - BOOT_LOCK
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<'_, SECBOOTADD0Rrs> {
        BOOT_LOCK_W::new(self, 0)
    }
    ///Bits 7:31 - SECBOOTADD0
    #[inline(always)]
    pub fn secbootadd0(&mut self) -> SECBOOTADD0_W<'_, SECBOOTADD0Rrs> {
        SECBOOTADD0_W::new(self, 7)
    }
}
/**FFlash secure boot address 0 register

You can [`read`](crate::Reg::read) this register and get [`secbootadd0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbootadd0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FLASH:SECBOOTADD0R)*/
pub struct SECBOOTADD0Rrs;
impl crate::RegisterSpec for SECBOOTADD0Rrs {
    type Ux = u32;
}
///`read()` method returns [`secbootadd0r::R`](R) reader structure
impl crate::Readable for SECBOOTADD0Rrs {}
///`write(|w| ..)` method takes [`secbootadd0r::W`](W) writer structure
impl crate::Writable for SECBOOTADD0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECBOOTADD0R to value 0
impl crate::Resettable for SECBOOTADD0Rrs {}
