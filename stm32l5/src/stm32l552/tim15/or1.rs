///Register `OR1` reader
pub type R = crate::R<OR1rs>;
///Register `OR1` writer
pub type W = crate::W<OR1rs>;
///Field `TI1_RMP` reader - Input capture 1 remap
pub type TI1_RMP_R = crate::BitReader;
///Field `TI1_RMP` writer - Input capture 1 remap
pub type TI1_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENCODER_MODE` reader - Encoder mode
pub type ENCODER_MODE_R = crate::FieldReader;
///Field `ENCODER_MODE` writer - Encoder mode
pub type ENCODER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Input capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Encoder mode
    #[inline(always)]
    pub fn encoder_mode(&self) -> ENCODER_MODE_R {
        ENCODER_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("encoder_mode", &self.encoder_mode())
            .field("ti1_rmp", &self.ti1_rmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Input capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<'_, OR1rs> {
        TI1_RMP_W::new(self, 0)
    }
    ///Bits 1:2 - Encoder mode
    #[inline(always)]
    pub fn encoder_mode(&mut self) -> ENCODER_MODE_W<'_, OR1rs> {
        ENCODER_MODE_W::new(self, 1)
    }
}
/**TIM15 option register 1

You can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#TIM15:OR1)*/
pub struct OR1rs;
impl crate::RegisterSpec for OR1rs {
    type Ux = u32;
}
///`read()` method returns [`or1::R`](R) reader structure
impl crate::Readable for OR1rs {}
///`write(|w| ..)` method takes [`or1::W`](W) writer structure
impl crate::Writable for OR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR1 to value 0
impl crate::Resettable for OR1rs {}
