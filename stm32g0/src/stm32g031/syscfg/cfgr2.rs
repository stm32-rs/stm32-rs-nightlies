///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `LOCKUP_LOCK` reader - Cortex-M0+ LOCKUP bit enable bit
pub type LOCKUP_LOCK_R = crate::BitReader;
///Field `LOCKUP_LOCK` writer - Cortex-M0+ LOCKUP bit enable bit
pub type LOCKUP_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM_PARITY_LOCK` reader - SRAM parity lock bit
pub type SRAM_PARITY_LOCK_R = crate::BitReader;
///Field `SRAM_PARITY_LOCK` writer - SRAM parity lock bit
pub type SRAM_PARITY_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVD_LOCK` reader - PVD lock enable bit
pub type PVD_LOCK_R = crate::BitReader;
///Field `PVD_LOCK` writer - PVD lock enable bit
pub type PVD_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECC_LOCK` reader - ECC error lock bit
pub type ECC_LOCK_R = crate::BitReader;
///Field `ECC_LOCK` writer - ECC error lock bit
pub type ECC_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM_PEF` reader - SRAM parity error flag
pub type SRAM_PEF_R = crate::BitReader;
///Field `SRAM_PEF` writer - SRAM parity error flag
pub type SRAM_PEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA1_CDEN` reader - PA1_CDEN
pub type PA1_CDEN_R = crate::BitReader;
///Field `PA1_CDEN` writer - PA1_CDEN
pub type PA1_CDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA3_CDEN` reader - PA3_CDEN
pub type PA3_CDEN_R = crate::BitReader;
///Field `PA3_CDEN` writer - PA3_CDEN
pub type PA3_CDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA5_CDEN` reader - PA5_CDEN
pub type PA5_CDEN_R = crate::BitReader;
///Field `PA5_CDEN` writer - PA5_CDEN
pub type PA5_CDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA6_CDEN` reader - PA6_CDEN
pub type PA6_CDEN_R = crate::BitReader;
///Field `PA6_CDEN` writer - PA6_CDEN
pub type PA6_CDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA13_CDEN` reader - PA13_CDEN
pub type PA13_CDEN_R = crate::BitReader;
///Field `PA13_CDEN` writer - PA13_CDEN
pub type PA13_CDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB0_CDEN` reader - PB0_CDEN
pub type PB0_CDEN_R = crate::BitReader;
///Field `PB0_CDEN` writer - PB0_CDEN
pub type PB0_CDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB1_CDEN` reader - PB1_CDEN
pub type PB1_CDEN_R = crate::BitReader;
///Field `PB1_CDEN` writer - PB1_CDEN
pub type PB1_CDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB2_CDEN` reader - PB2_CDEN
pub type PB2_CDEN_R = crate::BitReader;
///Field `PB2_CDEN` writer - PB2_CDEN
pub type PB2_CDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Cortex-M0+ LOCKUP bit enable bit
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM parity lock bit
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SRAM_PARITY_LOCK_R {
        SRAM_PARITY_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PVD lock enable bit
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC error lock bit
    #[inline(always)]
    pub fn ecc_lock(&self) -> ECC_LOCK_R {
        ECC_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - SRAM parity error flag
    #[inline(always)]
    pub fn sram_pef(&self) -> SRAM_PEF_R {
        SRAM_PEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - PA1_CDEN
    #[inline(always)]
    pub fn pa1_cden(&self) -> PA1_CDEN_R {
        PA1_CDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PA3_CDEN
    #[inline(always)]
    pub fn pa3_cden(&self) -> PA3_CDEN_R {
        PA3_CDEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PA5_CDEN
    #[inline(always)]
    pub fn pa5_cden(&self) -> PA5_CDEN_R {
        PA5_CDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PA6_CDEN
    #[inline(always)]
    pub fn pa6_cden(&self) -> PA6_CDEN_R {
        PA6_CDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PA13_CDEN
    #[inline(always)]
    pub fn pa13_cden(&self) -> PA13_CDEN_R {
        PA13_CDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - PB0_CDEN
    #[inline(always)]
    pub fn pb0_cden(&self) -> PB0_CDEN_R {
        PB0_CDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - PB1_CDEN
    #[inline(always)]
    pub fn pb1_cden(&self) -> PB1_CDEN_R {
        PB1_CDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PB2_CDEN
    #[inline(always)]
    pub fn pb2_cden(&self) -> PB2_CDEN_R {
        PB2_CDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("lockup_lock", &self.lockup_lock())
            .field("sram_parity_lock", &self.sram_parity_lock())
            .field("pvd_lock", &self.pvd_lock())
            .field("ecc_lock", &self.ecc_lock())
            .field("sram_pef", &self.sram_pef())
            .field("pa1_cden", &self.pa1_cden())
            .field("pa3_cden", &self.pa3_cden())
            .field("pa5_cden", &self.pa5_cden())
            .field("pa6_cden", &self.pa6_cden())
            .field("pa13_cden", &self.pa13_cden())
            .field("pb0_cden", &self.pb0_cden())
            .field("pb1_cden", &self.pb1_cden())
            .field("pb2_cden", &self.pb2_cden())
            .finish()
    }
}
impl W {
    ///Bit 0 - Cortex-M0+ LOCKUP bit enable bit
    #[inline(always)]
    #[must_use]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<CFGR2rs> {
        LOCKUP_LOCK_W::new(self, 0)
    }
    ///Bit 1 - SRAM parity lock bit
    #[inline(always)]
    #[must_use]
    pub fn sram_parity_lock(&mut self) -> SRAM_PARITY_LOCK_W<CFGR2rs> {
        SRAM_PARITY_LOCK_W::new(self, 1)
    }
    ///Bit 2 - PVD lock enable bit
    #[inline(always)]
    #[must_use]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W<CFGR2rs> {
        PVD_LOCK_W::new(self, 2)
    }
    ///Bit 3 - ECC error lock bit
    #[inline(always)]
    #[must_use]
    pub fn ecc_lock(&mut self) -> ECC_LOCK_W<CFGR2rs> {
        ECC_LOCK_W::new(self, 3)
    }
    ///Bit 8 - SRAM parity error flag
    #[inline(always)]
    #[must_use]
    pub fn sram_pef(&mut self) -> SRAM_PEF_W<CFGR2rs> {
        SRAM_PEF_W::new(self, 8)
    }
    ///Bit 16 - PA1_CDEN
    #[inline(always)]
    #[must_use]
    pub fn pa1_cden(&mut self) -> PA1_CDEN_W<CFGR2rs> {
        PA1_CDEN_W::new(self, 16)
    }
    ///Bit 17 - PA3_CDEN
    #[inline(always)]
    #[must_use]
    pub fn pa3_cden(&mut self) -> PA3_CDEN_W<CFGR2rs> {
        PA3_CDEN_W::new(self, 17)
    }
    ///Bit 18 - PA5_CDEN
    #[inline(always)]
    #[must_use]
    pub fn pa5_cden(&mut self) -> PA5_CDEN_W<CFGR2rs> {
        PA5_CDEN_W::new(self, 18)
    }
    ///Bit 19 - PA6_CDEN
    #[inline(always)]
    #[must_use]
    pub fn pa6_cden(&mut self) -> PA6_CDEN_W<CFGR2rs> {
        PA6_CDEN_W::new(self, 19)
    }
    ///Bit 20 - PA13_CDEN
    #[inline(always)]
    #[must_use]
    pub fn pa13_cden(&mut self) -> PA13_CDEN_W<CFGR2rs> {
        PA13_CDEN_W::new(self, 20)
    }
    ///Bit 21 - PB0_CDEN
    #[inline(always)]
    #[must_use]
    pub fn pb0_cden(&mut self) -> PB0_CDEN_W<CFGR2rs> {
        PB0_CDEN_W::new(self, 21)
    }
    ///Bit 22 - PB1_CDEN
    #[inline(always)]
    #[must_use]
    pub fn pb1_cden(&mut self) -> PB1_CDEN_W<CFGR2rs> {
        PB1_CDEN_W::new(self, 22)
    }
    ///Bit 23 - PB2_CDEN
    #[inline(always)]
    #[must_use]
    pub fn pb2_cden(&mut self) -> PB2_CDEN_W<CFGR2rs> {
        PB2_CDEN_W::new(self, 23)
    }
}
/**SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#SYSCFG:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
