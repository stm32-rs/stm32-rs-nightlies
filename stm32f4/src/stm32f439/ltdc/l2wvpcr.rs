///Register `L2WVPCR` reader
pub type R = crate::R<L2WVPCRrs>;
///Register `L2WVPCR` writer
pub type W = crate::W<L2WVPCRrs>;
///Field `WVSTPOS` reader - Window Vertical Start Position
pub type WVSTPOS_R = crate::FieldReader<u16>;
///Field `WVSTPOS` writer - Window Vertical Start Position
pub type WVSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `WVSPPOS` reader - Window Vertical Stop Position
pub type WVSPPOS_R = crate::FieldReader<u16>;
///Field `WVSPPOS` writer - Window Vertical Stop Position
pub type WVSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - Window Vertical Start Position
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Window Vertical Stop Position
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2WVPCR")
            .field("wvsppos", &self.wvsppos())
            .field("wvstpos", &self.wvstpos())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Window Vertical Start Position
    #[inline(always)]
    pub fn wvstpos(&mut self) -> WVSTPOS_W<'_, L2WVPCRrs> {
        WVSTPOS_W::new(self, 0)
    }
    ///Bits 16:26 - Window Vertical Stop Position
    #[inline(always)]
    pub fn wvsppos(&mut self) -> WVSPPOS_W<'_, L2WVPCRrs> {
        WVSPPOS_W::new(self, 16)
    }
}
/**Layerx Window Vertical Position Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l2wvpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2wvpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#LTDC:L2WVPCR)*/
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
