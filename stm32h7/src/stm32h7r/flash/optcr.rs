///Register `OPTCR` reader
pub type R = crate::R<OPTCRrs>;
///Register `OPTCR` writer
pub type W = crate::W<OPTCRrs>;
///Field `OPTLOCK` reader - Options lock When this bit is set write to all other bits in this register, and write to OTP words, option bytes and option bytes keys control registers, are ignored. Clearing this bit requires the correct write sequence to FLASH_OPTKEYR register (see this register for details). If a wrong sequence is executed, or the unlock sequence is performed twice, this bit remains locked until next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register.
pub type OPTLOCK_R = crate::BitReader;
///Field `OPTLOCK` writer - Options lock When this bit is set write to all other bits in this register, and write to OTP words, option bytes and option bytes keys control registers, are ignored. Clearing this bit requires the correct write sequence to FLASH_OPTKEYR register (see this register for details). If a wrong sequence is executed, or the unlock sequence is performed twice, this bit remains locked until next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register.
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PG_OPT` reader - Program options
pub type PG_OPT_R = crate::BitReader;
///Field `PG_OPT` writer - Program options
pub type PG_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KVEIE` reader - Key valid error interrupt enable bit This bit controls if an interrupt has to be generated when KVEF is set in FLASH_OPTISR.
pub type KVEIE_R = crate::BitReader;
///Field `KVEIE` writer - Key valid error interrupt enable bit This bit controls if an interrupt has to be generated when KVEF is set in FLASH_OPTISR.
pub type KVEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KTEIE` reader - Key transfer error interrupt enable bit This bit controls if an interrupt has to be generated when KTEF is set in FLASH_OPTISR.
pub type KTEIE_R = crate::BitReader;
///Field `KTEIE` writer - Key transfer error interrupt enable bit This bit controls if an interrupt has to be generated when KTEF is set in FLASH_OPTISR.
pub type KTEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTERRIE` reader - Option byte change error interrupt enable bit This bit controls if an interrupt has to be generated when an error occurs during an option byte change.
pub type OPTERRIE_R = crate::BitReader;
///Field `OPTERRIE` writer - Option byte change error interrupt enable bit This bit controls if an interrupt has to be generated when an error occurs during an option byte change.
pub type OPTERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Options lock When this bit is set write to all other bits in this register, and write to OTP words, option bytes and option bytes keys control registers, are ignored. Clearing this bit requires the correct write sequence to FLASH_OPTKEYR register (see this register for details). If a wrong sequence is executed, or the unlock sequence is performed twice, this bit remains locked until next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register.
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Program options
    #[inline(always)]
    pub fn pg_opt(&self) -> PG_OPT_R {
        PG_OPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 27 - Key valid error interrupt enable bit This bit controls if an interrupt has to be generated when KVEF is set in FLASH_OPTISR.
    #[inline(always)]
    pub fn kveie(&self) -> KVEIE_R {
        KVEIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Key transfer error interrupt enable bit This bit controls if an interrupt has to be generated when KTEF is set in FLASH_OPTISR.
    #[inline(always)]
    pub fn kteie(&self) -> KTEIE_R {
        KTEIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Option byte change error interrupt enable bit This bit controls if an interrupt has to be generated when an error occurs during an option byte change.
    #[inline(always)]
    pub fn opterrie(&self) -> OPTERRIE_R {
        OPTERRIE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR")
            .field("optlock", &self.optlock())
            .field("pg_opt", &self.pg_opt())
            .field("kveie", &self.kveie())
            .field("kteie", &self.kteie())
            .field("opterrie", &self.opterrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Options lock When this bit is set write to all other bits in this register, and write to OTP words, option bytes and option bytes keys control registers, are ignored. Clearing this bit requires the correct write sequence to FLASH_OPTKEYR register (see this register for details). If a wrong sequence is executed, or the unlock sequence is performed twice, this bit remains locked until next system reset. During the write access to set LOCK bit from 0 to 1, it is possible to change the other bits of this register.
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<'_, OPTCRrs> {
        OPTLOCK_W::new(self, 0)
    }
    ///Bit 1 - Program options
    #[inline(always)]
    pub fn pg_opt(&mut self) -> PG_OPT_W<'_, OPTCRrs> {
        PG_OPT_W::new(self, 1)
    }
    ///Bit 27 - Key valid error interrupt enable bit This bit controls if an interrupt has to be generated when KVEF is set in FLASH_OPTISR.
    #[inline(always)]
    pub fn kveie(&mut self) -> KVEIE_W<'_, OPTCRrs> {
        KVEIE_W::new(self, 27)
    }
    ///Bit 28 - Key transfer error interrupt enable bit This bit controls if an interrupt has to be generated when KTEF is set in FLASH_OPTISR.
    #[inline(always)]
    pub fn kteie(&mut self) -> KTEIE_W<'_, OPTCRrs> {
        KTEIE_W::new(self, 28)
    }
    ///Bit 30 - Option byte change error interrupt enable bit This bit controls if an interrupt has to be generated when an error occurs during an option byte change.
    #[inline(always)]
    pub fn opterrie(&mut self) -> OPTERRIE_W<'_, OPTCRrs> {
        OPTERRIE_W::new(self, 30)
    }
}
/**FLASH options control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OPTCR)*/
pub struct OPTCRrs;
impl crate::RegisterSpec for OPTCRrs {
    type Ux = u32;
}
///`read()` method returns [`optcr::R`](R) reader structure
impl crate::Readable for OPTCRrs {}
///`write(|w| ..)` method takes [`optcr::W`](W) writer structure
impl crate::Writable for OPTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTCR to value 0x01
impl crate::Resettable for OPTCRrs {
    const RESET_VALUE: u32 = 0x01;
}
