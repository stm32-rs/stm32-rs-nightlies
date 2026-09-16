///Register `PERFHPR1` reader
pub type R = crate::R<PERFHPR1rs>;
///Register `PERFHPR1` writer
pub type W = crate::W<PERFHPR1rs>;
///Field `HPR_MAX_STARVE` reader - HPR_MAX_STARVE
pub type HPR_MAX_STARVE_R = crate::FieldReader<u16>;
///Field `HPR_MAX_STARVE` writer - HPR_MAX_STARVE
pub type HPR_MAX_STARVE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `HPR_XACT_RUN_LENGTH` reader - HPR_XACT_RUN_LENGTH
pub type HPR_XACT_RUN_LENGTH_R = crate::FieldReader;
///Field `HPR_XACT_RUN_LENGTH` writer - HPR_XACT_RUN_LENGTH
pub type HPR_XACT_RUN_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:15 - HPR_MAX_STARVE
    #[inline(always)]
    pub fn hpr_max_starve(&self) -> HPR_MAX_STARVE_R {
        HPR_MAX_STARVE_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:31 - HPR_XACT_RUN_LENGTH
    #[inline(always)]
    pub fn hpr_xact_run_length(&self) -> HPR_XACT_RUN_LENGTH_R {
        HPR_XACT_RUN_LENGTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERFHPR1")
            .field("hpr_max_starve", &self.hpr_max_starve())
            .field("hpr_xact_run_length", &self.hpr_xact_run_length())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - HPR_MAX_STARVE
    #[inline(always)]
    pub fn hpr_max_starve(&mut self) -> HPR_MAX_STARVE_W<'_, PERFHPR1rs> {
        HPR_MAX_STARVE_W::new(self, 0)
    }
    ///Bits 24:31 - HPR_XACT_RUN_LENGTH
    #[inline(always)]
    pub fn hpr_xact_run_length(&mut self) -> HPR_XACT_RUN_LENGTH_W<'_, PERFHPR1rs> {
        HPR_XACT_RUN_LENGTH_W::new(self, 24)
    }
}
/**DDRCTRL high priority read CAM register 1

You can [`read`](crate::Reg::read) this register and get [`perfhpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perfhpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:PERFHPR1)*/
pub struct PERFHPR1rs;
impl crate::RegisterSpec for PERFHPR1rs {
    type Ux = u32;
}
///`read()` method returns [`perfhpr1::R`](R) reader structure
impl crate::Readable for PERFHPR1rs {}
///`write(|w| ..)` method takes [`perfhpr1::W`](W) writer structure
impl crate::Writable for PERFHPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PERFHPR1 to value 0x0f00_0001
impl crate::Resettable for PERFHPR1rs {
    const RESET_VALUE: u32 = 0x0f00_0001;
}
