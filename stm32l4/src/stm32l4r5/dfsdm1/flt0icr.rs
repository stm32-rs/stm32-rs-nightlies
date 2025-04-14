///Register `FLT0ICR` reader
pub type R = crate::R<FLT0ICRrs>;
///Register `FLT0ICR` writer
pub type W = crate::W<FLT0ICRrs>;
///Field `CLRJOVRF` reader - Clear the injected conversion overrun flag
pub type CLRJOVRF_R = crate::BitReader;
///Field `CLRJOVRF` writer - Clear the injected conversion overrun flag
pub type CLRJOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLRROVRF` reader - Clear the regular conversion overrun flag
pub type CLRROVRF_R = crate::BitReader;
///Field `CLRROVRF` writer - Clear the regular conversion overrun flag
pub type CLRROVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLRCKABF` reader - Clear the clock absence flag
pub type CLRCKABF_R = crate::FieldReader;
///Field `CLRCKABF` writer - Clear the clock absence flag
pub type CLRCKABF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLRSCDF` reader - Clear the short-circuit detector flag
pub type CLRSCDF_R = crate::FieldReader;
///Field `CLRSCDF` writer - Clear the short-circuit detector flag
pub type CLRSCDF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 2 - Clear the injected conversion overrun flag
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear the regular conversion overrun flag
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 16:23 - Clear the clock absence flag
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Clear the short-circuit detector flag
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT0ICR")
            .field("clrscdf", &self.clrscdf())
            .field("clrckabf", &self.clrckabf())
            .field("clrrovrf", &self.clrrovrf())
            .field("clrjovrf", &self.clrjovrf())
            .finish()
    }
}
impl W {
    ///Bit 2 - Clear the injected conversion overrun flag
    #[inline(always)]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<FLT0ICRrs> {
        CLRJOVRF_W::new(self, 2)
    }
    ///Bit 3 - Clear the regular conversion overrun flag
    #[inline(always)]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<FLT0ICRrs> {
        CLRROVRF_W::new(self, 3)
    }
    ///Bits 16:23 - Clear the clock absence flag
    #[inline(always)]
    pub fn clrckabf(&mut self) -> CLRCKABF_W<FLT0ICRrs> {
        CLRCKABF_W::new(self, 16)
    }
    ///Bits 24:31 - Clear the short-circuit detector flag
    #[inline(always)]
    pub fn clrscdf(&mut self) -> CLRSCDF_W<FLT0ICRrs> {
        CLRSCDF_W::new(self, 24)
    }
}
/**interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`flt0icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt0icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#DFSDM1:FLT0ICR)*/
pub struct FLT0ICRrs;
impl crate::RegisterSpec for FLT0ICRrs {
    type Ux = u32;
}
///`read()` method returns [`flt0icr::R`](R) reader structure
impl crate::Readable for FLT0ICRrs {}
///`write(|w| ..)` method takes [`flt0icr::W`](W) writer structure
impl crate::Writable for FLT0ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT0ICR to value 0
impl crate::Resettable for FLT0ICRrs {}
