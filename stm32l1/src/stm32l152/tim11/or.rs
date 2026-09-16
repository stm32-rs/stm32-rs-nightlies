///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `TI1_RMP` reader - TIM11 Input 1 remapping capability
pub type TI1_RMP_R = crate::FieldReader;
///Field `TI1_RMP` writer - TIM11 Input 1 remapping capability
pub type TI1_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ETR_RMP` reader - Timer11 ETR remap
pub type ETR_RMP_R = crate::BitReader;
///Field `ETR_RMP` writer - Timer11 ETR remap
pub type ETR_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TI1_RMP_RI` reader - Timer11 Input 1 remap for Routing Interface (RI)
pub type TI1_RMP_RI_R = crate::BitReader;
///Field `TI1_RMP_RI` writer - Timer11 Input 1 remap for Routing Interface (RI)
pub type TI1_RMP_RI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - TIM11 Input 1 remapping capability
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Timer11 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer11 Input 1 remap for Routing Interface (RI)
    #[inline(always)]
    pub fn ti1_rmp_ri(&self) -> TI1_RMP_RI_R {
        TI1_RMP_RI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("ti1_rmp", &self.ti1_rmp())
            .field("etr_rmp", &self.etr_rmp())
            .field("ti1_rmp_ri", &self.ti1_rmp_ri())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TIM11 Input 1 remapping capability
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<'_, ORrs> {
        TI1_RMP_W::new(self, 0)
    }
    ///Bit 2 - Timer11 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<'_, ORrs> {
        ETR_RMP_W::new(self, 2)
    }
    ///Bit 3 - Timer11 Input 1 remap for Routing Interface (RI)
    #[inline(always)]
    pub fn ti1_rmp_ri(&mut self) -> TI1_RMP_RI_W<'_, ORrs> {
        TI1_RMP_RI_W::new(self, 3)
    }
}
/**option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#TIM11:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u16;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
