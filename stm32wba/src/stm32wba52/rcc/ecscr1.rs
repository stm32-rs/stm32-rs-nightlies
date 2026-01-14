///Register `ECSCR1` reader
pub type R = crate::R<ECSCR1rs>;
///Register `ECSCR1` writer
pub type W = crate::W<ECSCR1rs>;
///Field `HSETRIM` reader - HSE32 clock trimming These bits provide user-programmable capacitor trimming value. It can be programmed to adjust the HSE32 oscillator frequency.
pub type HSETRIM_R = crate::FieldReader;
///Field `HSETRIM` writer - HSE32 clock trimming These bits provide user-programmable capacitor trimming value. It can be programmed to adjust the HSE32 oscillator frequency.
pub type HSETRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 16:21 - HSE32 clock trimming These bits provide user-programmable capacitor trimming value. It can be programmed to adjust the HSE32 oscillator frequency.
    #[inline(always)]
    pub fn hsetrim(&self) -> HSETRIM_R {
        HSETRIM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECSCR1")
            .field("hsetrim", &self.hsetrim())
            .finish()
    }
}
impl W {
    ///Bits 16:21 - HSE32 clock trimming These bits provide user-programmable capacitor trimming value. It can be programmed to adjust the HSE32 oscillator frequency.
    #[inline(always)]
    pub fn hsetrim(&mut self) -> HSETRIM_W<'_, ECSCR1rs> {
        HSETRIM_W::new(self, 16)
    }
}
/**RCC external clock sources calibration register 1

You can [`read`](crate::Reg::read) this register and get [`ecscr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecscr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RCC:ECSCR1)*/
pub struct ECSCR1rs;
impl crate::RegisterSpec for ECSCR1rs {
    type Ux = u32;
}
///`read()` method returns [`ecscr1::R`](R) reader structure
impl crate::Readable for ECSCR1rs {}
///`write(|w| ..)` method takes [`ecscr1::W`](W) writer structure
impl crate::Writable for ECSCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ECSCR1 to value 0x0020_0000
impl crate::Resettable for ECSCR1rs {
    const RESET_VALUE: u32 = 0x0020_0000;
}
