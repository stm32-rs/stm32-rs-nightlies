///Register `L2WVPCR` reader
pub type R = crate::R<L2WVPCRrs>;
///Register `L2WVPCR` writer
pub type W = crate::W<L2WVPCRrs>;
///Field `WVSTPOS` reader - WVSTPOS
pub type WVSTPOS_R = crate::FieldReader<u16>;
///Field `WVSTPOS` writer - WVSTPOS
pub type WVSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `WVSPPOS` reader - WVSPPOS
pub type WVSPPOS_R = crate::FieldReader<u16>;
///Field `WVSPPOS` writer - WVSPPOS
pub type WVSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - WVSTPOS
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - WVSPPOS
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
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
    ///Bits 0:11 - WVSTPOS
    #[inline(always)]
    pub fn wvstpos(&mut self) -> WVSTPOS_W<L2WVPCRrs> {
        WVSTPOS_W::new(self, 0)
    }
    ///Bits 16:27 - WVSPPOS
    #[inline(always)]
    pub fn wvsppos(&mut self) -> WVSPPOS_W<L2WVPCRrs> {
        WVSPPOS_W::new(self, 16)
    }
}
/**This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\[11:0\] bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\[11:0\] bits in the LTDC_AWCR register.

You can [`read`](crate::Reg::read) this register and get [`l2wvpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2wvpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L2WVPCR)*/
pub struct L2WVPCRrs;
impl crate::RegisterSpec for L2WVPCRrs {
    type Ux = u32;
}
///`read()` method returns [`l2wvpcr::R`](R) reader structure
impl crate::Readable for L2WVPCRrs {}
///`write(|w| ..)` method takes [`l2wvpcr::W`](W) writer structure
impl crate::Writable for L2WVPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2WVPCR to value 0
impl crate::Resettable for L2WVPCRrs {}
