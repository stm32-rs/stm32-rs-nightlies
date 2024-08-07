///Register `SYSCFG_CBR` reader
pub type R = crate::R<SYSCFG_CBRrs>;
///Register `SYSCFG_CBR` writer
pub type W = crate::W<SYSCFG_CBRrs>;
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
        f.debug_struct("SYSCFG_CBR")
            .field("cll", &self.cll())
            .field("pvdl", &self.pvdl())
            .finish()
    }
}
impl W {
    ///Bit 0 - CLL
    #[inline(always)]
    #[must_use]
    pub fn cll(&mut self) -> CLL_W<SYSCFG_CBRrs> {
        CLL_W::new(self, 0)
    }
    ///Bit 2 - PVDL
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<SYSCFG_CBRrs> {
        PVDL_W::new(self, 2)
    }
}
/**SYSCFG control timer break register

You can [`read`](crate::Reg::read) this register and get [`syscfg_cbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_CBR)*/
pub struct SYSCFG_CBRrs;
impl crate::RegisterSpec for SYSCFG_CBRrs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_cbr::R`](R) reader structure
impl crate::Readable for SYSCFG_CBRrs {}
///`write(|w| ..)` method takes [`syscfg_cbr::W`](W) writer structure
impl crate::Writable for SYSCFG_CBRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SYSCFG_CBR to value 0
impl crate::Resettable for SYSCFG_CBRrs {
    const RESET_VALUE: u32 = 0;
}
