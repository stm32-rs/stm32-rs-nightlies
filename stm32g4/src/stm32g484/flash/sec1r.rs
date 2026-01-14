///Register `SEC1R` reader
pub type R = crate::R<SEC1Rrs>;
///Register `SEC1R` writer
pub type W = crate::W<SEC1Rrs>;
///Field `SEC_SIZE1` reader - sets the number of pages used in the bank 1 Securable memory area.
pub type SEC_SIZE1_R = crate::FieldReader;
///Field `SEC_SIZE1` writer - sets the number of pages used in the bank 1 Securable memory area.
pub type SEC_SIZE1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BOOT_LOCK` reader - used to force boot from user Flash area.
pub type BOOT_LOCK_R = crate::BitReader;
///Field `BOOT_LOCK` writer - used to force boot from user Flash area.
pub type BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - sets the number of pages used in the bank 1 Securable memory area.
    #[inline(always)]
    pub fn sec_size1(&self) -> SEC_SIZE1_R {
        SEC_SIZE1_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 16 - used to force boot from user Flash area.
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC1R")
            .field("sec_size1", &self.sec_size1())
            .field("boot_lock", &self.boot_lock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - sets the number of pages used in the bank 1 Securable memory area.
    #[inline(always)]
    pub fn sec_size1(&mut self) -> SEC_SIZE1_W<'_, SEC1Rrs> {
        SEC_SIZE1_W::new(self, 0)
    }
    ///Bit 16 - used to force boot from user Flash area.
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<'_, SEC1Rrs> {
        BOOT_LOCK_W::new(self, 16)
    }
}
/**Flash Securable area bank1 register

You can [`read`](crate::Reg::read) this register and get [`sec1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#FLASH:SEC1R)*/
pub struct SEC1Rrs;
impl crate::RegisterSpec for SEC1Rrs {
    type Ux = u32;
}
///`read()` method returns [`sec1r::R`](R) reader structure
impl crate::Readable for SEC1Rrs {}
///`write(|w| ..)` method takes [`sec1r::W`](W) writer structure
impl crate::Writable for SEC1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEC1R to value 0xfff0_ff00
impl crate::Resettable for SEC1Rrs {
    const RESET_VALUE: u32 = 0xfff0_ff00;
}
