///Register `OR1` reader
pub type R = crate::R<OR1rs>;
///Register `OR1` writer
pub type W = crate::W<OR1rs>;
///Field `ITR1_RMP` reader - Internal trigger 1 remap
pub type ITR1_RMP_R = crate::BitReader;
///Field `ITR1_RMP` writer - Internal trigger 1 remap
pub type ITR1_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETR_RMP` reader - External trigger remap
pub type ETR_RMP_R = crate::BitReader;
///Field `ETR_RMP` writer - External trigger remap
pub type ETR_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TI4_RMP` reader - Input Capture 4 remap
pub type TI4_RMP_R = crate::FieldReader;
///Field `TI4_RMP` writer - Input Capture 4 remap
pub type TI4_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Internal trigger 1 remap
    #[inline(always)]
    pub fn itr1_rmp(&self) -> ITR1_RMP_R {
        ITR1_RMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - External trigger remap
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Input Capture 4 remap
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("itr1_rmp", &self.itr1_rmp())
            .field("etr_rmp", &self.etr_rmp())
            .field("ti4_rmp", &self.ti4_rmp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Internal trigger 1 remap
    #[inline(always)]
    pub fn itr1_rmp(&mut self) -> ITR1_RMP_W<'_, OR1rs> {
        ITR1_RMP_W::new(self, 0)
    }
    ///Bit 1 - External trigger remap
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<'_, OR1rs> {
        ETR_RMP_W::new(self, 1)
    }
    ///Bits 2:3 - Input Capture 4 remap
    #[inline(always)]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W<'_, OR1rs> {
        TI4_RMP_W::new(self, 2)
    }
}
/**TIM2 option register 1

You can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#TIM2:OR1)*/
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
