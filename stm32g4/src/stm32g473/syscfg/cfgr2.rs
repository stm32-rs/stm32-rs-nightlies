///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `CLL` reader - Core Lockup Lock
pub type CLL_R = crate::BitReader;
///Field `CLL` writer - Core Lockup Lock
pub type CLL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPL` reader - SRAM Parity Lock
pub type SPL_R = crate::BitReader;
///Field `SPL` writer - SRAM Parity Lock
pub type SPL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDL` reader - PVD Lock
pub type PVDL_R = crate::BitReader;
///Field `PVDL` writer - PVD Lock
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCL` reader - ECC Lock
pub type ECCL_R = crate::BitReader;
///Field `ECCL` writer - ECC Lock
pub type ECCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPF` reader - SRAM Parity Flag
pub type SPF_R = crate::BitReader;
///Field `SPF` writer - SRAM Parity Flag
pub type SPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Core Lockup Lock
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM Parity Lock
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PVD Lock
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC Lock
    #[inline(always)]
    pub fn eccl(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - SRAM Parity Flag
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("cll", &self.cll())
            .field("spl", &self.spl())
            .field("pvdl", &self.pvdl())
            .field("eccl", &self.eccl())
            .field("spf", &self.spf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Core Lockup Lock
    #[inline(always)]
    pub fn cll(&mut self) -> CLL_W<'_, CFGR2rs> {
        CLL_W::new(self, 0)
    }
    ///Bit 1 - SRAM Parity Lock
    #[inline(always)]
    pub fn spl(&mut self) -> SPL_W<'_, CFGR2rs> {
        SPL_W::new(self, 1)
    }
    ///Bit 2 - PVD Lock
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W<'_, CFGR2rs> {
        PVDL_W::new(self, 2)
    }
    ///Bit 3 - ECC Lock
    #[inline(always)]
    pub fn eccl(&mut self) -> ECCL_W<'_, CFGR2rs> {
        ECCL_W::new(self, 3)
    }
    ///Bit 8 - SRAM Parity Flag
    #[inline(always)]
    pub fn spf(&mut self) -> SPF_W<'_, CFGR2rs> {
        SPF_W::new(self, 8)
    }
}
/**configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#SYSCFG:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
