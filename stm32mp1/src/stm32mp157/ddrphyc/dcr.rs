///Register `DCR` reader
pub type R = crate::R<DCRrs>;
///Register `DCR` writer
pub type W = crate::W<DCRrs>;
///Field `DDRMD` reader - DDRMD
pub type DDRMD_R = crate::FieldReader;
///Field `DDRMD` writer - DDRMD
pub type DDRMD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DDR8BNK` reader - DDR8BNK
pub type DDR8BNK_R = crate::BitReader;
///Field `DDR8BNK` writer - DDR8BNK
pub type DDR8BNK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDQ` reader - PDQ
pub type PDQ_R = crate::FieldReader;
///Field `PDQ` writer - PDQ
pub type PDQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MPRDQ` reader - MPRDQ
pub type MPRDQ_R = crate::BitReader;
///Field `MPRDQ` writer - MPRDQ
pub type MPRDQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRTYPE` reader - DDRTYPE
pub type DDRTYPE_R = crate::FieldReader;
///Field `DDRTYPE` writer - DDRTYPE
pub type DDRTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NOSRA` reader - NOSRA
pub type NOSRA_R = crate::BitReader;
///Field `NOSRA` writer - NOSRA
pub type NOSRA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDR2T` reader - DDR2T
pub type DDR2T_R = crate::BitReader;
///Field `DDR2T` writer - DDR2T
pub type DDR2T_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDIMM` reader - UDIMM
pub type UDIMM_R = crate::BitReader;
///Field `UDIMM` writer - UDIMM
pub type UDIMM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDIMM` reader - RDIMM
pub type RDIMM_R = crate::BitReader;
///Field `RDIMM` writer - RDIMM
pub type RDIMM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TPD` reader - TPD
pub type TPD_R = crate::BitReader;
///Field `TPD` writer - TPD
pub type TPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - DDRMD
    #[inline(always)]
    pub fn ddrmd(&self) -> DDRMD_R {
        DDRMD_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - DDR8BNK
    #[inline(always)]
    pub fn ddr8bnk(&self) -> DDR8BNK_R {
        DDR8BNK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - PDQ
    #[inline(always)]
    pub fn pdq(&self) -> PDQ_R {
        PDQ_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - MPRDQ
    #[inline(always)]
    pub fn mprdq(&self) -> MPRDQ_R {
        MPRDQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - DDRTYPE
    #[inline(always)]
    pub fn ddrtype(&self) -> DDRTYPE_R {
        DDRTYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 27 - NOSRA
    #[inline(always)]
    pub fn nosra(&self) -> NOSRA_R {
        NOSRA_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - DDR2T
    #[inline(always)]
    pub fn ddr2t(&self) -> DDR2T_R {
        DDR2T_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - UDIMM
    #[inline(always)]
    pub fn udimm(&self) -> UDIMM_R {
        UDIMM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - RDIMM
    #[inline(always)]
    pub fn rdimm(&self) -> RDIMM_R {
        RDIMM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - TPD
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR")
            .field("ddrmd", &self.ddrmd())
            .field("ddr8bnk", &self.ddr8bnk())
            .field("pdq", &self.pdq())
            .field("mprdq", &self.mprdq())
            .field("ddrtype", &self.ddrtype())
            .field("nosra", &self.nosra())
            .field("ddr2t", &self.ddr2t())
            .field("udimm", &self.udimm())
            .field("rdimm", &self.rdimm())
            .field("tpd", &self.tpd())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - DDRMD
    #[inline(always)]
    pub fn ddrmd(&mut self) -> DDRMD_W<'_, DCRrs> {
        DDRMD_W::new(self, 0)
    }
    ///Bit 3 - DDR8BNK
    #[inline(always)]
    pub fn ddr8bnk(&mut self) -> DDR8BNK_W<'_, DCRrs> {
        DDR8BNK_W::new(self, 3)
    }
    ///Bits 4:6 - PDQ
    #[inline(always)]
    pub fn pdq(&mut self) -> PDQ_W<'_, DCRrs> {
        PDQ_W::new(self, 4)
    }
    ///Bit 7 - MPRDQ
    #[inline(always)]
    pub fn mprdq(&mut self) -> MPRDQ_W<'_, DCRrs> {
        MPRDQ_W::new(self, 7)
    }
    ///Bits 8:9 - DDRTYPE
    #[inline(always)]
    pub fn ddrtype(&mut self) -> DDRTYPE_W<'_, DCRrs> {
        DDRTYPE_W::new(self, 8)
    }
    ///Bit 27 - NOSRA
    #[inline(always)]
    pub fn nosra(&mut self) -> NOSRA_W<'_, DCRrs> {
        NOSRA_W::new(self, 27)
    }
    ///Bit 28 - DDR2T
    #[inline(always)]
    pub fn ddr2t(&mut self) -> DDR2T_W<'_, DCRrs> {
        DDR2T_W::new(self, 28)
    }
    ///Bit 29 - UDIMM
    #[inline(always)]
    pub fn udimm(&mut self) -> UDIMM_W<'_, DCRrs> {
        UDIMM_W::new(self, 29)
    }
    ///Bit 30 - RDIMM
    #[inline(always)]
    pub fn rdimm(&mut self) -> RDIMM_W<'_, DCRrs> {
        RDIMM_W::new(self, 30)
    }
    ///Bit 31 - TPD
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W<'_, DCRrs> {
        TPD_W::new(self, 31)
    }
}
/**DDRPHYC DC register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DCR)*/
pub struct DCRrs;
impl crate::RegisterSpec for DCRrs {
    type Ux = u32;
}
///`read()` method returns [`dcr::R`](R) reader structure
impl crate::Readable for DCRrs {}
///`write(|w| ..)` method takes [`dcr::W`](W) writer structure
impl crate::Writable for DCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCR to value 0x0b
impl crate::Resettable for DCRrs {
    const RESET_VALUE: u32 = 0x0b;
}
