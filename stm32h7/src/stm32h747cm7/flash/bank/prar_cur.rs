#[doc = "Register `PRAR_CUR` reader"]
pub type R = crate::R<PRAR_CURrs>;
#[doc = "Register `PRAR_CUR` writer"]
pub type W = crate::W<PRAR_CURrs>;
#[doc = "Field `PROT_AREA_START` reader - Bank 1 PCROP area start status bits"]
pub type PROT_AREA_START_R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_START` writer - Bank 1 PCROP area start status bits"]
pub type PROT_AREA_START_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PROT_AREA_END` reader - Bank 1 PCROP area end status bits"]
pub type PROT_AREA_END_R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_END` writer - Bank 1 PCROP area end status bits"]
pub type PROT_AREA_END_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMEP` reader - Bank 1 PCROP protected erase enable option status bit"]
pub type DMEP_R = crate::BitReader;
#[doc = "Field `DMEP` writer - Bank 1 PCROP protected erase enable option status bit"]
pub type DMEP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Bank 1 PCROP area start status bits"]
    #[inline(always)]
    pub fn prot_area_start(&self) -> PROT_AREA_START_R {
        PROT_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 1 PCROP area end status bits"]
    #[inline(always)]
    pub fn prot_area_end(&self) -> PROT_AREA_END_R {
        PROT_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmep(&self) -> DMEP_R {
        DMEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bank 1 PCROP area start status bits"]
    #[inline(always)]
    #[must_use]
    pub fn prot_area_start(&mut self) -> PROT_AREA_START_W<PRAR_CURrs> {
        PROT_AREA_START_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Bank 1 PCROP area end status bits"]
    #[inline(always)]
    #[must_use]
    pub fn prot_area_end(&mut self) -> PROT_AREA_END_W<PRAR_CURrs> {
        PROT_AREA_END_W::new(self, 16)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmep(&mut self) -> DMEP_W<PRAR_CURrs> {
        DMEP_W::new(self, 31)
    }
}
#[doc = "FLASH protection address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prar_cur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prar_cur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRAR_CURrs;
impl crate::RegisterSpec for PRAR_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prar_cur::R`](R) reader structure"]
impl crate::Readable for PRAR_CURrs {}
#[doc = "`write(|w| ..)` method takes [`prar_cur::W`](W) writer structure"]
impl crate::Writable for PRAR_CURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRAR_CUR to value 0"]
impl crate::Resettable for PRAR_CURrs {
    const RESET_VALUE: u32 = 0;
}
