#[doc = "Register `SCAR_CUR` reader"]
pub type R = crate::R<SCAR_CURrs>;
#[doc = "Register `SCAR_CUR` writer"]
pub type W = crate::W<SCAR_CURrs>;
#[doc = "Field `SEC_AREA_START` reader - Bank 1 lowest secure protected address"]
pub type SEC_AREA_START_R = crate::FieldReader<u16>;
#[doc = "Field `SEC_AREA_START` writer - Bank 1 lowest secure protected address"]
pub type SEC_AREA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SEC_AREA_END` reader - Bank 1 highest secure protected address"]
pub type SEC_AREA_END_R = crate::FieldReader<u16>;
#[doc = "Field `SEC_AREA_END` writer - Bank 1 highest secure protected address"]
pub type SEC_AREA_END_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMES` reader - Bank 1 secure protected erase enable option status bit"]
pub type DMES_R = crate::BitReader;
#[doc = "Field `DMES` writer - Bank 1 secure protected erase enable option status bit"]
pub type DMES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Bank 1 lowest secure protected address"]
    #[inline(always)]
    pub fn sec_area_start(&self) -> SEC_AREA_START_R {
        SEC_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 1 highest secure protected address"]
    #[inline(always)]
    pub fn sec_area_end(&self) -> SEC_AREA_END_R {
        SEC_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 1 secure protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmes(&self) -> DMES_R {
        DMES_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bank 1 lowest secure protected address"]
    #[inline(always)]
    #[must_use]
    pub fn sec_area_start(&mut self) -> SEC_AREA_START_W<SCAR_CURrs> {
        SEC_AREA_START_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Bank 1 highest secure protected address"]
    #[inline(always)]
    #[must_use]
    pub fn sec_area_end(&mut self) -> SEC_AREA_END_W<SCAR_CURrs> {
        SEC_AREA_END_W::new(self, 16)
    }
    #[doc = "Bit 31 - Bank 1 secure protected erase enable option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmes(&mut self) -> DMES_W<SCAR_CURrs> {
        DMES_W::new(self, 31)
    }
}
#[doc = "FLASH secure address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scar_cur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scar_cur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCAR_CURrs;
impl crate::RegisterSpec for SCAR_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scar_cur::R`](R) reader structure"]
impl crate::Readable for SCAR_CURrs {}
#[doc = "`write(|w| ..)` method takes [`scar_cur::W`](W) writer structure"]
impl crate::Writable for SCAR_CURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCAR_CUR to value 0"]
impl crate::Resettable for SCAR_CURrs {
    const RESET_VALUE: u32 = 0;
}
