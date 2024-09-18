///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `ETR_RMP` reader - Timer2 ETR remap
pub type ETR_RMP_R = crate::FieldReader;
///Field `ETR_RMP` writer - Timer2 ETR remap
pub type ETR_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TI4_RMP` reader - Internal trigger
pub type TI4_RMP_R = crate::FieldReader;
///Field `TI4_RMP` writer - Internal trigger
pub type TI4_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2 - Timer2 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - Internal trigger
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("etr_rmp", &self.etr_rmp())
            .field("ti4_rmp", &self.ti4_rmp())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Timer2 ETR remap
    #[inline(always)]
    #[must_use]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<ORrs> {
        ETR_RMP_W::new(self, 0)
    }
    ///Bits 3:4 - Internal trigger
    #[inline(always)]
    #[must_use]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W<ORrs> {
        TI4_RMP_W::new(self, 3)
    }
}
/**TIM2 option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#TIM3:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {
    const RESET_VALUE: u32 = 0;
}
