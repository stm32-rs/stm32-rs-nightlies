///Register `L2WVPCR` reader
pub type R = crate::R<L2WVPCRrs>;
///Register `L2WVPCR` writer
pub type W = crate::W<L2WVPCRrs>;
/**Field `WVSTPOS` reader - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\[10:0\]
must be ≤ AAH\[10:0\]
bits (programmed in LTDC_AWCR register).*/
pub type WVSTPOS_R = crate::FieldReader<u16>;
/**Field `WVSTPOS` writer - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\[10:0\]
must be ≤ AAH\[10:0\]
bits (programmed in LTDC_AWCR register).*/
pub type WVSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
/**Field `WVSPPOS` reader - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\[10:0\]
must be ≥ AVBP\[10:0\]
bits + 1 (programmed in LTDC_BPCR register).*/
pub type WVSPPOS_R = crate::FieldReader<u16>;
/**Field `WVSPPOS` writer - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\[10:0\]
must be ≥ AVBP\[10:0\]
bits + 1 (programmed in LTDC_BPCR register).*/
pub type WVSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    /**Bits 0:10 - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\[10:0\]
    must be ≤ AAH\[10:0\]
    bits (programmed in LTDC_AWCR register).*/
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x07ff) as u16)
    }
    /**Bits 16:26 - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\[10:0\]
    must be ≥ AVBP\[10:0\]
    bits + 1 (programmed in LTDC_BPCR register).*/
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2WVPCR")
            .field("wvstpos", &self.wvstpos())
            .field("wvsppos", &self.wvsppos())
            .finish()
    }
}
impl W {
    /**Bits 0:10 - window vertical start position These bits configure the first visible line of the layer window. WVSTPOS\[10:0\]
    must be ≤ AAH\[10:0\]
    bits (programmed in LTDC_AWCR register).*/
    #[inline(always)]
    pub fn wvstpos(&mut self) -> WVSTPOS_W<L2WVPCRrs> {
        WVSTPOS_W::new(self, 0)
    }
    /**Bits 16:26 - window vertical stop position These bits configure the last visible line of the layer window. WVSPPOS\[10:0\]
    must be ≥ AVBP\[10:0\]
    bits + 1 (programmed in LTDC_BPCR register).*/
    #[inline(always)]
    pub fn wvsppos(&mut self) -> WVSPPOS_W<L2WVPCRrs> {
        WVSPPOS_W::new(self, 16)
    }
}
/**LTDC layer 2 window vertical position configuration register

You can [`read`](crate::Reg::read) this register and get [`l2wvpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2wvpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:L2WVPCR)*/
pub struct L2WVPCRrs;
impl crate::RegisterSpec for L2WVPCRrs {
    type Ux = u32;
}
///`read()` method returns [`l2wvpcr::R`](R) reader structure
impl crate::Readable for L2WVPCRrs {}
///`write(|w| ..)` method takes [`l2wvpcr::W`](W) writer structure
impl crate::Writable for L2WVPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2WVPCR to value 0
impl crate::Resettable for L2WVPCRrs {
    const RESET_VALUE: u32 = 0;
}
