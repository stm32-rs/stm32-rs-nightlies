///Register `OPTCR` reader
pub type R = crate::R<OPTCRrs>;
///Register `OPTCR` writer
pub type W = crate::W<OPTCRrs>;
///Field `OPTLOCK` reader - FLASH_OPTCR lock option configuration bit
pub type OPTLOCK_R = crate::BitReader;
///Field `OPTLOCK` writer - FLASH_OPTCR lock option configuration bit
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTSTART` writer - Option byte start change option configuration bit
pub type OPTSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER` writer - Flash mass erase enable bit
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PG_OTP` reader - OTP program control bit
pub type PG_OTP_R = crate::BitReader;
///Field `PG_OTP` writer - OTP program control bit
pub type PG_OTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTCHANGEERRIE` reader - Option byte change error interrupt enable bit
pub type OPTCHANGEERRIE_R = crate::BitReader;
///Field `OPTCHANGEERRIE` writer - Option byte change error interrupt enable bit
pub type OPTCHANGEERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_BANK` reader - Bank swapping configuration bit
pub type SWAP_BANK_R = crate::BitReader;
impl R {
    ///Bit 0 - FLASH_OPTCR lock option configuration bit
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - OTP program control bit
    #[inline(always)]
    pub fn pg_otp(&self) -> PG_OTP_R {
        PG_OTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 30 - Option byte change error interrupt enable bit
    #[inline(always)]
    pub fn optchangeerrie(&self) -> OPTCHANGEERRIE_R {
        OPTCHANGEERRIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Bank swapping configuration bit
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR")
            .field("optlock", &self.optlock())
            .field("pg_otp", &self.pg_otp())
            .field("optchangeerrie", &self.optchangeerrie())
            .field("swap_bank", &self.swap_bank())
            .finish()
    }
}
impl W {
    ///Bit 0 - FLASH_OPTCR lock option configuration bit
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<OPTCRrs> {
        OPTLOCK_W::new(self, 0)
    }
    ///Bit 1 - Option byte start change option configuration bit
    #[inline(always)]
    pub fn optstart(&mut self) -> OPTSTART_W<OPTCRrs> {
        OPTSTART_W::new(self, 1)
    }
    ///Bit 4 - Flash mass erase enable bit
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<OPTCRrs> {
        MER_W::new(self, 4)
    }
    ///Bit 5 - OTP program control bit
    #[inline(always)]
    pub fn pg_otp(&mut self) -> PG_OTP_W<OPTCRrs> {
        PG_OTP_W::new(self, 5)
    }
    ///Bit 30 - Option byte change error interrupt enable bit
    #[inline(always)]
    pub fn optchangeerrie(&mut self) -> OPTCHANGEERRIE_W<OPTCRrs> {
        OPTCHANGEERRIE_W::new(self, 30)
    }
}
/**FLASH option control register

You can [`read`](crate::Reg::read) this register and get [`optcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#FLASH:OPTCR)*/
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
