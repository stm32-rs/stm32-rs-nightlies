///Register `SCAR_CUR` reader
pub type R = crate::R<SCAR_CURrs>;
///Register `SCAR_CUR` writer
pub type W = crate::W<SCAR_CURrs>;
///Field `SEC_AREA_START` reader - Bank 1 secure-only area start status bits
pub type SEC_AREA_START_R = crate::FieldReader<u16>;
///Field `SEC_AREA_START` writer - Bank 1 secure-only area start status bits
pub type SEC_AREA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `SEC_AREA_END` reader - Bank 1 secure-only area end status bits
pub type SEC_AREA_END_R = crate::FieldReader<u16>;
///Field `SEC_AREA_END` writer - Bank 1 secure-only area end status bits
pub type SEC_AREA_END_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `DMES` reader - Bank 1 secure access protected erase enable option status bit
pub type DMES_R = crate::BitReader;
///Field `DMES` writer - Bank 1 secure access protected erase enable option status bit
pub type DMES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - Bank 1 secure-only area start status bits
    #[inline(always)]
    pub fn sec_area_start(&self) -> SEC_AREA_START_R {
        SEC_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Bank 1 secure-only area end status bits
    #[inline(always)]
    pub fn sec_area_end(&self) -> SEC_AREA_END_R {
        SEC_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Bank 1 secure access protected erase enable option status bit
    #[inline(always)]
    pub fn dmes(&self) -> DMES_R {
        DMES_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCAR_CUR")
            .field("dmes", &self.dmes())
            .field("sec_area_end", &self.sec_area_end())
            .field("sec_area_start", &self.sec_area_start())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - Bank 1 secure-only area start status bits
    #[inline(always)]
    pub fn sec_area_start(&mut self) -> SEC_AREA_START_W<'_, SCAR_CURrs> {
        SEC_AREA_START_W::new(self, 0)
    }
    ///Bits 16:27 - Bank 1 secure-only area end status bits
    #[inline(always)]
    pub fn sec_area_end(&mut self) -> SEC_AREA_END_W<'_, SCAR_CURrs> {
        SEC_AREA_END_W::new(self, 16)
    }
    ///Bit 31 - Bank 1 secure access protected erase enable option status bit
    #[inline(always)]
    pub fn dmes(&mut self) -> DMES_W<'_, SCAR_CURrs> {
        DMES_W::new(self, 31)
    }
}
/**FLASH secure address for bank 1

You can [`read`](crate::Reg::read) this register and get [`scar_cur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_cur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCAR_CURrs;
impl crate::RegisterSpec for SCAR_CURrs {
    type Ux = u32;
}
///`read()` method returns [`scar_cur::R`](R) reader structure
impl crate::Readable for SCAR_CURrs {}
///`write(|w| ..)` method takes [`scar_cur::W`](W) writer structure
impl crate::Writable for SCAR_CURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCAR_CUR to value 0
impl crate::Resettable for SCAR_CURrs {}
