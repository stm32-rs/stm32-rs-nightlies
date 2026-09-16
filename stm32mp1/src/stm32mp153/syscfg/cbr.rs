///Register `CBR` reader
pub type R = crate::R<CBRrs>;
///Register `CBR` writer
pub type W = crate::W<CBRrs>;
///Field `CLL` reader - CLL
pub type CLL_R = crate::BitReader;
///Field `CLL` writer - CLL
pub type CLL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDL` reader - PVDL
pub type PVDL_R = crate::BitReader;
///Field `PVDL` writer - PVDL
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CLL
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - PVDL
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CBR")
            .field("cll", &self.cll())
            .field("pvdl", &self.pvdl())
            .finish()
    }
}
impl W {
    ///Bit 0 - CLL
    #[inline(always)]
    pub fn cll(&mut self) -> CLL_W<'_, CBRrs> {
        CLL_W::new(self, 0)
    }
    ///Bit 2 - PVDL
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W<'_, CBRrs> {
        PVDL_W::new(self, 2)
    }
}
/**SYSCFG control timer break register

You can [`read`](crate::Reg::read) this register and get [`cbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:CBR)*/
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
