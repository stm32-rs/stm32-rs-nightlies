///Register `M2_FN_MOD` reader
pub type R = crate::R<M2_FN_MODrs>;
///Register `M2_FN_MOD` writer
pub type W = crate::W<M2_FN_MODrs>;
///Field `READ_ISS_OVERRIDE` reader - READ_ISS_OVERRIDE
pub type READ_ISS_OVERRIDE_R = crate::BitReader;
///Field `READ_ISS_OVERRIDE` writer - READ_ISS_OVERRIDE
pub type READ_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRITE_ISS_OVERRIDE` reader - WRITE_ISS_OVERRIDE
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader;
///Field `WRITE_ISS_OVERRIDE` writer - WRITE_ISS_OVERRIDE
pub type WRITE_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - READ_ISS_OVERRIDE
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WRITE_ISS_OVERRIDE
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2_FN_MOD")
            .field("read_iss_override", &self.read_iss_override())
            .field("write_iss_override", &self.write_iss_override())
            .finish()
    }
}
impl W {
    ///Bit 0 - READ_ISS_OVERRIDE
    #[inline(always)]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<M2_FN_MODrs> {
        READ_ISS_OVERRIDE_W::new(self, 0)
    }
    ///Bit 1 - WRITE_ISS_OVERRIDE
    #[inline(always)]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<M2_FN_MODrs> {
        WRITE_ISS_OVERRIDE_W::new(self, 1)
    }
}
/**AXIMC master 2 issuing capability override functionality register

You can [`read`](crate::Reg::read) this register and get [`m2_fn_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2_fn_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#AXIMC_Mx:M2_FN_MOD)*/
pub struct M2_FN_MODrs;
impl crate::RegisterSpec for M2_FN_MODrs {
    type Ux = u32;
}
///`read()` method returns [`m2_fn_mod::R`](R) reader structure
impl crate::Readable for M2_FN_MODrs {}
///`write(|w| ..)` method takes [`m2_fn_mod::W`](W) writer structure
impl crate::Writable for M2_FN_MODrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M2_FN_MOD to value 0
impl crate::Resettable for M2_FN_MODrs {
    const RESET_VALUE: u32 = 0;
}