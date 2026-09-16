///Register `SNYTH_FREQ` reader
pub type R = crate::R<SNYTH_FREQrs>;
///Register `SNYTH_FREQ` writer
pub type W = crate::W<SNYTH_FREQrs>;
///Field `SYNTH_FRAC` reader - Fractional part of the PLL fractional divide factor (default: 868 MHz, XTAL: 48 MHz)
pub type SYNTH_FRAC_R = crate::FieldReader<u32>;
///Field `SYNTH_FRAC` writer - Fractional part of the PLL fractional divide factor (default: 868 MHz, XTAL: 48 MHz)
pub type SYNTH_FRAC_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
///Field `SYNTH_INT` reader - PLL integer divide factor (default: 868 MHz, XTAL: 48 MHz)
pub type SYNTH_INT_R = crate::FieldReader;
///Field `SYNTH_INT` writer - PLL integer divide factor (default: 868 MHz, XTAL: 48 MHz)
pub type SYNTH_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BS` reader - Synthesizer band selector, i.
pub type BS_R = crate::BitReader;
///Field `BS` writer - Synthesizer band selector, i.
pub type BS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:19 - Fractional part of the PLL fractional divide factor (default: 868 MHz, XTAL: 48 MHz)
    #[inline(always)]
    pub fn synth_frac(&self) -> SYNTH_FRAC_R {
        SYNTH_FRAC_R::new(self.bits & 0x000f_ffff)
    }
    ///Bits 20:27 - PLL integer divide factor (default: 868 MHz, XTAL: 48 MHz)
    #[inline(always)]
    pub fn synth_int(&self) -> SYNTH_INT_R {
        SYNTH_INT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bit 30 - Synthesizer band selector, i.
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNYTH_FREQ")
            .field("synth_frac", &self.synth_frac())
            .field("synth_int", &self.synth_int())
            .field("bs", &self.bs())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - Fractional part of the PLL fractional divide factor (default: 868 MHz, XTAL: 48 MHz)
    #[inline(always)]
    pub fn synth_frac(&mut self) -> SYNTH_FRAC_W<'_, SNYTH_FREQrs> {
        SYNTH_FRAC_W::new(self, 0)
    }
    ///Bits 20:27 - PLL integer divide factor (default: 868 MHz, XTAL: 48 MHz)
    #[inline(always)]
    pub fn synth_int(&mut self) -> SYNTH_INT_W<'_, SNYTH_FREQrs> {
        SYNTH_INT_W::new(self, 20)
    }
    ///Bit 30 - Synthesizer band selector, i.
    #[inline(always)]
    pub fn bs(&mut self) -> BS_W<'_, SNYTH_FREQrs> {
        BS_W::new(self, 30)
    }
}
/**SNYTH_FREQ register

You can [`read`](crate::Reg::read) this register and get [`snyth_freq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snyth_freq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:SNYTH_FREQ)*/
pub struct SNYTH_FREQrs;
impl crate::RegisterSpec for SNYTH_FREQrs {
    type Ux = u32;
}
///`read()` method returns [`snyth_freq::R`](R) reader structure
impl crate::Readable for SNYTH_FREQrs {}
///`write(|w| ..)` method takes [`snyth_freq::W`](W) writer structure
impl crate::Writable for SNYTH_FREQrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SNYTH_FREQ to value 0x0485_1615
impl crate::Resettable for SNYTH_FREQrs {
    const RESET_VALUE: u32 = 0x0485_1615;
}
