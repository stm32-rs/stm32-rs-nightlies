///Register `PERFWR1` reader
pub type R = crate::R<PERFWR1rs>;
///Register `PERFWR1` writer
pub type W = crate::W<PERFWR1rs>;
///Field `W_MAX_STARVE` reader - W_MAX_STARVE
pub type W_MAX_STARVE_R = crate::FieldReader<u16>;
///Field `W_MAX_STARVE` writer - W_MAX_STARVE
pub type W_MAX_STARVE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `W_XACT_RUN_LENGTH` reader - W_XACT_RUN_LENGTH
pub type W_XACT_RUN_LENGTH_R = crate::FieldReader;
///Field `W_XACT_RUN_LENGTH` writer - W_XACT_RUN_LENGTH
pub type W_XACT_RUN_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:15 - W_MAX_STARVE
    #[inline(always)]
    pub fn w_max_starve(&self) -> W_MAX_STARVE_R {
        W_MAX_STARVE_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:31 - W_XACT_RUN_LENGTH
    #[inline(always)]
    pub fn w_xact_run_length(&self) -> W_XACT_RUN_LENGTH_R {
        W_XACT_RUN_LENGTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERFWR1")
            .field("w_max_starve", &self.w_max_starve())
            .field("w_xact_run_length", &self.w_xact_run_length())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - W_MAX_STARVE
    #[inline(always)]
    pub fn w_max_starve(&mut self) -> W_MAX_STARVE_W<'_, PERFWR1rs> {
        W_MAX_STARVE_W::new(self, 0)
    }
    ///Bits 24:31 - W_XACT_RUN_LENGTH
    #[inline(always)]
    pub fn w_xact_run_length(&mut self) -> W_XACT_RUN_LENGTH_W<'_, PERFWR1rs> {
        W_XACT_RUN_LENGTH_W::new(self, 24)
    }
}
/**DDRCTRL write CAM register 1

You can [`read`](crate::Reg::read) this register and get [`perfwr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perfwr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PERFWR1)*/
pub struct PERFWR1rs;
impl crate::RegisterSpec for PERFWR1rs {
    type Ux = u32;
}
///`read()` method returns [`perfwr1::R`](R) reader structure
impl crate::Readable for PERFWR1rs {}
///`write(|w| ..)` method takes [`perfwr1::W`](W) writer structure
impl crate::Writable for PERFWR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PERFWR1 to value 0x0f00_007f
impl crate::Resettable for PERFWR1rs {
    const RESET_VALUE: u32 = 0x0f00_007f;
}
