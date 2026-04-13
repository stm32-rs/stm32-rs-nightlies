///Register `SCAR_CUR` reader
pub type R = crate::R<SCAR_CURrs>;
///Register `SCAR_CUR` writer
pub type W = crate::W<SCAR_CURrs>;
///Field `SEC_AREA_START` reader - lowest secure protected address
pub type SEC_AREA_START_R = crate::FieldReader<u16>;
///Field `SEC_AREA_START` writer - lowest secure protected address
pub type SEC_AREA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `SEC_AREA_END` reader - highest secure protected address
pub type SEC_AREA_END_R = crate::FieldReader<u16>;
///Field `SEC_AREA_END` writer - highest secure protected address
pub type SEC_AREA_END_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `DMES` reader - secure protected erase enable option status bit
pub type DMES_R = crate::BitReader;
///Field `DMES` writer - secure protected erase enable option status bit
pub type DMES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - lowest secure protected address
    #[inline(always)]
    pub fn sec_area_start(&self) -> SEC_AREA_START_R {
        SEC_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - highest secure protected address
    #[inline(always)]
    pub fn sec_area_end(&self) -> SEC_AREA_END_R {
        SEC_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - secure protected erase enable option status bit
    #[inline(always)]
    pub fn dmes(&self) -> DMES_R {
        DMES_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCAR_CUR")
            .field("sec_area_start", &self.sec_area_start())
            .field("sec_area_end", &self.sec_area_end())
            .field("dmes", &self.dmes())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - lowest secure protected address
    #[inline(always)]
    pub fn sec_area_start(&mut self) -> SEC_AREA_START_W<'_, SCAR_CURrs> {
        SEC_AREA_START_W::new(self, 0)
    }
    ///Bits 16:27 - highest secure protected address
    #[inline(always)]
    pub fn sec_area_end(&mut self) -> SEC_AREA_END_W<'_, SCAR_CURrs> {
        SEC_AREA_END_W::new(self, 16)
    }
    ///Bit 31 - secure protected erase enable option status bit
    #[inline(always)]
    pub fn dmes(&mut self) -> DMES_W<'_, SCAR_CURrs> {
        DMES_W::new(self, 31)
    }
}
/**FLASH secure address for bank 1

You can [`read`](crate::Reg::read) this register and get [`scar_cur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_cur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#FLASH:SCAR_CUR)*/
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
