///Register `LTDC_L1BFCR` reader
pub type R = crate::R<LTDC_L1BFCRrs>;
///Register `LTDC_L1BFCR` writer
pub type W = crate::W<LTDC_L1BFCRrs>;
///Field `BF2` reader - BF2
pub type BF2_R = crate::FieldReader;
///Field `BF2` writer - BF2
pub type BF2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BF1` reader - BF1
pub type BF1_R = crate::FieldReader;
///Field `BF1` writer - BF1
pub type BF1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - BF2
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - BF1
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_L1BFCR")
            .field("bf2", &self.bf2())
            .field("bf1", &self.bf1())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - BF2
    #[inline(always)]
    #[must_use]
    pub fn bf2(&mut self) -> BF2_W<LTDC_L1BFCRrs> {
        BF2_W::new(self, 0)
    }
    ///Bits 8:10 - BF1
    #[inline(always)]
    #[must_use]
    pub fn bf1(&mut self) -> BF1_W<LTDC_L1BFCRrs> {
        BF1_W::new(self, 8)
    }
}
/**This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1bfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1bfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1BFCR)*/
pub struct LTDC_L1BFCRrs;
impl crate::RegisterSpec for LTDC_L1BFCRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_l1bfcr::R`](R) reader structure
impl crate::Readable for LTDC_L1BFCRrs {}
///`write(|w| ..)` method takes [`ltdc_l1bfcr::W`](W) writer structure
impl crate::Writable for LTDC_L1BFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_L1BFCR to value 0x0607
impl crate::Resettable for LTDC_L1BFCRrs {
    const RESET_VALUE: u32 = 0x0607;
}
