///Register `PERFLPR1` reader
pub type R = crate::R<PERFLPR1rs>;
///Register `PERFLPR1` writer
pub type W = crate::W<PERFLPR1rs>;
///Field `LPR_MAX_STARVE` reader - LPR_MAX_STARVE
pub type LPR_MAX_STARVE_R = crate::FieldReader<u16>;
///Field `LPR_MAX_STARVE` writer - LPR_MAX_STARVE
pub type LPR_MAX_STARVE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `LPR_XACT_RUN_LENGTH` reader - LPR_XACT_RUN_LENGTH
pub type LPR_XACT_RUN_LENGTH_R = crate::FieldReader;
///Field `LPR_XACT_RUN_LENGTH` writer - LPR_XACT_RUN_LENGTH
pub type LPR_XACT_RUN_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:15 - LPR_MAX_STARVE
    #[inline(always)]
    pub fn lpr_max_starve(&self) -> LPR_MAX_STARVE_R {
        LPR_MAX_STARVE_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:31 - LPR_XACT_RUN_LENGTH
    #[inline(always)]
    pub fn lpr_xact_run_length(&self) -> LPR_XACT_RUN_LENGTH_R {
        LPR_XACT_RUN_LENGTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERFLPR1")
            .field("lpr_max_starve", &self.lpr_max_starve())
            .field("lpr_xact_run_length", &self.lpr_xact_run_length())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - LPR_MAX_STARVE
    #[inline(always)]
    pub fn lpr_max_starve(&mut self) -> LPR_MAX_STARVE_W<'_, PERFLPR1rs> {
        LPR_MAX_STARVE_W::new(self, 0)
    }
    ///Bits 24:31 - LPR_XACT_RUN_LENGTH
    #[inline(always)]
    pub fn lpr_xact_run_length(&mut self) -> LPR_XACT_RUN_LENGTH_W<'_, PERFLPR1rs> {
        LPR_XACT_RUN_LENGTH_W::new(self, 24)
    }
}
/**DDRCTRL low priority read CAM register 1

You can [`read`](crate::Reg::read) this register and get [`perflpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perflpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:PERFLPR1)*/
pub struct PERFLPR1rs;
impl crate::RegisterSpec for PERFLPR1rs {
    type Ux = u32;
}
///`read()` method returns [`perflpr1::R`](R) reader structure
impl crate::Readable for PERFLPR1rs {}
///`write(|w| ..)` method takes [`perflpr1::W`](W) writer structure
impl crate::Writable for PERFLPR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PERFLPR1 to value 0x0f00_007f
impl crate::Resettable for PERFLPR1rs {
    const RESET_VALUE: u32 = 0x0f00_007f;
}
