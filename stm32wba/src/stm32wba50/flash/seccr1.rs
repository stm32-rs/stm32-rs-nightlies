///Register `SECCR1` reader
pub type R = crate::R<SECCR1rs>;
///Register `SECCR1` writer
pub type W = crate::W<SECCR1rs>;
///Field `PG` reader - Secure programming
pub type PG_R = crate::BitReader;
///Field `PG` writer - Secure programming
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PER` reader - Secure page erase
pub type PER_R = crate::BitReader;
///Field `PER` writer - Secure page erase
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER` reader - Secure Flash mass erase This bit triggers the Flash secure mass erase (all Flash user pages) when set.
pub type MER_R = crate::BitReader;
///Field `MER` writer - Secure Flash mass erase This bit triggers the Flash secure mass erase (all Flash user pages) when set.
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PNB` reader - Secure page number selection These bits select the page to erase: ... Note that bit 9 is reserved on STM32WBA5xEx devices.
pub type PNB_R = crate::FieldReader;
///Field `PNB` writer - Secure page number selection These bits select the page to erase: ... Note that bit 9 is reserved on STM32WBA5xEx devices.
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `BWR` reader - Secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_R = crate::BitReader;
///Field `BWR` writer - Secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRT` reader - Secure start This bit triggers a secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
pub type STRT_R = crate::BitReader;
///Field `STRT` writer - Secure start This bit triggers a secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPIE` reader - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in FLASH_SECSR is set to 1.
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in FLASH_SECSR is set to 1.
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - Secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in FLASH_SECSR is set to 1.
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - Secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in FLASH_SECSR is set to 1.
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INV` reader - Flash memory security state invert This bit inverts the Flash memory security state.
pub type INV_R = crate::BitReader;
///Field `INV` writer - Flash memory security state invert This bit inverts the Flash memory security state.
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - Secure lock This bit is set only. When set, the FLASH_SECCR1 register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Secure lock This bit is set only. When set, the FLASH_SECCR1 register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Secure programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Secure page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Secure Flash mass erase This bit triggers the Flash secure mass erase (all Flash user pages) when set.
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:9 - Secure page number selection These bits select the page to erase: ... Note that bit 9 is reserved on STM32WBA5xEx devices.
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 14 - Secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    pub fn bwr(&self) -> BWR_R {
        BWR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Secure start This bit triggers a secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in FLASH_SECSR is set to 1.
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in FLASH_SECSR is set to 1.
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 29 - Flash memory security state invert This bit inverts the Flash memory security state.
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Secure lock This bit is set only. When set, the FLASH_SECCR1 register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCR1")
            .field("pg", &self.pg())
            .field("per", &self.per())
            .field("mer", &self.mer())
            .field("pnb", &self.pnb())
            .field("bwr", &self.bwr())
            .field("strt", &self.strt())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
            .field("inv", &self.inv())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Secure programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, SECCR1rs> {
        PG_W::new(self, 0)
    }
    ///Bit 1 - Secure page erase
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<'_, SECCR1rs> {
        PER_W::new(self, 1)
    }
    ///Bit 2 - Secure Flash mass erase This bit triggers the Flash secure mass erase (all Flash user pages) when set.
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<'_, SECCR1rs> {
        MER_W::new(self, 2)
    }
    ///Bits 3:9 - Secure page number selection These bits select the page to erase: ... Note that bit 9 is reserved on STM32WBA5xEx devices.
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W<'_, SECCR1rs> {
        PNB_W::new(self, 3)
    }
    ///Bit 14 - Secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    pub fn bwr(&mut self) -> BWR_W<'_, SECCR1rs> {
        BWR_W::new(self, 14)
    }
    ///Bit 16 - Secure start This bit triggers a secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR.
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<'_, SECCR1rs> {
        STRT_W::new(self, 16)
    }
    ///Bit 24 - Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in FLASH_SECSR is set to 1.
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<'_, SECCR1rs> {
        EOPIE_W::new(self, 24)
    }
    ///Bit 25 - Secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in FLASH_SECSR is set to 1.
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, SECCR1rs> {
        ERRIE_W::new(self, 25)
    }
    ///Bit 29 - Flash memory security state invert This bit inverts the Flash memory security state.
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<'_, SECCR1rs> {
        INV_W::new(self, 29)
    }
    ///Bit 31 - Secure lock This bit is set only. When set, the FLASH_SECCR1 register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, SECCR1rs> {
        LOCK_W::new(self, 31)
    }
}
/**FLASH secure control register

You can [`read`](crate::Reg::read) this register and get [`seccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#FLASH:SECCR1)*/
pub struct SECCR1rs;
impl crate::RegisterSpec for SECCR1rs {
    type Ux = u32;
}
///`read()` method returns [`seccr1::R`](R) reader structure
impl crate::Readable for SECCR1rs {}
///`write(|w| ..)` method takes [`seccr1::W`](W) writer structure
impl crate::Writable for SECCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCR1 to value 0x8000_0000
impl crate::Resettable for SECCR1rs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
