///Register `FLT4ICR` reader
pub type R = crate::R<FLT4ICRrs>;
///Register `FLT4ICR` writer
pub type W = crate::W<FLT4ICRrs>;
///Field `CLRJOVRF` reader - CLRJOVRF
pub type CLRJOVRF_R = crate::BitReader;
///Field `CLRJOVRF` writer - CLRJOVRF
pub type CLRJOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLRROVRF` reader - CLRROVRF
pub type CLRROVRF_R = crate::BitReader;
///Field `CLRROVRF` writer - CLRROVRF
pub type CLRROVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLRCKABF` reader - CLRCKABF
pub type CLRCKABF_R = crate::FieldReader;
///Field `CLRCKABF` writer - CLRCKABF
pub type CLRCKABF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLRSCDF` reader - CLRSCDF
pub type CLRSCDF_R = crate::FieldReader;
///Field `CLRSCDF` writer - CLRSCDF
pub type CLRSCDF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 2 - CLRJOVRF
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CLRROVRF
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 16:23 - CLRCKABF
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - CLRSCDF
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT4ICR")
            .field("clrjovrf", &self.clrjovrf())
            .field("clrrovrf", &self.clrrovrf())
            .field("clrckabf", &self.clrckabf())
            .field("clrscdf", &self.clrscdf())
            .finish()
    }
}
impl W {
    ///Bit 2 - CLRJOVRF
    #[inline(always)]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<FLT4ICRrs> {
        CLRJOVRF_W::new(self, 2)
    }
    ///Bit 3 - CLRROVRF
    #[inline(always)]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<FLT4ICRrs> {
        CLRROVRF_W::new(self, 3)
    }
    ///Bits 16:23 - CLRCKABF
    #[inline(always)]
    pub fn clrckabf(&mut self) -> CLRCKABF_W<FLT4ICRrs> {
        CLRCKABF_W::new(self, 16)
    }
    ///Bits 24:31 - CLRSCDF
    #[inline(always)]
    pub fn clrscdf(&mut self) -> CLRSCDF_W<FLT4ICRrs> {
        CLRSCDF_W::new(self, 24)
    }
}
/**DFSDM filter 4 interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`flt4icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt4icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:FLT4ICR)*/
pub struct FLT4ICRrs;
impl crate::RegisterSpec for FLT4ICRrs {
    type Ux = u32;
}
///`read()` method returns [`flt4icr::R`](R) reader structure
impl crate::Readable for FLT4ICRrs {}
///`write(|w| ..)` method takes [`flt4icr::W`](W) writer structure
impl crate::Writable for FLT4ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLT4ICR to value 0
impl crate::Resettable for FLT4ICRrs {}
