///Register `RDLSICR` reader
pub type R = crate::R<RDLSICRrs>;
///Register `RDLSICR` writer
pub type W = crate::W<RDLSICRrs>;
///Field `LSION` reader - LSION
pub type LSION_R = crate::BitReader;
///Field `LSION` writer - LSION
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSIRDY` reader - LSIRDY
pub type LSIRDY_R = crate::BitReader;
///Field `MRD` reader - MRD
pub type MRD_R = crate::FieldReader;
///Field `MRD` writer - MRD
pub type MRD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `EADLY` reader - EADLY
pub type EADLY_R = crate::FieldReader;
///Field `EADLY` writer - EADLY
pub type EADLY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SPARE` reader - SPARE
pub type SPARE_R = crate::FieldReader;
///Field `SPARE` writer - SPARE
pub type SPARE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - LSION
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSIRDY
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 16:20 - MRD
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:26 - EADLY
    #[inline(always)]
    pub fn eadly(&self) -> EADLY_R {
        EADLY_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:31 - SPARE
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDLSICR")
            .field("lsion", &self.lsion())
            .field("lsirdy", &self.lsirdy())
            .field("mrd", &self.mrd())
            .field("eadly", &self.eadly())
            .field("spare", &self.spare())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSION
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<'_, RDLSICRrs> {
        LSION_W::new(self, 0)
    }
    ///Bits 16:20 - MRD
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W<'_, RDLSICRrs> {
        MRD_W::new(self, 16)
    }
    ///Bits 24:26 - EADLY
    #[inline(always)]
    pub fn eadly(&mut self) -> EADLY_W<'_, RDLSICRrs> {
        EADLY_W::new(self, 24)
    }
    ///Bits 27:31 - SPARE
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W<'_, RDLSICRrs> {
        SPARE_W::new(self, 27)
    }
}
/**This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rdlsicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdlsicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RDLSICR)*/
pub struct RDLSICRrs;
impl crate::RegisterSpec for RDLSICRrs {
    type Ux = u32;
}
///`read()` method returns [`rdlsicr::R`](R) reader structure
impl crate::Readable for RDLSICRrs {}
///`write(|w| ..)` method takes [`rdlsicr::W`](W) writer structure
impl crate::Writable for RDLSICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RDLSICR to value 0
impl crate::Resettable for RDLSICRrs {}
