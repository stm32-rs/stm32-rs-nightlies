///Register `L2WHPCR` reader
pub type R = crate::R<L2WHPCRrs>;
///Register `L2WHPCR` writer
pub type W = crate::W<L2WHPCRrs>;
///Field `WHSTPOS` reader - window horizontal start position These bits configure the first visible pixel of a line of the layer window. WHSTPOS\[11:0\] must be ≤ AAW\[11:0\] bits (programmed in LTDC_AWCR register).
pub type WHSTPOS_R = crate::FieldReader<u16>;
///Field `WHSTPOS` writer - window horizontal start position These bits configure the first visible pixel of a line of the layer window. WHSTPOS\[11:0\] must be ≤ AAW\[11:0\] bits (programmed in LTDC_AWCR register).
pub type WHSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `WHSPPOS` reader - window horizontal stop position These bits configure the last visible pixel of a line of the layer window. WHSPPOS\[11:0\] must be ≥ AHBP\[11:0\] bits + 1 (programmed in LTDC_BPCR register).
pub type WHSPPOS_R = crate::FieldReader<u16>;
///Field `WHSPPOS` writer - window horizontal stop position These bits configure the last visible pixel of a line of the layer window. WHSPPOS\[11:0\] must be ≥ AHBP\[11:0\] bits + 1 (programmed in LTDC_BPCR register).
pub type WHSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - window horizontal start position These bits configure the first visible pixel of a line of the layer window. WHSTPOS\[11:0\] must be ≤ AAW\[11:0\] bits (programmed in LTDC_AWCR register).
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - window horizontal stop position These bits configure the last visible pixel of a line of the layer window. WHSPPOS\[11:0\] must be ≥ AHBP\[11:0\] bits + 1 (programmed in LTDC_BPCR register).
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2WHPCR")
            .field("whstpos", &self.whstpos())
            .field("whsppos", &self.whsppos())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - window horizontal start position These bits configure the first visible pixel of a line of the layer window. WHSTPOS\[11:0\] must be ≤ AAW\[11:0\] bits (programmed in LTDC_AWCR register).
    #[inline(always)]
    pub fn whstpos(&mut self) -> WHSTPOS_W<L2WHPCRrs> {
        WHSTPOS_W::new(self, 0)
    }
    ///Bits 16:27 - window horizontal stop position These bits configure the last visible pixel of a line of the layer window. WHSPPOS\[11:0\] must be ≥ AHBP\[11:0\] bits + 1 (programmed in LTDC_BPCR register).
    #[inline(always)]
    pub fn whsppos(&mut self) -> WHSPPOS_W<L2WHPCRrs> {
        WHSPPOS_W::new(self, 16)
    }
}
/**LTDC layer 2 window horizontal position configuration register

You can [`read`](crate::Reg::read) this register and get [`l2whpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2whpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:L2WHPCR)*/
pub struct L2WHPCRrs;
impl crate::RegisterSpec for L2WHPCRrs {
    type Ux = u32;
}
///`read()` method returns [`l2whpcr::R`](R) reader structure
impl crate::Readable for L2WHPCRrs {}
///`write(|w| ..)` method takes [`l2whpcr::W`](W) writer structure
impl crate::Writable for L2WHPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2WHPCR to value 0
impl crate::Resettable for L2WHPCRrs {}
