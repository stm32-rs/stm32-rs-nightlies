///Register `PRAR_CUR` reader
pub type R = crate::R<PRAR_CURrs>;
///Field `PROT_AREA_START` reader - Bank 1 lowest PCROP protected address
pub type PROT_AREA_START_R = crate::FieldReader<u16>;
///Field `PROT_AREA_END` reader - Bank 1 highest PCROP protected address
pub type PROT_AREA_END_R = crate::FieldReader<u16>;
///Field `DMEP` reader - Bank 1 PCROP protected erase enable option status bit
pub type DMEP_R = crate::BitReader;
impl R {
    ///Bits 0:11 - Bank 1 lowest PCROP protected address
    #[inline(always)]
    pub fn prot_area_start(&self) -> PROT_AREA_START_R {
        PROT_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Bank 1 highest PCROP protected address
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
            .field("prot_area_start", &self.prot_area_start())
            .field("prot_area_end", &self.prot_area_end())
            .field("dmep", &self.dmep())
            .finish()
    }
}
/**FLASH protection address for bank 1

You can [`read`](crate::Reg::read) this register and get [`prar_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRAR_CURrs;
impl crate::RegisterSpec for PRAR_CURrs {
    type Ux = u32;
}
///`read()` method returns [`prar_cur::R`](R) reader structure
impl crate::Readable for PRAR_CURrs {}
///`reset()` method sets PRAR_CUR to value 0
impl crate::Resettable for PRAR_CURrs {}
