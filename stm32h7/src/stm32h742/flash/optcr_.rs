///Register `OPTCR_` reader
pub type R = crate::R<OPTCR_rs>;
///Register `OPTCR_` writer
pub type W = crate::W<OPTCR_rs>;
///Field `OPTLOCK` reader - FLASH_OPTCR lock option configuration bit
pub type OPTLOCK_R = crate::BitReader;
///Field `OPTLOCK` writer - FLASH_OPTCR lock option configuration bit
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTSTART` reader - Option byte start change option configuration bit
pub type OPTSTART_R = crate::BitReader;
///Field `OPTSTART` writer - Option byte start change option configuration bit
pub type OPTSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER` reader - Flash mass erase enable bit
pub type MER_R = crate::BitReader;
///Field `MER` writer - Flash mass erase enable bit
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTCHANGEERRIE` reader - Option byte change error interrupt enable bit
pub type OPTCHANGEERRIE_R = crate::BitReader;
///Field `OPTCHANGEERRIE` writer - Option byte change error interrupt enable bit
pub type OPTCHANGEERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP_BANK` reader - Bank swapping configuration bit
pub type SWAP_BANK_R = crate::BitReader;
///Field `SWAP_BANK` writer - Bank swapping configuration bit
pub type SWAP_BANK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - FLASH_OPTCR lock option configuration bit
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option byte start change option configuration bit
    #[inline(always)]
    pub fn optstart(&self) -> OPTSTART_R {
        OPTSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Flash mass erase enable bit
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 4) & 1) != 0)
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
        f.debug_struct("OPTCR_")
            .field("optlock", &self.optlock())
            .field("optstart", &self.optstart())
            .field("mer", &self.mer())
            .field("optchangeerrie", &self.optchangeerrie())
            .field("swap_bank", &self.swap_bank())
            .finish()
    }
}
impl W {
    ///Bit 0 - FLASH_OPTCR lock option configuration bit
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<OPTCR_rs> {
        OPTLOCK_W::new(self, 0)
    }
    ///Bit 1 - Option byte start change option configuration bit
    #[inline(always)]
    pub fn optstart(&mut self) -> OPTSTART_W<OPTCR_rs> {
        OPTSTART_W::new(self, 1)
    }
    ///Bit 4 - Flash mass erase enable bit
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<OPTCR_rs> {
        MER_W::new(self, 4)
    }
    ///Bit 30 - Option byte change error interrupt enable bit
    #[inline(always)]
    pub fn optchangeerrie(&mut self) -> OPTCHANGEERRIE_W<OPTCR_rs> {
        OPTCHANGEERRIE_W::new(self, 30)
    }
    ///Bit 31 - Bank swapping configuration bit
    #[inline(always)]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W<OPTCR_rs> {
        SWAP_BANK_W::new(self, 31)
    }
}
/**FLASH option control register

You can [`read`](crate::Reg::read) this register and get [`optcr_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#FLASH:OPTCR_)*/
pub struct OPTCR_rs;
impl crate::RegisterSpec for OPTCR_rs {
    type Ux = u32;
}
///`read()` method returns [`optcr_::R`](R) reader structure
impl crate::Readable for OPTCR_rs {}
///`write(|w| ..)` method takes [`optcr_::W`](W) writer structure
impl crate::Writable for OPTCR_rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTCR_ to value 0
impl crate::Resettable for OPTCR_rs {}
