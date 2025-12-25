///Register `TARG6_FN_MOD_ISS_BM` reader
pub type R = crate::R<TARG6_FN_MOD_ISS_BMrs>;
///Register `TARG6_FN_MOD_ISS_BM` writer
pub type W = crate::W<TARG6_FN_MOD_ISS_BMrs>;
///Field `READ_ISS_OVERRIDE` reader - READ_ISS_OVERRIDE
pub type READ_ISS_OVERRIDE_R = crate::BitReader;
///Field `READ_ISS_OVERRIDE` writer - READ_ISS_OVERRIDE
pub type READ_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRITE_ISS_OVERRIDE` reader - Switch matrix write issuing override for target
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader;
///Field `WRITE_ISS_OVERRIDE` writer - Switch matrix write issuing override for target
pub type WRITE_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - READ_ISS_OVERRIDE
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Switch matrix write issuing override for target
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARG6_FN_MOD_ISS_BM")
            .field("read_iss_override", &self.read_iss_override())
            .field("write_iss_override", &self.write_iss_override())
            .finish()
    }
}
impl W {
    ///Bit 0 - READ_ISS_OVERRIDE
    #[inline(always)]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<'_, TARG6_FN_MOD_ISS_BMrs> {
        READ_ISS_OVERRIDE_W::new(self, 0)
    }
    ///Bit 1 - Switch matrix write issuing override for target
    #[inline(always)]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<'_, TARG6_FN_MOD_ISS_BMrs> {
        WRITE_ISS_OVERRIDE_W::new(self, 1)
    }
}
/**AXI interconnect - TARG x bus matrix issuing functionality register

You can [`read`](crate::Reg::read) this register and get [`targ6_fn_mod_iss_bm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`targ6_fn_mod_iss_bm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#AXI:TARG6_FN_MOD_ISS_BM)*/
pub struct TARG6_FN_MOD_ISS_BMrs;
impl crate::RegisterSpec for TARG6_FN_MOD_ISS_BMrs {
    type Ux = u32;
}
///`read()` method returns [`targ6_fn_mod_iss_bm::R`](R) reader structure
impl crate::Readable for TARG6_FN_MOD_ISS_BMrs {}
///`write(|w| ..)` method takes [`targ6_fn_mod_iss_bm::W`](W) writer structure
impl crate::Writable for TARG6_FN_MOD_ISS_BMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TARG6_FN_MOD_ISS_BM to value 0x04
impl crate::Resettable for TARG6_FN_MOD_ISS_BMrs {
    const RESET_VALUE: u32 = 0x04;
}
