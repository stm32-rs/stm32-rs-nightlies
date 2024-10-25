///Register `ABFSR` reader
pub type R = crate::R<ABFSRrs>;
///Register `ABFSR` writer
pub type W = crate::W<ABFSRrs>;
///Field `ITCM` reader - ITCM
pub type ITCM_R = crate::BitReader;
///Field `ITCM` writer - ITCM
pub type ITCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTCM` reader - DTCM
pub type DTCM_R = crate::BitReader;
///Field `DTCM` writer - DTCM
pub type DTCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBP` reader - AHBP
pub type AHBP_R = crate::BitReader;
///Field `AHBP` writer - AHBP
pub type AHBP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXIM` reader - AXIM
pub type AXIM_R = crate::BitReader;
///Field `AXIM` writer - AXIM
pub type AXIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPPB` reader - EPPB
pub type EPPB_R = crate::BitReader;
///Field `EPPB` writer - EPPB
pub type EPPB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXIMTYPE` reader - AXIMTYPE
pub type AXIMTYPE_R = crate::FieldReader;
///Field `AXIMTYPE` writer - AXIMTYPE
pub type AXIMTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - ITCM
    #[inline(always)]
    pub fn itcm(&self) -> ITCM_R {
        ITCM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DTCM
    #[inline(always)]
    pub fn dtcm(&self) -> DTCM_R {
        DTCM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHBP
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AXIM
    #[inline(always)]
    pub fn axim(&self) -> AXIM_R {
        AXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - EPPB
    #[inline(always)]
    pub fn eppb(&self) -> EPPB_R {
        EPPB_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:9 - AXIMTYPE
    #[inline(always)]
    pub fn aximtype(&self) -> AXIMTYPE_R {
        AXIMTYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ABFSR")
            .field("itcm", &self.itcm())
            .field("dtcm", &self.dtcm())
            .field("ahbp", &self.ahbp())
            .field("axim", &self.axim())
            .field("eppb", &self.eppb())
            .field("aximtype", &self.aximtype())
            .finish()
    }
}
impl W {
    ///Bit 0 - ITCM
    #[inline(always)]
    #[must_use]
    pub fn itcm(&mut self) -> ITCM_W<ABFSRrs> {
        ITCM_W::new(self, 0)
    }
    ///Bit 1 - DTCM
    #[inline(always)]
    #[must_use]
    pub fn dtcm(&mut self) -> DTCM_W<ABFSRrs> {
        DTCM_W::new(self, 1)
    }
    ///Bit 2 - AHBP
    #[inline(always)]
    #[must_use]
    pub fn ahbp(&mut self) -> AHBP_W<ABFSRrs> {
        AHBP_W::new(self, 2)
    }
    ///Bit 3 - AXIM
    #[inline(always)]
    #[must_use]
    pub fn axim(&mut self) -> AXIM_W<ABFSRrs> {
        AXIM_W::new(self, 3)
    }
    ///Bit 4 - EPPB
    #[inline(always)]
    #[must_use]
    pub fn eppb(&mut self) -> EPPB_W<ABFSRrs> {
        EPPB_W::new(self, 4)
    }
    ///Bits 8:9 - AXIMTYPE
    #[inline(always)]
    #[must_use]
    pub fn aximtype(&mut self) -> AXIMTYPE_W<ABFSRrs> {
        AXIMTYPE_W::new(self, 8)
    }
}
/**Auxiliary Bus Fault Status register

You can [`read`](crate::Reg::read) this register and get [`abfsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abfsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#AC:ABFSR)*/
pub struct ABFSRrs;
impl crate::RegisterSpec for ABFSRrs {
    type Ux = u32;
}
///`read()` method returns [`abfsr::R`](R) reader structure
impl crate::Readable for ABFSRrs {}
///`write(|w| ..)` method takes [`abfsr::W`](W) writer structure
impl crate::Writable for ABFSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ABFSR to value 0
impl crate::Resettable for ABFSRrs {
    const RESET_VALUE: u32 = 0;
}
