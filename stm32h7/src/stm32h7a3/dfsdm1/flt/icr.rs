///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**Clear the injected conversion overrun flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRJOVRFW {
    ///1: Writing ‘1’ clears the JOVRF bit in the DFSDM_FLTxISR register
    Clear = 1,
}
impl From<CLRJOVRFW> for bool {
    #[inline(always)]
    fn from(variant: CLRJOVRFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CLRJOVRF` reader - Clear the injected conversion overrun flag
pub type CLRJOVRF_R = crate::BitReader<CLRJOVRFW>;
impl CLRJOVRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLRJOVRFW> {
        match self.bits {
            true => Some(CLRJOVRFW::Clear),
            _ => None,
        }
    }
    ///Writing ‘1’ clears the JOVRF bit in the DFSDM_FLTxISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CLRJOVRFW::Clear
    }
}
///Field `CLRJOVRF` writer - Clear the injected conversion overrun flag
pub type CLRJOVRF_W<'a, REG> = crate::BitWriter<'a, REG, CLRJOVRFW>;
impl<'a, REG> CLRJOVRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing ‘1’ clears the JOVRF bit in the DFSDM_FLTxISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLRJOVRFW::Clear)
    }
}
/**Clear the regular conversion overrun flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRROVRFW {
    ///1: Writing ‘1’ clears the ROVRF bit in the DFSDM_FLTxISR register
    Clear = 1,
}
impl From<CLRROVRFW> for bool {
    #[inline(always)]
    fn from(variant: CLRROVRFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CLRROVRF` reader - Clear the regular conversion overrun flag
pub type CLRROVRF_R = crate::BitReader<CLRROVRFW>;
impl CLRROVRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLRROVRFW> {
        match self.bits {
            true => Some(CLRROVRFW::Clear),
            _ => None,
        }
    }
    ///Writing ‘1’ clears the ROVRF bit in the DFSDM_FLTxISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CLRROVRFW::Clear
    }
}
///Field `CLRROVRF` writer - Clear the regular conversion overrun flag
pub type CLRROVRF_W<'a, REG> = crate::BitWriter<'a, REG, CLRROVRFW>;
impl<'a, REG> CLRROVRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Writing ‘1’ clears the ROVRF bit in the DFSDM_FLTxISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLRROVRFW::Clear)
    }
}
///Field `CLRCKABF` reader - Clear the clock absence flag CLRCKABF\[y\]=0: Writing '0' has no effect CLRCKABF\[y\]=1: Writing '1' to position y clears the corresponding CKABF\[y\] bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\[y\]. Note: CLRCKABF\[7:0\] is present only in DFSDM_FLT0ICR register (filter x=0)
pub type CLRCKABF_R = crate::FieldReader;
///Field `CLRCKABF` writer - Clear the clock absence flag CLRCKABF\[y\]=0: Writing '0' has no effect CLRCKABF\[y\]=1: Writing '1' to position y clears the corresponding CKABF\[y\] bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\[y\]. Note: CLRCKABF\[7:0\] is present only in DFSDM_FLT0ICR register (filter x=0)
pub type CLRCKABF_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `CLRSCDF` reader - Clear the short-circuit detector flag CLRSCDF\[y\]=0: Writing '0' has no effect CLRSCDF\[y\]=1: Writing '1' to position y clears the corresponding SCDF\[y\] bit in the DFSDM_FLTxISR register Note: CLRSCDF\[7:0\] is present only in DFSDM_FLT0ICR register (filter x=0)
pub type CLRSCDF_R = crate::FieldReader;
///Field `CLRSCDF` writer - Clear the short-circuit detector flag CLRSCDF\[y\]=0: Writing '0' has no effect CLRSCDF\[y\]=1: Writing '1' to position y clears the corresponding SCDF\[y\] bit in the DFSDM_FLTxISR register Note: CLRSCDF\[7:0\] is present only in DFSDM_FLT0ICR register (filter x=0)
pub type CLRSCDF_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
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
    ///Bits 16:23 - Clear the clock absence flag CLRCKABF\[y\]=0: Writing '0' has no effect CLRCKABF\[y\]=1: Writing '1' to position y clears the corresponding CKABF\[y\] bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\[y\]. Note: CLRCKABF\[7:0\] is present only in DFSDM_FLT0ICR register (filter x=0)
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Clear the short-circuit detector flag CLRSCDF\[y\]=0: Writing '0' has no effect CLRSCDF\[y\]=1: Writing '1' to position y clears the corresponding SCDF\[y\] bit in the DFSDM_FLTxISR register Note: CLRSCDF\[7:0\] is present only in DFSDM_FLT0ICR register (filter x=0)
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("clrjovrf", &self.clrjovrf())
            .field("clrrovrf", &self.clrrovrf())
            .field("clrckabf", &self.clrckabf())
            .field("clrscdf", &self.clrscdf())
            .finish()
    }
}
impl W {
    ///Bit 2 - Clear the injected conversion overrun flag
    #[inline(always)]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<'_, ICRrs> {
        CLRJOVRF_W::new(self, 2)
    }
    ///Bit 3 - Clear the regular conversion overrun flag
    #[inline(always)]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<'_, ICRrs> {
        CLRROVRF_W::new(self, 3)
    }
    ///Bits 16:23 - Clear the clock absence flag CLRCKABF\[y\]=0: Writing '0' has no effect CLRCKABF\[y\]=1: Writing '1' to position y clears the corresponding CKABF\[y\] bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\[y\]. Note: CLRCKABF\[7:0\] is present only in DFSDM_FLT0ICR register (filter x=0)
    #[inline(always)]
    pub fn clrckabf(&mut self) -> CLRCKABF_W<'_, ICRrs> {
        CLRCKABF_W::new(self, 16)
    }
    ///Bits 24:31 - Clear the short-circuit detector flag CLRSCDF\[y\]=0: Writing '0' has no effect CLRSCDF\[y\]=1: Writing '1' to position y clears the corresponding SCDF\[y\] bit in the DFSDM_FLTxISR register Note: CLRSCDF\[7:0\] is present only in DFSDM_FLT0ICR register (filter x=0)
    #[inline(always)]
    pub fn clrscdf(&mut self) -> CLRSCDF_W<'_, ICRrs> {
        CLRSCDF_W::new(self, 24)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
