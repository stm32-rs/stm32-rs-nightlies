///Register `CBR` reader
pub type R = crate::R<CBRrs>;
///Register `CBR` writer
pub type W = crate::W<CBRrs>;
///Field `CM55L` reader - CM55 lockup lock enable
pub type CM55L_R = crate::BitReader;
///Field `CM55L` writer - CM55 lockup lock enable
pub type CM55L_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDL_LOCK` reader - PVD lock enable
pub type PVDL_LOCK_R = crate::BitReader;
///Field `PVDL_LOCK` writer - PVD lock enable
pub type PVDL_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPRAML` reader - Backup SRAM double ECC error lock
pub type BKPRAML_R = crate::BitReader;
///Field `BKPRAML` writer - Backup SRAM double ECC error lock
pub type BKPRAML_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CM55CACHEL` reader - CM55 cache double ECC error lock
pub type CM55CACHEL_R = crate::BitReader;
///Field `CM55CACHEL` writer - CM55 cache double ECC error lock
pub type CM55CACHEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CM55TCML` reader - CM55 TCM double ECC error lock
pub type CM55TCML_R = crate::BitReader;
///Field `CM55TCML` writer - CM55 TCM double ECC error lock
pub type CM55TCML_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CM55 lockup lock enable
    #[inline(always)]
    pub fn cm55l(&self) -> CM55L_R {
        CM55L_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - PVD lock enable
    #[inline(always)]
    pub fn pvdl_lock(&self) -> PVDL_LOCK_R {
        PVDL_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Backup SRAM double ECC error lock
    #[inline(always)]
    pub fn bkpraml(&self) -> BKPRAML_R {
        BKPRAML_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - CM55 cache double ECC error lock
    #[inline(always)]
    pub fn cm55cachel(&self) -> CM55CACHEL_R {
        CM55CACHEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CM55 TCM double ECC error lock
    #[inline(always)]
    pub fn cm55tcml(&self) -> CM55TCML_R {
        CM55TCML_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CBR")
            .field("cm55l", &self.cm55l())
            .field("pvdl_lock", &self.pvdl_lock())
            .field("bkpraml", &self.bkpraml())
            .field("cm55cachel", &self.cm55cachel())
            .field("cm55tcml", &self.cm55tcml())
            .finish()
    }
}
impl W {
    ///Bit 0 - CM55 lockup lock enable
    #[inline(always)]
    pub fn cm55l(&mut self) -> CM55L_W<'_, CBRrs> {
        CM55L_W::new(self, 0)
    }
    ///Bit 2 - PVD lock enable
    #[inline(always)]
    pub fn pvdl_lock(&mut self) -> PVDL_LOCK_W<'_, CBRrs> {
        PVDL_LOCK_W::new(self, 2)
    }
    ///Bit 3 - Backup SRAM double ECC error lock
    #[inline(always)]
    pub fn bkpraml(&mut self) -> BKPRAML_W<'_, CBRrs> {
        BKPRAML_W::new(self, 3)
    }
    ///Bit 5 - CM55 cache double ECC error lock
    #[inline(always)]
    pub fn cm55cachel(&mut self) -> CM55CACHEL_W<'_, CBRrs> {
        CM55CACHEL_W::new(self, 5)
    }
    ///Bit 6 - CM55 TCM double ECC error lock
    #[inline(always)]
    pub fn cm55tcml(&mut self) -> CM55TCML_W<'_, CBRrs> {
        CM55TCML_W::new(self, 6)
    }
}
/**SYSCFG control timer break register

You can [`read`](crate::Reg::read) this register and get [`cbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:CBR)*/
pub struct CBRrs;
impl crate::RegisterSpec for CBRrs {
    type Ux = u32;
}
///`read()` method returns [`cbr::R`](R) reader structure
impl crate::Readable for CBRrs {}
///`write(|w| ..)` method takes [`cbr::W`](W) writer structure
impl crate::Writable for CBRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CBR to value 0
impl crate::Resettable for CBRrs {}
