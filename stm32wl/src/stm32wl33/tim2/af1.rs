///Register `AF1` reader
pub type R = crate::R<AF1rs>;
///Register `AF1` writer
pub type W = crate::W<AF1rs>;
///Field `ETR_SEL` reader - ETRSEL\[2:0\]: External trigger source selection 000: TIMx External trigger legacy mode 001: TIMx External trigger source select COMP1_OUT Other: Reserved Note: These bits can't be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type ETR_SEL_R = crate::FieldReader;
///Field `ETR_SEL` writer - ETRSEL\[2:0\]: External trigger source selection 000: TIMx External trigger legacy mode 001: TIMx External trigger source select COMP1_OUT Other: Reserved Note: These bits can't be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
pub type ETR_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ETR_SEL_3` reader - ETRSEL\[2:0\]: External trigger source selection This field is not used in Blue51. Not available in IUM
pub type ETR_SEL_3_R = crate::BitReader;
///Field `ETR_SEL_3` writer - ETRSEL\[2:0\]: External trigger source selection This field is not used in Blue51. Not available in IUM
pub type ETR_SEL_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 14:16 - ETRSEL\[2:0\]: External trigger source selection 000: TIMx External trigger legacy mode 001: TIMx External trigger source select COMP1_OUT Other: Reserved Note: These bits can't be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn etr_sel(&self) -> ETR_SEL_R {
        ETR_SEL_R::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bit 17 - ETRSEL\[2:0\]: External trigger source selection This field is not used in Blue51. Not available in IUM
    #[inline(always)]
    pub fn etr_sel_3(&self) -> ETR_SEL_3_R {
        ETR_SEL_3_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF1")
            .field("etr_sel", &self.etr_sel())
            .field("etr_sel_3", &self.etr_sel_3())
            .finish()
    }
}
impl W {
    ///Bits 14:16 - ETRSEL\[2:0\]: External trigger source selection 000: TIMx External trigger legacy mode 001: TIMx External trigger source select COMP1_OUT Other: Reserved Note: These bits can't be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)
    #[inline(always)]
    pub fn etr_sel(&mut self) -> ETR_SEL_W<'_, AF1rs> {
        ETR_SEL_W::new(self, 14)
    }
    ///Bit 17 - ETRSEL\[2:0\]: External trigger source selection This field is not used in Blue51. Not available in IUM
    #[inline(always)]
    pub fn etr_sel_3(&mut self) -> ETR_SEL_3_W<'_, AF1rs> {
        ETR_SEL_3_W::new(self, 17)
    }
}
/**AF1 register

You can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM2:AF1)*/
pub struct AF1rs;
impl crate::RegisterSpec for AF1rs {
    type Ux = u32;
}
///`read()` method returns [`af1::R`](R) reader structure
impl crate::Readable for AF1rs {}
///`write(|w| ..)` method takes [`af1::W`](W) writer structure
impl crate::Writable for AF1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AF1 to value 0
impl crate::Resettable for AF1rs {}
