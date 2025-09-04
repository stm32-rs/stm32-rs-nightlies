///Register `PRAR_CUR` reader
pub type R = crate::R<PRAR_CURrs>;
///Register `PRAR_CUR` writer
pub type W = crate::W<PRAR_CURrs>;
///Field `PROT_AREA_START` reader - Bank 1 PCROP area start status bits
pub type PROT_AREA_START_R = crate::FieldReader<u16>;
///Field `PROT_AREA_START` writer - Bank 1 PCROP area start status bits
pub type PROT_AREA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `PROT_AREA_END` reader - Bank 1 PCROP area end status bits
pub type PROT_AREA_END_R = crate::FieldReader<u16>;
///Field `PROT_AREA_END` writer - Bank 1 PCROP area end status bits
pub type PROT_AREA_END_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `DMEP` reader - Bank 1 PCROP protected erase enable option status bit
pub type DMEP_R = crate::BitReader;
///Field `DMEP` writer - Bank 1 PCROP protected erase enable option status bit
pub type DMEP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Bank 1 PCROP area start status bits
    #[inline(always)]
    pub fn prot_area_start(&self) -> PROT_AREA_START_R {
        PROT_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Bank 1 PCROP area end status bits
    #[inline(always)]
    pub fn prot_area_end(&self) -> PROT_AREA_END_R {
        PROT_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Bank 1 PCROP protected erase enable option status bit
    #[inline(always)]
    pub fn dmep(&self) -> DMEP_R {
        DMEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRAR_CUR")
            .field("dmep", &self.dmep())
            .field("prot_area_end", &self.prot_area_end())
            .field("prot_area_start", &self.prot_area_start())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Bank 1 PCROP area start status bits
    #[inline(always)]
    pub fn prot_area_start(&mut self) -> PROT_AREA_START_W<PRAR_CURrs> {
        PROT_AREA_START_W::new(self, 0)
    }
    ///Bits 16:27 - Bank 1 PCROP area end status bits
    #[inline(always)]
    pub fn prot_area_end(&mut self) -> PROT_AREA_END_W<PRAR_CURrs> {
        PROT_AREA_END_W::new(self, 16)
    }
    ///Bit 31 - Bank 1 PCROP protected erase enable option status bit
    #[inline(always)]
    pub fn dmep(&mut self) -> DMEP_W<PRAR_CURrs> {
        DMEP_W::new(self, 31)
    }
}
/**FLASH protection address for bank 1

You can [`read`](crate::Reg::read) this register and get [`prar_cur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prar_cur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRAR_CURrs;
impl crate::RegisterSpec for PRAR_CURrs {
    type Ux = u32;
}
///`read()` method returns [`prar_cur::R`](R) reader structure
impl crate::Readable for PRAR_CURrs {}
///`write(|w| ..)` method takes [`prar_cur::W`](W) writer structure
impl crate::Writable for PRAR_CURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRAR_CUR to value 0
impl crate::Resettable for PRAR_CURrs {}
