///Register `TAMP_CFGR` reader
pub type R = crate::R<TAMP_CFGRrs>;
///Register `TAMP_CFGR` writer
pub type W = crate::W<TAMP_CFGRrs>;
///Field `OUT3_RMP` reader - TAMP_OUT3 mapping
pub type OUT3_RMP_R = crate::BitReader;
///Field `OUT3_RMP` writer - TAMP_OUT3 mapping
pub type OUT3_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TAMP_OUT3 mapping
    #[inline(always)]
    pub fn out3_rmp(&self) -> OUT3_RMP_R {
        OUT3_RMP_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMP_CFGR")
            .field("out3_rmp", &self.out3_rmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - TAMP_OUT3 mapping
    #[inline(always)]
    pub fn out3_rmp(&mut self) -> OUT3_RMP_W<TAMP_CFGRrs> {
        OUT3_RMP_W::new(self, 0)
    }
}
/**TAMP configuration register

You can [`read`](crate::Reg::read) this register and get [`tamp_cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#TAMP:TAMP_CFGR)*/
pub struct TAMP_CFGRrs;
impl crate::RegisterSpec for TAMP_CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`tamp_cfgr::R`](R) reader structure
impl crate::Readable for TAMP_CFGRrs {}
///`write(|w| ..)` method takes [`tamp_cfgr::W`](W) writer structure
impl crate::Writable for TAMP_CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TAMP_CFGR to value 0
impl crate::Resettable for TAMP_CFGRrs {
    const RESET_VALUE: u32 = 0;
}
