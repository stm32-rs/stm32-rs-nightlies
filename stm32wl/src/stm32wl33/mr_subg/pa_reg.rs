///Register `PA_REG` reader
pub type R = crate::R<PA_REGrs>;
///Register `PA_REG` writer
pub type W = crate::W<PA_REGrs>;
///Field `CFG_FILT` reader - FIR configuration:
pub type CFG_FILT_R = crate::FieldReader;
///Field `CFG_FILT` writer - FIR configuration:
pub type CFG_FILT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PA_DEGEN_ON` reader - Enable a 'degeneration' mode, which introduces a pre-distortion to linearize the power control curve.
pub type PA_DEGEN_ON_R = crate::BitReader;
///Field `PA_DEGEN_ON` writer - Enable a 'degeneration' mode, which introduces a pre-distortion to linearize the power control curve.
pub type PA_DEGEN_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - FIR configuration:
    #[inline(always)]
    pub fn cfg_filt(&self) -> CFG_FILT_R {
        CFG_FILT_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - Enable a 'degeneration' mode, which introduces a pre-distortion to linearize the power control curve.
    #[inline(always)]
    pub fn pa_degen_on(&self) -> PA_DEGEN_ON_R {
        PA_DEGEN_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PA_REG")
            .field("cfg_filt", &self.cfg_filt())
            .field("pa_degen_on", &self.pa_degen_on())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FIR configuration:
    #[inline(always)]
    pub fn cfg_filt(&mut self) -> CFG_FILT_W<'_, PA_REGrs> {
        CFG_FILT_W::new(self, 0)
    }
    ///Bit 3 - Enable a 'degeneration' mode, which introduces a pre-distortion to linearize the power control curve.
    #[inline(always)]
    pub fn pa_degen_on(&mut self) -> PA_DEGEN_ON_W<'_, PA_REGrs> {
        PA_DEGEN_ON_W::new(self, 3)
    }
}
/**PA_REG register

You can [`read`](crate::Reg::read) this register and get [`pa_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:PA_REG)*/
pub struct PA_REGrs;
impl crate::RegisterSpec for PA_REGrs {
    type Ux = u32;
}
///`read()` method returns [`pa_reg::R`](R) reader structure
impl crate::Readable for PA_REGrs {}
///`write(|w| ..)` method takes [`pa_reg::W`](W) writer structure
impl crate::Writable for PA_REGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PA_REG to value 0
impl crate::Resettable for PA_REGrs {}
