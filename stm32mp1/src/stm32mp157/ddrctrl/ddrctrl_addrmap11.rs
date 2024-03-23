#[doc = "Register `DDRCTRL_ADDRMAP11` reader"]
pub type R = crate::R<DDRCTRL_ADDRMAP11rs>;
#[doc = "Register `DDRCTRL_ADDRMAP11` writer"]
pub type W = crate::W<DDRCTRL_ADDRMAP11rs>;
#[doc = "Field `ADDRMAP_ROW_B10` reader - ADDRMAP_ROW_B10"]
pub type ADDRMAP_ROW_B10_R = crate::FieldReader;
#[doc = "Field `ADDRMAP_ROW_B10` writer - ADDRMAP_ROW_B10"]
pub type ADDRMAP_ROW_B10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B10"]
    #[inline(always)]
    pub fn addrmap_row_b10(&self) -> ADDRMAP_ROW_B10_R {
        ADDRMAP_ROW_B10_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B10"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b10(&mut self) -> ADDRMAP_ROW_B10_W<DDRCTRL_ADDRMAP11rs> {
        ADDRMAP_ROW_B10_W::new(self, 0)
    }
}
#[doc = "DDRCTRL address map register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ADDRMAP11rs;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP11rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_addrmap11::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP11rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_addrmap11::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP11rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP11 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP11rs {
    const RESET_VALUE: u32 = 0;
}
