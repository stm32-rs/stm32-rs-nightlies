///Register `LTDC_L2WHPCR` reader
pub type R = crate::R<LTDC_L2WHPCRrs>;
///Register `LTDC_L2WHPCR` writer
pub type W = crate::W<LTDC_L2WHPCRrs>;
///Field `WHSTPOS` reader - WHSTPOS
pub type WHSTPOS_R = crate::FieldReader<u16>;
///Field `WHSTPOS` writer - WHSTPOS
pub type WHSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `WHSPPOS` reader - WHSPPOS
pub type WHSPPOS_R = crate::FieldReader<u16>;
///Field `WHSPPOS` writer - WHSPPOS
pub type WHSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - WHSTPOS
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - WHSPPOS
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_L2WHPCR")
            .field("whstpos", &self.whstpos())
            .field("whsppos", &self.whsppos())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - WHSTPOS
    #[inline(always)]
    #[must_use]
    pub fn whstpos(&mut self) -> WHSTPOS_W<LTDC_L2WHPCRrs> {
        WHSTPOS_W::new(self, 0)
    }
    ///Bits 16:27 - WHSPPOS
    #[inline(always)]
    #[must_use]
    pub fn whsppos(&mut self) -> WHSPPOS_W<LTDC_L2WHPCRrs> {
        WHSPPOS_W::new(self, 16)
    }
}
/**This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\[11:0\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\[11:0\]
bits in the LTDC_AWCR register.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2whpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2whpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L2WHPCR)*/
pub struct LTDC_L2WHPCRrs;
impl crate::RegisterSpec for LTDC_L2WHPCRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_l2whpcr::R`](R) reader structure
impl crate::Readable for LTDC_L2WHPCRrs {}
///`write(|w| ..)` method takes [`ltdc_l2whpcr::W`](W) writer structure
impl crate::Writable for LTDC_L2WHPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_L2WHPCR to value 0
impl crate::Resettable for LTDC_L2WHPCRrs {
    const RESET_VALUE: u32 = 0;
}
