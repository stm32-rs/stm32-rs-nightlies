///Register `L1WVPCR` reader
pub type R = crate::R<L1WVPCRrs>;
///Register `L1WVPCR` writer
pub type W = crate::W<L1WVPCRrs>;
///Field `WVSTPOS` reader - window vertical start position
pub type WVSTPOS_R = crate::FieldReader<u16>;
///Field `WVSTPOS` writer - window vertical start position
pub type WVSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `WVSPPOS` reader - window vertical stop position
pub type WVSPPOS_R = crate::FieldReader<u16>;
///Field `WVSPPOS` writer - window vertical stop position
pub type WVSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - window vertical start position
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - window vertical stop position
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1WVPCR")
            .field("wvstpos", &self.wvstpos())
            .field("wvsppos", &self.wvsppos())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - window vertical start position
    #[inline(always)]
    pub fn wvstpos(&mut self) -> WVSTPOS_W<'_, L1WVPCRrs> {
        WVSTPOS_W::new(self, 0)
    }
    ///Bits 16:31 - window vertical stop position
    #[inline(always)]
    pub fn wvsppos(&mut self) -> WVSPPOS_W<'_, L1WVPCRrs> {
        WVSPPOS_W::new(self, 16)
    }
}
/**LTDC layerx window vertical position configuration register

You can [`read`](crate::Reg::read) this register and get [`l1wvpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1wvpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1WVPCR)*/
pub struct L1WVPCRrs;
impl crate::RegisterSpec for L1WVPCRrs {
    type Ux = u32;
}
///`read()` method returns [`l1wvpcr::R`](R) reader structure
impl crate::Readable for L1WVPCRrs {}
///`write(|w| ..)` method takes [`l1wvpcr::W`](W) writer structure
impl crate::Writable for L1WVPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1WVPCR to value 0
impl crate::Resettable for L1WVPCRrs {}
